use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::GameState;

use super::utils::center;

pub const BLOCK_SIZE: f32 = 100.;
pub const TOTAL_BLOCK_SIZE: f32 = BLOCK_SIZE + 5.;
pub const ROWS: i32 = 15;
pub const COLUMNS: i32 = 10;

#[derive(Component)]
pub struct MapBlock {
    pub row: i32,
    pub column: i32,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, map_creation);
    }
}

fn map_creation(mut commands: Commands) {
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + padding;
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let pos = center(column, row, total_block_size, COLUMNS, ROWS);
            commands
                .spawn_bundle(SpriteBundle {
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
                .insert(MapBlock { row, column })
                .insert(Name::new(format!("row: {} column: {}", row, column)));
        }
    }
}
