use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseScroll;
use std::fs;
use std::path::Path;
use crate::states::AppState;
use crate::navigation::{NavStack, push_state};
use crate::simulation_2d::SimulationState;
use crate::setup::{SetupConfig, SelectionItem, ConfigCategory};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveDropdown>()
           .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
           .add_systems(Update, handle_main_menu_buttons.run_if(in_state(AppState::MainMenu)))
           .add_systems(OnExit(AppState::MainMenu), cleanup_menu::<MainMenuEntity>)
           
           .add_systems(OnEnter(AppState::AlgorithmSelection2D), setup_algo_menu)
           .add_systems(Update, (
               handle_dropdown_toggle,
               handle_selection_input,
               handle_start_button,
               update_selection_buttons,
               update_start_button_visual,
               handle_scroll,
               update_dropdown_visibility,
               update_dropdown_labels,
           ).run_if(in_state(AppState::AlgorithmSelection2D)))
           
           .add_systems(OnExit(AppState::AlgorithmSelection2D), cleanup_menu::<AlgoMenuEntity>);
    }
}

#[derive(Resource, Default)]
struct ActiveDropdown(Option<ConfigCategory>);

#[derive(Component)]
struct MainMenuEntity;

#[derive(Component)]
struct AlgoMenuEntity;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct DropdownToggle(ConfigCategory);

#[derive(Component)]
struct DropdownContainer(ConfigCategory);

#[derive(Component)]
struct DropdownLabel(ConfigCategory);

#[derive(Component)]
enum MenuAction { Play2D, Play3D }

#[derive(Component)]
struct ScrollableList {
    position: f32,
    max_scroll: f32,
}

fn get_files(path: &str, ext: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_file() {
                    let name = entry.file_name().into_string().unwrap_or_default();
                    if name.ends_with(ext) { files.push(name); }
                }
            }
        }
    }
    files.sort();
    files
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
            row_gap: Val::Px(25.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        MainMenuEntity,
    )).with_children(|parent| {
        parent.spawn((Text::new("BawalPathFinder"), TextFont { font_size: 45.0, ..default() }, TextColor(Color::WHITE)));
        
        parent.spawn((
            Button,
            Node { width: Val::Px(250.0), height: Val::Px(60.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
            BackgroundColor(Color::srgb(0.2, 0.4, 0.8)), BorderColor::all(Color::srgb(0.4, 0.6, 1.0)), MenuAction::Play2D,
        )).with_children(|btn| { btn.spawn((Text::new("2D SIMULATION"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
        
        parent.spawn((
            Button,
            Node { width: Val::Px(250.0), height: Val::Px(60.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
            BackgroundColor(Color::srgb(0.2, 0.8, 0.4)), BorderColor::all(Color::srgb(0.4, 1.0, 0.6)), MenuAction::Play3D,
        )).with_children(|btn| { btn.spawn((Text::new("3D SIMULATION"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
    });
}

fn handle_main_menu_buttons(mut next_state: ResMut<NextState<AppState>>, interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action { MenuAction::Play2D => next_state.set(AppState::AlgorithmSelection2D), MenuAction::Play3D => next_state.set(AppState::Sim3D), }
        }
    }
}

macro_rules! render_dropdown {
    ($parent:expr, $prefix:expr, $items:expr, $category:expr) => {
        $parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        }).with_children(|col| {
            col.spawn((
                Button,
                Node { width: Val::Px(350.0), height: Val::Px(50.0), justify_content: JustifyContent::FlexStart, padding: UiRect::left(Val::Px(20.0)), align_items: AlignItems::Center, border: UiRect::all(Val::Px(1.0)), ..default() },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                DropdownToggle($category),
            )).with_children(|btn| { 
                btn.spawn((
                    Text::new(format!("{} : None", $prefix)), 
                    TextFont { font_size: 16.0, ..default() }, 
                    TextColor(Color::WHITE),
                    DropdownLabel($category)
                )); 
            });

            col.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(350.0),
                    height: Val::Px(200.0),
                    margin: UiRect::top(Val::Px(5.0)),
                    overflow: Overflow::clip_y(),
                    display: Display::None,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
                BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
                DropdownContainer($category),
            )).with_children(|viewport| {
                let item_count = $items.len() as f32;
                let max_s = (item_count * 40.0 - 200.0).max(0.0);

                viewport.spawn((
                    Node { flex_direction: FlexDirection::Column, width: Val::Percent(100.0), position_type: PositionType::Relative, top: Val::Px(0.0), ..default() },
                    ScrollableList { position: 0.0, max_scroll: max_s },
                )).with_children(|list| {
                    if $items.is_empty() {
                        list.spawn(Node { width: Val::Percent(100.0), height: Val::Px(40.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() })
                            .with_children(|empty| { empty.spawn((Text::new("Empty"), TextFont { font_size: 14.0, ..default() }, TextColor(Color::srgb(0.8, 0.2, 0.2)))); });
                    } else {
                        for opt in $items {
                            list.spawn((
                                Button,
                                Node { width: Val::Percent(100.0), height: Val::Px(40.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::bottom(Val::Px(1.0)), ..default() },
                                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                                BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
                                SelectionItem { category: $category, value: opt.clone() },
                            )).with_children(|btn| { btn.spawn((Text::new(opt), TextFont { font_size: 14.0, ..default() }, TextColor(Color::WHITE))); });
                        }
                    }
                });
            });
        });
    };
}

fn setup_algo_menu(mut commands: Commands, mut active_dropdown: ResMut<ActiveDropdown>) {
    active_dropdown.0 = None;
    commands.spawn((Camera2d, AlgoMenuEntity));

    let algos = vec!["AStar".to_string(), "UCS".to_string(), "GBFS".to_string()];
    let maps = get_files("Test/maps", ".yaml");
    let urdfs = get_files("Test/urdf", ".urdf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, row_gap: Val::Px(40.0), ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        AlgoMenuEntity,
    )).with_children(|parent| {
        parent.spawn((Text::new("SIMULATION SETUP"), TextFont { font_size: 35.0, ..default() }, TextColor(Color::WHITE)));
        
        parent.spawn(Node { 
            flex_direction: FlexDirection::Column, 
            align_items: AlignItems::Center, 
            ..default() 
        }).with_children(|col| {
            render_dropdown!(col, "Algorithm", algos, ConfigCategory::Algorithm);
            render_dropdown!(col, "Map", maps, ConfigCategory::Map);
            render_dropdown!(col, "URDF", urdfs, ConfigCategory::Urdf);
        });

        parent.spawn((
            Button,
            Node { padding: UiRect::axes(Val::Px(40.0), Val::Px(15.0)), margin: UiRect::top(Val::Px(20.0)), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)), BorderColor::all(Color::srgb(0.4, 0.4, 0.4)), StartButton,
        )).with_children(|btn| { btn.spawn((Text::new("START SIMULATION"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE))); });
    });
}

fn handle_dropdown_toggle(mut active_dropdown: ResMut<ActiveDropdown>, query: Query<(&Interaction, &DropdownToggle), Changed<Interaction>>) {
    for (interaction, toggle) in &query {
        if *interaction == Interaction::Pressed {
            if active_dropdown.0 == Some(toggle.0) { active_dropdown.0 = None; } 
            else { active_dropdown.0 = Some(toggle.0); }
        }
    }
}

fn update_dropdown_visibility(active_dropdown: Res<ActiveDropdown>, mut query: Query<(&mut Node, &DropdownContainer)>) {
    if active_dropdown.is_changed() {
        for (mut node, container) in &mut query {
            if active_dropdown.0 == Some(container.0) { node.display = Display::Flex; } 
            else { node.display = Display::None; }
        }
    }
}

fn update_dropdown_labels(config: Res<SetupConfig>, mut query: Query<(&mut Text, &DropdownLabel)>) {
    if config.is_changed() {
        for (mut text, label) in &mut query {
            match label.0 {
                ConfigCategory::Algorithm => {
                    let val = if config.algorithm.is_empty() { "None" } else { &config.algorithm };
                    **text = format!("Algorithm : {}", val);
                }
                ConfigCategory::Map => {
                    let val = if config.map_name.is_empty() { "None" } else { &config.map_name };
                    **text = format!("Map : {}", val);
                }
                ConfigCategory::Urdf => {
                    let val = if config.urdf_model.is_empty() { "None" } else { &config.urdf_model };
                    **text = format!("URDF : {}", val);
                }
            }
        }
    }
}

fn handle_scroll(mouse_scroll: Res<AccumulatedMouseScroll>, mut query_list: Query<(&mut ScrollableList, &mut Node, &Interaction)>) {
    if mouse_scroll.delta.y != 0.0 {
        for (mut scroll, mut node, interaction) in &mut query_list {
            if *interaction == Interaction::Hovered {
                let scroll_amount = mouse_scroll.delta.y * 30.0;
                scroll.position -= scroll_amount;
                scroll.position = scroll.position.clamp(0.0, scroll.max_scroll);
                node.top = Val::Px(-scroll.position);
            }
        }
    }
}

fn handle_selection_input(mut config: ResMut<SetupConfig>, mut active_dropdown: ResMut<ActiveDropdown>, query: Query<(&Interaction, &SelectionItem), Changed<Interaction>>) {
    for (interaction, item) in &query {
        if *interaction == Interaction::Pressed {
            match item.category {
                ConfigCategory::Algorithm => config.algorithm = item.value.clone(),
                ConfigCategory::Map => config.map_name = item.value.clone(),
                ConfigCategory::Urdf => config.urdf_model = item.value.clone(),
            }
            active_dropdown.0 = None;
        }
    }
}

fn update_selection_buttons(config: Res<SetupConfig>, mut query: Query<(&SelectionItem, &mut BackgroundColor, &mut BorderColor)>) {
    for (item, mut bg_color, mut border) in &mut query {
        let is_selected = match item.category { ConfigCategory::Algorithm => config.algorithm == item.value, ConfigCategory::Map => config.map_name == item.value, ConfigCategory::Urdf => config.urdf_model == item.value, };
        if is_selected {
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.4, 0.8)); *border = BorderColor::all(Color::srgb(0.4, 0.6, 1.0));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1)); *border = BorderColor::all(Color::srgb(0.2, 0.2, 0.2));
        }
    }
}

fn update_start_button_visual(config: Res<SetupConfig>, mut query: Query<(&mut BackgroundColor, &mut BorderColor), With<StartButton>>) {
    let is_ready = !config.algorithm.is_empty() && !config.map_name.is_empty() && !config.urdf_model.is_empty();
    for (mut bg_color, mut border) in &mut query {
        if is_ready { *bg_color = BackgroundColor(Color::srgb(0.1, 0.7, 0.2)); *border = BorderColor::all(Color::srgb(0.2, 0.9, 0.3));
        } else { *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3)); *border = BorderColor::all(Color::srgb(0.4, 0.4, 0.4)); }
    }
}

fn inject_map_to_yaml(map_name: &str) {
    let yaml_path = "ROS_workspace/src/navigation/config/nav2_params.yaml";
    if let Ok(content) = fs::read_to_string(yaml_path) {
        let mut new_lines = Vec::new();
        let mut in_map_server = false;
        let mut in_ros_params = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("map_server:") {
                in_map_server = true;
                new_lines.push(line.to_string());
            } else if in_map_server && trimmed.starts_with("ros__parameters:") {
                in_ros_params = true;
                new_lines.push(line.to_string());
            } else if in_map_server && in_ros_params && trimmed.starts_with("yaml_filename:") {
                let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                let docker_path = format!("/ros2_ws/src/navigation/maps/{}", map_name);
                new_lines.push(format!("{}yaml_filename: \"{}\"", indent, docker_path));
                in_map_server = false;
                in_ros_params = false;
            } else {
                if in_map_server && !trimmed.is_empty() && !line.starts_with(' ') && !trimmed.starts_with("map_server:") {
                    in_map_server = false;
                    in_ros_params = false;
                }
                new_lines.push(line.to_string());
            }
        }
        let _ = fs::write(yaml_path, new_lines.join("\n"));
    }
}

fn handle_start_button(config: Res<SetupConfig>, mut next_state: ResMut<NextState<AppState>>, mut sim_state: ResMut<SimulationState>, mut nav_stack: ResMut<NavStack>, interaction: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>) {
    for interaction in &interaction {
        if *interaction == Interaction::Pressed {
            if config.algorithm.is_empty() || config.map_name.is_empty() || config.urdf_model.is_empty() { return; }
            
            inject_map_to_yaml(&config.map_name);
            
            sim_state.selected_algorithm = config.algorithm.clone();
            push_state(AppState::AlgorithmSelection2D, AppState::Sim2DLoading, &mut next_state, &mut nav_stack);
        }
    }
}

fn cleanup_menu<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() { commands.entity(entity).despawn(); }
}