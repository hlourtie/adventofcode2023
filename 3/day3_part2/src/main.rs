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
        if vec[start.0-1][y] =='*' {
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
        if vec[start.0+1][y] == '*' {
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
    let mut vec_of_parts : Vec<(u32,(usize,usize),(usize,usize))> = Vec::new(); 
    
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line{
            vec.push(line.chars().collect());
        }  
    }
    let mut vec_of_stars :Vec<(usize,usize)> = Vec::new();
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
                        vec_of_parts.push((part_str.parse::<u32>().unwrap_or(u32::MIN),start,finish));
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
                     
                        vec_of_parts.push((part_str.parse::<u32>().unwrap_or(u32::MIN),start,finish));
                
                }else if i == vec.len()-1 && check_up(&start, &finish, &vec){
                    let mut part_str:String = String::new();
                    let mut vec_of_char : Vec<char> = Vec::new();
                    let mut y : usize = start.1;
                     while y <= finish.1{
                        vec_of_char.push(vec[i][y]);
                        y +=1;
                    }
                    part_str = vec_of_char.iter().collect();
                    vec_of_parts.push((part_str.parse::<u32>().unwrap_or(u32::MIN),start,finish));

                }
            }else if vec[i][j] == '*'{
                vec_of_stars.push((i,j));
            } 
            j += 1;
        }
        i += 1;
    }
    // println!("{:?}", vec_of_parts);
    // println!("{:?}", vec_of_stars);
    let mut gear_vec :Vec<u32> = Vec::new();
    for star in vec_of_stars{
        // print!("star: ");
        // println!("{:?}", star);
        let x_coordinate: Vec<usize> = if star.0 != 0 && star.0 != vec.len() -1{[star.0-1,star.0, star.0+1].to_vec()} else if star.0 == 0 {[star.0, star.0+1].to_vec()}else{[star.0-1,star.0].to_vec()};
        let y_coordinate : Vec<usize>= if star.1 != 0 && star.1 != vec[star.0].len(){[star.1-1,star.1,star.1+1].to_vec()} else if star.1 == 0{[star.1, star.1+1].to_vec()}else{[star.1-1, star.1].to_vec()};
        let mut vec_potential_gear : Vec<u32> = Vec::new();
        // print!{"xcoordinates: "};
        // println!("{:?}", x_coordinate);
        // print!{"ycoordinates: "};
        // println!("{:?}", y_coordinate);

        for part in &vec_of_parts{
            // print!{"part"};
            // println!{"{:?}", part};
            
            if x_coordinate.contains(&part.1.0){
               // println!{"x coordinate valid checking y coordinate"};
                let vec_of_y_coordinates = vec![part.1.1 .. part.2.1+1];
             
                let flattened: Vec<usize> = vec_of_y_coordinates.iter().flat_map(|range| range.clone()).collect();
                // print!("Flattened : ");
                // println!("{:?}", flattened);

                let contains_any = flattened.iter().any(|&x| y_coordinate.contains(&x));
                if contains_any{
                    vec_potential_gear.push(part.0);
                }
            }
        }
        // print!("Potential gears: ");
        // println!("{:?}", vec_potential_gear);
        if vec_potential_gear.len() == 2 {
            gear_vec.push(vec_potential_gear[0]*vec_potential_gear[1]);
        }
    }
   let sum = gear_vec.iter().fold(0, |acc, x| acc + x);
//    println!("{:?}",sum);
}