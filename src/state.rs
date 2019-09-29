use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ecs::prelude::{Component, DenseVecStorage},
};

pub const SCREEN_HEIGHT: f32 = 480.0;
pub const SCREEN_WIDTH: f32 = 480.0;

use log::info;

pub struct Bomberman;

impl SimpleState for Bomberman {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;


        world.register::<Board>();
        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        let sprites = load_sprites(world);

        // Place the camera
        initialise_blocks(world);
        initialise_men(world, &dimensions, sprites);
        init_camera(world, &dimensions);

        // Load our sprites and display them
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Handle<SpriteSheet> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/characters.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/characters_spritesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    return sheet_handle;

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
//    (0..3)
//        .map(|i| SpriteRender {
//            sprite_sheet: sheet_handle.clone(),
//            sprite_number: i,
//        })
//        .collect()
}

pub const MAN_HEIGHT: f32 = 32.0;
pub const MAN_WIDTH: f32 = 32.0;

pub enum Player {
    Player1,
    Player2,
}

pub struct Man {
    pub width: f32,
    pub height: f32,
    pub player: Player,
}

impl Man {
    fn new(player: Player) -> Man {
        Man {
            width: MAN_WIDTH,
            height: MAN_HEIGHT,
            player,
        }
    }
}

impl Component for Man {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_men(world: &mut World, dimensions: &ScreenDimensions, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    left_transform.set_translation_xyz(MAN_WIDTH / 2.0, MAN_HEIGHT / 2.0, 0.0);
    right_transform.set_translation_xyz(dimensions.width() - MAN_WIDTH / 2.0, MAN_HEIGHT / 2.0, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // Create a left plank entity.
    world
        .create_entity()
        .with(Man::new(Player::Player1))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(Man::new(Player::Player2))
        .with(right_transform)
        .with(sprite_render.clone())
        .build();
}

pub const BLOCK_HEIGHT: f32 = 32.0;
pub const BLOCK_WIDTH: f32 = 32.0;
pub const BOARD_HEIGHT: usize = 15;
pub const BOARD_WIDTH: usize = 15;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum BlockState {
    Empty,
    Bomb,
    Destructable,
    Indestructable,
}

pub struct Block {
    state: BlockState,
}

impl Block {
    fn new(state: BlockState) -> Block {
        Block{
            state,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.state == BlockState::Empty
    }
}

pub struct Board {
    pub blocks: Vec<Block>,
}

impl Board {
    fn new(blocks: Vec<Block>) -> Board { 
        Board{
            blocks,
        }
    }
}

impl Component for Board {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_blocks(world: &mut World) {
    let mut blocks = Vec::new();
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let block_state = match x%2 == 1 && y%2 == 1 {
                true => BlockState::Indestructable,
                false => BlockState::Empty,
            };
            blocks.push(Block::new(block_state))
        }
    }
    let board = Board::new(blocks);

    world.create_entity().with(board).build();
}