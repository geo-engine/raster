#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct SpatialBoundingBox2D {
    pub upper_left_coordinate: (f64, f64),
    pub lower_right_coordinate: (f64, f64),
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct TimeInterval {
    pub interval_start: usize,
    pub interval_end: usize,
}

pub trait SpatialBounded {
    fn spatial_bounds(&self) -> SpatialBoundingBox2D;
}

pub trait TemporalBounded {
    fn temporal_bounds(&self) -> TimeInterval;
}
