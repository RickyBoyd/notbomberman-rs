use amethyst::core::Transform;
use amethyst::core::math::{Vector3};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::{Man, Player, SCREEN_HEIGHT, SCREEN_WIDTH, MAN_WIDTH, MAN_HEIGHT};

pub struct ManSystem;

impl<'s> System<'s> for ManSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Man>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, men, input): Self::SystemData) {
        for (man, transform) in (&men, &mut transforms).join() {
            let (movement_x, movement_y) = match man.player {
                Player::Player1 => (input.axis_value("player1_x"), input.axis_value("player1_y")),
                Player::Player2 => (input.axis_value("player2_x"), input.axis_value("player2_y")),
            };

            let translation_x = get_new_pos(movement_x, transform.translation().x);
            let translation_y = get_new_pos(movement_y, transform.translation().y);
            transform.set_translation(Vector3::new(translation_x, translation_y, 0.0 as f32));
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
