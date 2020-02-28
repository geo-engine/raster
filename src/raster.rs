use std::convert::AsRef;

pub use super::geo_transform::GeoTransform;
pub use super::grid_dimension::{Dim, GridDimension, GridIndex};
pub use super::spatio_temporal_bounds::{
    SpatialBounded, SpatialBoundingBox2D, TemporalBounded, TimeInterval,
};

pub trait Raster<D: GridDimension, T: Copy, C>: SpatialBounded + TemporalBounded {
    fn dimension(&self) -> &D;
    fn no_data_value(&self) -> Option<T>;
    fn data_container(&self) -> &C;
    fn geo_transform(&self) -> &GeoTransform;
}

#[derive(Clone, Default, Debug)]
pub struct BaseRaster<D, T, C> {
    grid_dimension: D,
    data_container: C,
    no_data_value: Option<T>,
    geo_transform: GeoTransform,
    temporal_bounds: TimeInterval,
}

impl<D, T, C> BaseRaster<D, T, C> {
    pub fn new(
        grid_dimension: D,
        data_container: C,
        no_data_value: Option<T>,
        temporal_bounds: TimeInterval,
        geo_transform: GeoTransform,
    ) -> Self {
        Self {
            grid_dimension,
            data_container,
            no_data_value,
            temporal_bounds,
            geo_transform,
        }
    }
}

impl<D, T, C> TemporalBounded for BaseRaster<D, T, C> {
    fn temporal_bounds(&self) -> TimeInterval {
        self.temporal_bounds.clone()
    }
}

impl<D, C, T> SpatialBounded for BaseRaster<D, C, T>
where
    D: GridDimension,
{
    fn spatial_bounds(&self) -> SpatialBoundingBox2D {
        let top_left_coord = self.geo_transform.grid_2d_to_coordinate((0, 0));
        let lower_right_coord = self.geo_transform.grid_2d_to_coordinate((
            self.grid_dimension.y_axis_value(),
            self.grid_dimension.x_axis_value(),
        ));
        SpatialBoundingBox2D {
            upper_left_coordinate: top_left_coord,
            lower_right_coordinate: lower_right_coord,
        }
    }
}

impl<D, T, C> Raster<D, T, C> for BaseRaster<D, T, C>
where
    D: GridDimension,
    T: Copy,
{
    fn dimension(&self) -> &D {
        &self.grid_dimension
    }
    fn no_data_value(&self) -> Option<T> {
        self.no_data_value
    }
    fn data_container(&self) -> &C {
        &self.data_container
    }
    fn geo_transform(&self) -> &GeoTransform {
        &self.geo_transform
    }
}

pub trait GridPixelAccess<T, I> {
    fn pixel_value_grid(&self, grid_index: &I) -> T;
}

pub trait CoordinatePixelAccess<T> {
    fn pixel_value_coord(&self, coordinate: (f64, f64)) -> T;
}

impl<D, T, C, I> GridPixelAccess<T, I> for BaseRaster<D, T, C>
where
    D: GridDimension,
    I: GridIndex<D>,
    C: AsRef<[T]>,
    T: Copy,
{
    fn pixel_value_grid(&self, grid_index: &I) -> T {
        let index = grid_index.lin_space_index_unchecked(&self.grid_dimension);
        self.data_container.as_ref()[index]
    }
}

pub type SimpleRaster2d<T> = BaseRaster<Dim<[usize; 2]>, T, Vec<T>>;
pub type SimpleRaster3d<T> = BaseRaster<Dim<[usize; 3]>, T, Vec<T>>;

#[cfg(test)]
mod tests {
    use super::{Dim, GridPixelAccess, SimpleRaster2d, TimeInterval};

    #[test]
    fn simple_raster_2d() {
        let dim = [4, 5];
        let data = vec![9; 20];
        let geo_transform = [1.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let temporal_bounds: TimeInterval = TimeInterval::default();
        SimpleRaster2d::new(
            dim.into(),
            data,
            None,
            temporal_bounds,
            geo_transform.into(),
        );
    }

    #[test]
    fn simple_raster_2d_at_tuple() {
        let tuple_index = (2, 2);

        let dim = [4, 5];
        let data = vec![9; 20];
        let geo_transform = [1.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let temporal_bounds: TimeInterval = TimeInterval::default();
        let raster2d = SimpleRaster2d::new(
            dim.into(),
            data,
            None,
            temporal_bounds,
            geo_transform.into(),
        );
        let value = raster2d.pixel_value_grid(&tuple_index);
        assert!(value == 9);
    }

    #[test]
    fn simple_raster_2d_at_arr() {
        let dim_index = Dim::from([2, 2]);

        let dim = [4, 5];
        let data = vec![9; 20];
        let geo_transform = [1.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let temporal_bounds: TimeInterval = TimeInterval::default();
        let raster2d = SimpleRaster2d::new(
            dim.into(),
            data,
            None,
            temporal_bounds,
            geo_transform.into(),
        );
        let value = raster2d.pixel_value_grid(&dim_index);
        assert!(value == 9);
    }
}
