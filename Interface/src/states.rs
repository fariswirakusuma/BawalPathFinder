use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    AlgorithmSelection2D,
    Sim2DLoading,
    Sim2DLog,
    Sim2DRun,
    Sim3D,
}