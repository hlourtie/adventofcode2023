use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

fn sort_data (input:String) -> Vec<char>{

    let split : Vec<char> = input.chars().collect();
    split
}

fn get_data (input: &String) -> (Vec<char>,u32){
        
    let first_split : Vec<&str> = input.split_whitespace().collect();
    let insert_vec : Vec<char> = sort_data(first_split[0].to_string());
    let val : u32 = first_split[1].parse::<u32>().unwrap_or(u32::MIN);
    (insert_vec, val)

}

pub fn quicksort(arr: &mut Vec<(Vec<char>, u32)>) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort(arr: &mut Vec<(Vec<char>, u32)>, left: isize, right: isize) {
    println!("{:?}", arr);
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1);
        _quicksort(arr, partition_idx + 1, right);
    }
}

fn count_unique_items(vec: &Vec<char>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for &item in vec {
        let entry = counts.entry(item).or_insert(0);
        *entry += 1;
    }
    counts
}



fn is_smaller (vec1 : &Vec<char>,  vec2: &Vec<char>) -> bool{

    let mut return_val : bool = true;
    println!("vec1 {:?}", vec1);
    println!("vec2 {:?}", vec2);
    let mut count_hash1 : HashMap<char, usize> = count_unique_items(vec1);
    let mut count_hash2 : HashMap<char, usize> = count_unique_items(vec2);
    let len1 = if count_hash1.contains_key(&'J')  && count_hash1.len()-1 > 0  { count_hash1.len() - 1 } else{ count_hash1.len()};
    let len2 = if count_hash2.contains_key(&'J')  && count_hash2.len()-1 > 0   { count_hash2.len() - 1 } else { count_hash2.len()};
    println!("len1 {:?}", len1);
    println!("len2 {:?}", len2);
    if len1 < len2{
        return_val = false;
    }else if len1 > len2 {
        return_val = true;
    }else {

        let mut  max_value_in_hash_1 = 0;
        let mut  max_value_in_hash_2 = 0;
        match count_hash1.get(&'J') {
            Some(value) => {
                let max_entry1 = count_hash1.iter().max_by_key(|&(_, count)| count);
                max_value_in_hash_1 =  match max_entry1 {
                        Some((character, count)) => {
                        if *character == 'J' && *count != 5 as usize  && *count != 1 as usize{
                            let this_count = *count;
                            count_hash1.remove(&'J');
                            println!("popped count_hash1 {:?}", count_hash1);
                            let second_max_entry = count_hash1.values().cloned().max().unwrap();
                            second_max_entry + this_count
                            //     .iter()
                            //     .filter(|&(_, &count)| count != *count_hash1.values().max().unwrap() || count == *count_hash1.values().max().unwrap())
                            //     .max_by_key(|&(_, &count)| count);
                            // println!("COUNT {:?}", count);
                            // println!("secondmax entry {:?}", second_max_entry.unwrap().1);
                            // second_max_entry.unwrap().1 + count
                        }else if *count != 5 as usize{
                            count_hash1.values().cloned().max().unwrap() + count_hash1.get(&'J').unwrap()
                        }else{
                            5
                        }
                        }
                        None => {
                            0
                        }
                    };
                
            
                
                }
                None => {
                    max_value_in_hash_1 = count_hash1.values().cloned().max().unwrap();
                }
        }
        match count_hash2.get(&'J') {
            Some(value) => {
            let max_entry2 = count_hash2.iter().max_by_key(|&(_, count)| count);
            max_value_in_hash_2 = match max_entry2 {
                Some((character, count)) => {
                    println!("Character vec 2 {:?}", *character);
                    println!("COUNT vec 2.1 {:?}", count);
                    if *character == 'J' && *count != 5 as usize  && *count != 1 as usize{
                        let this_count = *count;
                        count_hash2.remove(&'J');
                        println!("popped count_hash2 {:?}", count_hash2);
                        let second_max_entry = count_hash2.values().cloned().max().unwrap();
                        second_max_entry + this_count
                        // let second_max_entry = count_hash2
                        // .iter()
                        // .filter(|&(_, &count)| count!= *count_hash2.values().max().unwrap() || count == *count_hash2.values().max().unwrap())
                        // .max_by_key(|&(_, &count)| count);
                        // println!("COUNT vec 2{:?}", count);
                        // println!("secondmax entry vec 2 {:?}", second_max_entry);
                        // second_max_entry.unwrap().1 + count
                        
                    }else if *count != 5 as usize {
                        count_hash2.values().cloned().max().unwrap() + count_hash2.get(&'J').unwrap()
                    }else{
                        5
                    }
                }
                None => {

                    0
                }
                }
            }
            None => {
                max_value_in_hash_2 = count_hash2.values().cloned().max().unwrap();
            }
    }
            
       
       
            println!("max_value_in_hash_1 {:?}", max_value_in_hash_1);
            println!("max_value_in_hash_2 {:?}", max_value_in_hash_2);
        if max_value_in_hash_1 > max_value_in_hash_2{
            return_val = false;
        }else if max_value_in_hash_1 < max_value_in_hash_2 {
            return_val = true;

        }else{
            

            let mut obj: HashMap<char, u8> = HashMap::new();

            obj.insert('T', 10);
            obj.insert('J', 0);
            obj.insert('Q', 12);
            obj.insert('K', 13);
            obj.insert('A', 14);
           

            for (e1, e2) in vec1.iter().zip(vec2.iter()){
                let mut element1:u8 = 0;
                let mut element2:u8 = 0;
                if e1.is_digit(10){
                    element1 = e1.to_digit(10).unwrap() as u8;
                }else{
                    element1 = *obj.get(e1).unwrap();
                }

                if e2.is_digit(10){
                    element2 = e2.to_digit(10).unwrap() as u8;
                }else{
                    element2 = *obj.get(e2).unwrap();
                }
                
                println!("ele1 {:?}", element1);
                println!("ele2 {:?}", element2);

                if element1 > element2{
                    return_val = false;
                    break;
                }else if element1 < element2{
                    return_val = true;
                    break;
                }
            }
        }
    }
    return_val
}

fn partition(arr: &mut Vec<(Vec<char>, u32)>, left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {

        if is_smaller( &arr[j as usize].0 , &arr[pivot as usize].0 ){

        // if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };

    let mut sorted_vec : Vec<(Vec<char>,u32)> = Vec::new();

    for (_line_num, line) in BufReader::new(file).lines().enumerate(){

        if let Ok(line) = line {
            sorted_vec.push(get_data(&line));
        }
    }

    println!("{:?}", sorted_vec);
    quicksort(&mut sorted_vec);
    println!("{:?}", sorted_vec);
    let result: u32 = sorted_vec
        .iter()
        .enumerate()
        .map(|(index, (_chars, value))| (*value * (index as u32 + 1)))
        .sum();
    println!("{:?}", result);

}

