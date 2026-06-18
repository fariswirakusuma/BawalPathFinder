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
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            row_gap: Val::Px(15.0),
            min_width: Val::Px(320.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.09, 0.1, 0.95)),
        BorderColor::all(Color::srgb(0.3, 0.4, 0.5)),
        PanelEntity, 
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("NAV2 METRICS"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.2, 0.8, 1.0)),
        ));

        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.4, 0.5)),
        ));

        parent.spawn((
            Text::new("Awaiting data..."),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            StatsText,
        ));

        parent.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(35.0),
                margin: UiRect::top(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.7, 0.2, 0.2)),
            BorderColor::all(Color::srgb(0.9, 0.4, 0.4)),
            ResetButton,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new("RESET PATH"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));
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