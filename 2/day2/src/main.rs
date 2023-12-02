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
    let second_split :Vec<&str> = result.split(";").collect();

    let mut final_vect : Vec<Vec<u32>> = Vec::new();

    for sect in second_split {

        let mut rgb_values : Vec<u32> = vec![0;3];
        let colors_combo: Vec<&str> = sect.split(",").collect();

        for color_line in colors_combo{

            let tokens: Vec<&str> = color_line.trim().split_whitespace().collect();

            if let Some(number_of_stones) = tokens.get(0) {

                if let Some(color) = tokens.get(1){

                    if let Ok(count) = number_of_stones.parse::<u32>() {

                        match color {
                            &"red" => rgb_values[0] += count,
                            &"green" => rgb_values[1] += count,
                            &"blue" => rgb_values[2] += count,
                            _ => (),
                        }
                    }
                }
            }
        }
        final_vect.push(rgb_values);
    }
    final_vect
}

fn is_possible(input:&String) -> bool {
    let final_vect = parse_input_into_2d_vec(input);
    println!("{:?}", final_vect);
    let bool_vec: Vec<_> = final_vect.into_iter().map(|x| {
        if x[0]> 12 || x[1]> 13 || x[2]>14 {
        false
        }else{
            true
        }
    }).collect();
    println!("{:?}", bool_vec);
    let mut returnabsvalue = true;
    for elem in bool_vec{
        println!("{:?}", elem);
        if !elem{
            returnabsvalue  = false;
            break;
        }
    }
        returnabsvalue
}

fn cube_of_line_possible (input: &String) -> u32 {
    let final_vect = parse_input_into_2d_vec(input);
    let mut max_values: Vec<u32> = Vec::new();

    for col in 0..final_vect[0].len() {

        let max_value = final_vect.iter().map(|row| row[col]).max().unwrap_or(u32::MIN);

        max_values.push(max_value);
    }
    println!("{:?}", max_values);
    max_values[0]*max_values[1]*max_values[2]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    let mut vec  = Vec::new();
    
    
    for (line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            if &args[2] == "1" { 
            
                    if is_possible(&line){
                        vec.push((line_num+1) as u32);
                    }
            
            }else if &args[2] == "2"{
                vec.push(cube_of_line_possible(&line));

            }else{
                println!("USAGE ./target/debug/day2 INTPUT_TEXT PART (as 1 or 2)")
            }
        }
    }
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    println!("{:?}",sum);
}