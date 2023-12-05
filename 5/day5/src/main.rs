use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

fn get_seeds(input: &String) -> Vec<i64> {

    let initial_split : Vec<&str> = input.split(":").collect();
    let result;
    if let Some(second_part) = initial_split.get(1) {
        result = second_part.trim();
    } else {
        println!("No substring found after ':'");
        result="";
    }

    let second_split :Vec<&str> = result.split_whitespace().collect();
    let mut return_vec: Vec<i64> = Vec::new();
    
    for sect in second_split {
        return_vec.push(sect.parse::<i64>().unwrap_or(i64::MIN));
    }
   return_vec
}

fn get_map_data(input:&String) ->  Vec<i64>{
    let init_split :Vec<&str> = input.split_whitespace().collect();
    let mut return_vect : Vec<i64> = Vec::new();
    for data in init_split{
        return_vect.push(data.parse::<i64>().unwrap_or(i64::MIN));
    }
    return_vect
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    let mut seeds: Vec<i64> = Vec::new();
    let mut current_data : String = String::new();
    let mut seed_to_soil_map : Vec<Vec<i64>> = Vec::new();
    let mut soil_to_fertilizer_map : Vec<Vec<i64>> = Vec::new();
    let mut fertilizer_to_water_map : Vec<Vec<i64>> = Vec::new();
    let mut water_to_light_map : Vec<Vec<i64>> = Vec::new();
    let mut light_to_temperature_map : Vec<Vec<i64>> = Vec::new();
    let mut temperature_to_humidity_map : Vec<Vec<i64>> = Vec::new();
    let mut humidity_to_location_map : Vec<Vec<i64>> = Vec::new();

    for (line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            if line_num == 0{
                seeds = get_seeds(&line);
            }else if line.len() != 0 && (line == "seed-to-soil map:" ||line == "soil-to-fertilizer map:" || line == "fertilizer-to-water map:" 
            || line == "water-to-light map:" ||line == "light-to-temperature map:" || line == "temperature-to-humidity map:" ||line == "humidity-to-location map:" ){
                current_data = line;
            }else if line.len() != 0 && current_data == "seed-to-soil map:"{
                seed_to_soil_map.push(get_map_data(&line));
            }else if line.len() != 0 && current_data == "soil-to-fertilizer map:"{
                soil_to_fertilizer_map.push(get_map_data(&line));
            }else if line.len() != 0 && current_data == "fertilizer-to-water map:"{
                fertilizer_to_water_map.push(get_map_data(&line));
            }else if line.len() != 0 && current_data == "water-to-light map:"{
                water_to_light_map.push(get_map_data(&line));
            }else if line.len() != 0 && current_data == "light-to-temperature map:"{
                light_to_temperature_map.push(get_map_data(&line));
            }else if line.len() != 0 && current_data == "temperature-to-humidity map:"{
                temperature_to_humidity_map.push(get_map_data(&line));
            }else if line.len() != 0 && current_data == "humidity-to-location map:"{
                humidity_to_location_map.push(get_map_data(&line));
            }
        }
    }
    // print!("begin of path Seed is : "); 
    let mut lowest : i64 = i64::MAX;
    if args.len() == 3 as usize && &args[2] == "2"{
        let mut range : i64 = 0;

        for elem in &humidity_to_location_map{
            if elem[0] + elem[2] > range{
                range = elem[0] + elem[2];
            }
        }

        for i in 0..range {
            let mut checked_seed = i; 
            for map in  &humidity_to_location_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
            for map in  &temperature_to_humidity_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
            for map in  &light_to_temperature_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
            for map in  &water_to_light_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
            for map in  &fertilizer_to_water_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
            for map in  &soil_to_fertilizer_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
            for map in  &seed_to_soil_map{
                if map[0] <= checked_seed && checked_seed < map[0]+map[2]{
                    if map[1] - map[0] > 0 {
                        checked_seed = checked_seed + map[1] - map[0];
                    }else{
                        checked_seed  = checked_seed + map[1] - map[0];
                    }
                    break;
                }
            }
        let mut j : usize = 0; 
        let mut tobreak:bool = false;
            
                while j < seeds.len()-1{
                        if seeds[j]<=checked_seed  && checked_seed < seeds[j]+ seeds[j+1]{
                            lowest = i;
                            tobreak = true;
                        }
                        j+=2;
                    }
        

            if tobreak {
                break;
            }
        }
    }else{

        for seed in seeds{
        
            let mut checked_seed = seed;
            // println!("{:?}", checked_seed);
            for map in  &seed_to_soil_map{
                if map[1] <= checked_seed && checked_seed < map[1]+map[2]{
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
        
            for map in  &soil_to_fertilizer_map{
                if map[1] <= checked_seed && checked_seed < map[1]+map[2]{
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
        
            for map in  &fertilizer_to_water_map{
                if map[1] <= checked_seed && checked_seed < map[1]+map[2]{
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
            
            for map in  &water_to_light_map{
                
                if map[1] <= checked_seed && checked_seed < (map[1]+ map[2]){
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
        
            for map in  &light_to_temperature_map{
                if map[1] <= checked_seed && checked_seed < map[1]+map[2]{
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
        
            for map in  &temperature_to_humidity_map{
                if map[1] <= checked_seed && checked_seed < map[1]+map[2]{
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
            
            for map in  &humidity_to_location_map{
                if map[1] <= checked_seed && checked_seed < map[1]+map[2]{
                    if map[0] - map[1] > 0 {
                        checked_seed = checked_seed + map[0] - map[1];
                    }else{
                        checked_seed  = checked_seed + map[0] - map[1];
                    }
                    break;
                }
            }
        
            if lowest >  checked_seed{
                lowest = checked_seed;
            }
        
        }
    }

    print!("lowest location ");

    println!("{:?}", lowest);

   
    // let sum = vec.iter().fold(0, |acc, x| acc + x);
    // println!("{:?}",sum);
    
}