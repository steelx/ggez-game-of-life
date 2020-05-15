use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::nalgebra::Point2;
use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use crate::cell::Cell;

pub struct GameOfLife {
    cols: usize,
    rows: usize,
    cell_width: usize,
    cell_height: usize,
    grid_line_vertical: graphics::Mesh,
    grid_line_horizontal: graphics::Mesh,
    cells: Vec<Cell>,
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
            cells: Self::generate_cells(30, cols, rows, cell_width, cell_height),
            cell_mesh,
        }
    }

    fn generate_cells(percent: usize, cols: usize, rows: usize, cell_width: usize, cell_height: usize) -> Vec<Cell> {
        assert!(percent <= 100, "percent must be between 00 and 100");

        let mut cells = vec![Cell::new(false); (cols * rows) as usize];
        let percentage_alive = percent * cells.len() / 100;

        // Fill alive Cells as per percentage given
        for i in 0..percentage_alive {
            cells[i] = Cell::new(true);
        }

        // Randomize Alive Cells
        let mut seed = rand::thread_rng();
        for i in (1..(cells.len()-1)).rev() {
            let random_index: usize = seed.gen_range(0, i);
            cells.swap(i, random_index);
        }

        cells
    }

    /// Look check's a Cell at given direction
    /// and returns true if a Cell is found
    fn look(&self, x: usize, y: usize, direction: (i8, i8)) -> bool {
        if !self.is_inside(x, y) {
            return false;
        }

        self.plus(x as i8, y as i8, direction)
    }

    fn is_inside(&self, x: usize, y: usize) -> bool {
        x > 0 && x < self.cols && y > 0 && y < self.rows
    }

    /// Plus accepts x, y of Cell PLUS direction tuple
    /// returns true if Cell exists in the given direction
    fn plus(&self, x: i8, y: i8, dir: (i8, i8)) -> bool {
        self.is_inside((x+dir.0) as usize, (y+dir.1) as usize) && self.get_cell((x+dir.0) as usize, (y+dir.1) as usize)
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[(x+(y*self.cols)) as usize].alive
    }
    fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[(x+(y*self.cols)) as usize] = Cell::new(alive);
    }

    /// find_neighbours of given co-ordinates
    fn find_neighbours(&self, x: usize, y: usize) -> i8 {
        let directions: HashMap<&'static str, (i8, i8)> = [
            ("n", (0, 1)),
            ("ne", (1, -1)),
            ("e", (1, 0)),
            ("se", (1, 1)),
            ("s", (0, 1)),
            ("sw", (-1, 1)),
            ("w", (-1, 0)),
            ("nw", (-1, -1)),
        ].iter().cloned().collect();

        let mut count: i8 = 0;
        for dir in direction_names() {
            if self.look(x, y, directions[dir]) {
                count += 1;
            }
        }
        count
    }

    pub fn next(&mut self) {
        let old_world = GameOfLife{
            cells: self.cells.clone(),
            cell_width: self.cell_width,
            cell_height: self.cell_height,
            cols: self.cols,
            rows: self.rows,
            grid_line_vertical: self.grid_line_vertical.clone(),
            grid_line_horizontal: self.grid_line_horizontal.clone(),
            cell_mesh: self.cell_mesh.clone(),
        };

        for x in 0..self.cols {
            for y in 0..self.rows {
                let count = old_world.find_neighbours(x, y);
                let cell = Cell::new(old_world.get_cell(x, y));
                let alive = cell.next_state(count);
                self.set_cell(x, y, alive);
            }
        }
    }

}

impl EventHandler for GameOfLife {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.next();

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
        for x in 0..self.cols {
            for y in 0..self.rows {
                let alive = self.get_cell(x,y);
                if alive {
                    let bounds = graphics::DrawParam::default()
                        .dest(Point2::new((x*self.cell_width) as f32, (y*self.cell_height) as f32));
                    graphics::draw(ctx, &self.cell_mesh, bounds)?;
                }
            }
        }

        graphics::present(ctx)
    }
}



const DIRECTION_NAMES: &'static str = "n ne e se s sw w nw";

pub fn direction_names() -> Vec<&'static str> {
    DIRECTION_NAMES
        .split(" ")
        // .map(|c| c.to_string())
        .collect::<Vec<&str>>()
}
