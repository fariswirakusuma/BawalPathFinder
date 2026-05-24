use bevy::prelude::*;
use bevy_html_tailwind::HtmlTailwindPlugin;

mod menu;
mod panels;
mod simulation_2d;
mod simulation_3d;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Sim2D,
    Sim3D,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HtmlTailwindPlugin { hot_reload: true })
        .init_state::<AppState>() 
        .add_plugins(menu::MenuPlugin)
        .add_plugins(simulation_2d::Simulation2dPlugin)
        .add_plugins(simulation_3d::Simulation3dPlugin)
        .add_plugins(panels::UiPanelPlugin)
        .run();
}