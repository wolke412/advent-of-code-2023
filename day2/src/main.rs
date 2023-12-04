use day2::modules::input;

const GAME_DATA_SEPARATOR:   &str = ":";
const GAME_ID_SEPARATOR:     &str = "Game";
const GAME_DRAW_SEPARATOR:   &str = ";";
const GAME_COLORS_SEPARATOR: &str = ",";

const COLOR_BLUE:  &str = "blue";
const COLOR_GREEN: &str = "green";
const COLOR_RED:   &str = "red";

const MAX_RED:   u8 = 12; 
const MAX_GREEN: u8 = 13;
const MAX_BLUE:  u8 = 14;

enum Color {

    Blue(u8),
    Green(u8),
    Red(u8)
}

struct Draw {
    red: u8,
    green: u8,
    blue: u8
}

impl Draw {

    pub fn new () -> Draw {
        Draw {
            red: 0,
            green: 0,
            blue: 0
        }
    }
    
    fn get_color_value (color: &str) -> Color {
        let mut spt = color.split_whitespace();

        let val = spt
            .next()
            .expect("Found invalid value");
        
        let clr = spt
            .next()
            .expect("Found invalid color");

        let act_val: u8 = val.parse().expect("Unable to parse str value to u8.") ;
        
        match clr {
            COLOR_RED => return Color::Red(act_val),
            COLOR_BLUE => return Color::Blue(act_val),
            COLOR_GREEN => return Color::Green(act_val),
            
            _ => panic!("Unexpect color name: {}", clr)
        }

    }

    pub fn exceeded (&self) -> bool {
    
           *&self.blue  > MAX_BLUE
        || *&self.red   > MAX_RED
        || *&self.green > MAX_GREEN
    }

    pub fn from(line: &str) -> ( u8, Vec<Draw> ) {
        
        let game_draws = line
            .split_once( GAME_DATA_SEPARATOR )
            .expect(format!("Error reading line: {}", line).as_str());

        let ( game, draws ) = game_draws;    
        let (_, mut game_id) = game.split_once(GAME_ID_SEPARATOR).expect("unable to parse game id.");
        
        game_id = game_id.trim_start();

        let u_gameid: u8 = game_id.parse().expect("Failed to parse game id");
        
        let r_draws: Vec<Draw> = draws
            .split( GAME_DRAW_SEPARATOR )
            .map( |draw| {
                
                let mut new_draw = Draw::new();

                draw.split( GAME_COLORS_SEPARATOR )
                   .for_each( |color| {
                        
                        let clr = Draw::get_color_value(color);

                        match clr {
                            
                            Color::Red(v) => new_draw.red = v,
                            Color::Green(v) => new_draw.green = v,
                            Color::Blue(v) => new_draw.blue = v
                        }
                    });
                
                new_draw

            })
            .collect();

        ( u_gameid, r_draws )
    }
    
}


fn main() {

    let file = input::file_in("static/input.txt");

    let mut game_id_count: u16 = 0;

    file.lines().for_each(|ln| {
        
        let (gid, draws ) = Draw::from( ln );

        for d in draws {
            if d.exceeded() { 
                return
            }
        }

        println!("Game {} is valid", gid);

        game_id_count += gid as u16;

    });

    println!("Sum of the valid games is: {}.", game_id_count);
}
