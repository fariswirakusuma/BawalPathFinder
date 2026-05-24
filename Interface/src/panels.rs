use bevy::prelude::*;
use crate::simulation_2d::SimulationState;
use crate::AppState;
use std::fs;

pub struct UiPanelPlugin;

#[derive(Component)]
struct PanelEntity;

impl Plugin for UiPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Sim2D), setup_panel)
           .add_systems(
               Update,
               (update_panel_stats, handle_reset_button).run_if(in_state(AppState::Sim2D)),
           )
           .add_systems(OnExit(AppState::Sim2D), cleanup_panel);
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
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(2.0)),
            row_gap: Val::Px(10.0), 
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.9)),
        BorderColor {
            top: Color::srgb(0.3, 0.3, 0.3),
            right: Color::srgb(0.3, 0.3, 0.3),
            bottom: Color::srgb(0.3, 0.3, 0.3),
            left: Color::srgb(0.3, 0.3, 0.3),
        },
        PanelEntity, 
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("NAV2 SIMULATION METRICS"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.0, 0.8, 1.0)),
        ));

        parent.spawn((
            Text::new("Awaiting data..."),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            StatsText,
        ));

        parent.spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(30.0),
                margin: UiRect::top(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            BorderColor {
                top: Color::srgb(1.0, 0.4, 0.4),
                right: Color::srgb(1.0, 0.4, 0.4),
                bottom: Color::srgb(1.0, 0.4, 0.4),
                left: Color::srgb(1.0, 0.4, 0.4),
            },
            ResetButton,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new("RESET"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

fn update_panel_stats(
    state: Res<SimulationState>,
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

        **text = format!(
            "Active Obstacles: {}\nPath Nodes: {}\nEst. Distance: {:.2}m",
            state.obstacles.len(),
            state.path.len(),
            total_distance
        );
    }
}

fn handle_reset_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
    mut state: ResMut<SimulationState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.5, 0.1, 0.1));
                
                state.path.clear();
                let _ = fs::write("../Test/backendTest/path_result.json", "[]");
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.9, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.8, 0.2, 0.2));
            }
        }
    }
}

fn cleanup_panel(mut commands: Commands, query: Query<Entity, With<PanelEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}