use bevy::prelude::*;
use states::AppState;
use navigation::NavStack;
use bevy_html_tailwind::HtmlTailwindPlugin; 

mod menu;
mod panels;
mod simulation_2d;
mod simulation_3d;
mod navigation;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .add_plugins(HtmlTailwindPlugin { hot_reload: true }) 
        
        .init_state::<AppState>()
        .insert_resource(NavStack::default())
        
        .add_plugins(menu::MenuPlugin)
        .add_plugins(simulation_2d::Simulation2dPlugin)
        .add_plugins(simulation_3d::Simulation3dPlugin)
        .add_plugins(panels::UiPanelPlugin)
        
        .run();
}