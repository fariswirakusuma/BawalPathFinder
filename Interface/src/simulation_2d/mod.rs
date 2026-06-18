use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde_json::Value;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use tungstenite::{connect, Message};
use url::Url;

use crate::states::AppState;
use crate::navigation::{NavStack, pop_state};
use crate::setup::SetupConfig;

pub mod message;
use self::message::{SimulationPayload, MapSize, Point2D, StepLog};

pub struct Simulation2dPlugin;

#[derive(Component)]
pub struct Sim2DEntity;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)] 
pub struct SimulationState {
    pub obstacles: Vec<Point>,
    pub path: Vec<Point>,
    pub selected_algorithm: String,
    pub start_pos: Option<Point>, 
    pub goal_pos: Option<Point>,  
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            obstacles: Vec::new(),
            path: Vec::new(),
            selected_algorithm: "AStar".to_string(),
            start_pos:None,
            goal_pos: None, 
        }
    }
}

#[derive(Resource, Default)]
pub struct PlannerLog {
    pub history: Vec<StepLog>, 
}

#[derive(Resource)]
pub struct RosBridge {
    pub tx: std::sync::Mutex<Sender<String>>,
    pub rx: std::sync::Mutex<Receiver<String>>,
}

#[derive(Component)]
pub struct LoadingUiMarker;

#[derive(Component)]
pub struct BackButton;

const SCALE: f32 = 400.0;
const GRID_SIZE: f32 = 0.05;
const ROSBRIDGE_URL: &str = "ws://127.0.0.1:9090";

impl Plugin for Simulation2dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationState>()
            .init_resource::<PlannerLog>()
            
            .add_systems(OnEnter(AppState::Sim2DLoading), (setup_rosbridge, setup_loading_screen))
            .add_systems(Update, check_backend_ready.run_if(in_state(AppState::Sim2DLoading)))
            .add_systems(OnExit(AppState::Sim2DLoading), cleanup_loading_screen)
            
            .add_systems(OnEnter(AppState::Sim2DRun), (setup_2d_grid, setup_back_button))
            .add_systems(
                Update,
                (
                    handle_click, 
                    receive_path_data, 
                    draw_visualization, 
                    handle_back_button
                )
                .run_if(in_state(AppState::Sim2DRun)),
            )
            .add_systems(OnExit(AppState::Sim2DRun), (cleanup_sim2d, cleanup_back_button));
    }
}

fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((Camera2d, LoadingUiMarker));
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0)),
        LoadingUiMarker,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Connecting to ROS 2 Backend...\nPlease Wait."),
            TextFont { font_size: 30.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}

fn check_backend_ready(
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_secs();
    if *timer > 2.5 {
        next_state.set(AppState::Sim2DRun);
        *timer = 0.0;
    }
}

fn cleanup_loading_screen(mut commands: Commands, query: Query<Entity, With<LoadingUiMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn setup_back_button(mut commands: Commands) {
    commands.spawn((
        Button,
        Node {
            width: Val::Px(100.0),
            height: Val::Px(40.0),
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        BackButton,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("< Back"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}

fn handle_back_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut nav_stack: ResMut<NavStack>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            pop_state(&mut next_state, &mut nav_stack);
        }
    }
}

fn cleanup_back_button(mut commands: Commands, query: Query<Entity, With<BackButton>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_2d_grid(mut commands: Commands, config: Res<SetupConfig>, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(4.5)),
        Sim2DEntity
    ));

    if !config.map_name.is_empty() {
        let png_name = config.map_name.replace(".yaml", ".png");
        let image_handle: Handle<Image> = asset_server.load(format!("Test/maps/{}", png_name));

        commands.spawn((
            Sprite {
                image: image_handle,
                custom_size: Some(Vec2::new(10.0 * SCALE, 10.0 * SCALE)),
                ..default()
            },
            Transform::from_xyz(-1.5, 0.0, -1.0),
            Sim2DEntity,
        ));
    }
}

fn setup_rosbridge(mut commands: Commands) {
    let (tx_out, rx_out) = mpsc::channel::<String>(); 
    let (tx_in, rx_in) = mpsc::channel::<String>(); 

    thread::spawn(move || {
        let url = Url::parse(ROSBRIDGE_URL).expect("URL tidak valid");
        let mut socket;
        loop {
            match connect(url.clone()) {
                Ok((s, _)) => {
                    socket = s;
                    break;
                }
                Err(_) => {
                    thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }

        match socket.get_mut() {
            tungstenite::stream::MaybeTlsStream::Plain(s) => s.set_nonblocking(true).unwrap(),
            _ => (),
        }

        let subscribe_plan = serde_json::json!({
            "op": "subscribe",
            "topic": "/plan",
            "type": "nav_msgs/msg/Path"
        });
        let _ = socket.send(Message::Text(subscribe_plan.to_string()));

        let subscribe_log = serde_json::json!({
            "op": "subscribe",
            "topic": "/planner_log",
            "type": "std_msgs/msg/String"
        });
        let _ = socket.send(Message::Text(subscribe_log.to_string()));

        loop {
            if let Ok(msg) = rx_out.try_recv() {
                let _ = socket.send(Message::Text(msg));
            }
            match socket.read() {
                Ok(Message::Text(text)) => {
                    let _ = tx_in.send(text);
                }
                _ => {} 
            }
            thread::sleep(std::time::Duration::from_secs_f32(0.01));
        }
    });

    commands.insert_resource(RosBridge {
        tx: std::sync::Mutex::new(tx_out),
        rx: std::sync::Mutex::new(rx_in),
    });
}

fn receive_path_data(
    mut state: ResMut<SimulationState>, 
    mut planner_log: ResMut<PlannerLog>,
    bridge: Res<RosBridge>
) {
    while let Ok(msg) = bridge.rx.lock().unwrap().try_recv() {
        if let Ok(parsed) = serde_json::from_str::<Value>(&msg) {
            let op = parsed["op"].as_str().unwrap_or("");
            let topic = parsed["topic"].as_str().unwrap_or("");

            if op == "publish" {
                match topic {
                    "/plan" => {
                        if let Some(poses) = parsed["msg"]["poses"].as_array() {
                            state.path.clear();
                            for pose_obj in poses {
                                if let Some(pos) = pose_obj["pose"]["position"].as_object() {
                                    let x = pos["x"].as_f64().unwrap_or(0.0) as f32;
                                    let y = pos["y"].as_f64().unwrap_or(0.0) as f32;
                                    state.path.push(Point { x, y });
                                }
                            }
                        }
                    },
                    "/planner_log" => {
                        if let Some(json_str) = parsed["msg"]["data"].as_str() {
                            if let Ok(log_data) = serde_json::from_str::<StepLog>(json_str) {
                                planner_log.history.push(log_data);
                                if planner_log.history.len() > 8 {
                                    planner_log.history.remove(0);
                                }
                            } else {
                                eprintln!("Gagal parsing isi JSON log: {}", json_str);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Sim2DEntity>>,
    mut state: ResMut<SimulationState>,
    mut planner_log: ResMut<PlannerLog>, // <-- Tambahkan parameter ini
    bridge: Res<RosBridge>,
) {
    let mut data_changed = false;
    let mut click_pos = None;

    if buttons.just_pressed(MouseButton::Left) || buttons.just_pressed(MouseButton::Right) || buttons.just_pressed(MouseButton::Middle) {
        if let Some(window) = q_windows.iter().next() {
            if let Some((camera, camera_transform)) = q_camera.iter().next() {
                if let Some(cursor_position) = window.cursor_position() {
                    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                        let sim_x = world_position.x / SCALE;
                        let sim_y = world_position.y / SCALE;
                        let snapped_x = (sim_x / GRID_SIZE).round() * GRID_SIZE;
                        let snapped_y = (sim_y / GRID_SIZE).round() * GRID_SIZE;
                        click_pos = Some((snapped_x, snapped_y, sim_x, sim_y));
                    }
                }
            }
        }
    }

    if let Some((snapped_x, snapped_y, sim_x, sim_y)) = click_pos {
        if buttons.just_pressed(MouseButton::Right) {
            state.goal_pos = Some(Point { x: snapped_x, y: snapped_y });
            data_changed = true;
        } else if buttons.just_pressed(MouseButton::Middle) {
            state.start_pos = Some(Point { x: snapped_x, y: snapped_y });
            data_changed = true;
        } else if buttons.just_pressed(MouseButton::Left) {
            let click_radius = 0.05;
            let mut removed = false;
            state.obstacles.retain(|p| {
                let dist = ((p.x - sim_x).powi(2) + (p.y - sim_y).powi(2)).sqrt();
                if dist < click_radius {
                    removed = true;
                    false
                } else {
                    true
                }
            });

            if !removed {
                state.obstacles.push(Point { x: snapped_x, y: snapped_y });
            }
            data_changed = true;
        }
    }

    if data_changed {
        state.path.clear();
        planner_log.history.clear();

        if let (Some(start), Some(goal)) = (&state.start_pos, &state.goal_pos) {
            let payload = SimulationPayload {
                map_size: MapSize { width: 20.0, height: 20.0, resolution: 0.05 },
                start: Point2D { x: start.x as f64, y: start.y as f64 },
                goal: Point2D { x: goal.x as f64, y: goal.y as f64 },
                algorithm: state.selected_algorithm.clone(),
                obstacles: state.obstacles.iter().map(|p| Point2D { x: p.x as f64, y: p.y as f64 }).collect(),
            };

            if let Ok(json_str) = serde_json::to_string(&payload) {
                let pub_msg = serde_json::json!({
                    "op": "publish",
                    "topic": "/frontend/obstacles",
                    "msg": { "data": json_str } 
                });
                let _ = bridge.tx.lock().unwrap().send(pub_msg.to_string());
            }
        }
    }
}

fn draw_visualization(mut gizmos: Gizmos, state: Res<SimulationState>) {
    // Custom grid visual (0.5 meter per kotak), tidak mengganggu GRID_SIZE untuk snapping
    const VISUAL_GRID_SIZE: f32 = 0.5; 
    
    for i in -40..=40 {
        let offset = (i as f32) * VISUAL_GRID_SIZE * SCALE;
        let color = if i == 0 { Color::srgb(0.4, 0.4, 0.4) } else { Color::srgb(0.15, 0.15, 0.15) };
        gizmos.line_2d(Vec2::new(-2000.0, offset), Vec2::new(2000.0, offset), color);
        gizmos.line_2d(Vec2::new(offset, -2000.0), Vec2::new(offset, 2000.0), color);
    }

    for obs in &state.obstacles {
        let center = Vec2::new(obs.x * SCALE, obs.y * SCALE);
        let size = GRID_SIZE * SCALE;
        let half = size / 2.0;
        gizmos.rect_2d(center, Vec2::splat(size), Color::srgb(1.0, 0.1, 0.1));
        gizmos.line_2d(center - Vec2::new(half, half), center + Vec2::new(half, half), Color::srgb(1.0, 0.1, 0.1));
        gizmos.line_2d(center - Vec2::new(half, -half), center + Vec2::new(half, -half), Color::srgb(1.0, 0.1, 0.1));
    }

    if state.path.len() > 1 {
        for i in 0..state.path.len() - 1 {
            let p1 = &state.path[i];
            let p2 = &state.path[i + 1];
            gizmos.line_2d(
                Vec2::new(p1.x * SCALE, p1.y * SCALE),
                Vec2::new(p2.x * SCALE, p2.y * SCALE),
                Color::srgb(0.2, 0.9, 0.2),
            );
        }
    }
    
    let path_len = state.path.len();
    for (i, p) in state.path.iter().enumerate() {
        let center = Vec2::new(p.x * SCALE, p.y * SCALE);
        if i > 0 && i < path_len - 1 {
            gizmos.circle_2d(center, 12.0, Color::srgb(0.0, 0.6, 1.0));
        }
    }
    
    if let Some(start) = &state.start_pos {
        gizmos.circle_2d(Vec2::new(start.x * SCALE, start.y * SCALE), 40.0, Color::srgb(1.0, 1.0, 0.0));
    }
    
    if let Some(goal) = &state.goal_pos {
        let goal_vec = Vec2::new(goal.x * SCALE, goal.y * SCALE);
        gizmos.circle_2d(goal_vec, 48.0, Color::srgb(1.0, 0.0, 1.0));
        gizmos.line_2d(goal_vec - Vec2::new(32.0, 32.0), goal_vec + Vec2::new(32.0, 32.0), Color::srgb(1.0, 0.0, 1.0));
        gizmos.line_2d(goal_vec - Vec2::new(-32.0, 32.0), goal_vec + Vec2::new(-32.0, 32.0), Color::srgb(1.0, 0.0, 1.0));
    }
}

fn cleanup_sim2d(mut commands: Commands, query: Query<Entity, With<Sim2DEntity>>, mut logs: ResMut<PlannerLog>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<RosBridge>();
    logs.history.clear();
}