use amethyst::core::Transform;
use amethyst::core::math::{Vector3};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::{Man, Player, Board, SCREEN_HEIGHT, MAN_HEIGHT, BLOCK_HEIGHT, BLOCK_WIDTH, BOARD_WIDTH, BOARD_HEIGHT};

pub struct ManSystem;

impl<'s> System<'s> for ManSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Man>,
        ReadStorage<'s, Board>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, men, boards, input): Self::SystemData) {
        for (man, transform) in (&men, &mut transforms).join() {
            let (movement_x, movement_y) = match man.player {
                Player::Player1 => (input.axis_value("player1_x"), input.axis_value("player1_y")),
                Player::Player2 => (input.axis_value("player2_x"), input.axis_value("player2_y")),
            };

            

            let translation_x = get_new_pos(movement_x, transform.translation().x);
            let translation_y = get_new_pos(movement_y, transform.translation().y);

            let new_pos = Vector3::new(translation_x, translation_y, 0.0 as f32);
            for board in boards.join() {
                let final_pos = pos_after_collisions(&new_pos, board);
                transform.set_translation(final_pos);
            }
        }
    }
}

fn get_new_pos(movement: Option<f32>, starting_translation: f32) -> f32 {
    if let Some(mv_amount) = movement {
        if mv_amount != 0.0 {
            let scaled_amount = 1.2 * mv_amount as f32;
            return (starting_translation + scaled_amount)
                     .min(SCREEN_HEIGHT - MAN_HEIGHT * 0.5)
                     .max(MAN_HEIGHT * 0.5);
        }
    }
    return starting_translation;
}

fn pos_after_collisions(new_pos: &Vector3<f32>, board: &Board) -> Vector3<f32> {
    for block in &board.blocks {
        if block.is_empty() {
            continue
        }
        
    }
    new_pos.clone()
}

enum HorizontalBlockCollision {
    Left,
    Right,
    Neither,
}

fn board_index(column: usize, row: usize) -> usize {
    row * BOARD_WIDTH + column
}

fn in_blocks(new_pos: &Vector3<f32>) -> Vec<usize> {
    let mut block_indexes = Vec::new();
    let x = new_pos.x;
    let y = new_pos.y;

    let in_column = (x / BLOCK_WIDTH) as usize;
    let in_row = (y / BLOCK_HEIGHT) as usize;
    block_indexes.push(board_index(in_column, in_row));

    let horizontal = if in_column != 0 && (x % BLOCK_WIDTH) < BLOCK_WIDTH / 2.0 {
        block_indexes.push(board_index(in_column - 1, in_row));
        HorizontalBlockCollision::Left
    } else if in_column != (BOARD_WIDTH - 1) && (x % BLOCK_WIDTH) > BLOCK_WIDTH / 2.0 {
        block_indexes.push(board_index(in_column + 1, in_row));
        HorizontalBlockCollision::Right
    } else {
        HorizontalBlockCollision::Neither
    };
    
    if in_row != 0 && (y % BLOCK_HEIGHT) < BLOCK_HEIGHT / 2.0 {
        block_indexes.push(board_index(in_column, in_row - 1));
        match horizontal {
            Left =>  block_indexes.push(board_index(in_column - 1, in_row - 1)),
            Right => block_indexes.push(board_index(in_column + 1, in_row - 1)),
            Neither => (),
        }
    } else if in_row != BOARD_HEIGHT && (y % BLOCK_HEIGHT) > BLOCK_HEIGHT / 2.0 {
        block_indexes.push(board_index(in_column, in_row + 1))
    }


    block_indexes
}