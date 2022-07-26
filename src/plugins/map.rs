use bevy::{prelude::*, sprite::Anchor, render::camera::Camera2d};
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{GameState, MyAssets};

use super::utils::center;

pub const BLOCK_SIZE: f32 = 100.;
pub const ROWS: i32 = 15;
pub const COLUMNS: i32 = 10;

#[derive(Component)]
pub struct Row {
	pub row_number: i32,
}

#[derive(Component)]
pub struct Column {
	pub column_number: i32,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_enter_system(GameState::Playing, map_creation);
	}
}

fn map_creation(mut commands:Commands, windows: Res<Windows>) {
	// let window = windows.get_primary().unwrap();
	let padding = 5f32;
	let total_block_size = BLOCK_SIZE + padding;
	for row in 0..ROWS {
		for column in 0..COLUMNS {
			let pos = center(column, row, total_block_size, COLUMNS, ROWS);
			commands.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: Color::rgba(0.88, 0.96, 0.99, 1.),
					custom_size: Some(Vec2::splat(BLOCK_SIZE)),
					..Default::default()
				},
				transform: Transform {
					translation: pos,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Row {row_number: row})
			.insert(Column {column_number: column});			
		}
	}
}

pub fn to_world_coordinates(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window, 
    target_position: Vec2
) -> Vec3 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (target_position / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    
    world_pos
}
