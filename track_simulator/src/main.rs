extern crate piston_window;

use piston_window::*;
use std::net::TcpListener;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Rectangle {
    top_left: Point,
    bottom_left: Point,
    bottom_right: Point,
    top_right: Point,
    velocity_x: f64,
    velocity_y: f64,
}

fn start_server(shared_rect: Arc<Mutex<Rectangle>>) {
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:65535").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            loop {
                let rect = shared_rect.lock().unwrap();
                // 修改这里来发送所有四个角点的坐标
                let data = format!(
                    "{},{},{},{},{},{},{},{}",
                    rect.top_left.x, rect.top_left.y,
                    rect.bottom_left.x, rect.bottom_left.y,
                    rect.bottom_right.x, rect.bottom_right.y,
                    rect.top_right.x, rect.top_right.y
                );
                if let Err(e) = stream.write(data.as_bytes()) {
                    eprintln!("Failed to send data: {}", e);
                    break;
                }
                if let Err(e) = stream.flush() {
                    eprintln!("Failed to flush stream: {}", e);
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    });
}

impl Rectangle {
    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 根据矩形的运动方向和速度调整透视变换的参数
        //let perspective_shift_x = 0.0;
        //let perspective_shift_y = 0.0;
        let perspective_shift_x = rng.gen_range(-1.0..1.0) * self.velocity_x.abs();
        let perspective_shift_y = rng.gen_range(-1.0..1.0) * self.velocity_y.abs();

        // 更新矩形位置的逻辑，同时加入透视变化

        self.top_left.x += self.velocity_x + perspective_shift_x;
        self.top_left.y += self.velocity_y + perspective_shift_y;

        self.bottom_left.x += self.velocity_x - perspective_shift_x;
        self.bottom_left.y += self.velocity_y - perspective_shift_y;

        self.bottom_right.x += self.velocity_x - perspective_shift_x;
        self.bottom_right.y += self.velocity_y - perspective_shift_y;

        self.top_right.x += self.velocity_x + perspective_shift_x;
        self.top_right.y += self.velocity_y + perspective_shift_y;

        // 检测边界并重新初始化位置
        if self.top_left.x > 1280.0 || self.top_left.y > 1080.0 ||
           self.bottom_right.x > 1280.0 || self.bottom_right.y > 1080.0 {
            self.reset_position();
        }

        // 打印四个角的坐标
        println!("Top Left: ({}, {})", self.top_left.x, self.top_left.y);
        println!("Bottom Left: ({}, {})", self.bottom_left.x, self.bottom_left.y);
        println!("Bottom Right: ({}, {})", self.bottom_right.x, self.bottom_right.y);
        println!("Top Right: ({}, {})", self.top_right.x, self.top_right.y);
        println!("---------***---------");
    }


    fn reset_position(&mut self) {
        // 重置矩形的位置
        self.top_left = Point { x: 50.0, y: 50.0 };
        self.bottom_left = Point { x: 50.0, y: 150.0 };
        self.bottom_right = Point { x: 150.0, y: 150.0 };
        self.top_right = Point { x: 150.0, y: 50.0 };
    }

    fn draw<G: Graphics>(&self, transform: math::Matrix2d, g: &mut G) {
        // 绘制矩形的四个边
        line([0.0, 0.0, 0.0, 1.0], 1.0, [self.top_left.x, self.top_left.y, self.bottom_left.x, self.bottom_left.y], transform, g);
        line([0.0, 0.0, 0.0, 1.0], 1.0, [self.bottom_left.x, self.bottom_left.y, self.bottom_right.x, self.bottom_right.y], transform, g);
        line([0.0, 0.0, 0.0, 1.0], 1.0, [self.bottom_right.x, self.bottom_right.y, self.top_right.x, self.top_right.y], transform, g);
        line([0.0, 0.0, 0.0, 1.0], 1.0, [self.top_right.x, self.top_right.y, self.top_left.x, self.top_left.y], transform, g);

        // 绘制四个角的红点
        let red = [1.0, 0.0, 0.0, 1.0]; // 红色
        let point_size = 5.0; // 点的大小
        ellipse(red, ellipse::circle(self.top_left.x, self.top_left.y, point_size), transform, g);
        ellipse(red, ellipse::circle(self.bottom_left.x, self.bottom_left.y, point_size), transform, g);
        ellipse(red, ellipse::circle(self.bottom_right.x, self.bottom_right.y, point_size), transform, g);
        ellipse(red, ellipse::circle(self.top_right.x, self.top_right.y, point_size), transform, g);
    }
}

fn main() {
    let canvas_width = 1280;
    let canvas_height = 1080;
    let scale_factor = 0.5; // 缩放因子，根据需要调整

    let window_width = (canvas_width as f64 * scale_factor) as u32;
    let window_height = (canvas_height as f64 * scale_factor) as u32;

    let mut window: PistonWindow = WindowSettings::new("四点运动模拟器", [window_width, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let rect = Rectangle {
        top_left: Point { x: 50.0, y: 50.0 },
        bottom_left: Point { x: 50.0, y: 150.0 },
        bottom_right: Point { x: 150.0, y: 150.0 },
        top_right: Point { x: 150.0, y: 50.0 },
        velocity_x: 2.0,
        velocity_y: 1.0,
    };

    let shared_rect = Arc::new(Mutex::new(rect)); // 创建共享的矩形
    start_server(shared_rect.clone()); // 启动服务器线程

    let update_interval = 1.0 / 150.0; // 更新频率为150Hz
    let mut accumulator = 0.0;

    while let Some(e) = window.next() {
        if let Some(u) = e.update_args() {
            accumulator += u.dt;
            while accumulator >= update_interval {
                let mut rect = shared_rect.lock().unwrap();
                rect.update();
                accumulator -= update_interval;
            }
        }
    
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            let transform = c.transform.scale(scale_factor, scale_factor);
            let rect = shared_rect.lock().unwrap().clone(); // 从共享数据中克隆
            rect.draw(transform, g);
        });
    }
}