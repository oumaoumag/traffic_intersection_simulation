# Road Intersection Simulation

A dynamic traffic intersection simulation built in Rust that models a two-way road crossing with autonomous vehicles and traffic light controls. The simulation provides an interactive environment for visualizing traffic flow, managing congestion, and demonstrating collision avoidance.

## Features

- Interactive vehicle spawning from four directions
- Autonomous vehicle movement with intelligent routing
- Traffic light system with alternating cycles
- Real-time collision avoidance
- Multiple vehicle routing options (left, straight, right)
- Debug mode for development
- Adjustable simulation speed
- Pause/Resume functionality

## Prerequisites

- Rust (latest stable version)
- SDL2 development libraries

### Installing SDL2

#### Ubuntu/Debian
```bash
sudo apt-get install libsdl2-dev
```

#### macOS
```bash
brew install sdl2
```

#### Windows
Download SDL2 development libraries from [SDL2's website](https://www.libsdl.org/download-2.0.php) and set up according to SDL2 documentation.

## Building and Running

1. Clone the repository:
```bash
git clone https://github.com/yourusername/road_intersection.git
cd road_intersection
```

2. Build the project:
```bash
cargo build --release
```

3. Run the simulation:
```bash
cargo run --release
```

## Controls

- **Arrow Keys**: Spawn vehicles from different directions
- **R**: Spawn vehicle from random direction
- **Space**: Pause/Resume simulation
- **D**: Toggle debug mode
- **+/-**: Adjust simulation speed
- **Esc** (press twice): Exit application

## Vehicle Routes

Vehicles are color-coded based on their intended route:
- **Yellow**: Left turn
- **Blue**: Straight
- **Cyan**: Right turn

## Project Structure

```
src/
├── main.rs           # Application entry point
├── lib.rs           # Library root
├── constants.rs     # Global constants
├── models/         # Core data structures
│   ├── direction.rs
│   ├── route.rs
│   ├── traffic_light.rs
│   └── vehicle.rs
└── simulation/     # Simulation logic
    └── intersection.rs
```

## Configuration

Key simulation parameters can be adjusted in `src/constants.rs`:
- Window dimensions
- Road and lane widths
- Vehicle properties
- Traffic light timing

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- SDL2 Rust community
- Rust game development community
