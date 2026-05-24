use bevy::prelude::*;
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_menu)
           .add_systems(Update, handle_menu_buttons.run_if(in_state(AppState::MainMenu)))
           .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct MainMenuEntity;

#[derive(Component)]
enum MenuAction {
    Play2D,
    Play3D,
}

fn setup_menu(mut commands: Commands) {
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

fn handle_menu_buttons(
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
                    MenuAction::Play2D => next_state.set(AppState::Sim2D),
                    MenuAction::Play3D => next_state.set(AppState::Sim3D),
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

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}