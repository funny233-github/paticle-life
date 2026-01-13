# Particle Life

A particle life simulation built with the Bevy game engine in Rust. This project simulates the behavior of particles with configurable interactions, creating complex emergent patterns and behaviors.

## Features

- **Particle Simulation**: Simulates thousands of particles with customizable physics parameters
- **17 Particle Types**: Amber, Blue, Cyan, Emerald, Fuchsia, Green, Indigo, Lime, Orange, Pink, Purple, Red, Rose, Sky, Teal, Violet, and Yellow
- **Configurable Interactions**: Each particle type can have different attraction/repulsion rules with every other type
- **Runtime Console Commands**: Tweak simulation parameters in real-time without restarting
- **Spatial Hashing**: Efficient spatial partitioning for optimized neighbor queries
- **Interactive Camera**: Move around the simulation with keyboard controls

## Controls

### Game Controls
- **WASD**: Move camera
- **T**: Toggle physics simulation updates
- **Backtick (`)**: Open/close console

### Console Commands

#### Set Parameters
```
set boundary <width> <height>     # Set map boundary dimensions
set r <value>                     # Set interaction distance
set repel_force <value>           # Set repel force magnitude
set dt_half <value>               # Set half-life period of velocity
set dt <value>                    # Set time step for particle updates
set init_particle_num <value>     # Set initial number of particles
```

#### Print Parameters
```
print boundary                    # Print map boundary dimensions
print interaction                 # Print particle interaction table
print r                           # Print interaction distance
print repel_force                 # Print repel force magnitude
print temperature                 # Print half-life period of velocity
print dt                          # Print time step for particle updates
print config                      # Print all configuration values
```

#### Manage Interactions
```
interaction <target> <source> <value>    # Set interaction between two particle types
reset_interaction                         # Reset interactions from CSV file
random_interaction                       # Set all interactions to random values
```

#### Other Commands
```
respawn_particle                # Respawn all particles
help                            # Show all available commands
```

## Installation

### Prerequisites
- Rust 2024 edition or later
- Cargo package manager

### Building

```bash
# Clone the repository
git clone <repository-url>
cd particle-life

# Build the project
cargo build --release

# Run the simulation
cargo run --release
```

## Configuration

### Particle Interaction Table

The simulation loads particle interactions from `particle_interactions.csv`. This file defines how each particle type interacts with every other type:

- **Positive values**: Attraction
- **Negative values**: Repulsion
- **Zero**: No interaction

The CSV format uses particle types as both row and column headers, with the matrix values representing the interaction strength from source to target.

### Default Configuration

```toml
init_particle_num = 2000       # Number of particles to spawn
map_width = 2000.0            # Map boundary width
map_height = 2000.0           # Map boundary height
r = 300.0                     # Interaction radius
repel_force = 1.0             # Repel force magnitude
dt = 1.0                      # Time step for physics
dt_half = 1.0                 # Half-life period of velocity
```

## Project Structure

```
src/
├── components/          # Bevy components (ParticleMarker, ParticleType, etc.)
├── resources/           # Bevy resources (ParticleConfig, InteractionTable, etc.)
├── systems/            # Bevy systems (update_particle, spawn_particle, etc.)
└── lib.rs              # Main library with plugins and console commands
```

### Core Systems

- **setup**: Initialize particle interactions and spawn initial particles
- **update_particle**: Update particle physics using spatial hashing
- **sync_transform**: Sync particle positions to Bevy transforms for rendering
- **move_camera**: Handle camera movement with WASD
- **toggle_particle_update**: Toggle physics updates with T key
- **update_input_focus**: Manage focus between game and console

## Performance

The simulation uses spatial hashing to optimize neighbor queries:
- Particles are divided into spatial chunks based on their positions
- Only particles in adjacent chunks are checked for interactions
- Significantly reduces O(n²) complexity to near O(n) in practice

Default settings simulate 2000 particles efficiently on modern hardware.

## License

This project is open source and available under the same terms as the Bevy engine.

## Contributing

Contributions are welcome! Please ensure all code passes `cargo clippy` and follows Rust best practices.

```bash
# Run clippy before committing
cargo clippy

# Run tests
cargo test
```
