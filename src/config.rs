#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpeedLevel {
    Slow,
    #[default]
    Normal,
    Fast,
}

impl SpeedLevel {
    pub fn factor(&self) -> f32 {
        match self {
            SpeedLevel::Slow => 0.45,
            SpeedLevel::Normal => 1.0,
            SpeedLevel::Fast => 1.2,
        }
    }
    pub fn to_u32(&self) -> u32 {
        match self {
            SpeedLevel::Slow => 1,
            SpeedLevel::Normal => 2,
            SpeedLevel::Fast => 3,
        }
    }
}

pub const SPEED_RATIO: f32 = 0.35;
pub const GRAVITY: f32 = 2310.0;
pub const JUMP_HEIGHT_RATIO_OF_POLE: f32 = 0.8;
pub const NET_HEIGHT_RATIO: f32 = 0.45;
pub const NET_WIDTH_RATIO: f32 = 0.03;
pub const NET_COLLISION_WIDTH: f32 = 5.0;
pub const GROUND_OFFSET_RATIO: f32 = 0.12;

pub const PLAYER_HEIGHT_RATIO: f32 = 0.2;
pub const PLAYER_ASPECT_RATIO: f32 = 0.8;

pub const BALL_SIZE_RATIO: f32 = 0.1;
pub const BALL_BOUNCE: f32 = 1.25;
pub const BALL_GRAVITY: f32 = 1050.0;
pub const BALL_MAX_SPEED: f32 = 840.0;
pub const BALL_HORIZONTAL_FRICTION: f32 = 0.99;
pub const BALL_ROTATION_FACTOR: f32 = 0.05;
pub const BALL_MAX_SPIN: f32 = 5.0;

pub const WINNING_SCORE: u32 = 5;

pub const ASSET_BACK: &str = "back.png";
pub const ASSET_BALL: &str = "ball.png";
pub const ASSET_POLE: &str = "pole.png";

pub const ASSET_PLAYER_STATIONARY: &str = "player/stationary.png";
pub const ASSET_PLAYER_JUMP: &str = "player/jump.png";
pub const ASSET_PLAYER_RUN1: &str = "player/run1.png";
pub const ASSET_PLAYER_RUN2: &str = "player/run2.png";
