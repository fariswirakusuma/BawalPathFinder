use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapSize {
    pub width: f64,
    pub height: f64,
    pub resolution: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimulationPayload {
    pub map_size: MapSize,
    pub goal: Point2D,
    pub algorithm: String,
    pub obstacles: Vec<Point2D>,
}