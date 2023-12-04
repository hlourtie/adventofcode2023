use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

fn parse_input_into_2d_vec(input: &String) -> Vec<Vec<u32>> {
    let initial_split : Vec<&str> = input.split(":").collect();
    let result;
    if let Some(second_part) = initial_split.get(1) {
        result = second_part.trim();
    } else {
        println!("No substring found after ':'");
        result="";
    }

    let second_split :Vec<&str> = result.split("|").collect();

    let mut final_vect : Vec<Vec<u32>> = Vec::new();

    for sect in second_split {

        let mut num_values : Vec<u32> = Vec::new();
        let number_lines: Vec<&str> = sect.split_whitespace().collect();

        for num in number_lines{
            num_values.push(num.parse::<u32>().unwrap_or(u32::MIN));
        }
        final_vect.push(num_values);
    }
    final_vect
}
fn game_value_no_rules(input: &String) -> usize {
    let v2d = parse_input_into_2d_vec(input);
    //println!("{:?}", v2d);
    let flattened: Vec<_> = v2d[0].iter().filter(|&x| v2d[1].contains(&x)).collect();
    //println!("{:?}", flattened);
    if flattened.len()>0{
        u32::pow(2, (flattened.len() - 1) as u32) as usize
    }else{
        0
    }
}

fn game_value_wit_rules(input: &String) -> usize {
    let v2d = parse_input_into_2d_vec(input);
    //println!("{:?}", v2d);
    let flattened: Vec<_> = v2d[0].iter().filter(|&x| v2d[1].contains(&x)).collect();
    //println!("{:?}", flattened);
    flattened.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    let mut vec : Vec<usize>  = Vec::new();
    
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            if &args[2] == "1" { 
                vec.push(game_value_no_rules(&line));
            }else if &args[2] == "2"{
                vec.push(game_value_wit_rules(&line));
            }
        }
    }

    if &args[2] == "2"{
        let mut vec_counter: Vec<usize> = vec![1;vec.len()];
        let mut i = 0;
        while i < vec.len(){
            let mut j: usize = 1;
            while j <= vec[i]{
                println!("{:?}",j);
                if i+j < vec.len(){
                    vec_counter[i+j] += 1 * vec_counter[i];
                }
                j+=1;
            }
            i+=1;
        }
        println!("{:?}", vec_counter);
        let sum2 = vec_counter.iter().fold(0, |acc, x| acc + x);
        println!("{:?}",sum2);
    }


    println!("{:?}", vec);
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    println!("{:?}",sum);
    
}