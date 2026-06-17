use bevy::prelude::*;
use crate::states::AppState;
use crate::navigation::{NavStack, push_state};
use crate::simulation_2d::SimulationState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
           .add_systems(Update, handle_main_menu_buttons.run_if(in_state(AppState::MainMenu)))
           .add_systems(OnExit(AppState::MainMenu), cleanup_menu::<MainMenuEntity>)
           
           .add_systems(OnEnter(AppState::AlgorithmSelection2D), setup_algo_menu)
           .add_systems(Update, handle_algo_buttons.run_if(in_state(AppState::AlgorithmSelection2D)))
           .add_systems(OnExit(AppState::AlgorithmSelection2D), cleanup_menu::<AlgoMenuEntity>);
    }
}

#[derive(Component)]
struct MainMenuEntity;

#[derive(Component)]
struct AlgoMenuEntity;

#[derive(Component)]
enum MenuAction {
    Play2D,
    Play3D,
}

#[derive(Component)]
enum AlgoAction {
    AStar,
    Dijkstra,
    GBFS,
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((Camera2d, MainMenuEntity));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        MainMenuEntity,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("NAV2 SIMULATOR"),
            TextFont { font_size: 40.0, ..default() },
        ));
        
        parent.spawn((
            Button,
            Node { width: Val::Px(200.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
            MenuAction::Play2D,
        )).with_children(|btn| {
            btn.spawn((Text::new("2D SIMULATION"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE)));
        });
        
        parent.spawn((
            Button,
            Node { width: Val::Px(200.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
            MenuAction::Play3D,
        )).with_children(|btn| {
            btn.spawn((Text::new("3D SIMULATION"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE)));
        });
    });
}

fn handle_main_menu_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    MenuAction::Play2D => next_state.set(AppState::AlgorithmSelection2D), // Dialihkan ke menu algoritma
                    MenuAction::Play3D => next_state.set(AppState::Sim3D), // (Implementasi nanti)
                }
            }
            Interaction::Hovered => *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            Interaction::None => {
                match action {
                    MenuAction::Play2D => *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
                    MenuAction::Play3D => *color = BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                }
            }
        }
    }
}

fn setup_algo_menu(mut commands: Commands) {
    commands.spawn((Camera2d, AlgoMenuEntity));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        AlgoMenuEntity,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("SELECT ALGORITHM"),
            TextFont { font_size: 40.0, ..default() },
        ));
        
        let btn_node = Node { width: Val::Px(200.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() };
        
        parent.spawn((Button, btn_node.clone(), BackgroundColor(Color::srgb(0.8, 0.4, 0.2)), AlgoAction::AStar))
            .with_children(|btn| { btn.spawn((Text::new("A* (A-Star)"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
            
        parent.spawn((Button, btn_node.clone(), BackgroundColor(Color::srgb(0.8, 0.4, 0.2)), AlgoAction::Dijkstra))
            .with_children(|btn| { btn.spawn((Text::new("Dijkstra"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
            
        parent.spawn((Button, btn_node.clone(), BackgroundColor(Color::srgb(0.8, 0.4, 0.2)), AlgoAction::GBFS))
            .with_children(|btn| { btn.spawn((Text::new("GBFS"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
    });
}

fn handle_algo_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut sim_state: ResMut<SimulationState>,
    mut nav_stack: ResMut<NavStack>, // Pastikan resource ini terdaftar
    mut interaction_query: Query<
        (&Interaction, &AlgoAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // 1. Set parameter algoritma
                sim_state.selected_algorithm = match action {
                    AlgoAction::AStar => "AStar".to_string(),
                    AlgoAction::Dijkstra => "Dijkstra".to_string(),
                    AlgoAction::GBFS => "GBFS".to_string(),
                };

                // 2. Transisi state via NavStack
                // Menggunakan helper function dari previous response
                push_state(
                    AppState::AlgorithmSelection2D, 
                    AppState::Sim2DLoading, 
                    &mut next_state, 
                    &mut nav_stack
                );
            }
            Interaction::Hovered => *color = BackgroundColor(Color::srgb(0.9, 0.6, 0.3)),
            Interaction::None => *color = BackgroundColor(Color::srgb(0.8, 0.4, 0.2)),
        }
    }
}


fn cleanup_menu<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}