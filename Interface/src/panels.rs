use bevy::prelude::*;
use crate::simulation_2d::{SimulationState, PlannerLog};
use crate::states::AppState;
use crate::setup::SetupConfig;
use std::fs;

pub struct UiPanelPlugin;

#[derive(Component)]
struct PanelEntity;

impl Plugin for UiPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Sim2DRun), setup_panel)
           .add_systems(
               Update,
               (update_panel_stats, handle_reset_button).run_if(in_state(AppState::Sim2DRun)),
           )
           .add_systems(OnExit(AppState::Sim2DRun), cleanup_panel);
    }
}

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct ResetButton;

fn setup_panel(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        PanelEntity,
    )).with_children(|root| {
        root.spawn(Node { flex_grow: 1.0, ..default() });

        root.spawn((
            Node {
                width: Val::Px(340.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(25.0)),
                border: UiRect::left(Val::Px(2.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.06, 0.08, 0.98)), // Warna background dipertajam
            BorderColor::all(Color::srgb(0.2, 0.3, 0.4)),
        ))
        .with_children(|sidebar| {
            // HEADER TITTLE
            sidebar.spawn((
                Text::new("NAV2 METRICS"),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(0.2, 0.8, 1.0)),
            ));

            sidebar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.3, 0.4)),
            ));

            sidebar.spawn((
                Node {
                    flex_grow: 1.0, 
                    ..default()
                },
            )).with_children(|stats_container| {
                stats_container.spawn((
                    Text::new("Awaiting data..."),
                    TextFont { font_size: 15.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    StatsText,
                ));
            });

            sidebar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)), // Membuat ujung box membulat
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.12, 0.15, 1.0)),
                BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            )).with_children(|info| {
                info.spawn((
                    Text::new("MOUSE CONTROLS"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
                info.spawn((
                    Text::new("• Mid Click  : Set Start\n• Right Click: Set Goal\n• Left Click : Obstacle"),
                    TextFont { font_size: 13.0, ..default() },
                    TextColor(Color::srgb(0.85, 0.85, 0.85)),
                ));
            });
            sidebar.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(45.0), 
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.7, 0.2, 0.2)),
                BorderColor::all(Color::srgb(0.9, 0.4, 0.4)),
                ResetButton,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("RESET PATH"),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

fn update_panel_stats(
    state: Res<SimulationState>,
    planner_log: Res<PlannerLog>,
    config: Res<SetupConfig>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    for mut text in &mut query {
        let mut total_distance = 0.0;
        if state.path.len() > 1 {
            for i in 0..state.path.len() - 1 {
                let dx = state.path[i + 1].x - state.path[i].x;
                let dy = state.path[i + 1].y - state.path[i].y;
                total_distance += (dx.powi(2) + dy.powi(2)).sqrt();
            }
        }

        let mut log_text = String::new();
        if planner_log.history.is_empty() {
            log_text = "Waiting for algorithm process...".to_string();
        } else {
            for log in &planner_log.history {
                let formula = match config.algorithm.as_str() {
                    "AStar" => format!("f=g+h -> {:.1}={:.1}+{:.1}", log.f, log.g, log.h),
                    "UCS" => format!("f=g -> {:.1}={:.1}", log.f, log.g),
                    "GBFS" => format!("f=h -> {:.1}={:.1}", log.f, log.h),
                    _ => format!("f: {:.1}", log.f),
                };
                log_text.push_str(&format!("Node [{}]: {}\n", log.index, formula));
            }
        }

        **text = format!(
            "Algorithm   : {}\nObstacles   : {}\nPath Nodes  : {}\nDistance    : {:.2}m\n\n[ CALCULATION LOG ]\n{}",
            config.algorithm,
            state.obstacles.len(),
            state.path.len(),
            total_distance,
            log_text
        );
    }
}

fn handle_reset_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut state: ResMut<SimulationState>,
) {
    for (interaction, mut color, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.5, 0.1, 0.1));
                *border = BorderColor::all(Color::srgb(0.6, 0.2, 0.2));
                
                state.path.clear();
                let _ = fs::write("../Test/backendTest/path_result.json", "[]");
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.8, 0.3, 0.3));
                *border = BorderColor::all(Color::srgb(1.0, 0.5, 0.5));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.7, 0.2, 0.2));
                *border = BorderColor::all(Color::srgb(0.9, 0.4, 0.4));
            }
        }
    }
}

fn cleanup_panel(mut commands: Commands, query: Query<Entity, With<PanelEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}