use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{
    plugins::map::{COLUMNS, ROWS},
    GameState,
};

use super::{
    map::{MapBlock, BLOCK_SIZE, TOTAL_BLOCK_SIZE},
    utils::{collide, random_color},
    // utils::center,
};

#[derive(Component, Default, Inspectable)]
pub struct Brick {
    pub dots: [Dot; 4],
}

#[derive(Component, Default, Inspectable)]
pub struct Dot {
    pub x: i8,
    pub y: i8,
}

#[derive(Component)]
struct FallingDot;

#[derive(Inspectable, Default, Debug)]
pub enum BlockType {
    #[default]
    I,
    J,
    L,
    O,
    T,
    S,
    Z,
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(0..=6) {
            // rand 0.8
            0 => BlockType::I,
            1 => BlockType::J,
            2 => BlockType::L,
            3 => BlockType::O,
            4 => BlockType::T,
            5 => BlockType::S,
            6 => BlockType::Z,
            _ => BlockType::I,
        }
    }
}

#[derive(Component, Inspectable, Default)]
pub struct FallingBlock {
    pub block_type: BlockType,
    pub row: i32,
    pub column: i32,
    pub rotation: f32,
    pub brick: Brick,
}

impl FallingBlock {
    fn brick(piece: &BlockType) -> Brick {
        match piece {
            BlockType::I => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: -1, y: 0 },
                    Dot { x: -2, y: 0 },
                ],
            },
            BlockType::J => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: -1, y: 0 },
                    Dot { x: -1, y: 1 },
                ],
            },
            BlockType::L => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: 1, y: 1 },
                    Dot { x: -1, y: 0 },
                ],
            },
            BlockType::O => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: 0, y: 1 },
                    Dot { x: 1, y: 1 },
                ],
            },
            BlockType::T => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: -1, y: 0 },
                    Dot { x: 0, y: 1 },
                ],
            },
            BlockType::S => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: 1, y: 0 },
                    Dot { x: 0, y: -1 },
                    Dot { x: -1, y: -1 },
                ],
            },
            BlockType::Z => Brick {
                dots: [
                    Dot { x: 0, y: 0 },
                    Dot { x: -1, y: 0 },
                    Dot { x: 0, y: -1 },
                    Dot { x: 1, y: -1 },
                ],
            },
        }
    }
    fn spawn(
        brick: Brick,
        commands: &mut Commands,
        start_column: i32,
        block_type: BlockType,
    ) -> Entity {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 20.),
                    // rotation: Quat::from_rotation_z(PI),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                let random_color = random_color();
                for dot in &brick.dots {
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: random_color,
                                custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                TOTAL_BLOCK_SIZE * dot.x as f32,
                                TOTAL_BLOCK_SIZE * dot.y as f32,
                                1.,
                            )),
                            ..Default::default()
                        })
                        .insert(Dot { x: dot.x, y: dot.y })
                        .insert(FallingDot);
                }
            })
            .insert(FallingBlock {
                block_type,
                row: 0,
                column: start_column,
                brick,
                ..Default::default()
            })
            .id()
    }
    fn rotate(&mut self) {
        self.rotation += PI / 2.;
    }
}

#[derive(Component)]
struct SpawningTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct FallingTimer {
    pub normal_timer: Timer,
    pub fast_timer: Timer,
}

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, setup_falling_timer)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(spawn_block)
                    .with_system(falling_block)
                    .with_system(block_controller)
                    .with_system(stop_falling)
                    .into(),
            );
    }
}

fn setup_falling_timer(mut commands: Commands) {
    commands.insert_resource(FallingTimer {
        normal_timer: Timer::new(Duration::from_secs(1), true),
        fast_timer: Timer::new(Duration::from_millis(500), true),
    });
    commands.insert_resource(SpawningTimer {
        timer: Timer::new(Duration::from_secs(5), true),
    });
}

fn spawn_block(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawningTimer>,
    time: Res<Time>,
    blocks_query: Query<(Entity, &MapBlock)>,
    falling_query: Query<&FallingBlock>,
) {
    if falling_query.is_empty() {
        spawn_timer.timer.tick(time.delta());
        if spawn_timer.timer.finished() {
            let block_start_column = rand::thread_rng().gen_range(0..COLUMNS);
            for (block_e, block) in blocks_query.iter() {
                if block.column == block_start_column && block.row == 0 {
                    let random_block_type: BlockType = rand::random();
                    let vec = FallingBlock::brick(&random_block_type);
                    let block = FallingBlock::spawn(
                        vec,
                        &mut commands,
                        block_start_column,
                        random_block_type,
                    );
                    commands.entity(block_e).add_child(block);
                }
            }
        }
    }
}

fn falling_block(
    mut falling_query: Query<(&mut FallingBlock, &mut Parent, &mut Transform)>,
    mut blocks_query: Query<(Entity, &MapBlock, &mut Transform), Without<FallingBlock>>,
    time: Res<Time>,
    mut falling_timer: ResMut<FallingTimer>,
) {
    if let Ok((mut falling, mut falling_parent, mut falling_transform)) =
        falling_query.get_single_mut()
    {
        let mut next_row: i32 = 1;
        for (block_e, block, mut block_transform) in blocks_query.iter_mut() {
            if block.column == falling.column && block.row == falling.row + 1 {
                *falling_parent = Parent(block_e);
                falling_transform.rotation = Quat::from_rotation_z(falling.rotation);
                *block_transform = Transform::from_translation(block_transform.translation);
                next_row = block.row;
            }
        }
        falling_timer.normal_timer.tick(time.delta());
        if falling_timer.normal_timer.finished() {
            falling.row = next_row;
        }
    }
}

fn block_controller(keyboard: Res<Input<KeyCode>>, mut falling_query: Query<&mut FallingBlock>) {
    for mut block in falling_query.iter_mut() {
        if keyboard.just_pressed(KeyCode::Up) && !matches!(block.block_type, BlockType::O) {
            block.rotate();
        }
    }
}

fn stop_falling(
    mut commands: Commands,
    falling_query: Query<&GlobalTransform, With<FallingDot>>,
    block_query: Query<(Entity, &Children), With<FallingBlock>>,
    dots_query: Query<&GlobalTransform, (With<Dot>, Without<FallingDot>)>,
    map_blocks_query: Query<(&MapBlock, &GlobalTransform)>,
) {
    for falling_dot_transform in falling_query.iter() {
        for (map_block, map_block_transform) in map_blocks_query.iter() {
            let map_block_pos = map_block_transform.translation;
            let falling_dot_pos = falling_dot_transform.translation;
            let last_row_check = map_block.row == ROWS - 1 && falling_dot_pos.y == map_block_pos.y;
            if dots_query.is_empty() {
                if last_row_check {
                    if let Ok((block_e, block_children)) = block_query.get_single() {
                        for child in block_children.iter() {
                            commands.entity(*child).remove::<FallingDot>();
                        }
                        commands.entity(block_e).remove::<FallingBlock>();
                    }
                }
            } else {
                for dot_transform in dots_query.iter() {
                    let dot_pos = dot_transform.translation;
                    if collide(falling_dot_pos, dot_pos) || last_row_check {
                        if let Ok((block_e, block_children)) = block_query.get_single() {
                            for child in block_children.iter() {
                                commands.entity(*child).remove::<FallingDot>();
                            }
                            commands.entity(block_e).remove::<FallingBlock>();
                        }
                    }
                }
            }
        }
    }
}
