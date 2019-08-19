use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::{Man, Player};

pub struct ManSystem;

impl<'s> System<'s> for ManSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Man>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, men, input): Self::SystemData) {
        for (man, transform) in (&men, &mut transforms).join() {
            let movement = match man.player {
                Player::Player1 => input.axis_value("player1_y"),
                Player::Player2 => input.axis_value("player2_y"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    transform.prepend_translation_y(scaled_amount);
                    println!("moving {}", mv_amount);
                }
            }
        }
    }
}