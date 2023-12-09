use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;

fn loop_it_in(initial_vec:&Vec<i64>) -> i64{
    let mut next_vec = Vec::new();
    println!("initial vec{:?}", initial_vec);
    for i in 0..initial_vec.len()-1{
        next_vec.push( initial_vec[i+1] - initial_vec[i]);
    }
    println!("next vec {:?}", next_vec);
    let mut next_step_value : i64 = 0;
    if next_vec.clone().iter().all(|x| *x == 0 as i64){
        return initial_vec[0];
    }else{
        next_step_value = loop_it_in(&next_vec);
    }
    println!("next step value {:?}", next_step_value);
    initial_vec[initial_vec.len() - 1 as usize] + next_step_value

}

fn get_next_value(input:String) -> i64{
    let vec_val : Vec<i64> =  input.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i64>().unwrap_or(i64::MIN)).collect();
    
    loop_it_in(&vec_val)
   
}

fn main(){
    let args : Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vec_of_next_value : Vec<i64> = Vec::new();

    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            vec_of_next_value.push(get_next_value(line));
        }
    }
    let sum =  vec_of_next_value.iter().fold(0,|acc,x| acc+x);
    println!("final sum {:?}", sum);
}