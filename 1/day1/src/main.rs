use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use regex::Regex;


pub fn get_number_part1(input:&String) -> u32 {
    let pat = r"(\d)";

    let regex = Regex::new(pat).unwrap();
    let first_digit: u32 ;
    if let Some(first_match) = regex.find(input) {
        first_digit = first_match.as_str().chars().next().unwrap().to_digit(10).unwrap();
    }else{
        first_digit = 0;
    }

    let last_digit = regex.find_iter(input).last().expect("REASON for it to fail").as_str().chars().next().unwrap().to_digit(10).unwrap();
    (first_digit * 10 )+ last_digit
}

pub fn get_number_part2(input:&String)-> u32{
    let pat1 = r"(?:one|two|three|four|five|six|seven|eight|nine|\d)";
    let pat2 =  r"(?:eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)";
    let mut obj: HashMap<&str, u32> = HashMap::new();

    obj.insert("one", 1);
    obj.insert("two", 2);
    obj.insert("three", 3);
    obj.insert("four", 4);
    obj.insert("five", 5);
    obj.insert("six", 6);
    obj.insert("seven", 7);
    obj.insert("eight", 8);
    obj.insert("nine", 9);

    let mut obj2 : HashMap<&str, u32>  = HashMap::new();
    obj2.insert("eno", 1);
    obj2.insert("owt", 2);
    obj2.insert("eerht", 3);
    obj2.insert("ruof", 4);
    obj2.insert("evif", 5);
    obj2.insert("xis", 6);
    obj2.insert("neves", 7);
    obj2.insert("thgie", 8);
    obj2.insert("enin", 9);

    let regex = Regex::new(pat1).unwrap();
    let regex2 = Regex::new(pat2).unwrap();
    let first_digit;
    if let Some(first_match) = regex.find(input) {
        first_digit = first_match.as_str();
    }else{
        first_digit = " ";
    }
    let first_digit_as_a_num : u32;
    if first_digit.chars().next().unwrap().is_digit(10) {
        first_digit_as_a_num = first_digit.chars().next().unwrap().to_digit(10).unwrap();
    }else{
        let intermed =obj.get(first_digit).unwrap();
        first_digit_as_a_num =  *intermed as u32;
    }
   
    let reversed:String = input.chars().rev().collect();
    let last_digit;
    if let Some(last_match) = regex2.find(reversed.as_str()) {
        last_digit = last_match.as_str();
    }else{
        last_digit = "";
    }

    let last_digit_as_a_num : u32;
    
    if last_digit.chars().next().unwrap().is_digit(10) { 
        last_digit_as_a_num = last_digit.chars().next().unwrap().to_digit(10).unwrap()
    }else{
        let intermed = obj2.get(last_digit).unwrap();
        last_digit_as_a_num = *intermed as u32;
    }
    (first_digit_as_a_num * 10 )+ last_digit_as_a_num
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    let mut vec = Vec::new();
    let part_select = &args[2];
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){

        if let Ok(line) = line{
            if part_select == "1"{ 
                vec.push(get_number_part1(&line));
            }else if part_select == "2"{
                vec.push(get_number_part2(&line));
            }
        }
    }
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    println!("{:?}",sum);
}