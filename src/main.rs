use lachess::*;
use lachess::{Board, Position, MoveResult, Piece, PieceType, PieceColor};

use ggez::graphics::{self, Color, Rect, DrawMode, Mesh, Image, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::input::keyboard::KeyInput;
use std::collections::HashMap;

const side: f32 = 60.0; // pixels per square

struct MainState {
    board: lachess::Board, 
    images: HashMap<String, Image>,
    start: Option<lachess::Position>, 
    end: Option<lachess::Position>,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> MainState {
        MainState {
            board: lachess::Board::starting_position(),
            images: HashMap::from([
                ("bpawn".to_string(), Image::from_path(ctx, "/bpawn.png").unwrap()),
                ("bknight".to_string(), Image::from_path(ctx, "/bknight.png").unwrap()),
                ("bbishop".to_string(), Image::from_path(ctx, "/bbishop.png").unwrap()),
                ("brook".to_string(), Image::from_path(ctx, "/brook.png").unwrap()),
                ("bqueen".to_string(), Image::from_path(ctx, "/bqueen.png").unwrap()),
                ("bking".to_string(), Image::from_path(ctx, "/bking.png").unwrap()), 
                

                ("wpawn".to_string(), Image::from_path(ctx, "/wpawn.png").unwrap()),
                ("wknight".to_string(), Image::from_path(ctx, "/wknight.png").unwrap()),
                ("wbishop".to_string(), Image::from_path(ctx, "/wbishop.png").unwrap()),
                ("wrook".to_string(), Image::from_path(ctx, "/wrook.png").unwrap()),
                ("wqueen".to_string(), Image::from_path(ctx, "/wqueen.png").unwrap()),
                ("wking".to_string(), Image::from_path(ctx, "/wking.png").unwrap()), 
            ]),
            start: None,
            end: None,
        }
    }

    pub fn get_image(&mut self, piece: lachess::Piece) -> Image {
        match piece {
            lachess::Piece{ type_: lachess::PieceType::Pawn, color: lachess::PieceColor::Black }   => self.images["bpawn"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Knight, color: lachess::PieceColor::Black }   => self.images["bknight"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Bishop, color: lachess::PieceColor::Black }   => self.images["bbishop"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Rook, color: lachess::PieceColor::Black }   => self.images["brook"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Queen, color: lachess::PieceColor::Black }   => self.images["bqueen"].clone(),
            lachess::Piece{ type_: lachess::PieceType::King, color: lachess::PieceColor::Black }   => self.images["bking"].clone(),

            lachess::Piece{ type_: lachess::PieceType::Pawn, color: lachess::PieceColor::White }   => self.images["wpawn"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Knight, color: lachess::PieceColor::White }   => self.images["wknight"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Bishop, color: lachess::PieceColor::White }   => self.images["wbishop"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Rook, color: lachess::PieceColor::White }   => self.images["wrook"].clone(),
            lachess::Piece{ type_: lachess::PieceType::Queen, color: lachess::PieceColor::White }   => self.images["wqueen"].clone(),
            lachess::Piece{ type_: lachess::PieceType::King, color: lachess::PieceColor::White }   => self.images["wking"].clone(),
        }
    }
}   

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(200, 200, 200));

        for row in (0..8).rev() {
            for col in 0..8 {
                let posr = ((7-row) as f32) * side;
                let posc = (col as f32) * side;
                let pos = lachess::Position{
                    file: col as i8,
                    rank: row as i8,
                };

                // DRAW BOARD
                // set color of square
                let is_white: bool = (row+col)%2==1;
                let mut color = if is_white {Color::from_rgb(255, 182, 193)} else {Color::from_rgb(255, 105, 180) };


                // selected squares
                if let Some(from) = self.start && let Some(to) = self.end { // move made
                    println!("making move");
                    match self.board.make_move(from, to) {
                        MoveResult::Normal => {
                            println!("Move made");
                            self.start = None;
                            self.end = None;
                        },
                        MoveResult::Promotion => {
                            println!("Pawn promotion!");
                            self.board.resolve_promotion(PieceType::Queen).unwrap();
                        },
                        MoveResult::Illegal => {
                            println!("Illegal move");
                            self.start = None;
                            self.end = None;
                        }
                    }
                }
                else if let Some(from) = self.start { // square selected
                    if from.rank as i32 == row && from.file as i32 == col {
                        color = Color::from_rgb(100, 126, 204); // blue
                    }

                    // generate valid moves
                    let legal_destinations = self.board.legal_moves(from);

                    for &to in &legal_destinations {
                        if to.rank as i32 == row && to.file as i32 == col {
                            color = Color::from_rgb(173, 216, 230); // light blue
                        }
                    }
                }

                let rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(posc, posr, side, side),
                    color,
                )?;
                canvas.draw(&rect, [0.0, 0.0]);

                // DRAW PIECES
                let pos = lachess::Position{
                    file: col as i8,
                    rank: row as i8,
                };
                if let Some(piece) = self.board.piece_at_pos(pos) {
                    let piece_type = piece.type_;
    
                    let img = self.get_image(piece);
                    // draw centered
                    let scale_x = side / img.width() as f32;
                    let scale_y = side / img.height() as f32;
                    canvas.draw(
                        &img,
                        DrawParam::default()
                            .dest([posc + side/2.0, posr + side/2.0]) 
                            .offset([0.5, 0.5])                        
                            .scale([scale_x, scale_y]),               
                    );
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32,) -> GameResult {
        println!("Button Pressed");

        // check if mouse is inside the board 
        if x>=0.0 && x<8.0*side && y>=0.0 && y<8.0*side {
            let col = (x / side) as usize; 
            let row = 7-(y / side) as usize; 
            println!("Button pressed on square: row {}, col {}", row, col);

            let selected_pos = lachess::Position {
                file: col as i8,
                rank: row as i8,
            };

            if selected_pos.is_on_board() {
                
                if let Some(alr_selected_square) = self.start.clone() {
                    if alr_selected_square == selected_pos { self.start=None; } 
                    else { self.end = Some(selected_pos) };
                }
                else if let Some(piece) = self.board.piece_at_pos(selected_pos) {
                    println!("Piece chosen: {:?}", piece);
                    self.start = Some(selected_pos);
                }
            }
            else {
                self.start=None; 
            }
        }

        Ok(())
    }

}

pub fn main() {
    println!("Hello, world!");
    
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "Sofie")
    .add_resource_path("./resources")
    .build()
    .unwrap();

    let state = MainState::new(&mut ctx); 
    event::run(ctx, event_loop, state);
}



