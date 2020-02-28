pub mod geo_transform;
pub mod grid_dimension;
pub mod raster;
pub mod spatio_temporal_bounds;
pub use raster::Raster;

// Index types for 1,2,3 dimensional grids
pub type Ix = usize;
pub type Ix1 = Ix;
pub type Ix2 = (Ix, Ix);
pub type Ix3 = (Ix, Ix, Ix);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
