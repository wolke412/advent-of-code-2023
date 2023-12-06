use std::{time::Instant};
use day5::modules::input;

struct Range {
    from: u64,
    to: u64,
    expr: i64
}

impl Range {
    pub fn new ( src: u64, dest: u64, size: u64 ) -> Self {
        
        let expr = dest as i64 - src as i64;

        Range {
            from: src,
            to: src + size,
            expr
        }
    }
}



struct CatMap {
    ranges: Vec<Range>
}

impl CatMap {

    pub fn convert ( &self, num: u64 ) -> u64 {
        
        for range in &self.ranges {
            if num >= range.from && num < range.to {
                return ( num as i64 + range.expr ) as u64;
            }           
        }

        num
    } 
}

fn get_seeds(str: &str) -> Vec<u64> {
    
    let ( _, seeds_slice ) = str.split_once(": ").expect("Error finding seeds.");

    parse_sliced_numbers(seeds_slice.trim())
}

fn parse_sliced_numbers( str: &str ) -> Vec<u64> {

    // performance: count whitesspaces and prealloc vec
    str 
        .split_whitespace()
        .filter_map(|nums| nums.parse::<u64>().ok())
        .collect()
}


// expected maps 
const NUMBER_OF_MAPS: u8 = 7;

fn main () {

    let bench_start = Instant::now();

    let file = input::file_in("static/input.txt");
    
    let mut lines = file
        .lines()
        .into_iter();
    
    let seeds = get_seeds(
        lines
            .next()
            .unwrap()
    );

    println!("Seeds are: {:?}", seeds);

    let mut maps_strings: Vec<Vec<&str>> = Vec::with_capacity(7);
    
    lines
        .for_each(|f| { 
            
            if f.is_empty() { 
                maps_strings.push(Vec::new());
                return;
            }

            if  f.as_bytes()[0].is_ascii_digit() {
                let l = maps_strings.len() - 1;
                maps_strings[l].push( f );
            }
        });

    
    let a: Vec<CatMap> = maps_strings.into_iter()
        .map( | vecstr | {
            let nums =  vecstr.into_iter().map(|str| parse_sliced_numbers(str)).map( |n| Range::new(n[1], n[0], n[2]) ).collect();
            
            CatMap { 
                ranges: nums           
            }

        }).collect();

    let min = seeds.into_iter().map(|mut seed| {
        print!("Seed: {seed} -> ");
        
        a.iter().for_each(|map| {
            seed = map.convert( seed );
            print!("{seed} -> ");
        });

        println!();
        
        seed
    }).min();

    let act_min = min.unwrap();

    println!("Min location is {act_min}");
    println!("Took: {:?}", bench_start.elapsed());
}
