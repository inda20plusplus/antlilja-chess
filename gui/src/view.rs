use chess::Pos;
use crate::game_controller::GameController;
use piston_window::{clear, Context, G2d, rectangle, image, Texture, TextureSettings};

const COLOR_1: [f32; 4] = [0.29, 0.39, 0.54, 1.0];
const COLOR_2: [f32; 4] = [0.82, 0.87, 0.96, 1.0];
const COLOR_3: [f32; 4] = [0.34, 0.31, 0.31, 1.0];
const COLOR_4: [f32; 4] = [0.58, 0.59, 0.63, 1.0];
const COLOR_5: [f32; 4] = [0.15, 0.12, 0.10, 1.0];

pub struct ViewSettings {
    pub background_color: [f32; 4],
    pub border_color: [f32; 4],
    pub white_color: [f32; 4],
    pub black_color: [f32; 4],
    pub move_color: [f32; 4],
}

impl Default for ViewSettings {
    fn default() -> ViewSettings {
        ViewSettings {
            background_color: COLOR_3,
            border_color: COLOR_5,
            white_color: COLOR_1,
            black_color: COLOR_2,
            move_color: COLOR_4,
        }
    }
}

pub struct View {
    pub settings: ViewSettings,
}

impl View {
    pub fn new(settings: ViewSettings) -> View {
        View {
            settings,
        }
    }

    pub fn render(&mut self, controller: &GameController, c: Context, g: &mut G2d) {
        let view_size = c.get_view_size();
        let width = view_size[0];
        let height = view_size[1];

        let min_padding = width / 5.0;

        let (board_size, x_padding, y_padding) = if width - min_padding * 2.0 <= height {
            let board_size: f64 = width - 2.0 * min_padding;

            let x_padding: f64 = min_padding;
            let y_padding: f64 = (height - board_size) / 2.0;

            (board_size, x_padding, y_padding)
        } else {
            let board_size: f64 = height;

            let x_padding: f64 = (width - height) / 2.0;
            let y_padding: f64 = 0.0;

            (board_size, x_padding, y_padding)
        };


        // Draw background
        clear(self.settings.background_color, g);

        // Draw board
        rectangle(
            self.settings.border_color,
            [x_padding, y_padding, board_size, board_size],
            c.transform,
            g,
        );

        let cell_size = (board_size - 4.0) / 8.0;
        for x in 0..8 {
            for y in 0..8 {
                let current_color = if x % 2 == y % 2 {
                    self.settings.black_color
                } else {
                    self.settings.white_color
                };

                let x_pos = x_padding + 2.0 + cell_size * x as f64;
                let y_pos = y_padding + 2.0 + cell_size * y as f64;

                rectangle(
                    current_color,
                    [x_pos, y_pos, cell_size, cell_size],
                    c.transform,
                    g,
                );

                let piece = controller.game.at_pos(Pos::new_xy(x, y));

                if !piece.is_empty() {
                }
            }
        }

    }
}
