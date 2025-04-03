extern crate sdl2;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 800;

const WORLD_MAP: [[i32; MAP_HEIGHT]; MAP_WIDTH] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

fn get_wall_color(tile: i32) -> Color {
    match tile {
        1 => Color::RGB(255, 0, 0),
        2 => Color::RGB(0, 255, 0),
        3 => Color::RGB(0, 0, 255),
        4 => Color::RGB(255, 255, 255),
        _ => Color::RGB(255, 255, 0),
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pos_x = 22.0;
    let mut pos_y = 12.00;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;

    let mut old_time = Instant::now();

    'main: loop {
        let frame_time = old_time.elapsed().as_secs_f64();
        old_time = Instant::now();

        canvas
            .window_mut()
            .set_title(&format!("Raycaster - {:.2} FPS", 1.0 / frame_time))
            .unwrap();

        let move_speed = frame_time * 5.0;
        let rot_speed = frame_time * 3.0;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    ..
                } => {
                    if WORLD_MAP[(pos_x + dir_x * move_speed) as usize][pos_y as usize] == 0 {
                        pos_x += dir_x * move_speed;
                    }
                    if WORLD_MAP[pos_x as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                        pos_y += dir_y * move_speed;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => {
                    let old_dir_x = dir_x;
                    dir_x = dir_x * (rot_speed).cos() - dir_y * (rot_speed).sin();
                    dir_y = old_dir_x * (rot_speed).sin() + dir_y * (rot_speed).cos();
                    let old_plane_x = plane_x;
                    plane_x = plane_x * (rot_speed).cos() - plane_y * (rot_speed).sin();
                    plane_y = old_plane_x * (rot_speed).sin() + plane_y * (rot_speed).cos();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    ..
                } => {
                    if WORLD_MAP[(pos_x - dir_x * move_speed) as usize][pos_y as usize] == 0 {
                        pos_x -= dir_x * move_speed;
                    }
                    if WORLD_MAP[pos_x as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                        pos_y -= dir_y * move_speed;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => {
                    let old_dir_x = dir_x;
                    dir_x = dir_x * (-rot_speed).cos() - dir_y * (-rot_speed).sin();
                    dir_y = old_dir_x * (-rot_speed).sin() + dir_y * (-rot_speed).cos();
                    let old_plane_x = plane_x;
                    plane_x = plane_x * (-rot_speed).cos() - plane_y * (-rot_speed).sin();
                    plane_y = old_plane_x * (-rot_speed).sin() + plane_y * (-rot_speed).cos();
                }

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for x in 0..SCREEN_WIDTH {
            let camera_x = 2.0 * (x as f64) / (SCREEN_WIDTH as f64) - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;

            let mut map_x = pos_x as i32;
            let mut map_y = pos_y as i32;

            let mut side_dist_x: f64;
            let mut side_dist_y: f64;

            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();

            let perp_wall_dist: f64;
            let mut step_x = 0;
            let mut step_y = 0;
            let mut hit = false;
            let mut side: bool = false;

            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (pos_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - pos_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (pos_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - pos_y) * delta_dist_y;
            }

            while !hit {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = false;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = true;
                }
                if WORLD_MAP[map_x as usize][map_y as usize] > 0 {
                    hit = true
                }
            }

            if !side {
                perp_wall_dist = side_dist_x - delta_dist_x;
            } else {
                perp_wall_dist = side_dist_y - delta_dist_y;
            }

            let line_height = (SCREEN_HEIGHT as f64 / perp_wall_dist) as i32;

            let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
            if draw_end >= SCREEN_HEIGHT as i32 {
                draw_end = SCREEN_HEIGHT as i32 - 1
            }
            let mut color = get_wall_color(WORLD_MAP[map_x as usize][map_y as usize]);
            if side {
                color = Color::RGB(color.r / 2, color.g / 2, color.b / 2);
            }
            canvas.set_draw_color(color);
            canvas
                .draw_line((x as i32, draw_start), (x as i32, draw_end))
                .unwrap();
        }
        canvas.present();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
