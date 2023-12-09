use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;

fn loop_it_in(initial_vec:&Vec<i64>) -> i64{
    let next_vec : Vec<i64> = initial_vec.windows(2).map(|win| win[1] - win[0]).collect();
    if next_vec.iter().all(|x| *x == 0 ){
        return initial_vec[0];
    }else{
       return initial_vec[initial_vec.len() - 1 as usize] - loop_it_in(&next_vec);
    }
}

fn get_next_value(input:String) -> i64{
    let vec_val : Vec<i64> =  input.split_whitespace().map(|x| x.parse::<i64>().unwrap_or(i64::MIN)).collect();
    loop_it_in(&vec_val)
}

fn main(){
    let args : Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut sum : i64 = 0;
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            sum += get_next_value(line);
        }
    }
    println!("final sum {:?}", sum);
}