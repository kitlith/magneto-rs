use std::marker::PhantomData;

#[derive(Default)]
pub struct Term<const D: usize, T>(PhantomData<T>);

macro_rules! degrees {
    ($lit:literal $(,)?) => {
        Term<$lit, ()>
    };
    ($lit:literal, $($tail:tt)*) => {
        Term<$lit, degrees!($($tail)*)>
    };
    (..$ty:ty) => { $ty }
}

pub trait DegreeItem {
    const DEGREE: usize;
    type TAIL: DegreeItem;
    const SUMMED_POWERS: usize;
    const COEFFICIENTS: usize;
    const DIMENSIONS: usize;
}

impl<const D: usize, T: DegreeItem> DegreeItem for Term<D, T> {
    const DEGREE: usize = D;
    type TAIL = T;
    const SUMMED_POWERS: usize = (D * 2 + 1) * T::SUMMED_POWERS;
    const COEFFICIENTS: usize = (D + 1) * T::COEFFICIENTS;
    const DIMENSIONS: usize = 1 + T::DIMENSIONS;
}

impl DegreeItem for () {
    const DEGREE: usize = 0;
    type TAIL = ();
    const SUMMED_POWERS: usize = 1;
    const COEFFICIENTS: usize = 1;
    const DIMENSIONS: usize = 0;
}

const fn index_degree<T: DegreeItem>(idx: usize) -> Option<usize> {
    if idx > T::DIMENSIONS {
        None
    } else if idx == 0 {
        Some(T::DEGREE)
    } else {
        index_degree::<T::TAIL>(idx - 1)
    }
}

impl<const D: usize, T> Term<D, T> where Self: DegreeItem {
    pub const fn index(idx: usize) -> Option<usize> {
        index_degree::<Self>(idx)
    }
}

pub type A = degrees!(1, 2, 3, 4,);

pub const TEST: usize = A::SUMMED_POWERS;