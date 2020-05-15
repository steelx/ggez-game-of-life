use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::nalgebra::Point2;
use std::collections::HashMap;

extern crate rand;
use rand::Rng;

pub struct GameOfLife {
    cols: usize,
    rows: usize,
    cell_width: usize,
    cell_height: usize,
    grid_line_vertical: graphics::Mesh,
    grid_line_horizontal: graphics::Mesh,
    cells: HashMap<(usize, usize), bool>,
    cell_mesh: graphics::Mesh,
}

impl GameOfLife {
    pub fn new(ctx: &mut Context) -> GameOfLife {
        // Load/create resources such as images here.

        let (win_width, win_height) = graphics::drawable_size(ctx);
        let cols: usize = 50;
        let rows: usize = 50;
        let cell_width = win_width as usize / cols;
        let cell_height = win_height as usize / rows;

        let rect = graphics::Rect::new(0.0, 0.0, 1.0, win_height);
        let rect2 = graphics::Rect::new(0.0, 0.0, win_width, 1.0);
        let color = graphics::Color::from_rgb(220, 220, 220);
        let grid_line_vertical = match graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color) {
            Ok(mesh) => mesh,
            Err(_e) => panic!("Could not create grid_line_vertical")
        };

        let grid_line_horizontal = match graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect2, color) {
            Ok(mesh) => mesh,
            Err(_e) => panic!("Could not create grid_line_vertical")
        };



        let cell_mesh_rect = graphics::Rect::new(0.0, 0.0, cell_width as f32, cell_height as f32);
        let cell_mesh = match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            cell_mesh_rect,
            graphics::Color::from_rgb(255, 51, 255),
        ) {
            Ok(mesh) => mesh,
            Err(_e) => panic!("Could not create cell_mesh")
        };

        GameOfLife {
            cell_width,
            cell_height,
            cols,
            rows,
            grid_line_vertical,
            grid_line_horizontal,
            cells: Self::generate_cells(0.3, cols, rows, cell_width, cell_height),
            cell_mesh,
        }
    }

    fn generate_cells(percent: f32, cols: usize, rows: usize, cell_width: usize, cell_height: usize) -> HashMap<(usize, usize), bool> {
        assert!(percent <= 1.0, "percent must be between 0.0 and 1.0");

        let mut cells = HashMap::new();
        let mut seed = rand::thread_rng();

        // Fill alive Cells as per percentage given
        for x in 0..cols {
            for y in 0..rows {
                let random_index: f32 = seed.gen();
                if random_index < percent {
                    cells.insert((x * cell_width, y * cell_height), true);
                }
            }
        }

        cells
    }
    
}

impl EventHandler for GameOfLife {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...
        for tile_size in 0..self.cols {
            let bounds = graphics::DrawParam::default().dest(Point2::new((tile_size * self.cell_width) as f32, 0.0));
            graphics::draw(ctx, &self.grid_line_vertical, bounds)?;

            let bounds = graphics::DrawParam::default()
                .dest(Point2::new(0.0, (tile_size * self.cell_height) as f32));
            graphics::draw(ctx, &self.grid_line_horizontal, bounds)?;
        }

        //draw cells
        for (&cell, &alive) in self.cells.iter() {
            if alive {
                let bounds = graphics::DrawParam::default()
                    .dest(Point2::new(cell.0 as f32, cell.1 as f32));
                graphics::draw(ctx, &self.cell_mesh, bounds)?;
            }
        }

        graphics::present(ctx)
    }
}
