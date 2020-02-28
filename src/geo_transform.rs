pub type GdalGeoTransform = [f64; 6];

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct GeoTransform {
    upper_left_x_coordinate: f64,
    x_pixel_size: f64,
    x_rotation: f64,
    upper_left_y_coordinate: f64,
    y_rotation: f64,
    y_pixel_size: f64,
}

impl GeoTransform {
    pub fn grid_2d_to_coordinate(&self, grid_index: (usize, usize)) -> (f64, f64) {
        let (grid_index_y, grid_index_x) = grid_index;
        let coord_x = self.upper_left_x_coordinate
            + (grid_index_x as f64) * self.x_pixel_size
            + (grid_index_y as f64) * self.x_rotation;
        let coord_y = self.upper_left_y_coordinate
            + (grid_index_x as f64) * self.y_rotation
            + (grid_index_y as f64) * self.y_pixel_size;
        (coord_x, coord_y)
    }

    pub fn coordinate_to_grid_2d(&self, coordiante: (f64, f64)) -> (usize, usize) {
        if self.x_rotation != 0.0 || self.y_rotation != 0.0 {
            panic!();
        }

        let (coord_x, coord_y) = coordiante;
        let grid_x_index = ((coord_x - self.upper_left_x_coordinate) / self.x_pixel_size) as usize;
        let grid_y_index = ((coord_y - self.upper_left_y_coordinate) / self.y_pixel_size) as usize;
        (grid_y_index, grid_x_index)
    }

    pub fn new(
        upper_left_x_coordinate: f64,
        x_pixel_size: f64,
        x_rotation: f64,
        upper_left_y_coordinate: f64,
        y_rotation: f64,
        y_pixel_size: f64,
    ) -> Self {
        Self {
            upper_left_x_coordinate: upper_left_x_coordinate,
            x_pixel_size: x_pixel_size,
            x_rotation: x_rotation,
            upper_left_y_coordinate: upper_left_y_coordinate,
            y_rotation: y_rotation,
            y_pixel_size: y_pixel_size,
        }
    }
}

impl From<GdalGeoTransform> for GeoTransform {
    fn from(gdal_geo_transform: GdalGeoTransform) -> Self {
        Self::new(
            gdal_geo_transform[0],
            gdal_geo_transform[1],
            gdal_geo_transform[2],
            gdal_geo_transform[3],
            gdal_geo_transform[4],
            gdal_geo_transform[5],
        )
    }
}

impl Into<GdalGeoTransform> for GeoTransform {
    fn into(self) -> GdalGeoTransform {
        [
            self.upper_left_x_coordinate,
            self.x_pixel_size,
            self.x_rotation,
            self.upper_left_y_coordinate,
            self.y_rotation,
            self.y_pixel_size,
        ]
    }
}
