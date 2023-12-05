use std::time::Instant;

use day3::modules::input;

const LAST_VALID_INDEX: usize = 139;

fn make_digit_from_buffer( buf: &[u8; 4], size: usize ) -> u16 {

    let mut mult: u16 = ( 1..size ).fold( 1, |s, _| s * 10 );
    let mut val: u16 = 0;

    for d in buf {
        val += *d as u16 * mult;
        mult /= 10
    }

    val 
}


#[derive(Debug)]
struct Num {    
    number: u16,
    size: usize,
    position: usize
}

impl Num {

    pub fn get_range ( &self ) -> ( usize, usize ) {

        let n = self;

        let mut start = n.position;
        let mut end = n.position + n.size - 1;

        if start > 0 { start -= 1 }
        if end < LAST_VALID_INDEX { end += 1 }

        (start, end)
    }
}

struct Line {

    symbols:  [bool; 140],
    numbers: Vec<Num>
}

impl Line {
    
    pub fn from( string: &str ) -> Self {
            
        let mut buf = [0 as u8; 4];
        let mut cur = 0;
        
        let mut sym = [false; 140];
        let mut nums: Vec<Num> = Vec::new();

        string.chars().enumerate().for_each(|(i, ch)| {
            
            if ch.is_digit(10) {
                
                buf[cur] = ch.to_digit(10).unwrap() as u8;
                cur += 1;

                if i != LAST_VALID_INDEX {
                    return 
                }
            }

            if cur > 0 {

                let n = Num {
                    number:  make_digit_from_buffer(&buf, cur as usize ),
                    size: cur,
                    position: i - cur
                };

                nums.push( n );
                cur = 0;
            } 

            if ch != '.' && ! ch.is_digit(10){
                sym[i] = true;
            }    
        });

        Line {
            symbols: sym,
            numbers: nums
        }
    }

    pub fn has_symbols_at_number ( &self, n: &Num) -> bool {
 
        let (start, end) = n.get_range();

        for i in start..=end {
            if self.symbols[ i ] {
                return true
            }
        }

        false
    }

    // passing both to avoid mutipe fn calls and loop constructors
    pub fn compare_and_fold ( &self, l1: Option<&Line>, l2: Option<&Line> ) -> u16 {
        let mut sum: u16 = 0;

        let nums = &self.numbers;

        for n in nums {

            // compares to its own symbols
            if self.has_symbols_at_number( &n ) {
                sum += n.number;
                continue
            }
            
            // compares to last line symbols
            if let Some(ln) = l1 { 
                if ln.has_symbols_at_number( &n ) {
                    sum += n.number;
                    continue;
                }
            }

            // compares to next line symbols
            if let Some(ln) = l2 { 
                if ln.has_symbols_at_number( &n ) {
                    sum += n.number;
                    continue
                }
            }
        }

        sum
    }
}

fn main () {

    let bench_start = Instant::now();

    let file = input::file_in("static/input.txt");

    let mut total_count: u32 = 0;

    let mut last_line: Option<Line>;
    let mut current_line: Option<Line> = None;
    let mut next_line: Option<Line> = None;

    for line in file.lines().into_iter() {

        last_line = current_line;
        current_line = next_line;

        next_line = Some( Line::from( line ) );

        if current_line.is_none() {
            continue;
        }

        let sum = current_line
            .as_ref()
            .unwrap()
            .compare_and_fold( 
                last_line.as_ref(), 
                next_line.as_ref() 
            );
        
        total_count += sum as u32;
    };

    // last line 
    last_line = current_line;
    current_line = next_line;

    total_count += current_line
        .as_ref()
        .unwrap()
        .compare_and_fold(last_line.as_ref(), None.as_ref()) as u32; 


    println!("Total count: {total_count}");
    println!("Time elapsed: {:.4?}", bench_start.elapsed());
}
