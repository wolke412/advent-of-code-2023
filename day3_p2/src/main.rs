use std::{time::Instant, collections::HashMap};

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


#[derive(Debug, Clone)]
struct Num {    
    number: u16,
    size: usize,
    position: usize
}

struct Gear {
    x: usize,
    y: usize
}


struct NumGear {
    num: Num,
    gear: Gear
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
    index: usize,
    gears: [bool;140],

    numbers: Vec<Num>
}

impl Clone for Line {

    fn clone(&self) -> Self {
        Line {
            index: self.index.clone(),
            gears: self.gears.clone(),
            numbers: self.numbers.clone()

        }
    }
}

impl Line {
    
    pub fn from( string: &str, index: usize ) -> Self {
            
        let mut buf = [0 as u8; 4];
        let mut cur = 0;
        
        let mut sym = [false; 140];
        let mut nums: Vec<Num> = Vec::new();
        let mut gears = [false; 140];

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

            if ch == '*' {
                gears[ i ] = true
            }

        });

        Line {
            index,
            gears,
            numbers: nums
        }
    }

    pub fn get_gears_in_slice ( &self, n: &Num) -> Vec<usize> {
 
        let (start, end) = n.get_range();
        
        (start..=end).filter( |idx| self.gears[*idx] ).collect()
    }

    // passing both to avoid mutipe fn calls and loop constructors
    pub fn get_neighboring_gears ( &self, l1: Option<&Line>, l2: Option<&Line> ) -> Vec<NumGear> {
        let mut numgs: Vec< NumGear > = Vec::new();
        
        self.numbers.iter().for_each(| number | {
            let mut gears: Vec<Gear> = Vec::new();
            
            gears.append(
                &mut self.get_gears_in_slice(number).into_iter().map( | x | Gear{ x, y: self.index } ).collect()
            );

            if let Some(ln) = l1 {
                gears.append(
                    &mut ln.get_gears_in_slice(number).into_iter().map( | x | Gear{ x, y: ln.index } ).collect()
                );
            }

            if let Some(ln) = l2 {
                gears.append(
                    &mut ln.get_gears_in_slice(number).into_iter().map( | x | Gear{ x, y: ln.index } ).collect()
                );
            }
            

            gears.into_iter().for_each(|gear| {
                numgs.push(NumGear { num: number.clone(), gear });
            });
        });

        numgs

    }
}

fn main () {

    let bench_start = Instant::now();

    let file = input::file_in("static/input.txt");

    let mut with_gears: Vec<NumGear> = Vec::with_capacity(200); // reasonable number 

    let mut last_line: Option<Line> = None;
    let mut current_line: Option<Line> = None;
    let mut next_line: Option<Line> = None;

    file.lines().enumerate().into_iter().for_each( |(i, line)| {

        last_line = current_line.clone();
        current_line = next_line.clone();

        next_line = Some( Line::from( line, i ) );

        if current_line.is_none() {
            return;
        }

    
        current_line
            .as_ref()
            .unwrap()
            .get_neighboring_gears( 
                last_line.as_ref(), 
                next_line.as_ref() 
            ).into_iter().for_each(| num_gear | with_gears.push( num_gear ))
        
        
    });

    // last line 
    last_line = current_line.clone();
    current_line = next_line.clone();
        
    with_gears.append(
        &mut current_line
        .as_ref()
        .unwrap()
        .get_neighboring_gears( 
            last_line.as_ref(), 
            next_line.as_ref() 
            )
        );


    // group numbers by gear

    let mut gear_map: HashMap<String, Vec<u16>> = HashMap::new();
    for wg in with_gears {
        
        let gloc = format!("{},{}",wg.gear.x, wg.gear.y);
        
        let g: Option<&Vec<u16>> = gear_map.get( &gloc );

        println!("Gear at {},{}: {}", wg.gear.x, wg.gear.y, wg.num.number);
        
        if g.is_none() {
            gear_map.insert(gloc, vec![wg.num.number] );
            continue;
        } 

        if g.is_some() {

            let mut a = g.unwrap().clone();
            
            // no need to push more
            if a.len() > 2 {
                continue;
            }

            a.push( wg.num.number);

            gear_map.insert(gloc, a);

        }    
    }
    

    let sum = gear_map
        .keys()
        .into_iter()
        .filter(| k | {
            let a = gear_map.get(k.to_owned());
            a.is_some() && a.unwrap().len() == 2  
        })
        .map( | k |  {
            let a = gear_map.get(k).unwrap();
            
            let n1 = a[0] as u64;
            let n2 = a[1] as u64;
            
            let r = n1 * n2;
            
            println!("{k}: Prod of {n1} * {n2} is {r}");
            return r;
        })
        .sum::<u64>();

    println!("Tem sum is {sum}");
    println!("Time elapsed: {:.4?}", bench_start.elapsed());
}
w