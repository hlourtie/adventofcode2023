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
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_vector(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0; // Or handle the case as needed
    }

    let mut result = numbers[0];

    for &number in &numbers[1..] {
        result = lcm(result, number);
    }

    result
}

fn main(){
    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("Failed to open file: {:?}", why));
    let mut map : HashMap<String, (String, String)> = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut vec_of_node : Vec<(String, String)> = Vec::new();
    for (line_num, line ) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            
            if line_num == 0 {
                instructions = line.chars().collect();
            }else if line_num != 1{
                let (key, values) = get_data(&line);
                map.insert(key.clone(), values.clone());
                if key.ends_with('A') {
                    vec_of_node.push(values);
                }
            }
        }
    }
    let mut i : usize = 0 as usize;
    let len = instructions.len();
    let mut vec_of_steps: Vec<u64> = vec![0; vec_of_node.len()];
    loop{

        vec_of_node = vec_of_node.iter().map(|x| {if instructions[i%len as usize] == 'R' {  map.get(&x.1).unwrap().clone()} else { map.get(&x.0).unwrap().clone() }}).collect();
        i+=1;
        let instruction = instructions[i%len as usize];
        let mut j : usize = 0 as usize;
        while j < vec_of_node.len(){
            let end_in_z = if instruction == 'R'{(vec_of_node[j].1).ends_with('Z')}else{(vec_of_node[j].0).ends_with('Z')};
            if end_in_z && vec_of_steps[j] == 0 {
                vec_of_steps[j] = i as u64 + 1 as u64;
            }
            j+=1;
        }

        let all_z = vec_of_steps.iter().all(|s| *s != 0 as u64);
        
        
        if all_z{
            break;
        }
    }

    println!("vec of steps: {:?}", vec_of_steps);
    let lcm = lcm_of_vector(vec_of_steps);
    println!("number of steps: {:?}", lcm);
}