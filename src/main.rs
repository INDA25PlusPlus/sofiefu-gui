// chess library
use lachess::*;
use lachess::{Board, Position, MoveResult, Piece, PieceType, PieceColor};

// gui
use ggez::graphics::{self, Color, Rect, DrawMode, Mesh, Image, DrawParam};
use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::input::keyboard::KeyInput;
use std::collections::HashMap;
use std::char::from_digit;
const side: f32 = 60.0; // pixels per square

// networking
use std::net::TcpStream;
mod network;

struct MainState {
    board: lachess::Board, 
    images: HashMap<String, Image>,
    start: Option<lachess::Position>, 
    end: Option<lachess::Position>,
    stream: TcpStream,
    my_color: bool, // black=0, white=1
    my_turn: bool,
}

// gui stuff
impl MainState {
    pub fn new(ctx: &mut ggez::Context, my_color: bool, my_turn: bool, stream: TcpStream) -> MainState {
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
            stream,
            my_color,
            my_turn,
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

impl MainState {
    pub fn get_character(&mut self, piece: lachess::Piece) -> char {
        match piece {
            lachess::Piece{ type_: lachess::PieceType::Pawn, color: lachess::PieceColor::Black }   => 'p',
            lachess::Piece{ type_: lachess::PieceType::Knight, color: lachess::PieceColor::Black }   => 'n',
            lachess::Piece{ type_: lachess::PieceType::Bishop, color: lachess::PieceColor::Black }   => 'b',
            lachess::Piece{ type_: lachess::PieceType::Rook, color: lachess::PieceColor::Black }   => 'r',
            lachess::Piece{ type_: lachess::PieceType::Queen, color: lachess::PieceColor::Black }   => 'q',
            lachess::Piece{ type_: lachess::PieceType::King, color: lachess::PieceColor::Black }   => 'k',

            lachess::Piece{ type_: lachess::PieceType::Pawn, color: lachess::PieceColor::White }   => 'P',
            lachess::Piece{ type_: lachess::PieceType::Knight, color: lachess::PieceColor::White }   => 'N',
            lachess::Piece{ type_: lachess::PieceType::Bishop, color: lachess::PieceColor::White }   => 'B',
            lachess::Piece{ type_: lachess::PieceType::Rook, color: lachess::PieceColor::White }   => 'R',
            lachess::Piece{ type_: lachess::PieceType::Queen, color: lachess::PieceColor::White }   => 'Q',
            lachess::Piece{ type_: lachess::PieceType::King, color: lachess::PieceColor::White }   => 'K',
        }
    }

    pub fn generate_message(&mut self, promotion: bool, start_r: i8, start_c: i8, end_r: i8, end_c: i8) -> String {
        let mut msg = String::from("ChessMOVE:");

        // send move - 1-indexed + promotion
        msg.push((start_c as u8+65) as char); // 65 --> 'A'
        if let Some(sr) = from_digit((start_r+1) as u32, 10) { msg.push(sr); }
        msg.push((end_c as u8+65) as char); 
        if let Some(er) = from_digit((end_r+1) as u32, 10) { msg.push(er); }

        if promotion { msg.push('q'); }
        else { msg.push('0'); }
        msg.push(':');


        // game state
        if self.board.is_checkmate() { 
            if (self.my_color && self.my_turn) || (!self.my_color && !self.my_turn) { msg.push_str("1-0"); }
            else { msg.push_str("0-1"); }
        }
        else if self.board.is_stalemate() {msg.push_str("1-1"); }
        else { msg.push_str("0-0"); }
        msg.push(':');


        // send board
        for row in (0..8).rev() {
            let mut ct: u32 = 0;
            for col in 0..8 {
                let pos = lachess::Position{
                    file: col as i8,
                    rank: row as i8,
                };

                if let Some(piece) = self.board.piece_at_pos(pos) {
                    if ct>0 && let Some(digit) = from_digit(ct, 10) { msg.push(digit); }
                    msg.push(self.get_character(piece));
                    ct=0;
                }
                else if col == 7 {
                    if ct>0 && let Some(digit) = from_digit(ct, 10) { msg.push(digit); }
                }
                else {
                    ct=ct+1;
                }
            }
            msg.push('/');
        }
        msg.push(':');

        // padding
        while msg.len() < 128 {
            msg.push('0');
        }
        return msg;
    }

    pub fn rage_quit(&mut self) -> GameResult<()> {
        let msg = "ChessQUIT::000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
        network::send_message(&mut self.stream, msg);
        return Err(GameError::EventLoopError("opponent gave invalid move".to_string())); 
    }  

    pub fn receive_move(&mut self, msg: &String) -> GameResult<()> {
        println!("mottagit drag");
        if &msg[0..9] == "ChessQUIT" { return Err(GameError::EventLoopError("opponent quited".to_string())); } 

        // decode move
        let mut from = lachess::Position{ file: 10, rank: 10 };
        let mut to = lachess::Position{ file: 10, rank: 10 };
        if let Some(start_c) = msg.chars().nth(9) && let Some(start_r) = msg.chars().nth(10) {
            from.rank = ((start_c as u32)-65) as i8;
            from.file = ((start_r as u32)-48) as i8;
        } 
        if let Some(end_c) = msg.chars().nth(11)  && let Some(end_r) = msg.chars().nth(12) {
            to.rank = ((end_c as u32)-65) as i8;
            to.file = ((end_r as u32)-48) as i8;
        } 

        match self.board.make_move(from, to) {
            MoveResult::Normal => {
                let my_msg = self.generate_message(false, from.rank, from.file, to.rank, to.file);
                
                let mut ct = 0; let mut i = 0;
                while i < 128 {
                    if let Some(chA) = my_msg.chars().nth(i) && let Some(chB) = msg.chars().nth(i) { if chA != chB { self.rage_quit(); } } 
                    if let Some(ch) = my_msg.chars().nth(i){ if ch == ':' {ct+=1;} }
                    if ct == 4 {break;}
                }
            },
            MoveResult::Promotion => {
                // change to the one they send
                self.board.resolve_promotion(PieceType::Queen).unwrap();
                let my_msg = self.generate_message(true, from.rank, from.file, to.rank, to.file);

                let mut ct = 0; let mut i = 0;
                while i < 128 {
                    if let Some(chA) = my_msg.chars().nth(i) && let Some(chB) = msg.chars().nth(i) { if chA != chB { self.rage_quit(); } } 
                    if let Some(ch) = my_msg.chars().nth(i){ if ch == ':' {ct+=1;} }
                    if ct == 4 {break;}
                }
            },
            MoveResult::Illegal => {
                return self.rage_quit();
            }
        }
        return Ok(());
    }

}


impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // vänta på meddelande från motståndare
        if let Some(msg) = network::try_receive_message(&mut self.stream) {
            if self.my_turn { self.rage_quit(); }
            let res = self.receive_move(&msg);
            self.my_turn = true;
            return res;
        }
        return Ok(());
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
                if let Some(from) = self.start && let Some(to) = self.end && self.my_turn { // move made
                    println!("making move");
                    match self.board.make_move(from, to) {
                        MoveResult::Normal => {
                            println!("move made");
                            self.start = None; self.end = None; 
                            let msg = self.generate_message(false, from.rank, from.file, to.rank, to.file);
                            network::send_message(&mut self.stream, msg);
                            self.my_turn = false;
                        },
                        MoveResult::Promotion => {
                            println!("pawn promotion!");
                            self.board.resolve_promotion(PieceType::Queen).unwrap();
                            let msg = self.generate_message(true, from.rank, from.file, to.rank, to.file);

                            network::send_message(&mut self.stream, msg);
                            self.my_turn = false;

                        },
                        MoveResult::Illegal => {
                            println!("illegal move");
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
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "Sofie")
    .add_resource_path("./resources")
    .build()
    .unwrap();

    // PLAY AS WHITE (CLIENT)
    // match network::start_client() {
    //     Ok(stream) => {
    //         let state = MainState::new(&mut ctx, true, ture, stream); 
    //         event::run(ctx, event_loop, state);
    //     }
    //     Err(e) => {
    //         println!("failed to start server");
    //     }
    // }

    // PLAY AS BLACK (SERVER)
    match network::start_server() {
        Ok(stream) => {
            let state = MainState::new(&mut ctx, false, false, stream); 
            event::run(ctx, event_loop, state);
        }
        Err(e) => {
            println!("failed to start server");
        }
    }
}



