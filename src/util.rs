use bevy::prelude::*;

pub fn close_on_esc(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    primary_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some(entity) = primary_window.iter().next() {
            commands.entity(entity).despawn();
        }
    }
}
