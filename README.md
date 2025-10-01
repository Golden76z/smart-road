# Smart Road Intersection 🚗

A traffic simulation featuring autonomous vehicles (AVs) navigating a smart intersection without traditional traffic lights. This project demonstrates an intelligent traffic management algorithm that prevents collisions while minimizing congestion.

## Overview

Traditional traffic lights and current intersection management strategies are designed for human drivers. With the advent of autonomous vehicles, new traffic strategies must be developed. This simulation implements a smart intersection management system that:

- **Prevents collisions** through predictive vehicle coordination
- **Minimizes congestion** by dynamically adjusting vehicle speeds
- **Tracks safety metrics** including close calls and timing statistics
- **Simulates realistic physics** with velocity, distance, and time calculations

## Features

### ✨ Core Functionality

- **Smart Intersection Algorithm**: Vehicles automatically adjust speed based on traffic conditions
- **Multiple Vehicle Routes**: Each lane supports left turns, straight, and right turns
- **Real-time Physics**: Velocity = Distance / Time calculations for realistic movement
- **Safety Distance Management**: Vehicles maintain safe distances to prevent collisions
- **Close Call Detection**: Tracks near-misses for safety analysis
- **Comprehensive Statistics**: Detailed metrics on vehicle performance and safety

### 🎮 Controls

| Key | Action |
|-----|--------|
| `↑` Arrow Up | Spawn vehicle from South (going North) |
| `↓` Arrow Down | Spawn vehicle from North (going South) |
| `→` Arrow Right | Spawn vehicle from West (going East) |
| `←` Arrow Left | Spawn vehicle from East (going West) |
| `R` | Toggle random vehicle generation |
| `ESC` | Exit simulation and display statistics |

### 📊 Statistics Tracked

- **Max Concurrent Vehicles**: Highest number of vehicles in the intersection simultaneously
- **Total Vehicles Passed**: Total count of vehicles that completed the intersection
- **Max/Min Velocity**: Fastest and slowest speeds achieved
- **Max/Min Time**: Longest and shortest times to pass through the intersection
- **Close Calls**: Number of safety distance violations

## Installation

### Prerequisites

- **Rust**: Version 1.70 or higher ([Install Rust](https://www.rust-lang.org/tools/install))
- **SDL2**: Required for rendering and window management

#### Installing SDL2

**Ubuntu/Debian:**
```bash
sudo apt-get install libsdl2-dev
```

**Fedora:**
```bash
sudo dnf install SDL2-devel
```

**macOS (using Homebrew):**
```bash
brew install sdl2
```

**Arch Linux:**
```bash
sudo pacman -S sdl2
```

**Windows:**
- Download SDL2 development libraries from [SDL2 releases](https://github.com/libsdl-org/SDL/releases)
- Follow the [Rust-SDL2 Windows installation guide](https://github.com/Rust-SDL2/rust-sdl2#windows)

### Building the Project

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd smart-road
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the simulation:**
   ```bash
   cargo run --release
   ```

## Project Structure

```
smart-road/
├── Cargo.toml              # Project configuration and dependencies
├── README.md               # This file
├── ROADMAP.md             # Development roadmap
├── ISSUES.md              # GitHub issues for task breakdown
└── src/
    ├── main.rs            # Main game loop and initialization
    ├── constants.rs       # Configuration constants
    ├── types.rs           # Common types and enums
    ├── vehicle.rs         # Vehicle physics and behavior
    ├── intersection.rs    # Smart intersection algorithm
    ├── statistics.rs      # Statistics tracking
    ├── input.rs           # Keyboard input handling
    └── renderer.rs        # Rendering and animation
```

## How It Works

### Architecture

The simulation is built with a modular architecture:

1. **Main Loop** (`main.rs`): Orchestrates the simulation at 60 FPS
2. **Intersection Manager** (`intersection.rs`): Implements the core traffic algorithm
3. **Vehicle Physics** (`vehicle.rs`): Handles movement, rotation, and pathfinding
4. **Renderer** (`renderer.rs`): Draws the intersection and vehicles
5. **Statistics** (`statistics.rs`): Tracks and reports performance metrics

### Smart Intersection Algorithm

The algorithm prevents collisions through several strategies:

1. **Collision Course Detection**: Identifies when vehicles' paths will intersect
2. **Dynamic Speed Adjustment**: Slows down vehicles when conflicts are detected
3. **Safe Distance Maintenance**: Ensures minimum spacing between vehicles
4. **Lane-Aware Logic**: Vehicles in the same lane maintain extra spacing
5. **Priority System**: Vehicles closer to the intersection have higher priority

### Vehicle Physics

Each autonomous vehicle follows realistic physics:

- **Position**: Updated based on velocity and delta time
- **Velocity**: Three states (Slow: 20px/s, Normal: 50px/s, Fast: 80px/s)
- **Rotation**: Smooth turning animation through the intersection
- **Route Following**: Vehicles adhere to their assigned lane and direction

### Animation

The renderer provides:

- **Road Network**: Cross-intersection with lane markings
- **Vehicle Sprites**: Color-coded by direction for easy tracking
- **Direction Indicators**: Yellow markers show vehicle facing direction
- **Smooth Movement**: 60 FPS animation with interpolation

## Configuration

Customize the simulation by editing `src/constants.rs`:

```rust
// Adjust vehicle speeds
pub const MIN_VELOCITY: f64 = 20.0;
pub const NORMAL_VELOCITY: f64 = 50.0;
pub const MAX_VELOCITY: f64 = 80.0;

// Modify safety parameters
pub const SAFE_DISTANCE: f64 = 100.0;
pub const CLOSE_CALL_THRESHOLD: f64 = 50.0;

// Change spawn rates
pub const RANDOM_SPAWN_COOLDOWN: u32 = 120; // frames
```

## Development

### Debug Mode

Enable debug output by setting in `src/constants.rs`:

```rust
pub const DEBUG_MODE: bool = true;
pub const SHOW_HITBOXES: bool = true;
```

## Roadmap

See [ROADMAP.md](ROADMAP.md) for planned features and development phases.

## Contributing

See [ISSUES.md](ISSUES.md) for a breakdown of tasks suitable for contributions.

## License

See [LICENSE](LICENSE) for details.

## Acknowledgments

- Inspired by the Rust Piscine `road_intersection` project
- Built with [SDL2](https://www.libsdl.org/) for cross-platform graphics

---

**Note**: This is an educational project demonstrating algorithmic thinking, Rust programming, and game development concepts. It is not intended for real-world traffic simulation.
