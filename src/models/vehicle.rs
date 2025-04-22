use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::constants::{
    LANE_WIDTH, VEHICLE_HEIGHT, VEHICLE_SAFE_DISTANCE, VEHICLE_SPEED, VEHICLE_WIDTH, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use crate::models::direction::Direction;
use crate::models::route::Route;
use crate::models::traffic_light::{TrafficLight, TrafficLightState};

// Vehicle struct
#[derive(Debug, Clone)]
pub struct Vehicle {
    pub position: (f32, f32),
    pub direction: Direction,
    pub route: Route,
    pub color: Color,
    pub has_passed_intersection: bool,
}

impl Vehicle {
    pub fn new(direction: Direction, route: Route) -> Self {
        let position = match direction {
            Direction::North => (
                WINDOW_WIDTH as f32 / 2.0 - LANE_WIDTH as f32 / 2.0,
                WINDOW_HEIGHT as f32,
            ),
            Direction::South => (
                WINDOW_WIDTH as f32 / 2.0 + LANE_WIDTH as f32 / 2.0,
                0.0,
            ),
            Direction::East => (
                0.0,
                WINDOW_HEIGHT as f32 / 2.0 + LANE_WIDTH as f32 / 2.0,
            ),
            Direction::West => (
                WINDOW_WIDTH as f32,
                WINDOW_HEIGHT as f32 / 2.0 - LANE_WIDTH as f32 / 2.0,
            ),
        };

        // Assign color based on route
        let color = match route {
            Route::Left => Color::RGB(255, 255, 0),   // Yellow
            Route::Straight => Color::RGB(0, 0, 255), // Blue
            Route::Right => Color::RGB(0, 255, 255),  // Cyan
        };

        Vehicle {
            position,
            direction,
            route,
            color,
            has_passed_intersection: false,
        }
    }

    pub fn update(&mut self, traffic_lights: &[TrafficLight], vehicles: &[Vehicle]) {
        // Check if vehicle should stop at traffic light
        let should_stop_at_light = self.should_stop_at_traffic_light(traffic_lights);

        // Check if vehicle should stop for another vehicle
        let should_stop_for_vehicle = self.should_stop_for_vehicle(vehicles);

        if !should_stop_at_light && !should_stop_for_vehicle {
            // Move the vehicle based on its direction
            match self.direction {
                Direction::North => self.position.1 -= VEHICLE_SPEED,
                Direction::South => self.position.1 += VEHICLE_SPEED,
                Direction::East => self.position.0 += VEHICLE_SPEED,
                Direction::West => self.position.0 -= VEHICLE_SPEED,
            }

            // Check if vehicle has passed the intersection
            if !self.has_passed_intersection {
                let intersection_center_x = WINDOW_WIDTH as f32 / 2.0;
                let intersection_center_y = WINDOW_HEIGHT as f32 / 2.0;

                match self.direction {
                    Direction::North => {
                        if self.position.1 < intersection_center_y {
                            self.has_passed_intersection = true;
                            self.change_direction_based_on_route();
                        }
                    }
                    Direction::South => {
                        if self.position.1 > intersection_center_y {
                            self.has_passed_intersection = true;
                            self.change_direction_based_on_route();
                        }
                    }
                    Direction::East => {
                        if self.position.0 > intersection_center_x {
                            self.has_passed_intersection = true;
                            self.change_direction_based_on_route();
                        }
                    }
                    Direction::West => {
                        if self.position.0 < intersection_center_x {
                            self.has_passed_intersection = true;
                            self.change_direction_based_on_route();
                        }
                    }
                }
            }
        }
    }

    fn change_direction_based_on_route(&mut self) {
        let intersection_center_x = WINDOW_WIDTH as f32 / 2.0;
        let intersection_center_y = WINDOW_HEIGHT as f32 / 2.0;
        let lane_offset = LANE_WIDTH as f32 / 2.0;

        match (self.direction, self.route) {
            (Direction::North, Route::Left) => {
                self.direction = Direction::West;
                self.position.1 = intersection_center_y - lane_offset;
            }
            (Direction::North, Route::Right) => {
                self.direction = Direction::East;
                self.position.1 = intersection_center_y + lane_offset;
            }
            (Direction::South, Route::Left) => {
                self.direction = Direction::East;
                self.position.1 = intersection_center_y + lane_offset;
            }
            (Direction::South, Route::Right) => {
                self.direction = Direction::West;
                self.position.1 = intersection_center_y - lane_offset;
            }
            (Direction::East, Route::Left) => {
                self.direction = Direction::North;
                self.position.0 = intersection_center_x - lane_offset;
            }
            (Direction::East, Route::Right) => {
                self.direction = Direction::South;
                self.position.0 = intersection_center_x + lane_offset;
            }
            (Direction::West, Route::Left) => {
                self.direction = Direction::South;
                self.position.0 = intersection_center_x + lane_offset;
            }
            (Direction::West, Route::Right) => {
                self.direction = Direction::North;
                self.position.0 = intersection_center_x - lane_offset;
            }
            _ => {}, // Straight, no change
        }
    }

    fn should_stop_at_traffic_light(&self, traffic_lights: &[TrafficLight]) -> bool {
        if self.has_passed_intersection {
            return false;
        }

        // Find the traffic light for this vehicle's direction
        for light in traffic_lights {
            if light.direction == self.direction {
                if light.state == TrafficLightState::Red {
                    // Calculate distance to intersection
                    let intersection_center_x = WINDOW_WIDTH as f32 / 2.0;
                    let intersection_center_y = WINDOW_HEIGHT as f32 / 2.0;

                    match self.direction {
                        Direction::North => {
                            let distance_to_intersection = self.position.1 - intersection_center_y;
                            if distance_to_intersection > 0.0
                                && distance_to_intersection < VEHICLE_SAFE_DISTANCE
                            {
                                return true;
                            }
                        }
                        Direction::South => {
                            let distance_to_intersection = intersection_center_y - self.position.1;
                            if distance_to_intersection > 0.0
                                && distance_to_intersection < VEHICLE_SAFE_DISTANCE
                            {
                                return true;
                            }
                        }
                        Direction::East => {
                            let distance_to_intersection = intersection_center_x - self.position.0;
                            if distance_to_intersection > 0.0
                                && distance_to_intersection < VEHICLE_SAFE_DISTANCE
                            {
                                return true;
                            }
                        }
                        Direction::West => {
                            let distance_to_intersection = self.position.0 - intersection_center_x;
                            if distance_to_intersection > 0.0
                                && distance_to_intersection < VEHICLE_SAFE_DISTANCE
                            {
                                return true;
                            }
                        }
                    }
                }
                break;
            }
        }

        false
    }

    fn should_stop_for_vehicle(&self, vehicles: &[Vehicle]) -> bool {
        for other in vehicles {
            // Skip self comparison
            if std::ptr::eq(self, other) {
                continue;
            }

            // Only check vehicles in the same direction and lane
            if self.direction == other.direction {
                match self.direction {
                    Direction::North => {
                        if (self.position.0 - other.position.0).abs() < VEHICLE_WIDTH as f32
                            && self.position.1 > other.position.1
                            && self.position.1 - other.position.1 < VEHICLE_SAFE_DISTANCE
                        {
                            return true;
                        }
                    }
                    Direction::South => {
                        if (self.position.0 - other.position.0).abs() < VEHICLE_WIDTH as f32
                            && self.position.1 < other.position.1
                            && other.position.1 - self.position.1 < VEHICLE_SAFE_DISTANCE
                        {
                            return true;
                        }
                    }
                    Direction::East => {
                        if (self.position.1 - other.position.1).abs() < VEHICLE_HEIGHT as f32
                            && self.position.0 < other.position.0
                            && other.position.0 - self.position.0 < VEHICLE_SAFE_DISTANCE
                        {
                            return true;
                        }
                    }
                    Direction::West => {
                        if (self.position.1 - other.position.1).abs() < VEHICLE_HEIGHT as f32
                            && self.position.0 > other.position.0
                            && self.position.0 - other.position.0 < VEHICLE_SAFE_DISTANCE
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);

        let (width, height) = match self.direction {
            Direction::North | Direction::South => (VEHICLE_WIDTH, VEHICLE_HEIGHT),
            Direction::East | Direction::West => (VEHICLE_HEIGHT, VEHICLE_WIDTH),
        };

        canvas
            .fill_rect(Rect::new(
                self.position.0 as i32 - (width / 2) as i32,
                self.position.1 as i32 - (height / 2) as i32,
                width,
                height,
            ))
            .expect("Failed to render vehicle");
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.position.0 < -50.0
            || self.position.0 > WINDOW_WIDTH as f32 + 50.0
            || self.position.1 < -50.0
            || self.position.1 > WINDOW_HEIGHT as f32 + 50.0
    }
}
