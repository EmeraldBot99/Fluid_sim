

use minifb::{Key, Window, WindowOptions,MouseButton};

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

    pub fn advect(&mut self, dt: f32) {
        let old_vel_x = self.vel_x.clone();
        let old_vel_y = self.vel_y.clone();
        
        for j in 0..self.height {
            for i in 1..self.width {
                let pos_x = i as f32;
                let pos_y = j as f32 + 0.5;
                
                let (u, v) = self.interpolate_velocity(pos_x, pos_y, &old_vel_x, &old_vel_y);
                
                let traced_x = (pos_x - dt * u).max(0.0).min(self.width as f32);
                let traced_y = (pos_y - dt * v).max(0.0).min(self.height as f32);
                
                self.vel_x[j][i] = self.interpolate_x_velocity(traced_x, traced_y, &old_vel_x);
            }
        }
        
        for i in 0..self.width {
            for j in 1..self.height {
                let pos_x = i as f32 + 0.5; 
                let pos_y = j as f32;
                
                let (u, v) = self.interpolate_velocity(pos_x, pos_y, &old_vel_x, &old_vel_y);
                
                let traced_x = (pos_x - dt * u).max(0.0).min(self.width as f32);
                let traced_y = (pos_y - dt * v).max(0.0).min(self.height as f32);
                
                self.vel_y[i][j] = self.interpolate_y_velocity(traced_x, traced_y, &old_vel_y);
            }
        }
        
        self.set_boundary_conditions();
    }
    
    fn interpolate_velocity(&self, x: f32, y: f32, vel_x: &Vec<Vec<f32>>, vel_y: &Vec<Vec<f32>>) -> (f32, f32) {
        let u = self.interpolate_x_velocity(x, y, vel_x);
        let v = self.interpolate_y_velocity(x, y, vel_y);
        (u, v)
    }
    
    fn interpolate_x_velocity(&self, x: f32, y: f32, vel_x: &Vec<Vec<f32>>) -> f32 {
        let i = (x.floor() as isize).max(0).min(self.width as isize - 1) as usize;
        let j = ((y - 0.5).floor() as isize).max(0).min(self.height as isize - 2) as usize; 
        
        let fx = x - x.floor();
        let fy = (y - 0.5) - (y - 0.5).floor();
        
        let v00 = vel_x[j][i];
        let v10 = vel_x[j][i.min(self.width - 1)];
        let v01 = vel_x[j + 1][i];  
        let v11 = vel_x[j + 1][i.min(self.width - 1)];
        
        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;
        v0 * (1.0 - fy) + v1 * fy
    }
    
    fn interpolate_y_velocity(&self, x: f32, y: f32, vel_y: &Vec<Vec<f32>>) -> f32 {
        let i = ((x - 0.5).floor() as isize).max(0).min(self.width as isize - 2) as usize; 
        let j = (y.floor() as isize).max(0).min(self.height as isize - 1) as usize;
        
        let fx = (x - 0.5) - (x - 0.5).floor();
        let fy = y - y.floor();
        
        let v00 = vel_y[i][j];
        let v10 = vel_y[i + 1][j]; 
        let v01 = vel_y[i][j.min(self.height - 1)];
        let v11 = vel_y[i + 1][j.min(self.height - 1)];
        
        let v0 = v00 * (1.0 - fx) + v10 * fx;
        let v1 = v01 * (1.0 - fx) + v11 * fx;
        v0 * (1.0 - fy) + v1 * fy
    }


    
    pub fn update_grid(&mut self){
        for x in &mut self.vel_y{
            for y in x{
                // *y -= 9.81 * (1.0/60.0);
            }
        }

        self.advect(1.0);

        self.set_boundary_conditions();
        // let test = self.vel_y[50][50];
        // println!("{test}")
    }

    pub fn update_vel(&mut self, x: usize, y: usize, dir_x: i32, dir_y:i32, amount: f32){
        self.vel_x[y][x] += amount * dir_x as f32;
        self.vel_y[x][y] += amount * dir_y as f32;

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

    let mut grid = Grid::new(WIDTH, HEIGHT);

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let (mut prev_mouse_x, mut prev_mouse_y) = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let (curr_mouse_x, curr_mouse_y) = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));

        if window.get_mouse_down(MouseButton::Left) {
            let dir_x:i32  = if prev_mouse_x - curr_mouse_x > 0.0 { 1 } else { -1 };
            let dir_y:i32= if prev_mouse_y - curr_mouse_y > 0.0 { 1 } else { -1 };

            grid.update_vel(curr_mouse_x as usize, curr_mouse_y as usize, dir_x, dir_y, 0.5);
        }

        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                // let u = x as f32 / (WIDTH as f32 - 1.0);
                // let v = y as f32 / (HEIGHT as f32 - 1.0);
                if x == 50 && y == 50 {
                    // println!("test");
                }
                let r = (grid.vel_x[y][x].abs() * 255.0) as u32;
                let g = (grid.vel_y[x][y].abs() ) as u32;
                let b = (grid.pressure[y][x] * 255.0) as u32;

                let color = (r << 16) | (g << 8) | b;
                
                buffer[y * WIDTH + x] = color;
            }
        }
        
        grid.update_grid();
        (prev_mouse_x, prev_mouse_y) = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));
        println!("new frame");
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

    }
}
