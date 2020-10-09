use crate::game_controller::{GameController, State};
use chess::{Color, Pos};
use piston_window::{self, clear, ellipse, rectangle, Context, G2d, G2dTexture, Image};
use std::path::Path;

const COLOR_1: [f32; 4] = [0.29, 0.39, 0.54, 1.0];
const COLOR_2: [f32; 4] = [0.82, 0.87, 0.96, 1.0];
const COLOR_3: [f32; 4] = [0.34, 0.31, 0.31, 1.0];
const COLOR_4: [f32; 4] = [0.58, 0.59, 0.63, 1.0];
const COLOR_5: [f32; 4] = [0.15, 0.12, 0.10, 1.0];

#[derive(Copy, Clone)]
pub struct ViewSettings {
    pub board_size: f64,
    pub padding: f64,
    pub promotion_width: f64,
    pub promotion_height: f64,
    pub background_color: [f32; 4],
    pub border_color: [f32; 4],
    pub white_color: [f32; 4],
    pub black_color: [f32; 4],
    pub move_color: [f32; 4],
}

impl ViewSettings {
    pub fn board_pos(&self) -> [f64; 2] {
        [self.padding, 0.0]
    }

    pub fn promotion_pos(&self) -> [f64; 2] {
        let x_padding = self.padding + (self.board_size - self.promotion_width) / 2.0;
        let y_padding = (self.board_size - self.promotion_height) / 2.0;

        [x_padding, y_padding]
    }
}

impl Default for ViewSettings {
    fn default() -> ViewSettings {
        ViewSettings {
            board_size: 640.0,
            padding: 192.0,
            promotion_width: 248.0,
            promotion_height: 248.0,
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
    textures: Vec<G2dTexture>,
}

impl View {
    pub fn new(settings: ViewSettings, textures: Vec<G2dTexture>) -> View {
        View { settings, textures }
    }

    pub fn create_textures(window: &mut piston_window::PistonWindow) -> Vec<G2dTexture> {
        use piston_window::{Texture, TextureSettings};

        let colors = ["white".to_string(), "black".to_string()];
        let piece_types = [
            "pawn".to_string(),
            "rook".to_string(),
            "knight".to_string(),
            "bishop".to_string(),
            "queen".to_string(),
            "king".to_string(),
        ];

        let mut textures: Vec<G2dTexture> = vec![];

        for color in colors.iter() {
            for piece in piece_types.iter() {
                println!(
                    "Fetching texture: {:?}",
                    Path::new(&format!("gui/resources/pieces/{}-{}.png", color, piece))
                );
                let texture: piston_window::G2dTexture = Texture::from_path(
                    &mut window.create_texture_context(),
                    &Path::new(&format!("gui/resources/pieces/{}-{}.png", color, piece)),
                    piston_window::Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap();

                textures.push(texture);
            }
        }

        textures
    }

    pub fn render(&mut self, controller: &GameController, c: Context, g: &mut G2d) {
        // Draw background
        clear(self.settings.background_color, g);

        // Draw board
        self.draw_board(&controller, &c, g);

        match &controller.state {
            State::Promotion(_) => self.draw_promotion_choice(&controller, &c, g),
            _ => (),
        }
    }

    fn draw_board(&mut self, controller: &GameController, c: &Context, g: &mut G2d) {
        let board_size = self.settings.board_size;
        let board_pos = self.settings.board_pos();

        // Draw border
        rectangle(
            self.settings.border_color,
            [board_pos[0], board_pos[1], board_size, board_size],
            c.transform,
            g,
        );

        // Draw cells
        let cell_size = (board_size - 4.0) / 8.0;
        for x in 0..8 {
            for y in 0..8 {
                let mut current_color = if x % 2 != y % 2 {
                    self.settings.black_color
                } else {
                    self.settings.white_color
                };

                if let Some(selected) = controller.selected_square {
                    if selected[0] == x && selected[1] == y {
                        current_color = self.settings.move_color
                    }
                };

                let x_pos = board_pos[0] + 2.0 + cell_size * x as f64;
                let y_pos = board_pos[1] + 2.0 + cell_size * (7.0 - y as f64);
                let cell = [x_pos, y_pos, cell_size, cell_size];

                rectangle(current_color, cell, c.transform, g);

                if let Some(moves) = &controller.current_moves {
                    if moves.contains_key(&[x, y]) {
                        ellipse(self.settings.move_color, cell, c.transform, g)
                    }
                };

                let piece = controller.game.at_pos(Pos::new_xy(x as u8, y as u8));

                if !piece.is_empty() {
                    let index = match piece.color() {
                        Color::White => piece.get_type() as u8 - 1,
                        Color::Black => piece.get_type() as u8 + 5,
                    };
                    let image = Image::new().rect([x_pos, y_pos, cell_size, cell_size]);

                    image.draw(
                        &self.textures[index as usize],
                        &piston_window::DrawState::default(),
                        c.transform,
                        g,
                    )
                }
            }
        }
    }

    fn draw_promotion_choice(&mut self, controller: &GameController, c: &Context, g: &mut G2d) {
        // Dim everything else
        let mut dim = self.settings.background_color;
        dim[3] = 0.5;
        let [w, h] = c.get_view_size();
        rectangle(dim, [0.0, 0.0, w, h], c.transform, g);

        // Draw choices
        let x_padding = self.settings.padding
            + (self.settings.board_size - self.settings.promotion_width) / 2.0;
        let y_padding = (self.settings.board_size - self.settings.promotion_height) / 2.0;

        rectangle(
            self.settings.border_color,
            [
                x_padding,
                y_padding,
                self.settings.promotion_width,
                self.settings.promotion_width,
            ],
            c.transform,
            g,
        );

        let cell_size = 120.0;

        let base_index = match controller.game.current_color() {
            Color::White => -1,
            Color::Black => 5,
        };

        for x in 0..2 {
            for y in 0..2 {
                let current_color = if x % 2 == y % 2 {
                    self.settings.black_color
                } else {
                    self.settings.white_color
                };

                let x_pos = x_padding + 4.0 + cell_size * x as f64;
                let y_pos = y_padding + 4.0 + cell_size * y as f64;
                let cell = [x_pos, y_pos, cell_size, cell_size];

                rectangle(current_color, cell, c.transform, g);

                let index = if x == 0 && y == 0 {
                    base_index + 5 // Queen
                } else if x == 0 && y == 1 {
                    base_index + 2 // Rook
                } else if x == 1 && y == 0 {
                    base_index + 3 // Knight
                } else {
                    base_index + 4 // Bishop
                };

                let image = Image::new().rect([x_pos, y_pos, cell_size, cell_size]);

                image.draw(
                    &mut self.textures[index as usize],
                    &piston_window::DrawState::default(),
                    c.transform,
                    g,
                )
            }
        }
    }
}
