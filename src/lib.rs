use nalgebra::{Vector3, U10, matrix, Matrix, Matrix4x3};
use nalgebra_lapack::Eigen;

pub mod transpose;
//pub mod cxx;
//pub mod expand;

use transpose::SelfTranspose;

trait Magneto {
    fn sample(&mut self, sample: Vector3<f64>);
    fn finalize(self) -> Matrix4x3<f64>;
}

// TODO: allow for options other than f64?
#[derive(Clone)]
pub struct CalibrationState {
    sample_count: usize,
    sum: f64,
    ata: SelfTranspose<f64, U10>
}

impl CalibrationState {
    pub fn new() -> Self {
        CalibrationState { sample_count: 0, sum: 0.0, ata: SelfTranspose::new() }
    }

    pub fn sample(&mut self, sample: Vector3<f64>) {
        self.sample_count += 1;
        self.sum += sample.norm();

        // TODO: filter outliers

        self.ata.feed_row(&matrix![
            sample.x * sample.x,
            sample.y * sample.y,
            sample.z * sample.z,
            sample.y * sample.z * 2.0,
            sample.x * sample.z * 2.0,
            sample.x * sample.y * 2.0,
            sample.x * 2.0,
            sample.y * 2.0,
            sample.z * 2.0,
            1.0
        ]);
    }

    // TODO: strictly speaking we probably don't need to consume the state,
    //  which would allow for incremental/continuous refinement of the calibration.
    #[allow(non_snake_case)]
    pub fn output(self) -> Matrix4x3<f64> {
        // aside: could we extract these from the matrix itself?
        let avg_norm = self.sum / self.sample_count as f64;
        dbg!(avg_norm);

        let hm = avg_norm;

        let S = self.ata.mat;
        let S11 = S.fixed_slice::<6, 6>(0, 0);
        let S12 = S.fixed_slice::<6, 4>(0, 6);
        let S12t = S.fixed_slice::<4, 6>(6, 0);
        let S22 = S.fixed_slice::<4, 4>(6, 6);

        
        let S22 = S22.lu().try_inverse().unwrap();

        // "Calculate S22a = S22 * S12t   4*6 = 4x4 * 4x6   C = AB"
        let S22a = S22 * S12t;
        // "Then calculate S22b = S12 * S22a      ( 6x6 = 6x4 * 4x6)"
        let S22b = S12 * S22a;
        // "Calculate SS = S11 - S22b"
        let SS = S11 - S22b;        

        // "Create pre-inverted constraint matrix C"
        let C = matrix![
            0.0, 0.5, 0.5,  0.0,  0.0,  0.0;
            0.5, 0.0, 0.5,  0.0,  0.0,  0.0;
            0.5, 0.5, 0.0,  0.0,  0.0,  0.0;
            0.0, 0.0, 0.0, -0.25, 0.0,  0.0;
            0.0, 0.0, 0.0,  0.0, -0.25, 0.0;
            0.0, 0.0, 0.0,  0.0,  0.0, -0.25
        ];
        let E = C * SS;

        let gen_eigen = Eigen::new(E, true, true).unwrap();

        let mut maxval = gen_eigen.eigenvalues[0];
        let mut index = 0;
        for iii in 1..6 {
            let b = gen_eigen.eigenvalues[iii];
            if b > maxval {
                maxval = b;
                index = iii;
            }
        }

        let mut v1 = gen_eigen.eigenvectors.unwrap().column(index).normalize();
        if v1[0] < 0.0 {
            //v1 = -v1;
            v1.neg_mut();
        }

        // "Calculate v2 = S22a * v1      ( 4x1 = 4x6 * 6x1)"
        let mut v2 = S22a * v1;
        v2.neg_mut();

        let Q = matrix![
            v1[0], v1[5], v1[4];
            v1[5], v1[1], v1[3];
            v1[4], v1[3], v1[2];
        ];
        let U = v2.fixed_rows::<3>(0);
        let J = v2[3];

        let mut B = Q.lu().try_inverse().unwrap() * U;
        B.neg_mut();

        // TODO: is there a builtin for A^T * B * A?
        let btqb = B.tr_mul(&(Q * B))[0];

        // "Calculate SQ, the square root of matrix Q"
        let mut sym_eigen = Q.symmetric_eigen();
        //Matrix::from_diagonal(&sym_eigen.eigenvalues.map(|v| v.sqrt()));
        for val in sym_eigen.eigenvalues.iter_mut() {
            *val = val.sqrt();
        }
        let Dz = Matrix::from_diagonal(&sym_eigen.eigenvalues);

        for mut col in sym_eigen.eigenvectors.column_iter_mut() {
            col.normalize_mut();
        }

        let SQ = (sym_eigen.eigenvectors * Dz) * sym_eigen.eigenvectors.transpose();

        let hmb = (btqb - J).sqrt();

        let A_1 = SQ * (hm / hmb);

        let mut BAinv = A_1.insert_row(0, 0.0);
        BAinv.set_row(0, &B.transpose());

        BAinv//.transpose()
    }
}