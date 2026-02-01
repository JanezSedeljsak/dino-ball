use bevy::prelude::*;

mod config;
mod state;
mod types;
mod util;

use types::*;
use util::close_on_esc;
use bevy::winit::WinitWindows;
use winit::window::Icon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dino Ball".to_string(),
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<state::GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            input_system,
            player_movement, 
            ball_system,
            score_and_reset_system,
            close_on_esc, 
            dynamic_layout,
            player_animation,
            win_system,
            button_system,
            set_window_icon
        ))
        .run();
}

fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<state::GameState>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        game_state.player1_score = 0;
        game_state.player2_score = 0;
        game_state.is_ball_active = false;
        game_state.game_over = false;
        game_state.winner = None;
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<RestartButton>),
    >,
    mut game_state: ResMut<state::GameState>,
    mut commands: Commands,
    win_ui: Query<Entity, With<WinUI>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                game_state.player1_score = 0;
                game_state.player2_score = 0;
                game_state.is_ball_active = false;
                game_state.game_over = false;
                game_state.winner = None;
                for entity in win_ui.iter() {
                    commands.entity(entity).despawn();
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

fn set_window_icon(
    windows: Option<NonSend<WinitWindows>>,
    mut is_done: Local<bool>,
) {
    if *is_done { return; }
    let Some(windows) = windows else { return };
    let icon_path = format!("assets/icon.png");
    match image::open(&icon_path) {
        Ok(img) => {
            let rgba = img.into_rgba8();
            let (width, height) = rgba.dimensions();
            let data = rgba.into_raw();
            
            match Icon::from_rgba(data, width, height) {
                Ok(icon) => {
                    for window in windows.windows.values() {
                        window.set_window_icon(Some(icon.clone()));
                    }
                    *is_done = true;
                }
                Err(e) => {
                    eprintln!("Failed to create icon: {:?}", e);
                    *is_done = true;
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open icon at {:?}: {:?}", icon_path, e);
            *is_done = true
        }
    }
}

fn dynamic_layout(
    mut background_query: Query<&mut Sprite, (With<Background>, Without<Net>, Without<Player1>, Without<Player2>, Without<Ball>)>,
    mut net_query: Query<(&mut Sprite, &mut Transform), (With<Net>, Without<Background>, Without<Player1>, Without<Player2>, Without<Ball>)>,
    mut player_query: Query<&mut Sprite, (Or<(With<Player1>, With<Player2>)>, Without<Background>, Without<Net>, Without<Ball>)>,
    mut ball_query: Query<&mut Sprite, (With<Ball>, Without<Background>, Without<Net>, Without<Player1>, Without<Player2>)>,
    mut score1_query: Query<&mut Transform, (With<ScoreText1>, Without<ScoreText2>, Without<Sprite>)>,
    mut score2_query: Query<&mut Transform, (With<ScoreText2>, Without<ScoreText1>, Without<Sprite>)>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(window) = windows.iter().next() else { return };
    let width = window.width();
    let height = window.height();

    for mut sprite in background_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(width, height));
    }

    let net_height = height * config::NET_HEIGHT_RATIO;
    let net_width = width * config::NET_WIDTH_RATIO;
    for (mut sprite, mut transform) in net_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(net_width, net_height));
        transform.translation.y = -height / 2.0 + net_height / 2.0;
    }

    let player_height = height * config::PLAYER_HEIGHT_RATIO;
    let player_width = player_height * config::PLAYER_ASPECT_RATIO;
    for mut sprite in player_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(player_width, player_height));
    }

    let ball_dim = height * config::BALL_SIZE_RATIO;
    for mut sprite in ball_query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(ball_dim, ball_dim));
    }

    for mut transform in score1_query.iter_mut() {
        transform.translation = Vec3::new(-width / 2.0 + 100.0, height / 2.0 - 80.0, 10.0);
    }
    for mut transform in score2_query.iter_mut() {
        transform.translation = Vec3::new(width / 2.0 - 100.0, height / 2.0 - 80.0, 10.0);
    }
}

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let mut h = 0.0;
    if d != 0.0 {
        if max == r {
            h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
        } else if max == g {
            h = (b - r) / d + 2.0;
        } else {
            h = (r - g) / d + 4.0;
        }
        h /= 6.0;
    }
    let s = if max == 0.0 { 0.0 } else { d / max };
    let v = max;
    (h, s, v)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

fn load_and_shift(path: &str, shift: f32) -> Option<Image> {
    let full_path = format!("assets/{}", path);
    if let Ok(img) = image::open(&full_path) {
        let mut rgba = img.into_rgba8();
        for pixel in rgba.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            if a > 0 {
                let (h, s, v) = rgb_to_hsv(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
                let nh = if s > 0.15 { (h + shift).fract() } else { h };
                let (nr, ng, nb) = hsv_to_rgb(nh, s, v);
                pixel.0 = [(nr * 255.0).round() as u8, (ng * 255.0).round() as u8, (nb * 255.0).round() as u8, a];
            }
        }
        
        let width = rgba.width();
        let height = rgba.height();
        let data = rgba.into_raw();
        
        Some(Image::new(
            bevy::render::render_resource::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            data,
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            bevy::asset::RenderAssetUsages::default(),
        ))
    } else {
        None
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            image: asset_server.load(config::ASSET_BACK),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Background,
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load(config::ASSET_POLE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Net,
    ));

    let p1_anims = PlayerAnimations {
        stationary: asset_server.load(config::ASSET_PLAYER_STATIONARY),
        jump: asset_server.load(config::ASSET_PLAYER_JUMP),
        run1: asset_server.load(config::ASSET_PLAYER_RUN1),
        run2: asset_server.load(config::ASSET_PLAYER_RUN2),
    };

    commands.spawn((
        Sprite {
            image: p1_anims.stationary.clone(),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 2.0),
        Player1,
        Velocity::default(),
        p1_anims.clone(),
        PlayerState::Idle,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        FacingLeft(false),
    ));

    let shift = 0.45;
    let p2_anims = PlayerAnimations {
        stationary: load_and_shift(config::ASSET_PLAYER_STATIONARY, shift).map(|i| images.add(i)).unwrap_or_else(|| p1_anims.stationary.clone()),
        jump: load_and_shift(config::ASSET_PLAYER_JUMP, shift).map(|i| images.add(i)).unwrap_or_else(|| p1_anims.jump.clone()),
        run1: load_and_shift(config::ASSET_PLAYER_RUN1, shift).map(|i| images.add(i)).unwrap_or_else(|| p1_anims.run1.clone()),
        run2: load_and_shift(config::ASSET_PLAYER_RUN2, shift).map(|i| images.add(i)).unwrap_or_else(|| p1_anims.run2.clone()),
    };

    commands.spawn((
        Sprite {
            image: p2_anims.stationary.clone(),
            color: Color::WHITE, 
            flip_x: true,
            ..default()
        },
        Transform::from_xyz(300.0, 0.0, 2.0),
        Player2,
        Velocity::default(),
        p2_anims,
        PlayerState::Idle,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        FacingLeft(true),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load(config::ASSET_BALL),
            ..default()
        },
        Transform::from_xyz(0.0, 200.0, 5.0),
        Ball,
        Velocity::default(),
        AngularVelocity::default(),
    ));

    commands.spawn((
        Text2d::new("00"),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 10.0),
        ScoreText1,
    ));

    commands.spawn((
        Text2d::new("00"),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 10.0),
        ScoreText2,
    ));
}

fn ball_system(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut AngularVelocity), (With<Ball>, Without<Player1>, Without<Player2>)>,
    player1_query: Query<(&Transform, &Velocity), (With<Player1>, Without<Ball>, Without<Player2>)>,
    player2_query: Query<(&Transform, &Velocity), (With<Player2>, Without<Ball>, Without<Player1>)>,
    mut game_state: ResMut<state::GameState>,
    time: Res<Time>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Some(window) = windows.iter().next() else { return };
    let delta = time.delta_secs_f64() as f32;
    let width = window.width();
    let height = window.height();
    let screen_half_width = width / 2.0;
    let screen_half_height = height / 2.0;

    let Some((mut ball_transform, mut ball_velocity, mut ball_angular_velocity)) = ball_query.iter_mut().next() else { return };

    if game_state.game_over {
        ball_velocity.0 = Vec2::ZERO;
        return;
    }

    if !game_state.is_ball_active {
        let target_x = if game_state.serving_player == state::ServingPlayer::Player1 {
            if let Some(p1) = player1_query.iter().next() {
                p1.0.translation.x
            } else { 0.0 }
        } else {
            if let Some(p2) = player2_query.iter().next() {
                p2.0.translation.x
            } else { 0.0 }
        };
        ball_transform.translation.x = target_x;
        ball_transform.translation.y = -screen_half_height + height * config::GROUND_OFFSET_RATIO + 150.0;
        ball_transform.rotation = Quat::IDENTITY;
        ball_velocity.0 = Vec2::ZERO;
        ball_angular_velocity.0 = 0.0;

        let ball_pos = ball_transform.translation.xy();
        let ball_radius = (height * config::BALL_SIZE_RATIO) / 2.0;
        let player_height = height * config::PLAYER_HEIGHT_RATIO;
        let player_width = player_height * config::PLAYER_ASPECT_RATIO;

        let players = if game_state.serving_player == state::ServingPlayer::Player1 {
            player1_query.iter().collect::<Vec<_>>()
        } else {
            player2_query.iter().collect::<Vec<_>>()
        };

        let net_height = height * config::NET_HEIGHT_RATIO;
        let target_jump_height = net_height * config::JUMP_HEIGHT_RATIO_OF_POLE;
        let serve_velocity = (2.0 * config::GRAVITY * target_jump_height).sqrt();

        for (p_transform, p_velocity) in players {
            let p_pos = p_transform.translation.xy();
            
            let serve_pressed = if game_state.serving_player == state::ServingPlayer::Player1 {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::KeyA)
            } else {
                keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ArrowRight)
            };

            let is_moving_up = p_velocity.0.y > 10.0;

            if (serve_pressed || is_moving_up) && ball_pos.x + ball_radius > p_pos.x - player_width/2.0 &&
               ball_pos.x - ball_radius < p_pos.x + player_width/2.0 &&
               ball_pos.y + ball_radius > p_pos.y - player_height/2.0 &&
               ball_pos.y - ball_radius < p_pos.y + player_height/2.0 
            {
                game_state.is_ball_active = true;
                ball_velocity.0.y = serve_velocity; 
                ball_velocity.0.x = (ball_pos.x - p_pos.x) * 15.0;
                
                let hit_dir = (ball_pos.x - p_pos.x) / (player_width / 2.0);
                ball_angular_velocity.0 = (-hit_dir * 30.0).clamp(-config::BALL_MAX_SPIN, config::BALL_MAX_SPIN);
            }
        }
    } else {
        ball_velocity.0.y -= config::BALL_GRAVITY * delta;
        ball_velocity.0.x *= config::BALL_HORIZONTAL_FRICTION;
        
        ball_transform.translation.x += ball_velocity.0.x * delta;
        ball_transform.translation.y += ball_velocity.0.y * delta;
        
        ball_transform.rotate_z(ball_angular_velocity.0 * delta); // angular velocity
        ball_transform.rotate_z(-ball_velocity.0.x * config::BALL_ROTATION_FACTOR * delta);
        ball_angular_velocity.0 *= 0.98; // friction

        let ball_radius = (height * config::BALL_SIZE_RATIO) / 2.0;

        if ball_transform.translation.x - ball_radius < -screen_half_width {
            ball_transform.translation.x = -screen_half_width + ball_radius;
            ball_velocity.0.x *= -config::BALL_BOUNCE;
        }
        if ball_transform.translation.x + ball_radius > screen_half_width {
            ball_transform.translation.x = screen_half_width - ball_radius;
            ball_velocity.0.x *= -config::BALL_BOUNCE;
        }
        if ball_transform.translation.y + ball_radius > screen_half_height {
            ball_transform.translation.y = screen_half_height - ball_radius;
            ball_velocity.0.y *= -config::BALL_BOUNCE;
        }

        let net_half_thickness = config::NET_COLLISION_WIDTH;
        let net_height = height * config::NET_HEIGHT_RATIO;
        let net_top_y = -screen_half_height + net_height;
        
        if ball_transform.translation.y < net_top_y + ball_radius {
            if ball_transform.translation.x.abs() < net_half_thickness + ball_radius {
                if ball_transform.translation.y > net_top_y - 10.0 {
                    let pole_top_center = Vec2::new(0.0, net_top_y);
                    let b_pos = ball_transform.translation.xy();
                    let dist_vec = b_pos - pole_top_center;
                    let normal = dist_vec.normalize_or_zero();
                    
                    let speed = ball_velocity.0.length().max(300.0);
                    ball_velocity.0 = normal * (speed + 50.0);
                    
                    let overlap = ball_radius - dist_vec.length();
                    if overlap > 0.0 {
                        ball_transform.translation.x += normal.x * overlap;
                        ball_transform.translation.y += normal.y * overlap;
                    }
                } else {
                    ball_velocity.0.x *= -config::BALL_BOUNCE;
                    ball_transform.translation.x = ball_transform.translation.x.signum() * (net_half_thickness + ball_radius + 1.0);
                }
            }
        }

        let player_height = height * config::PLAYER_HEIGHT_RATIO;
        let player_width = player_height * config::PLAYER_ASPECT_RATIO;
        let p_radius = (player_width + player_height) / 4.0;

        for (p_transform, _) in player1_query.iter().chain(player2_query.iter()) {
            let p_pos = p_transform.translation.xy();
            let b_pos = ball_transform.translation.xy();
            let dist_vec = b_pos - p_pos;
            let distance = dist_vec.length();

            if distance < p_radius + ball_radius {
                let normal = dist_vec.normalize_or_zero();
                let speed = ball_velocity.0.length().max(650.0);
                ball_velocity.0 = normal * (speed + 300.0);
                ball_velocity.0 = ball_velocity.0.clamp_length_max(config::BALL_MAX_SPEED);
                
                // spin on hit
                let hit_dir = (b_pos.x - p_pos.x) / (player_width / 2.0);
                ball_angular_velocity.0 = (-hit_dir * 50.0).clamp(-config::BALL_MAX_SPIN, config::BALL_MAX_SPIN);
                
                let overlap = (p_radius + ball_radius) - distance;
                ball_transform.translation.x += normal.x * overlap;
                ball_transform.translation.y += normal.y * overlap;
            }
        }
    }
}

fn score_and_reset_system(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut game_state: ResMut<state::GameState>,
    mut score1_query: Query<&mut Text2d, (With<ScoreText1>, Without<ScoreText2>)>,
    mut score2_query: Query<&mut Text2d, (With<ScoreText2>, Without<ScoreText1>)>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(window) = windows.iter().next() else { return };
    let height = window.height();
    let ground_y = -height / 2.0 + height * config::GROUND_OFFSET_RATIO;
    let Some((ball_transform, mut ball_velocity)) = ball_query.iter_mut().next() else { return };

    if game_state.is_ball_active && ball_transform.translation.y < ground_y {
        if ball_transform.translation.x < 0.0 {
            game_state.player2_score += 1;
            game_state.serving_player = state::ServingPlayer::Player2;
            if game_state.player2_score >= config::WINNING_SCORE {
                game_state.game_over = true;
                game_state.winner = Some(2);
            }
        } else {
            game_state.player1_score += 1;
            game_state.serving_player = state::ServingPlayer::Player1;
            if game_state.player1_score >= config::WINNING_SCORE {
                game_state.game_over = true;
                game_state.winner = Some(1);
            }
        }
        game_state.is_ball_active = false;
        ball_velocity.0 = Vec2::ZERO;
    }

    if let Some(mut text) = score1_query.iter_mut().next() {
        text.0 = format!("{:02}", game_state.player1_score);
    }
    if let Some(mut text) = score2_query.iter_mut().next() {
        text.0 = format!("{:02}", game_state.player2_score);
    }
}

fn win_system(
    mut commands: Commands,
    game_state: Res<state::GameState>,
    win_ui: Query<Entity, With<WinUI>>,
) {
    if game_state.game_over && win_ui.iter().next().is_none() {
        let winner_text = format!("Player {} won!", game_state.winner.unwrap_or(1));
        
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            WinUI,
        )).with_children(|parent| {
            parent.spawn((
                Text::new(winner_text),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    margin: UiRect::top(Val::Px(40.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                RestartButton,
            )).with_children(|button_parent| {
                button_parent.spawn((
                    Text::new("OK"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut PlayerState, &mut FacingLeft, &mut Sprite, Option<&Player1>, Option<&Player2>)>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    game_state: Res<state::GameState>,
) {
    let delta = time.delta_secs_f64() as f32;
    let Some(window) = windows.iter().next() else { return };

    if game_state.game_over { return; }

    let width = window.width();
    let height = window.height();
    let screen_half_width = width / 2.0;
    let ground_y = -height / 2.0 + height * config::GROUND_OFFSET_RATIO;
    
    let current_player_speed = width * config::SPEED_RATIO; 
    let player_height = height * config::PLAYER_HEIGHT_RATIO;
    let player_width = player_height * config::PLAYER_ASPECT_RATIO;

    let net_height = height * config::NET_HEIGHT_RATIO;
    let target_jump_height = net_height * config::JUMP_HEIGHT_RATIO_OF_POLE;
    let jump_velocity = (2.0 * config::GRAVITY * target_jump_height).sqrt();

    for (mut transform, mut velocity, mut state, mut facing, mut sprite, p1, p2) in player_query.iter_mut() {
        let mut horizontal_direction = 0.0;

        if p1.is_some() {
            if keyboard_input.pressed(KeyCode::KeyA) {
                horizontal_direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                horizontal_direction += 1.0;
            }
            if keyboard_input.just_pressed(KeyCode::KeyW) && transform.translation.y <= ground_y + 1.0 {
                velocity.0.y = jump_velocity;
            }
        } else if p2.is_some() {
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                horizontal_direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                horizontal_direction += 1.0;
            }
            if keyboard_input.just_pressed(KeyCode::ArrowUp) && transform.translation.y <= ground_y + 1.0 {
                velocity.0.y = jump_velocity;
            }
        }

        transform.translation.x += horizontal_direction * current_player_speed * delta;

        if horizontal_direction < 0.0 {
            facing.0 = true;
        } else if horizontal_direction > 0.0 {
            facing.0 = false;
        }
        sprite.flip_x = facing.0;

        velocity.0.y -= config::GRAVITY * delta;
        transform.translation.y += velocity.0.y * delta;

        if transform.translation.y < ground_y {
            transform.translation.y = ground_y;
            velocity.0.y = 0.0;
        }

        if transform.translation.y > ground_y + 1.0 {
            *state = PlayerState::Jumping;
        } else if horizontal_direction.abs() > 0.1 {
            *state = PlayerState::Running;
        } else {
            *state = PlayerState::Idle;
        }

        let half_player_width = player_width / 2.0;
        let net_half_thickness = config::NET_COLLISION_WIDTH;

        if p1.is_some() {
            transform.translation.x = transform.translation.x.clamp(
                -screen_half_width + half_player_width,
                -net_half_thickness - half_player_width
            );
        } else {
            transform.translation.x = transform.translation.x.clamp(
                net_half_thickness + half_player_width,
                screen_half_width - half_player_width
            );
        }
    }
}

fn player_animation(
    time: Res<Time>,
    mut query: Query<(&PlayerAnimations, &PlayerState, &mut AnimationTimer, &mut Sprite)>,
) {
    for (anims, state, mut timer, mut sprite) in query.iter_mut() {
        match state {
            PlayerState::Idle => {
                sprite.image = anims.stationary.clone();
            }
            PlayerState::Jumping => {
                sprite.image = anims.jump.clone();
            }
            PlayerState::Running => {
                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    if sprite.image == anims.run1 {
                        sprite.image = anims.run2.clone();
                    } else {
                        sprite.image = anims.run1.clone();
                    }
                } else if sprite.image != anims.run1 && sprite.image != anims.run2 {
                    sprite.image = anims.run1.clone();
                }
            }
        }
    }
}