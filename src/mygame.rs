use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::mint::Point2;
use ggez::input;
use std::collections::HashMap;

extern crate rand;
use rand::Rng;
use crate::cell::Cell;
use crate::mouse::Mouse;

pub struct GameOfLife {
    cols: usize,
    rows: usize,
    cell_width: usize,
    cell_height: usize,
    cells: Vec<Cell>,
    cells_next_life: Vec<Cell>,
    cell_mesh: graphics::Mesh,
    mouse: Mouse,
    directions: HashMap<&'static str, (i8, i8)>,
}

impl GameOfLife {
    pub fn new(ctx: &mut Context) -> GameOfLife {
        // Load/create resources such as images here.

        let (win_width, win_height) = graphics::drawable_size(ctx);
        let cols: usize = 60;
        let rows: usize = 60;
        let cell_width = win_width as usize / cols;
        let cell_height = win_height as usize / rows;


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

        let directions: HashMap<&'static str, (i8, i8)> = [
            ("n", (0, -1)),
            ("ne", (1, -1)),
            ("e", (1, 0)),
            ("se", (1, 1)),
            ("s", (0, 1)),
            ("sw", (-1, 1)),
            ("w", (-1, 0)),
            ("nw", (-1, -1)),
        ].iter().cloned().collect();

        GameOfLife {
            cell_width,
            cell_height,
            cols,
            rows,
            cells: Self::generate_cells(30, cols, rows),
            cells_next_life: vec![Cell::new(false); (cols * rows) as usize],
            cell_mesh,
            mouse: Default::default(),
            directions,
        }
    }

    fn generate_cells(percent: usize, cols: usize, rows: usize) -> Vec<Cell> {
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

    fn add_cell_at_direction(&mut self, x: i8, y: i8, dir: (i8, i8)) {
        self.set_cell((x+dir.0) as usize, (y+dir.1) as usize, true);
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[(x+(y*self.cols)) as usize].alive
    }
    fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[(x+(y*self.cols)) as usize] = Cell::new(alive);
    }
    fn set_next_life(&mut self, x: usize, y: usize, alive: bool) {
        self.cells_next_life[(x+(y*self.cols)) as usize] = Cell::new(alive);
    }

    /// find_neighbours of given co-ordinates
    fn find_neighbours(&self, x: usize, y: usize) -> i8 {

        let mut count: i8 = 0;
        for dir in direction_names(DIRECTIONS_ALL) {
            if self.look(x, y, self.directions[dir]) {
                count += 1;
            }
        }
        count
    }

    pub fn next(&mut self) {

        self.cells_next_life = vec![Cell::new(false); (self.cols * self.rows) as usize];

        for x in 0..self.cols {
            for y in 0..self.rows {
                let count = self.find_neighbours(x, y);
                let cell = Cell::new(self.get_cell(x, y));
                let alive = cell.next_state(count);
                self.set_next_life(x, y, alive);
            }
        }

        self.cells = self.cells_next_life.clone();
    }

    fn add_cell_on_grid(&mut self, x: usize, y: usize) {
        if self.is_inside(x, y) {
            self.set_cell(x, y, true);
        }
    }

    fn add_pattern(&mut self, x: usize, y: usize, directions_follow_pattern: &'static str) {
        let (mut x1, mut y1) = (x as i8, y as i8);
        for dir in direction_names(directions_follow_pattern) {
            if !self.look(x1 as usize, y1 as usize, self.directions[dir]) {
                let direction = self.directions[dir];
                self.add_cell_at_direction(x1, y1, direction);
                x1 = x1 + direction.0;
                y1 = y1 + direction.1;
            }
        }
    }

}

impl EventHandler for GameOfLife {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.next();

        if input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left) {
            if input::mouse::position(ctx) != self.mouse.relative_position() {
                self.mouse.set_position(input::mouse::position(ctx));
                let mouse_position = self.mouse.grid_position(self.cell_width as f32, self.cell_height as f32);
                // println!("button pressed x: {}, y: {}", mouse_position.x, mouse_position.y);

                let (x, y) = (mouse_position.x as usize, mouse_position.y as usize);
                self.add_cell_on_grid(x, y);

                if input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::SHIFT) {

                    self.add_pattern(x, y, "se s w w");//glider pattern

                }

            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        //draw cells
        for x in 0..self.cols {
            for y in 0..self.rows {
                let alive = self.get_cell(x,y);
                if alive {
                    let bounds = graphics::DrawParam::default()
                        .dest(Point2{x: (x*self.cell_width) as f32, y: (y*self.cell_height) as f32});
                    graphics::draw(ctx, &self.cell_mesh, bounds)?;
                }
            }
        }

        graphics::present(ctx)
    }
}



const DIRECTIONS_ALL: &'static str = "n ne e se s sw w nw";

pub fn direction_names(directions: &'static str) -> Vec<&'static str> {
    directions
        .split(" ")
        // .map(|c| c.to_string())
        .collect::<Vec<&str>>()
}
