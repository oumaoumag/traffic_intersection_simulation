use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::time::Instant;

use rand::Rng;
use road_intersection::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use road_intersection::models::direction::Direction;
use road_intersection::simulation::intersection::Intersection;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Road Intersection", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut intersection = Intersection::new();

    // Enhancement variables
    let mut is_paused = false;
    let mut simulation_speed = 1.0f32;
    let mut debug_mode = false;
    let mut confirm_exit = false;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    if confirm_exit {
                        break 'running;
                    } else {
                        confirm_exit = true;
                        println!("Press Escape again to exit");
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    if confirm_exit {
                        break 'running;
                    } else {
                        confirm_exit = true;
                        println!("Press Escape again to exit");
                    }
                }
                Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                    match keycode {
                        Keycode::Up => intersection.spawn_vehicle(Direction::South),
                        Keycode::Down => intersection.spawn_vehicle(Direction::North),
                        Keycode::Left => intersection.spawn_vehicle(Direction::East),
                        Keycode::Right => intersection.spawn_vehicle(Direction::West),
                        Keycode::R => {
                            let mut rng = rand::thread_rng();
                            let direction = match rng.gen_range(0..4) {
                                0 => Direction::North,
                                1 => Direction::South,
                                2 => Direction::East,
                                _ => Direction::West,
                            };
                            intersection.spawn_vehicle(direction);
                        },
                        // Enhancement: Pause/Resume
                        Keycode::Space => {
                            is_paused = !is_paused;
                            println!("Simulation {}", if is_paused { "PAUSED" } else { "RESUMED" });
                        },
                        // Enhancement: Debug Mode
                        Keycode::D => {
                            debug_mode = !debug_mode;
                            println!("Debug Mode {}", if debug_mode { "ON" } else { "OFF" });
                        },
                        // Enhancement: Speed Control
                        Keycode::Equals | Keycode::KpPlus => {
                            simulation_speed = f32::min(simulation_speed * 1.5, 5.0);
                            println!("Speed: {:.1}x", simulation_speed);
                        },
                        Keycode::Minus | Keycode::KpMinus => {
                            simulation_speed = f32::max(simulation_speed / 1.5, 0.25);
                            println!("Speed: {:.1}x", simulation_speed);
                        },
                        _ => {}
                    }
                    confirm_exit = false; // Reset exit confirmation on any other key press
                }
                _ => {}
            }
        }

        // Update simulation if not paused
        if !is_paused {
            intersection.update();
        }

        // Render
        canvas.set_draw_color(Color::RGB(0, 128, 0)); // Green background for grass
        canvas.clear();

        intersection.render(&mut canvas);

        // Render debug information if debug mode is on
        if debug_mode {
            // Draw a simple indicator that debug mode is on
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(sdl2::rect::Rect::new(10, 10, 20, 20))
                .expect("Failed to render debug indicator");
        }

        canvas.present();

        // Cap FPS based on simulation speed
        std::thread::sleep(Duration::new(0, (1_000_000_000f64 / (60.0 * simulation_speed as f64)) as u32));
    }

    Ok(())
}
