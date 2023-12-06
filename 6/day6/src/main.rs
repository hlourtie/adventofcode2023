use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

fn get_data_one_num(input:&String) -> f64 {
    let first_split : Vec<&str> = input.split(":").collect();
    let result;
    if let Some(second_part) = first_split.get(1) {
        result = second_part.trim();
    } else {
        println!("No substring found after ':'");
        result="";
    }
    let second_split : String = result.split_whitespace().collect();
   
    let return_val = second_split.parse::<f64>().unwrap_or(f64::MIN);
    return_val
}



fn get_data(input:&String) -> Vec<u32>{

    let first_split : Vec<&str> = input.split(":").collect();
    let result;
    if let Some(second_part) = first_split.get(1) {
        result = second_part.trim();
    } else {
        println!("No substring found after ':'");
        result="";
    }
    let second_split :Vec<&str> = result.split_whitespace().collect();
    let mut return_vec: Vec<u32> = Vec::new();
    
    for sect in second_split {
        return_vec.push(sect.parse::<u32>().unwrap_or(u32::MIN));
    }
   return_vec
}

fn get_number_of_possibilities(time:u32, distance:u32) -> u32 {
    let mut count:u32 = 0;
    for x in 0..=time{
        let res = time * x - x*x;
        if res > distance{
            count +=1;
        }
    }
    count

}

fn main(){

    let args: Vec<String> = env::args().collect();

    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    if args.len() > 2 && &args[2] == "1"{
        let mut vec_time : Vec<u32>  = Vec::new();
        let mut vec_dist : Vec<u32> = Vec::new();
        for (line_num, line) in BufReader::new(file).lines().enumerate(){
            if let Ok(line) = line{
                if line_num == 0{
                    vec_time = get_data(&line);
                }else if line_num == 1{
                    vec_dist = get_data(&line);
                }
            }
        }
        let mut vec_solutions : Vec<u32> = Vec::new();
        let mut i: usize = 0;

        while i < vec_time.len(){
            vec_solutions.push(get_number_of_possibilities(vec_time[i], vec_dist[i]));
            i+=1;
        }
        println!("{:?}", vec_solutions);
        let mul = vec_solutions.iter().fold(1, |acc, x| acc * x);
        println!("the multiplier {}", mul);
    }else{
        // as we are dealing with large numbers in part 2 I decided to go the mathematical way more than the brute force but both are very possible.
        let mut time : f64 = 0.0;
        let mut dist: f64 = 0.0;
        for (line_num, line) in BufReader::new(file).lines().enumerate(){
            if let Ok(line) = line{
                if line_num == 0{
                    time = get_data_one_num(&line);
                }else if line_num == 1{
                    dist = get_data_one_num(&line);
                }
            }
        }
        
        /*  using the math to solve second degree equation using discriminant
        **  -a x^2 +time x - distance
        **  solving for x gives us two boundaries and we just calculate the diff inbetween.
        */
        
        let a : f64 = -1.0;
     
        let discriminant = time * time - 4.0 * a * (-dist);
    
        if discriminant >= 0.0 {
            
            let root1 = ( - time  + discriminant.sqrt() )/ ( 2.0 * a );
            let root2 = (- time  - discriminant.sqrt()) / (2.0 * a);

            let lower_bound = root1.min(root2);
            let upper_bound = root1.max(root2);

            let result = (upper_bound - lower_bound ) as u64;
            println!("final count: {}", result);
        }
    }


}