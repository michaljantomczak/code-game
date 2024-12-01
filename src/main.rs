use raylib::consts::KeyboardKey::*;
use raylib::ffi::Rectangle;
use raylib::prelude::*;
use std::collections::HashMap;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 800;

struct PlayerAction {
    image: Texture2D,
    width_frame: i32,
    quantity_frame: i32,
    speed: f32,
}

struct Player {
    position: Vector2,
    actions: HashMap<String, PlayerAction>,
    frame: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vector2::new(0.0, 0.0),
            actions: HashMap::new(),
            frame: 0,
        }
    }

    pub fn register_action(
        &mut self,
        name: String,
        image: Texture2D,
        width_frame: i32,
        quantity_frame: i32,
        speed: f32,
    ) {
        self.actions.insert(
            name,
            PlayerAction {
                image: image,
                width_frame,
                quantity_frame,
                speed,
            },
        );
    }

    pub fn draw(&mut self, mut d: RaylibDrawHandle, time: f64) {
        let action = self.actions.get("walk").unwrap();

        let frame = (time / action.speed as f64) as i32 % action.quantity_frame;

        d.draw_texture_pro(
            &action.image,
            Rectangle {
                x: frame as f32 * action.width_frame as f32,
                y: 0.0,
                width: action.width_frame as f32,
                height: action.width_frame as f32,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: action.width_frame as f32,
                height: action.width_frame as f32,
            },
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );

        self.frame += 1;

        if (self.frame > action.quantity_frame) {
            self.frame = 0;
        }
    }
}


pub fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib [core] example - basic window")
        .vsync()
        .build();
    rl.set_target_fps(60);

    let mut players = Player::new();
    let texture = rl.load_texture(&thread, "assets/samurai/walk.png").unwrap();

    players.register_action("walk".to_string(), texture, 128, 8, 0.1);

    while !rl.window_should_close() {
        let time = rl.get_time();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        players.draw(d, time);
    }
}
