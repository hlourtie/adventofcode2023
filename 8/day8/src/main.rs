use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

fn get_data(input:&String) -> (String, (String, String)) {

    let parts: Vec<&str> = input.split("=").collect();
    let key = parts[0].trim();
    let values = parts[1]
    .trim_matches(|c| c == '(' || c == ')' || c == ' ')
    .split(',')
    .map(|s| s.trim())
    .collect::<Vec<&str>>();
    (key.to_string(),(values[0].to_string(), values[1].to_string()))
}

fn main(){
    let args: Vec<String> = env::args().collect();
    
    let file = match File::open(&Path::new(&args[1])){

        Err(why)=> panic!("the {:?}", why),
        Ok(contents) => contents
    };
    let mut map : HashMap<String, (String, String)> = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut node : (String,String) = ("".to_string(),"".to_string());
    let mut vec_of_node : Vec<(String, String)> = Vec::new();
    for (line_num, line ) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            
            if line_num == 0 {
                instructions = line.chars().collect();
            }else if line_num != 1{
                let (key, values) = get_data(&line);
                map.insert(key, (values.0, values.1));
                
                
            }
        }
    }
    let mut i : usize = 0 as usize;
    let len = instructions.len();
    node = map.get("AAA").unwrap().clone();
     println!("node at start: {:?}", node);
    loop{
        // println!("instruction: {:?}", instructions[i%len as usize] );
         println!("node: {:?}", node);
        node = if instructions[i%len as usize] == 'R'{  map.get(&node.1).unwrap().clone() }else{ map.get(&node.0).unwrap().clone() };
        i+=1;
        let right = node.1.clone();
        let left = node.0.clone();
        
        if (right.to_string() == "ZZZ" && instructions[i%len as usize] == 'R' )|| (instructions[i%len as usize] == 'L' && left.to_string() =="ZZZ"){
            i+=1;
            break;
        }
    
    }

    println!("number of steps: {:?}", i);
}