use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use tungstenite::{connect, Message};
use url::Url;

use crate::AppState;
pub mod message;
use self::message::{SimulationPayload, MapSize, Point2D};

pub struct Simulation2dPlugin;

#[derive(Component)]
pub struct Sim2DEntity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)] 
pub struct SimulationState {
    pub obstacles: Vec<Point>,
    pub path: Vec<Point>,
    pub selected_algorithm: String,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            obstacles: Vec::new(),
            path: Vec::new(),
            selected_algorithm: "AStar".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct RosBridge {
    pub tx: std::sync::Mutex<Sender<String>>,
    pub rx: std::sync::Mutex<Receiver<String>>,
}

const SCALE: f32 = 400.0;
const GRID_SIZE: f32 = 0.05;
const ROSBRIDGE_URL: &str = "ws://127.0.0.1:9090";

impl Plugin for Simulation2dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationState>()
            .add_systems(OnEnter(AppState::Sim2D), (setup_2d_grid, setup_rosbridge))
            .add_systems(
                Update,
                (handle_click, receive_path_data, draw_visualization)
                    .run_if(in_state(AppState::Sim2D)),
            )
            .add_systems(OnExit(AppState::Sim2D), cleanup_sim2d);
    }
}

fn setup_2d_grid(mut commands: Commands) {
    commands.spawn((Camera2d, Sim2DEntity));
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
                    println!("Terhubung ke ROSBridge di {}", ROSBRIDGE_URL);
                    break;
                }
                Err(_) => {
                    println!("Menunggu ROSBridge siap...");
                    thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }

        match socket.get_mut() {
            tungstenite::stream::MaybeTlsStream::Plain(s) => s.set_nonblocking(true).unwrap(),
            _ => (),
        }

        let subscribe_msg = serde_json::json!({
            "op": "subscribe",
            "topic": "/plan",
            "type": "nav_msgs/msg/Path"
        });
        let _ = socket.send(Message::Text(subscribe_msg.to_string()));

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
            
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    commands.insert_resource(RosBridge {
        tx: std::sync::Mutex::new(tx_out),
        rx: std::sync::Mutex::new(rx_in),
    });
}


fn receive_path_data(mut state: ResMut<SimulationState>, bridge: Res<RosBridge>) {
    while let Ok(msg) = bridge.rx.lock().unwrap().try_recv() {
        if let Ok(parsed) = serde_json::from_str::<Value>(&msg) {
            if parsed["op"] == "publish" && parsed["topic"] == "/plan" {
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
            }
        }
    }
}

fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut state: ResMut<SimulationState>,
    bridge: Res<RosBridge>,
) {
    let mut data_changed = false;

    if buttons.just_pressed(MouseButton::Right) {
        state.obstacles.clear();
        data_changed = true;
    } else if buttons.just_pressed(MouseButton::Left) {
        if let Some(window) = q_windows.iter().next() {
            if let Some((camera, camera_transform)) = q_camera.iter().next() {
                if let Some(cursor_position) = window.cursor_position() {
                    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                        let sim_x = world_position.x / SCALE;
                        let sim_y = world_position.y / SCALE;

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
                            let snapped_x = (sim_x / GRID_SIZE).round() * GRID_SIZE;
                            let snapped_y = (sim_y / GRID_SIZE).round() * GRID_SIZE;
                            state.obstacles.push(Point { x: snapped_x, y: snapped_y });
                        }
                        data_changed = true;
                    }
                }
            }
        }
    }

    if data_changed {
        let payload = SimulationPayload {
            map_size: MapSize { width: 20.0, height: 20.0, resolution: 0.05 },
            goal: Point2D { x: 5.0, y: 5.0 },
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

fn draw_visualization(mut gizmos: Gizmos, state: Res<SimulationState>) {
    for i in -40..=40 {
        let offset = (i as f32) * GRID_SIZE * SCALE;
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
        if i == 0 {
            gizmos.circle_2d(center, 6.0, Color::srgb(1.0, 1.0, 0.0)); 
        } else if i == path_len - 1 {
            gizmos.circle_2d(center, 6.0, Color::srgb(1.0, 0.0, 1.0)); 
        } else {
            gizmos.circle_2d(center, 3.0, Color::srgb(0.0, 0.6, 1.0));
        }
    }
}

fn cleanup_sim2d(mut commands: Commands, query: Query<Entity, With<Sim2DEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<RosBridge>();
}