use bevy::prelude::*;

// 1. Tell Rust to look for these modules inside your src directory
mod panels;
mod simulation_2d;
mod simulation_3d;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 2. Load your custom Bevy plugins from your subfolders
        .add_plugins(simulation_2d::Simulation2dPlugin)
        .add_plugins(simulation_3d::Simulation3dPlugin)
        .add_plugins(panels::UiPanelPlugin)
        .run();
}