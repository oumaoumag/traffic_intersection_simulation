use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::constants::TRAFFIC_LIGHT_SIZE;
use crate::models::direction::Direction;

// Traffic light state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLightState {
    Red,
    Green,
}

// Traffic light struct
pub struct TrafficLight {
    pub position: Point,
    pub state: TrafficLightState,
    pub direction: Direction,
}

impl TrafficLight {
    pub fn new(position: Point, direction: Direction) -> Self {
        TrafficLight {
            position,
            state: TrafficLightState::Red,
            direction,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let color = match self.state {
            TrafficLightState::Red => Color::RGB(255, 0, 0),
            TrafficLightState::Green => Color::RGB(0, 255, 0),
        };

        canvas.set_draw_color(color);
        canvas
            .fill_rect(Rect::new(
                self.position.x,
                self.position.y,
                TRAFFIC_LIGHT_SIZE,
                TRAFFIC_LIGHT_SIZE,
            ))
            .expect("Failed to render traffic light");
    }
}
