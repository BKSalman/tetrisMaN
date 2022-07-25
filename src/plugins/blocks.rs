use std::time::Duration;

use rand::prelude::*;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::{GameState, plugins::map::{ROWS, COLUMNS}};

use super::map::{Row, Column, BLOCK_SIZE};

enum BlockType {
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock,
}

#[derive(Component)]
struct FallingBlock {
    block_type: BlockType,
}

impl FallingBlock {
    fn spawn(piece: BlockType) -> [Vec3; 3] {
        match piece {
            BlockType::TBlock => {
                [Vec3::new(0., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(1., 1., 1.)]
            },
            _=> {
                [Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.), Vec3::new(0., 1., 0.)]
            }
        }
    }
}

#[derive(Component)]
struct SpawningTimer {
    timer: Timer,
}

#[derive(Component)]
struct FallingTimer {
    normal_timer: Timer,
    fast_timer: Timer,
}

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Playing, setup_falling_timer)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(spawn_block)
                    .into()
            );
    }
}

fn setup_falling_timer(mut commands: Commands) {
    commands
        .insert_resource(
            FallingTimer{
                normal_timer: Timer::new(Duration::from_secs(1), true),
                fast_timer: Timer::new(Duration::from_millis(500), true)
            }
        );
    commands
        .insert_resource(
            SpawningTimer {
                timer: Timer::new(Duration::from_secs(3), true)
            }
        );
}

fn spawn_block(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawningTimer>,
    time: Res<Time>,
    blocks_query: Query<(Entity, &Row, &Column)>,
){
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        println!("finished!");
        let block_start_row = rand::thread_rng().gen_range(0..ROWS);
        let block_start_column = rand::thread_rng().gen_range(0..COLUMNS);
        for (block_e, block_row, block_column) in blocks_query.iter() {
            if block_column.column_number == block_start_column && 
                block_row.row_number == 0 {
                let block = commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0., 0., 5.),
                        ..Default::default()
                    }
                ).id();
                commands.entity(block_e)
                    .add_child(block);
            }
        }
    }
    
}
