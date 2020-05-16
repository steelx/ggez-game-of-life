use ggez::mint::Point2;

#[derive(Default)]
pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub mouse_down: bool,
}

impl Mouse {

    pub fn grid_position(&self, tile_size_width: f32, tile_size_height: f32) -> Point2<f32> {
        Point2{x: ((self.x) / tile_size_width) as f32, y: ((self.y) / tile_size_height) as f32}
    }

    pub fn relative_position(&self) -> Point2<f32> {
        Point2{x: self.x, y: self.y}
    }

    pub fn set_position(&mut self, pos: Point2<f32>) {
        self.x = pos.x;
        self.y = pos.y;
    }
}
