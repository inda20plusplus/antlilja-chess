extern crate chess;
extern crate gui;

use chess::game::{Game, GameResult};
use gui::game_controller::GameController;
use gui::view::{View, ViewSettings};
use piston_window::{PistonWindow, WindowSettings};


fn main() {
    let mut game = Game::default();

    let mut controller = GameController::new(game);

    let mut window: PistonWindow = WindowSettings::new("Chess", [1024, 640])
        .build()
        .unwrap();

    let textures = View::create_textures(&mut window);
    let mut view = View::new(ViewSettings::default(), textures);

    while let Some(event) = window.next() {
        controller.event(&event);

        window.draw_2d(&event, |context, graphics, _| {
            view.render(&controller, context, graphics);
        });
    };
}
