use bevy::prelude::*;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ServingPlayer {
    #[default]
    Player1,
    Player2,
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
