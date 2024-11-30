use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const BOARD_SIZE: i32 = 10;
const TILE_SIZE: i32 = 80;
const TILE_SPACING: i32 = 2;
const SCREEN_WIDTH: i32 = BOARD_SIZE * (TILE_SIZE + TILE_SPACING);
const SCREEN_HEIGHT: i32 = BOARD_SIZE * (TILE_SIZE + TILE_SPACING);
const PLAYER_SPEED: f32 = 3.0;


pub struct Player {
    position: Vector2,
    image: Texture2D,
}

pub struct Board {
    tiles: Vec<Vec<i32>>,
}

fn get_player_position(position: Vector2, off_x: i32, off_y: i32) -> Vector2 {
    Vector2::new(
        (position.x as i32 * TILE_SIZE + (TILE_SPACING * position.x as i32) + off_x).as_f32(),
        (position.y as i32 * TILE_SIZE + (TILE_SPACING * position.y as i32) + off_y).as_f32(),
    )
}

fn is_player_allowed_to_move(position: Vector2) -> bool {
    position.x >= 0.0 && position.x < BOARD_SIZE as f32 && position.y >= 0.0 && position.y < BOARD_SIZE as f32
}

pub fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib [core] example - basic window")
        .vsync()
        .build();
    rl.set_target_fps(10);

    let tile_texture = rl.load_texture(&thread, "assets/g26.png").unwrap();
    let mut player = Player {
        position: Vector2::new(0.0, 0.0),
        image: rl.load_texture(&thread, "assets/player.png").unwrap(),
    };

    let player_offset_x = (TILE_SIZE - player.image.width()) / 2;
    let player_offset_y = (TILE_SIZE - player.image.height()) / 2;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_down(KEY_DOWN) && is_player_allowed_to_move(Vector2::new(player.position.x, player.position.y + 1.0)) {
            player.position.y += 1.0f32;
        }

        if d.is_key_down(KEY_UP) && is_player_allowed_to_move(Vector2::new(player.position.x, player.position.y - 1.0)) {
            player.position.y -= 1.0f32;
        }

        if d.is_key_down(KEY_LEFT) && is_player_allowed_to_move(Vector2::new(player.position.x - 1.0, player.position.y)) {
            player.position.x -= 1.0f32;
        }

        if d.is_key_down(KEY_RIGHT) && is_player_allowed_to_move(Vector2::new(player.position.x + 1.0, player.position.y)) {
            player.position.x += 1.0f32;
        }


        d.clear_background(Color::BLACK);
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                d.draw_texture_v(
                    &tile_texture,
                    Vector2::new(
                        (x * TILE_SIZE + (TILE_SPACING * x)).as_f32(),
                        (y * TILE_SIZE + (TILE_SPACING * y)).as_f32(),
                    ),
                    Color::WHITE,
                );
            }
        }

        d.draw_texture_v(
            &player.image,
            get_player_position(player.position, player_offset_x, player_offset_y),
            Color::WHITE,
        );
    }
}
