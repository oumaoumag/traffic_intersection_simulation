use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Instant;

use crate::constants::{
    LANE_WIDTH, ROAD_WIDTH, TRAFFIC_LIGHT_CYCLE_TIME, TRAFFIC_LIGHT_SIZE, VEHICLE_SAFE_DISTANCE, WINDOW_HEIGHT, WINDOW_WIDTH
};
use crate::models::direction::Direction;
use crate::models::route::Route;
use crate::models::traffic_light::{TrafficLight, TrafficLightState};
use crate::models::vehicle::Vehicle;

// Intersection struct to manage the simulation
pub struct Intersection {
    pub traffic_lights: Vec<TrafficLight>,
    pub vehicles: Vec<Vehicle>,
    pub last_traffic_light_change: Instant,
    pub last_vehicle_spawn: [Instant; 4], // One for each direction
}

impl Intersection {
    pub fn new() -> Self {
        // Create traffic lights for each direction
        let traffic_lights = vec![
            TrafficLight::new(
                Point::new(
                    (WINDOW_WIDTH / 2 - ROAD_WIDTH / 2 - TRAFFIC_LIGHT_SIZE) as i32,
                    (WINDOW_HEIGHT / 2 + ROAD_WIDTH / 2) as i32,
                ),
                Direction::North,
            ),
            TrafficLight::new(
                Point::new(
                    (WINDOW_WIDTH / 2 + ROAD_WIDTH / 2) as i32,
                    (WINDOW_HEIGHT / 2  - ROAD_WIDTH / 2 - TRAFFIC_LIGHT_SIZE) as i32,
                ),
                Direction::South,
            ),
            TrafficLight::new(
                Point::new(
                    (WINDOW_WIDTH / 2 - ROAD_WIDTH / 2 - TRAFFIC_LIGHT_SIZE) as i32,
                    (WINDOW_HEIGHT / 2 - ROAD_WIDTH / 2 - TRAFFIC_LIGHT_SIZE) as i32,
                ),
                Direction::East,
            ),
            TrafficLight::new(
                Point::new(
                    (WINDOW_WIDTH / 2 + ROAD_WIDTH / 2 ) as i32,
                    (WINDOW_HEIGHT / 2 + ROAD_WIDTH / 2 ) as i32,
                ),
                Direction::West,
            ),
        ];

        // Set initial traffic light states (North-South green, East-West red)
        let mut intersection = Intersection {
            traffic_lights,
            vehicles: Vec::new(),
            last_traffic_light_change: Instant::now(),
            last_vehicle_spawn: [Instant::now(); 4],
        };

        intersection.update_traffic_lights();
        intersection
    }

    pub fn update(&mut self) {
        // Update traffic lights
        if self.last_traffic_light_change.elapsed().as_millis() > TRAFFIC_LIGHT_CYCLE_TIME as u128 {
            self.update_traffic_lights();
            self.last_traffic_light_change = Instant::now();
        }

        // Update vehicles
        let vehicles_clone = self.vehicles.clone();
        for vehicle in &mut self.vehicles {
            vehicle.update(&self.traffic_lights, &vehicles_clone);
        }

        // Remove vehicles that are out of bounds
        self.vehicles.retain(|v| !v.is_out_of_bounds());
    }

    fn update_traffic_lights(&mut self) {
        // Simple traffic light algorithm: alternate between North-South and East-West
        let north_south_green = self.traffic_lights[0].state == TrafficLightState::Green;

        for light in &mut self.traffic_lights {
            match light.direction {
                Direction::North | Direction::South => {
                    light.state = if north_south_green {
                        TrafficLightState::Red
                    } else {
                        TrafficLightState::Green
                    };
                }
                Direction::East | Direction::West => {
                    light.state = if north_south_green {
                        TrafficLightState::Green
                    } else {
                        TrafficLightState::Red
                    };
                }
            }
        }
    }

    pub fn spawn_vehicle(&mut self, direction: Direction) {
        let dir_index = match direction {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        };

        // Check if enough time has passed since the last spawn in this direction
        if self.last_vehicle_spawn[dir_index].elapsed().as_millis() < 1000 {
            return;
        }

        // Check if there's already a vehicle too close to the spawn point
        for vehicle in &self.vehicles {
            if vehicle.direction == direction {
                let too_close = match direction {
                    Direction::North => vehicle.position.1 > WINDOW_HEIGHT as f32 - VEHICLE_SAFE_DISTANCE,
                    Direction::South => vehicle.position.1 < VEHICLE_SAFE_DISTANCE,
                    Direction::East => vehicle.position.0 < VEHICLE_SAFE_DISTANCE,
                    Direction::West => vehicle.position.0 > WINDOW_WIDTH as f32 - VEHICLE_SAFE_DISTANCE,
                };

                if too_close {
                    return;
                }
            }
        }

        // Generate a random route
        let mut rng = rand::thread_rng();
        let route = match rng.gen_range(0..3) {
            0 => Route::Left,
            1 => Route::Straight,
            _ => Route::Right,
        };

        // Create and add the new vehicle
        self.vehicles.push(Vehicle::new(direction, route));
        self.last_vehicle_spawn[dir_index] = Instant::now();
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // Render roads
        canvas.set_draw_color(Color::RGB(100, 100, 100)); // Road color
        canvas
            .fill_rect(Rect::new(
                (WINDOW_WIDTH / 2 - ROAD_WIDTH / 2) as i32,
                0,
                ROAD_WIDTH,
                WINDOW_HEIGHT,
            ))
            .expect("Failed to render vertical road");
        canvas
            .fill_rect(Rect::new(
                0,
                (WINDOW_HEIGHT / 2 - ROAD_WIDTH / 2) as i32,
                WINDOW_WIDTH,
                ROAD_WIDTH,
            ))
            .expect("Failed to render horizontal road");

        // Render lane markings
        canvas.set_draw_color(Color::RGB(255, 255, 255)); // White for lane markings

        // Vertical road lane marking
        canvas
            .fill_rect(Rect::new(
                (WINDOW_WIDTH / 2) as i32 - 2,
                0,
                4,
                WINDOW_HEIGHT,
            ))
            .expect("Failed to render vertical lane marking");

        // Horizontal road lane marking
        canvas
            .fill_rect(Rect::new(
                0,
                (WINDOW_HEIGHT / 2) as i32 - 2,
                WINDOW_WIDTH,
                4,
            ))
            .expect("Failed to render horizontal lane marking");

        // Render traffic lights
        for light in &self.traffic_lights {
            light.render(canvas);
        }

        // Render vehicles
        for vehicle in &self.vehicles {
            vehicle.render(canvas);
        }
    }
}
