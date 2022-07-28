use bevy::prelude::*;
use rand::prelude::*;

use super::map::TOTAL_BLOCK_SIZE;

pub fn center(
    current_column: i32,
    current_row: i32,
    total_size: f32,
    columns: i32,
    rows: i32,
) -> Vec3 {
    Vec3::new(
        ((current_column as f32 * total_size) - (columns as f32 * total_size) / 2.)
            + total_size / 2.,
        (-current_row as f32 * total_size) + (rows as f32 * total_size) / 2.,
        1.,
    )
}

pub fn collide(falling_location: Vec3, dot_location: Vec3) -> bool {
    falling_location.y - TOTAL_BLOCK_SIZE == dot_location.y && falling_location.x == dot_location.x
}

pub fn random_color() -> Color {
    let random = rand::thread_rng().gen_range(1..=7);
    match random {
        1 => Color::PURPLE,
        2 => Color::GOLD,
        3 => Color::VIOLET,
        4 => Color::YELLOW,
        5 => Color::PINK,
        6 => Color::TOMATO,
        7 => Color::AQUAMARINE,
        _ => Color::GREEN,
    }
}

// pub fn reparented_to(this: &GlobalTransform, transform: GlobalTransform) -> Transform {
//     let (spos, srot, sscale) = (this.translation, this.rotation, this.scale);
//     let (tpos, trot, tscale) = (transform.translation, transform.rotation, transform.scale);
//     Transform {
//         translation: trot.inverse() * (spos - tpos) / tscale,
//         rotation: trot.inverse() * srot,
//         scale: sscale / tscale,
//     }
// }

// pub fn to_world_coordinates(
//     camera: &Camera,
//     camera_transform: &GlobalTransform,
//     window: &Window,
//     target_position: Vec2,
// ) -> Vec3 {
//     let window_size = Vec2::new(window.width() as f32, window.height() as f32);
//     let ndc = (target_position / window_size) * 2.0 - Vec2::ONE;
//     let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
//     let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

//     world_pos
// }
