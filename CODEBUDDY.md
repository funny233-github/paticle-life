# Bevy Game Test - Project Overview

## Project Description

This is a **Particle Life simulation** built with Rust and the Bevy game engine. It simulates emergent behavior through particle interactions, creating complex self-organizing patterns.

## Key Features

- **17 particle types** with configurable interactions (289 unique pairs)
- **Real-time physics simulation** with spatial partitioning for performance
- **Interactive console commands** (press ` to toggle) for runtime configuration
- **Optimized rendering** with separated physics/rendering cycles
- **1,661 lines of code** across 27 files

## Project Structure

```
src/
├── components/     # ECS components (position, velocity, particle types)
├── bundles/        # ECS bundles (particle spawning)
├── resources/      # Simulation configuration & interaction tables
├── systems/        # Physics, rendering, camera, input systems
├── main.rs         # Application entry point
└── lib.rs          # Core library & console commands

particle_interactions.csv  # 17x17 interaction force matrix
```

## Controls

- **WASD**: Move camera
- **T**: Toggle simulation
- **R**: Respawn particles
- **+/-**: Zoom
- **`** (backtick): Open console for commands

## Dependencies

- **Bevy 0.17.3** - Game engine
- **bevy_console 0.16.0** - In-game console
- **csv 1.3** - CSV parsing for interaction tables
- **serde 1.0** - Serialization
- **rand 0.9.2** - Random number generation
- **clap 4.5.54** - Command-line argument parsing

## Build Configuration

- Rust edition: 2024
- Dev profile: opt-level 1 (for dependencies: opt-level 3)
- This provides fast compiles while maintaining performance

## Console Commands

The in-game console (toggle with backtick) supports:

- `set`: Modify simulation parameters (boundary, d1/d2/d3 distances, repel_force, dt, particle_num)
- `print`: Display current configuration
- `interaction`: Set forces between particle types
- `reset_interaction`: Reload from CSV
- `random_interaction`: Randomize all interactions
- `respawn_particle`: Respawn particles with new configuration

## Technical Highlights

1. **Spatial Partitioning**: Grid-based chunk system (d3 chunk size) for O(1) neighbor lookups
2. **Separated Physics/Rendering**: Position component separate from Transform for performance
3. **Runtime Configuration**: Real-time parameter tuning without restart
4. **Physics Model**: Collision repulsion, interaction forces, velocity damping, boundary wrapping
5. **Performance Optimizations**: Spatial hashing, conditional system execution, separate update cycles

## Particle Types

17 types with distinct colors: Amber, Blue, Cyan, Emerald, Fuchsia, Green, Indigo, Lime, Orange, Pink, Purple, Red, Rose, Sky, Teal, Violet, Yellow

## Physics Model Details

- Collision repulsion at distance < d1
- Interaction forces between d1 and d3
- Smooth transitions using interpolation factors
- Velocity damping (half-life decay)
- Boundary wrapping

The project demonstrates advanced Bevy ECS patterns, performance optimization techniques, and interactive development workflows.
