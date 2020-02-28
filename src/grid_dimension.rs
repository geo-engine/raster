use super::{Ix, Ix2, Ix3};

use std::fmt::Debug;
//use std::ops::{};
use std::ops::{Index, IndexMut};
pub trait GridDimension:
    Clone
    + Eq
    + Debug
    + Send
    + Sync
    + Default
    + IndexMut<usize, Output = usize>
    + Index<usize, Output = usize>
{
    type IndexPattern;
    const NDIM: usize;
    fn number_of_dimensions(&self) -> usize;
    fn as_pattern(&self) -> Self::IndexPattern;
    fn number_of_elements(&self) -> usize;
    fn strides(&self) -> Self;
    fn slice(&self) -> &[Ix];
    fn stride_offset(index: &Self, strides: &Self) -> usize;
    fn x_axis_value(&self) -> Ix;
    fn y_axis_value(&self) -> Ix;
    // fn z_axis_value(&self) -> Option<Ix>;
}

pub trait GridIndex<D>: Debug {
    fn lin_space_index_unchecked(&self, dim: &D) -> usize;
}

impl<D> GridIndex<D> for D
where
    D: GridDimension,
{
    fn lin_space_index_unchecked(&self, dim: &D) -> usize {
        D::stride_offset(self, &dim.strides())
    }
}

// impl GridIndex for usize, (usize, usize) and (usize, usize, usize).
impl GridIndex<Dim<[Ix; 1]>> for Ix {
    fn lin_space_index_unchecked(&self, dim: &Dim<[Ix; 1]>) -> usize {
        Dim::<[Ix; 1]>::stride_offset(&Dim::from(*self), &dim.strides())
    }
}

impl GridIndex<Dim<[Ix; 2]>> for Ix2 {
    fn lin_space_index_unchecked(&self, dim: &Dim<[Ix; 2]>) -> usize {
        Dim::<[Ix; 2]>::stride_offset(&Dim::from(*self), &dim.strides())
    }
}

impl GridIndex<Dim<[Ix; 3]>> for Ix3 {
    fn lin_space_index_unchecked(&self, dim: &Dim<[Ix; 3]>) -> usize {
        Dim::<[Ix; 3]>::stride_offset(&Dim::from(*self), &dim.strides())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Dim<I: ?Sized> {
    index: I,
}

impl<I> Dim<I> {
    pub(crate) fn new(index: I) -> Dim<I> {
        Dim { index }
    }
    pub(crate) fn ix(&self) -> &I {
        &self.index
    }
    pub(crate) fn ixm(&mut self) -> &mut I {
        &mut self.index
    }
}

impl From<Ix> for Dim<[Ix; 1]> {
    fn from(size_1d: Ix) -> Self {
        Self::new([size_1d])
    }
}

impl From<Ix2> for Dim<[Ix; 2]> {
    fn from(size_2d: Ix2) -> Self {
        Self::new([size_2d.0, size_2d.1])
    }
}

impl From<Ix3> for Dim<[Ix; 3]> {
    fn from(size_3d: Ix3) -> Self {
        Self::new([size_3d.0, size_3d.1, size_3d.2])
    }
}

impl From<[Ix; 2]> for Dim<[Ix; 2]> {
    fn from(size_2d: [Ix; 2]) -> Self {
        Self::new(size_2d)
    }
}

impl From<[Ix; 3]> for Dim<[Ix; 3]> {
    fn from(size_3d: [Ix; 3]) -> Self {
        Self::new(size_3d)
    }
}

impl GridDimension for Dim<[Ix; 1]> {
    type IndexPattern = Ix;
    const NDIM: usize = 1;
    fn number_of_dimensions(&self) -> usize {
        1
    }
    fn number_of_elements(&self) -> usize {
        self.ix()[0]
    }
    fn as_pattern(&self) -> Self::IndexPattern {
        self.index.len()
    }
    fn strides(&self) -> Self {
        Dim::new([1])
    }
    fn slice(&self) -> &[Ix] {
        &self.index
    }
    fn stride_offset(index: &Self, strides: &Self) -> usize {
        index[0] * strides[0]
    }
    fn x_axis_value(&self) -> Ix {
        self.ix()[0]
    }
    fn y_axis_value(&self) -> Ix {
        1
    }
}

impl Index<usize> for Dim<[Ix; 1]> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.ix()[index]
    }
}

impl IndexMut<usize> for Dim<[Ix; 1]> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ixm()[index]
    }
}

impl GridDimension for Dim<[Ix; 2]> {
    type IndexPattern = Ix2;
    const NDIM: usize = 2;
    fn number_of_dimensions(&self) -> usize {
        2
    }
    fn number_of_elements(&self) -> usize {
        self.ix()[0] * self.ix()[1]
    }
    fn as_pattern(&self) -> Self::IndexPattern {
        (self.ix()[1], self.ix()[0])
    }
    fn strides(&self) -> Self {
        Dim::new([self.ix()[1] * 1, 1])
    }
    fn slice(&self) -> &[Ix] {
        &self.index
    }
    fn stride_offset(index: &Self, strides: &Self) -> usize {
        index[1] * strides[1] + index[0] * strides[0]
    }
    fn x_axis_value(&self) -> Ix {
        self.ix()[1]
    }
    fn y_axis_value(&self) -> Ix {
        self.ix()[0]
    }
}

impl Index<usize> for Dim<[Ix; 2]> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.ix()[index]
    }
}

impl IndexMut<usize> for Dim<[Ix; 2]> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ixm()[index]
    }
}

impl GridDimension for Dim<[Ix; 3]> {
    type IndexPattern = Ix3;
    const NDIM: usize = 3;
    fn number_of_dimensions(&self) -> usize {
        3
    }
    fn number_of_elements(&self) -> usize {
        self.ix()[3] * self.ix()[1] * self.ix()[0]
    }
    fn as_pattern(&self) -> Self::IndexPattern {
        (self.ix()[0], self.ix()[1], self.ix()[2])
    }
    fn strides(&self) -> Self {
        Dim::new([self.ix()[1] * self.ix()[2], self.ix()[2], 1])
    }
    fn slice(&self) -> &[Ix] {
        &self.index
    }
    fn stride_offset(index: &Self, strides: &Self) -> usize {
        index[0] * strides[0] + index[1] * strides[1] + index[2] * strides[2]
    }
    fn x_axis_value(&self) -> Ix {
        self.ix()[2]
    }
    fn y_axis_value(&self) -> Ix {
        self.ix()[1]
    }
}

impl Index<usize> for Dim<[Ix; 3]> {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.ix()[index]
    }
}

impl IndexMut<usize> for Dim<[Ix; 3]> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ixm()[index]
    }
}

#[cfg(test)]
mod tests {
    use super::{Dim, GridDimension};
    const TEST_1D_DIM_ARR: [usize; 1] = [8];
    #[test]
    fn dim_1d() {
        let dim_1d = Dim::new(TEST_1D_DIM_ARR);
        assert_eq!(dim_1d.index, TEST_1D_DIM_ARR);
        assert_eq!(dim_1d.ix(), &TEST_1D_DIM_ARR);
    }
    #[test]
    fn dim_1d_strides() {
        let dim_1d = Dim::new(TEST_1D_DIM_ARR);
        assert_eq!(dim_1d.strides(), Dim::new([1]));
        assert_eq!(dim_1d.strides().slice(), &[1]);
    }
    #[test]
    fn dim_1d_index() {
        let dim_1d = Dim::new(TEST_1D_DIM_ARR);
        assert_eq!(dim_1d[0], 8);
    }
    #[test]
    fn dim_1d_stride_offset() {
        let dim_1d = Dim::new(TEST_1D_DIM_ARR);
        let dim_1d_used_as_index = Dim::new([5]);
        assert_eq!(
            GridDimension::stride_offset(&dim_1d.strides(), &dim_1d_used_as_index),
            5
        );
    }
    #[test]
    fn dim_1d_ix_index() {
        use super::GridIndex;
        let dim_1d = Dim::new(TEST_1D_DIM_ARR);
        let dim_1d_used_as_index = Dim::new([5]);
        assert_eq!(
            dim_1d_used_as_index.lin_space_index_unchecked(&dim_1d),
            5.lin_space_index_unchecked(&dim_1d)
        )
    }

    const TEST_2D_DIM_ARR: [usize; 2] = [8, 3];
    #[test]
    fn dim_2d() {
        let dim_2d = Dim::new(TEST_2D_DIM_ARR);
        assert_eq!(dim_2d.index, TEST_2D_DIM_ARR);
        assert_eq!(dim_2d.ix(), &TEST_2D_DIM_ARR);
    }
    #[test]
    fn dim_2d_strides() {
        let dim_2d = Dim::new(TEST_2D_DIM_ARR);
        assert_eq!(dim_2d.strides(), Dim::new([3, 1]));
        assert_eq!(dim_2d.strides().slice(), &[3, 1]);
    }
    #[test]
    fn dim_2d_index() {
        let dim_2d = Dim::new(TEST_2D_DIM_ARR);
        assert_eq!(dim_2d[0], 8);
        assert_eq!(dim_2d[1], 3);
    }
    #[test]
    fn dim_2d_stride_offset() {
        let dim_2d = Dim::new(TEST_2D_DIM_ARR);
        let dim_2d_used_as_index = Dim::new([2, 2]);
        assert_eq!(
            GridDimension::stride_offset(&dim_2d.strides(), &dim_2d_used_as_index),
            8
        );
    }
    #[test]
    fn dim_2d_ix_index() {
        use super::GridIndex;
        let dim_2d = Dim::new(TEST_2D_DIM_ARR);
        let dim_2d_used_as_index = Dim::new([2, 2]);
        assert_eq!(
            dim_2d_used_as_index.lin_space_index_unchecked(&dim_2d),
            (2, 2).lin_space_index_unchecked(&dim_2d)
        )
    }

    const TEST_3D_DIM_ARR: [usize; 3] = [13, 8, 3];
    #[test]
    fn dim_3d() {
        let dim_3d = Dim::new(TEST_3D_DIM_ARR);
        assert_eq!(dim_3d.index, TEST_3D_DIM_ARR);
        assert_eq!(dim_3d.ix(), &TEST_3D_DIM_ARR);
    }
    #[test]
    fn dim_3d_strides() {
        let dim_3d = Dim::new(TEST_3D_DIM_ARR);
        assert_eq!(dim_3d.strides(), Dim::new([8 * 3, 3, 1]));
        assert_eq!(dim_3d.strides().slice(), &[8 * 3, 3, 1]);
    }
    #[test]
    fn dim_3d_index() {
        let dim_3d = Dim::new(TEST_3D_DIM_ARR);
        assert_eq!(dim_3d[0], 13);
        assert_eq!(dim_3d[1], 8);
        assert_eq!(dim_3d[2], 3);
    }
    #[test]
    fn dim_3d_stride_offset() {
        let dim_3d = Dim::new(TEST_3D_DIM_ARR);
        let dim_3d_used_as_index = Dim::new([2, 2, 2]);
        assert_eq!(
            GridDimension::stride_offset(&dim_3d.strides(), &dim_3d_used_as_index),
            2 * 8 * 3 + 2 * 3 + 2 * 1
        );
    }
    #[test]
    fn dim_3d_ix_index() {
        use super::GridIndex;
        let dim_3d = Dim::new(TEST_3D_DIM_ARR);
        let dim_3d_used_as_index = Dim::new([2, 2, 2]);
        assert_eq!(
            dim_3d_used_as_index.lin_space_index_unchecked(&dim_3d),
            (2, 2, 2).lin_space_index_unchecked(&dim_3d)
        )
    }
}
