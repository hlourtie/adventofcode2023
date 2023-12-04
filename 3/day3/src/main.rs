use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;


fn check_up(start: &(usize,usize), finish:&(usize,usize), vec: &Vec<Vec<char>>) -> bool{
    // need to check between [i-1][j-1] et [i-1][j+1]
    let  mut y = if start.1 == 0 {start.1}else { start.1 - 1};
    let finish_ref = if finish.1 == vec[finish.0].len() - 1 { finish.1} else { finish.1 + 1};
    
    while y <= finish_ref {
        if !vec[start.0-1][y].is_digit(10) && vec[start.0 - 1 ][y] !='.' {
            return true;
        }
        y+=1;
    }

    false
}

fn check_down(start: &(usize,usize), finish:&(usize,usize), vec: &Vec<Vec<char>>) -> bool{
    let mut y = if start.1 == 0 {start.1} else { start.1 - 1};
    let finish_ref = if finish.1 == vec[finish.0].len() - 1 {finish.1} else { finish.1 + 1};
    while y <= finish_ref{
        if !vec[start.0+1][y].is_digit(10) && vec[start.0+1][y] != '.'{
            return true;
        }
        y+=1;
    }
    false
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = match File::open(&Path::new(&args[1])){
        Err(why)=> panic!("couldn't open and read the file because {}", why),
        Ok(contents) => contents
    };
    let mut vec : Vec<Vec<char>>  = Vec::new();
    let mut vec_of_parts : Vec<u32> = Vec::new(); 
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            vec.push(line.chars().collect());
        }  
    }
    let mut i:usize = 0;
    while i < vec.len(){
        let mut j : usize = 0;
        while j < vec[i].len() {

            if vec[i][j].is_digit(10){
                let start : (usize,usize) = (i, j);
                while j< vec[i].len() && vec[i][j].is_digit(10){
                    j+=1;
                }
                j-=1;
                let finish : (usize,usize) = (i,j);
                
                
                // get all the not corner cases out of the way

                if i != 0 && i != vec.len() - 1{
                    if (start.1 != 0 && vec[i][start.1 - 1] != '.' )||(finish.1 != vec[i].len() - 1 && vec[i][finish.1 + 1] != '.') || check_up( &start, &finish, &vec) || check_down( &start, &finish, &vec){
                        let mut part_str:String = String::new();
                        let mut vec_of_char : Vec<char> = Vec::new();
                        let mut y : usize = start.1;
                         while y <= finish.1{
                            vec_of_char.push(vec[i][y]);
                            y +=1;
                        }
                        part_str = vec_of_char.iter().collect();
                        print!("part num free form: ");
                        println!("{:?}", part_str);
                        vec_of_parts.push(part_str.parse::<u32>().unwrap_or(u32::MIN));
                    }
                }else if i == 0 &&  check_down(&start, &finish, &vec){
                    let mut part_str:String = String::new();
                        let mut vec_of_char : Vec<char> = Vec::new();
                        let mut y : usize = start.1;
                         while y <= finish.1{
                            vec_of_char.push(vec[i][y]);
                            y +=1;
                        }
                        part_str = vec_of_char.iter().collect();
                        print!("part num first line: ");
                        println!("{:?}", part_str);
                        vec_of_parts.push(part_str.parse::<u32>().unwrap_or(u32::MIN));
                
                }else if i == vec.len()-1 && check_up(&start, &finish, &vec){
                    println!("last line");
                    let mut part_str:String = String::new();
                    let mut vec_of_char : Vec<char> = Vec::new();
                    let mut y : usize = start.1;
                     while y <= finish.1{
                        vec_of_char.push(vec[i][y]);
                        y +=1;
                    }
                    part_str = vec_of_char.iter().collect();
                    print!("part num last line: ");
                    println!("{:?}", part_str);
                    vec_of_parts.push(part_str.parse::<u32>().unwrap_or(u32::MIN));

                }
                // get start and end coordinate of th
                // check parameter arount coordinates
                // add | to split
                // and advance
                
            }
            j += 1;
        }
        i += 1;
    }
    println!("{:?}", vec_of_parts);
   let sum = vec_of_parts.iter().fold(0, |acc, x| acc + x);
   println!("{:?}",sum);
}