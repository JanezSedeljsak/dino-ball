use bevy::prelude::*;

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Net;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct ScoreText1;

#[derive(Component)]
pub struct ScoreText2;

#[derive(Component, Clone)]
pub struct PlayerAnimations {
    pub stationary: Handle<Image>,
    pub jump: Handle<Image>,
    pub run1: Handle<Image>,
    pub run2: Handle<Image>,
}

#[derive(Component, PartialEq, Default, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idle,
    Running,
    Jumping,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default)]
pub struct FacingLeft(pub bool);

#[derive(Component)]
pub struct WinUI;

#[derive(Component)]
pub struct RestartButton;
