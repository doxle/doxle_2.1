use crate::point::Point;
use crate::polygon::Polygon;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub polygon: Polygon,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            polygon: Polygon::new(),
        }
    }
    pub fn handle_click(&mut self, position: Point) {
        self.polygon.add_point(position);
    }
    pub fn clear_polygon(&mut self) {
        self.polygon.clear_points()
    }
}
