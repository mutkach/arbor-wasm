use macroquad::prelude::*;
use tile::Tile;
use rand::rand;
mod tile;


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut tiles : Vec<Tile> = Vec::<Tile>::new();
    let mut log : Vec<String> = Vec::<String>::new();
    //let tile = Tile::new(10, 11, String::from("*"));
    for _ in 1..20 {
        let _x = rand::RandomRange::gen_range(100, 650) as u16; 
        let _y = rand::RandomRange::gen_range(100, 650) as u16; 
        let x = _x - _x.rem_euclid(50);
        let y = _y - _y.rem_euclid(50);
        let tmp_tile = Tile::new(x, y, String::from("0"));
        let log_entry = format!("x:{}, y:{}", tmp_tile.x, tmp_tile.y);
        log.push(log_entry);
        tiles.push(tmp_tile);
    }

    let log_text = log.join("\n");

    //tiles.push(tile);
    loop {
        //let mut tiles : Vec<Tile>;
        //


        clear_background(BLACK);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);


        for t in &tiles {
            draw_text(&t.glyph, t.y as f32, t.x  as f32, 50.0, WHITE);
        }




        //draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&log_text, 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
