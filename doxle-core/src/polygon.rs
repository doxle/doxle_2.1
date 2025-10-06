use crate::point::Point;

#[derive(Clone, Debug, Default)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            points: Vec::<Point>::new(),
        }
    }
    pub fn add_point(&mut self, new_point: Point) {
        self.points.push(new_point);
    }
    pub fn count_points(&self) -> usize {
        self.points.len()
    }
    pub fn clear_points(&mut self) {
        self.points.clear()
    }
}
