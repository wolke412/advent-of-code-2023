use std::{time::Instant, str::from_utf8 };
use day5::modules::input;

struct Range {
    from: u64,
    to: u64,
    expr: i64
}

impl Range {

    pub fn new (str: &str ) -> Self {
        
        let v = parse_sliced_numbers(str);
        let ( dest, src, size ) = (v[0], v[1], v[2]);

        let expr = dest as i64 - src as i64;

        Range {
            from: src,
            to: src + size,
            expr
        }
    }

    /*pub fn new ( src: u64, dest: u64, size: u64 ) -> Self {
        
        let expr = dest as i64 - src as i64;

        Range {
            from: src,
            to: src + size,
            expr
        }
    }*/
}



struct CatMap {
    ranges: Vec<Range>
}

impl CatMap {

    pub fn n_w_cap( cap: usize ) -> Self {
        CatMap { ranges: Vec::with_capacity(cap) }
    }

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
    
    str 
        .chars()
        //.split_whitespace()
        .map(|_| 0)
        //.filter_map(|nums| nums.parse::<u64>().ok())
        .collect::<Vec<u64>>()
    
}

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

    
    println!("Calc seeds Took: {:?}", bench_start.elapsed());

    let mut maps_strings: Vec<Vec<&str>> = Vec::with_capacity(7);
    
    lines
        .for_each(|f| { 
            
            if f.is_empty() { 
                maps_strings.push(Vec::new());
                return;
            }

            //if  f.as_bytes()[0].is_ascii_digit() {
                let l = maps_strings.len() - 1;
                maps_strings[l].push( f );
            //}
        });

    
    println!("Populate str vec Took: {:?}", bench_start.elapsed());
    
    let a: Vec<CatMap> = maps_strings.into_iter()
        
        .map( | vecstr | {
            let mut cm = CatMap::n_w_cap( vecstr.len() - 1 );
            
            vecstr[1..].iter().for_each(|str| cm.ranges.push(Range::new(str)) );
            
            cm
        }).collect();

    println!("create maps: {:?}", bench_start.elapsed());

    let min = seeds.into_iter().map(|mut seed| {
        //print!("Seed: {seed} -> ");
        
        a.iter().for_each(|map| {
            seed = map.convert( seed );
            //print!("{seed} -> ");
        });

        //println!();
        
        seed
    }).min();

    
    println!("Jumps bet. maps: {:?}", bench_start.elapsed());

    let act_min = min.unwrap();

    println!("Min location is {act_min}");
    println!("Took: {:?}", bench_start.elapsed());

}
