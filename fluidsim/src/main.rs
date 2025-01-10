

use minifb::{Key, Window, WindowOptions};

#[derive(Clone)]

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub pressure: Vec<Vec<f32>>,
    pub density: Vec<Vec<f32>>,
    pub vel_x: Vec<Vec<f32>>,
    pub vel_y: Vec<Vec<f32>>,
}

impl Grid {
    pub fn new(width:usize,height:usize) -> Self{
        Grid{ 
            width,
            height,
            pressure: vec![vec![0.0; width]; height],
            density: vec![vec![0.0; width]; height],
            vel_x: vec![vec![0.0; width + 1]; height],
            vel_y: vec![vec![0.0; height + 1]; width],
        }
    }

    pub fn get_center_vel(&self, i: usize, j: usize) -> (f32,f32){
        let x_avg = (self.vel_x[j][i] + self.vel_x[j][i + 1]) * 0.5;
        let y_avg = (self.vel_y[i][j] + self.vel_y[i][j + 1]) * 0.5;
        (x_avg, y_avg)
    }

    pub fn set_boundary_conditions(&mut self) {
        for j in 0..self.height {
            self.vel_x[j][0] = 0.0; 
            self.vel_x[j][self.width] = 0.0;  
        }


        for i in 0..self.width {
            self.vel_y[i][0] = 0.0; 
            self.vel_y[i][self.height] = 0.0; 
        }
    }

    pub fn calculate_divergence(&self, i: usize, j: usize) -> f32 {
        (self.vel_x[j][i + 1] - self.vel_x[j][i])+ (self.vel_y[i][j + 1] - self.vel_y[i][j])
    }

    pub fn update_grid(&mut self){
        for x in &mut self.vel_y{
            for y in x{
                *y -= 9.81 * (1/60);
            }
        }
        // let test = self.vel_y[50][50];
        // println!("{test}")
    }

}

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut grid = Grid::new(WIDTH+1, HEIGHT+1);

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                // let u = x as f32 / (WIDTH as f32 - 1.0);
                // let v = y as f32 / (HEIGHT as f32 - 1.0);
                if x == 50 && y == 50 {
                    // println!("test");
                }
                let r = (grid.vel_x[y][x] * 255.0) as u32;
                let g = (grid.vel_y[x][y].abs() ) as u32;
                let b = (grid.pressure[y][x] * 255.0) as u32;

                let color = (r << 16) | (g << 8) | b;
                
                buffer[y * WIDTH + x] = color;
            }
        }
        
        grid.update_grid();
        println!("new frame");
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
