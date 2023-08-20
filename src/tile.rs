use macroquad::color::{Color, RED, GREEN, BLACK, MAGENTA};



pub struct Tile {
    pub x: u16,
    pub y: u16,
    pub glyph : String,
    pub fg: Color,
    pub bg: Color,
    pub blink: bool,

}



impl Tile {
    pub fn default() -> Tile {
        Tile {
            x : 1,
            y : 1,
            glyph : String::from("@"),
            fg: Color::default(),
            bg: Color::default(),
            blink : false,
        }

    }
    pub fn new(x: u16, y: u16, glyph : String) -> Tile {
        Tile {
            x,
            y,
            glyph,
            fg: Color::default(),
            bg: Color::default(),
            blink : false,
        }
    }
}
