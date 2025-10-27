# z01 - Smart Road - Rust school project

A traffic simulation featuring autonomous vehicles (AVs) navigating a smart intersection without traditional traffic lights. This project demonstrates an intelligent traffic management algorithm that prevents collisions while minimizing congestion.

## Features

### Core Functionality

- **Smart Intersection Algorithm**: Vehicles automatically adjust speed based on traffic conditions
- **Multiple Vehicle Routes**: Each lane supports left turns, straight, and right turns
- **Real-time Physics**: Velocity = Distance / Time calculations for realistic movement
- **Safety Distance Management**: Vehicles maintain safe distances to prevent collisions
- **Close Call Detection**: Tracks near-misses for safety analysis
- **Comprehensive Statistics**: Detailed metrics on vehicle performance and safety

### Controls

| Key | Action |
|-----|--------|
| `R` | Toggle **Random Mode** |
| `M` | Toggle **Manual Mode** |
| `ESC` | Exit simulation and display statistics |

#### Random Mode
In **Random Mode**, pressing a directional key spawns a vehicle entering from that direction and heading toward a **random destination**.

| Key | Spawns Vehicle From | Heading |
|-----|----------------------|----------|
| `↑` Arrow Up | South | Random direction |
| `↓` Arrow Down | North | Random direction |
| `→` Arrow Right | West | Random direction |
| `←` Arrow Left | East | Random direction |

#### Manual Mode
In **Manual Mode**, you first select **which lane** the vehicle will enter from using the directional keys,  
then choose **where it will go** using number keys:

| Step | Key | Action |
|------|-----|--------|
| 1️⃣ | `↑` / `↓` / `→` / `←` | Select lane (vehicle entry direction) |
| 2️⃣ | `1` | Set destination **West** (left turn) |
|  | `2` | Set destination **Forward** (straight ahead) |
|  | `3` | Set destination **East** (right turn) |

### Statistics Tracked

- **Total Vehicles Passed**: Total count of vehicles that completed the intersection
- **Max/Min Velocity**: Fastest and slowest speeds achieved
- **Max/Min Time**: Longest and shortest times to pass through the intersection
- **Close Calls**: Number of safety distance violations

### Building the Project

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Golden76z/smart-road
   cd smart-road/src/simulation
   ```

2. **Run the simulation:**
   ```bash
   cargo run
   ```

## Project Structure

```
smart-road/
├── Cargo.toml                 # Project configuration and dependencies
├── README.md                  # Project documentation
└── src/
    ├── main.rs                # Main entry point – initializes simulation, rendering, and settings

    ├── config/
    │   ├── mod.rs
    │   ├── broadcast.rs       # Logging and message broadcasting system
    │   ├── constants.rs       # All constants (spawn points, destinations, velocities, etc.)
    │   ├── game_settings.rs   # Central configuration and state management
    │   ├── keyboard.rs        # Input key mapping configuration
    │   ├── spawn_manager.rs   # Spawn cooldown and vehicle spawn control
    │   ├── statistics.rs      # Statistics and performance tracking
    │   ├── types.rs           # Common enums and types (Lane, Direction, etc.)
    │   └── ui.rs              # Configuration for UI layout and visual elements

    ├── render/
    │   ├── mod.rs
    │   ├── renderer.rs        # Main renderer handling frame updates
    │   ├── map.rs             # Map drawing and layout
    │   ├── panel.rs           # Info and statistics panels
    │   ├── keybinds_panel.rs  # Panel displaying control keybinds
    │   ├── textures.rs        # Texture loading and sprite management
    │   ├── ui.rs              # Rendering of interface elements
    │   └── vehicle.rs         # Vehicle sprite & hitbox rendering

    └── simulation/
        ├── mod.rs
        ├── input.rs           # Runtime input handling
        ├── movement.rs        # Main loop update for vehicles
        ├── utils.rs           # Utility functions for simulation
        └── vehicle.rs         # Vehicle physics and behavior logic
```

## License

See [LICENSE](LICENSE) for details.

## Acknowledgments

- Inspired by the Rust Piscine `road_intersection` project
- Built with [SDL2](https://www.libsdl.org/) for cross-platform graphics

---

**Note**: This is an educational project demonstrating algorithmic thinking, Rust programming, and game development concepts. It is not intended for real-world traffic simulation.
