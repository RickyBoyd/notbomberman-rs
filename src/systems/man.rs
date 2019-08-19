use amethyst::core::Transform;
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
            if let Some(mv_amount) = movement_x {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let man_x = transform.translation().x;
                    transform.set_translation_x(
                        (man_x + scaled_amount)
                            .min(SCREEN_WIDTH - MAN_WIDTH * 0.5)
                            .max(MAN_WIDTH * 0.5),
                    );
                    transform.prepend_translation_x(scaled_amount);
                    println!("moving {}", mv_amount);
                }
            }
            if let Some(mv_amount) = movement_y {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    let man_y = transform.translation().y;
                    transform.set_translation_y(
                        (man_y + scaled_amount)
                            .min(SCREEN_HEIGHT - MAN_HEIGHT * 0.5)
                            .max(MAN_HEIGHT * 0.5),
                    );
                    transform.prepend_translation_y(scaled_amount);
                    println!("moving {}", mv_amount);
                }
            }
        }
    }
}