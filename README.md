# Smart Road Intersection 🚗

A traffic simulation featuring autonomous vehicles (AVs) navigating a smart intersection without traditional traffic lights. This project demonstrates an intelligent traffic management algorithm that prevents collisions while minimizing congestion.

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

### Building the Project

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Golden76z/smart-road
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
├── Cargo.toml                 # Project configuration and dependencies
├── README.md                  # Project documentation
├── ROADMAP.md                 # Development roadmap
├── ISSUES.md                  # Task breakdown and known issues
└── src/
    ├── main.rs                # Main entry point – initializes simulation, rendering, and settings

    ├── config/                # Game and simulation configuration modules
    │   ├── mod.rs             # Module declarations for config
    │   ├── broadcast.rs       # Logging and message broadcasting system
    │   ├── constants.rs       # All constants (spawn points, destinations, velocities, etc.)
    │   ├── game_settings.rs   # Central configuration and state management
    │   ├── keyboard.rs        # Input key mapping configuration
    │   ├── spawn_manager.rs   # Spawn cooldown and vehicle spawn control
    │   ├── statistics.rs      # Statistics and performance tracking
    │   ├── types.rs           # Common enums and types (Lane, Direction, etc.)
    │   └── ui.rs              # Configuration for UI layout and visual elements

    ├── render/                # Rendering and UI display system
    │   ├── mod.rs             # Module declarations for rendering
    │   ├── renderer.rs        # Main renderer handling frame updates
    │   ├── map.rs             # Map drawing and layout
    │   ├── panel.rs           # Info and statistics panels
    │   ├── keybinds_panel.rs  # Panel displaying control keybinds
    │   ├── textures.rs        # Texture loading and sprite management
    │   ├── ui.rs              # Rendering of interface elements (buttons, labels, etc.)
    │   └── vehicle.rs         # Vehicle sprite rendering and animation

    └── simulation/            # Core simulation logic
        ├── mod.rs             # Module declarations for simulation
        ├── input.rs           # Runtime input handling
        ├── intersection.rs    # Smart intersection logic and traffic rules
        ├── movement.rs        # Vehicle movement and lane updates
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
