use bevy::prelude::*;

pub fn center(current_column: i32, current_row: i32,  total_size: f32, columns: i32, rows: i32) -> Vec3 {
	Vec3::new(((current_column as f32 * total_size) - (columns as f32 * total_size)/2.) + total_size/2.,
		 (-current_row as f32 * total_size) + (rows as f32 * total_size)/2., 1.)
}
