extern crate chess;
extern crate gui;

use chess::game::Game;
use gui::game_controller::GameController;
use gui::network::ConnectionHandler;
use gui::view::{View, ViewSettings};
use piston_window::*;
use std::io::{self, Read};

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    input.trim()
}

fn main() {
    println!("Enter if game is Local or Remote\nFormat: local | host | remote");
    let connection = match &get_input()[..] {
        "local" => None,
        "host" => Some(ConnectionHandler { is_host: true }),
        "remote" => Some(ConnectionHandler { is_host: false }),
        _ => panic!("Invalid input"),
    };

    let game = Game::default();

    let view_settings = ViewSettings::default();

    let mut controller = GameController::new(game, connection, view_settings);

    let mut window: PistonWindow = WindowSettings::new("Chess", [1024, 640]).build().unwrap();

    let textures = View::create_textures(&mut window);
    let mut view = View::new(view_settings, textures);

    while let Some(event) = window.next() {
        controller.event(&event);

        window.draw_2d(&event, |context, graphics, _| {
            view.render(&controller, context, graphics);
        });
    }
}
