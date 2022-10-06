use nalgebra::{
    dimension::DimName,
    core::{OMatrix, Vector, RowVector, Storage},
    allocator::Allocator,
    DefaultAllocator,
    Scalar,
    U1,
    Dim, ClosedAdd, ClosedMul, IsContiguous, SliceStorage,
};

use num_traits::{Zero, One};

#[derive(Clone)]
pub struct SelfTranspose<T, D> where D: Dim, DefaultAllocator: Allocator<T, D, D> {
    pub mat: OMatrix<T, D, D>
}

type RowVectorSlice<'a, T, D, RStride = D, CStride = U1> = RowVector<T, D, SliceStorage<'a, T, U1, D, RStride, CStride>>;

impl<T, D> SelfTranspose<T, D> where D: Dim, DefaultAllocator: Allocator<T, D, D> {
    pub fn new_generic(dim: D) -> Self where T: Zero + Scalar {
        SelfTranspose {
            mat: OMatrix::zeros_generic(dim, dim)
        }
    }

    pub fn new() -> Self where T: Zero + Scalar, D: DimName {
        SelfTranspose { mat: OMatrix::zeros_generic(D::name(), D::name()) }
    }

    // TODO: remove IsContiguous reqirement
    pub fn feed_col<S>(&mut self, data: &Vector<T, D, S>) where 
        S: Storage<T, D> + IsContiguous,
        T: Scalar + ClosedAdd + ClosedMul + Zero + One,
        DefaultAllocator: Allocator<T, U1, D>,
    {
        // transpose vector without copying
        let transpose = RowVectorSlice::<T, D>::from_slice(data.as_slice());
        // let transpose = data.transpose();

        // self.mat += data * transpose;
        // self.mat = 1 * data * transpose + 1 * self.mat
        // self.mat.gemm(T::one(), data, &transpose, T::one());
        self.feed_row(&transpose)
    }

    pub fn feed_row<S>(&mut self, data: &RowVector<T, D, S>) where
        S: Storage<T, U1, D>,
        T: Scalar + ClosedAdd + ClosedMul + Zero + One
    {
        // self.mat += data.tr_mul(data);
        // self.mat += data.transpose() * data;
        // self.mat = 1 * data.transpose() * data + 1 * self.mat;
        self.mat.gemm_tr(T::one(), data, data, T::one());
    }

    // pub fn finalize(self) -> OMatrix<T, D, D> {
    //     self.mat
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::{Dynamic, OMatrix, RowOVector, U2};

    struct NaiveSelfTranspose<T> where T: Scalar {
        mat: Vec<RowOVector<T, U2>>
    }

    impl<T: Scalar + ClosedAdd + ClosedMul + Zero + One> /*TransposeTest<T> for*/ NaiveSelfTranspose<T> {
        fn feed_row<S>(&mut self, data: &RowVector<T, U2, S>) where S: Storage<T, U1, U2> {
            self.mat.push(data.clone_owned());
        }

        fn finalize(self) -> OMatrix<T, U2, U2> {
            let data_mat: OMatrix<_, Dynamic, _> = OMatrix::from_rows(&self.mat[..]);
            data_mat.tr_mul(&data_mat)
        }
    }

    use rand::prelude::*;

    #[test]
    fn test() {
        let mut r = rand::thread_rng();
        for _ in 0..100000 {
            let mut mock = NaiveSelfTranspose::<f64> { mat: Vec::new() };
            let mut tested = SelfTranspose::<f64, U2>::new();
            for _ in 0..100 {
                let row = RowOVector::<f64, U2>::new(r.gen(), r.gen());
                mock.feed_row(&row);
                tested.feed_row(&row);
            }
            //assert_eq!(mock.finalize(), tested.mat);
            approx::assert_ulps_eq!(mock.finalize(), tested.mat, max_ulps = 10)
        }
    }
}