extern crate sdl2;

use std::collections::HashSet;
use std::env;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color, PixelFormatEnum};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 960;
const TEXTURE_HEIGHT: usize = 64;
const TEXTURE_WIDTH: usize = 64;

const WORLD_MAP: [[i32; MAP_HEIGHT]; MAP_WIDTH] = [
    [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 7, 7, 7, 7, 7, 7, 7, 7,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 4, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 0, 7, 7, 7, 7, 7,
    ],
    [
        4, 0, 5, 0, 0, 0, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 6, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
    ],
    [
        4, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 8, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 0, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 1,
    ],
    [
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    [
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    ],
    [
        6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    [
        4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 6, 0, 6, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    ],
    [
        4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 5, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    ],
    [
        4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
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

fn load_texture(filename: &str) -> Vec<u32> {
    let decoder =
        png::Decoder::new(std::fs::File::open(filename).expect("Failed to open texture file"));
    let mut reader = decoder.read_info().expect("Failed to read PNG info");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .expect("Failed to decode PNG frame");

    let width = info.width as usize;
    let height = info.height as usize;
    if width != TEXTURE_WIDTH || height != TEXTURE_HEIGHT {
        panic!(
            "Texture {} has dimensions {}x{}, expected {}x{}",
            filename, width, height, TEXTURE_WIDTH, TEXTURE_HEIGHT
        );
    }

    let mut texture = Vec::with_capacity(TEXTURE_WIDTH * TEXTURE_HEIGHT);
    match info.color_type {
        png::ColorType::Rgba => {
            for i in (0..buf.len()).step_by(4) {
                let color = ((buf[i + 3] as u32) << 24)
                    | ((buf[i] as u32) << 16)
                    | ((buf[i + 1] as u32) << 8)
                    | (buf[i + 2] as u32);
                texture.push(color);
            }
        }
        png::ColorType::Rgb => {
            for i in (0..buf.len()).step_by(3) {
                let color = (255u32 << 24)
                    | ((buf[i] as u32) << 16)
                    | ((buf[i + 1] as u32) << 8)
                    | (buf[i + 2] as u32);
                texture.push(color);
            }
        }
        _ => panic!(
            "Unsupported color type {:?} in texture {}",
            info.color_type, filename
        ),
    }

    texture
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let textured = args.contains(&String::from("--textured"));
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Raycaster", WINDOW_WIDTH, WINDOW_HEIGHT)
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
    let mut pos_y = 11.50;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;

    let mut textures: Vec<Vec<u32>> = Vec::with_capacity(8);

    for _ in 0..8 {
        textures.push(vec![0; TEXTURE_HEIGHT * TEXTURE_WIDTH]);
    }

    if textured {
        textures[0] = load_texture("textures/eagle.png");
        textures[1] = load_texture("textures/redbrick.png");
        textures[2] = load_texture("textures/purplestone.png");
        textures[3] = load_texture("textures/greystone.png");
        textures[4] = load_texture("textures/bluestone.png");
        textures[5] = load_texture("textures/mossy.png");
        textures[6] = load_texture("textures/wood.png");
        textures[7] = load_texture("textures/colorstone.png");
    } else {
        for x in 0..TEXTURE_WIDTH {
            for y in 0..TEXTURE_HEIGHT {
                let xor_color = (x * 256 / TEXTURE_WIDTH) ^ (y * 256 / TEXTURE_HEIGHT);
                let y_color = y * 256 / TEXTURE_HEIGHT;
                let xy_color = y * 128 / TEXTURE_HEIGHT + x * 128 / TEXTURE_WIDTH;

                let idx = TEXTURE_WIDTH * y + x;

                textures[0][idx] = 65536 * 254 * ((x != y && x != TEXTURE_WIDTH - y) as u32);
                textures[1][idx] = (xy_color + 256 * xy_color + 65536 * xy_color) as u32;
                textures[2][idx] = (256 * xy_color + 65536 * xy_color) as u32;
                textures[3][idx] = (xor_color + 256 * xor_color + 65536 * xor_color) as u32;
                textures[4][idx] = (256 * xor_color) as u32;
                textures[5][idx] = 65536 * 192 * ((x % 16 != 0 && y % 16 != 0) as u32);
                textures[6][idx] = (65536 * y_color) as u32;
                textures[7][idx] = 128 + 256 * 128 + 65536 * 128;
            }
        }
    }

    let mut old_time = Instant::now();
    let mut pixel_buffer = Vec::with_capacity((SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize);
    pixel_buffer.resize((SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize, 0);

    let mut buffer = vec![0u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
    let texture_creator = canvas.texture_creator();
    let mut sdl_texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .expect("Failed to create texture");
    'main: loop {
        let frame_time = old_time.elapsed().as_secs_f64();
        old_time = Instant::now();
        buffer.fill(0);

        canvas
            .window_mut()
            .set_title(&format!("Raycaster - {:.2} FPS", 1.0 / frame_time))
            .unwrap();

        let move_speed = frame_time * 5.0;
        let rot_speed = frame_time * 3.0;

        let pressed_keys: HashSet<_> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        if pressed_keys.contains(&Keycode::W) || pressed_keys.contains(&Keycode::UP) {
            if WORLD_MAP[(pos_x + dir_x * move_speed) as usize][pos_y as usize] == 0 {
                pos_x += dir_x * move_speed;
            }
            if WORLD_MAP[pos_x as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                pos_y += dir_y * move_speed;
            }
        }

        if pressed_keys.contains(&Keycode::S) || pressed_keys.contains(&Keycode::DOWN) {
            if WORLD_MAP[(pos_x - dir_x * move_speed) as usize][pos_y as usize] == 0 {
                pos_x -= dir_x * move_speed;
            }
            if WORLD_MAP[pos_x as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                pos_y -= dir_y * move_speed;
            }
        }

        if pressed_keys.contains(&Keycode::A) || pressed_keys.contains(&Keycode::LEFT) {
            let old_dir_x = dir_x;
            dir_x = dir_x * rot_speed.cos() - dir_y * rot_speed.sin();
            dir_y = old_dir_x * rot_speed.sin() + dir_y * rot_speed.cos();
            let old_plane_x = plane_x;
            plane_x = plane_x * rot_speed.cos() - plane_y * rot_speed.sin();
            plane_y = old_plane_x * rot_speed.sin() + plane_y * rot_speed.cos();
        }

        if pressed_keys.contains(&Keycode::D) || pressed_keys.contains(&Keycode::RIGHT) {
            let old_dir_x = dir_x;
            dir_x = dir_x * (-rot_speed).cos() - dir_y * (-rot_speed).sin();
            dir_y = old_dir_x * (-rot_speed).sin() + dir_y * (-rot_speed).cos();
            let old_plane_x = plane_x;
            plane_x = plane_x * (-rot_speed).cos() - plane_y * (-rot_speed).sin();
            plane_y = old_plane_x * (-rot_speed).sin() + plane_y * (-rot_speed).cos();
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

            let pitch = 0;

            let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2 + pitch;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2 + pitch;
            if draw_end >= SCREEN_HEIGHT as i32 {
                draw_end = SCREEN_HEIGHT as i32 - 1
            }

            let texture_number = WORLD_MAP[map_x as usize][map_y as usize] - 1;

            let mut wall_x = if side {
                pos_x + perp_wall_dist * ray_dir_x
            } else {
                pos_y + perp_wall_dist * ray_dir_y
            };
            wall_x -= wall_x.floor();

            let mut tex_x = (wall_x * TEXTURE_WIDTH as f64) as i32;

            if !side && ray_dir_x > 0.0 {
                tex_x = TEXTURE_WIDTH as i32 - tex_x - 1;
            }
            if side && ray_dir_y < 0.0 {
                tex_x = TEXTURE_WIDTH as i32 - tex_x - 1;
            }

            let step = 1.0 * TEXTURE_HEIGHT as f64 / line_height as f64;
            let mut tex_pos: f64 =
                (draw_start - pitch - SCREEN_HEIGHT as i32 / 2 + line_height / 2) as f64 * step;

            for y in draw_start..draw_end {
                let tex_y = (tex_pos as i32) & (TEXTURE_HEIGHT as i32 - 1);
                tex_pos += step;

                let mut color = textures[texture_number as usize]
                    [(TEXTURE_HEIGHT as i32 * tex_y + tex_x) as usize];

                if side {
                    color = (color >> 1) & 0x7F7F7F;
                }
                buffer[y as usize * SCREEN_WIDTH as usize + x as usize] = color;
            }
        }
        sdl_texture
            .with_lock(None, |pixels: &mut [u8], pitch: usize| {
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        let i = y * SCREEN_WIDTH + x;
                        let color = buffer[i as usize];

                        let offset = (y * pitch as u32 + x * 4) as usize;
                        pixels[offset..offset + 4].copy_from_slice(&color.to_ne_bytes());
                    }
                }
            })
            .expect("Failed to lock texture");
        canvas
            .copy(&sdl_texture, None, None)
            .expect("Render failed");
        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
