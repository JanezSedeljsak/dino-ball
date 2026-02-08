use bevy::prelude::*;
use crate::config;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ServingPlayer {
    #[default]
    Player1,
    Player2,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppMode {
    #[default]
    Menu,
    Playing,
}

#[derive(Resource)]
pub struct GameSpeed {
    pub level: config::SpeedLevel,
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self {
            level: config::SpeedLevel::Normal,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    pub player1_score: u32,
    pub player2_score: u32,
    pub serving_player: ServingPlayer,
    pub is_ball_active: bool,
    pub game_over: bool,
    pub winner: Option<u32>,
}
