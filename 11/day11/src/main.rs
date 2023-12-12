use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;

fn count_in_interval(values: &Vec<i64>, x: i64, y: i64) -> i64 {
    values.iter()
          .filter(|&&value| value >= x && value <= y)
          .count() as i64
}

fn insert_column(galaxy: & Vec<Vec<char>> , vec_col: &mut Vec<i64> ) {
   

    let num_rows = galaxy.len();
    let num_cols = galaxy[0].len();
   

    for col in 0..num_cols {
        if (0..num_rows).all(|row| galaxy[row][col] == '.') {
            vec_col.push(col as i64);
        }
    }
}

fn insert_rows( galaxy : &Vec<Vec<char>>,  vec_rows: &mut Vec<i64> ) {
    
    for (i, row) in galaxy.iter().enumerate() {
        if row.iter().all(|&x| x == '.') {
            vec_rows.push(i as i64);
        }
    }

   

}

fn collect_coordinates(vec_of_planet: &mut Vec<(i64, i64)>, galaxy: Vec<Vec<char>>){

    for (i, row) in galaxy.iter().enumerate(){
        for (j,c) in row.iter().enumerate(){
            if *c == '#'{
                vec_of_planet.push((i as i64, j as i64));
            }
        }
    }

}


fn main(){
    let args : Vec<String> = env::args().collect();

    let file  = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let skip_size : i64 = if args.len() == 3 { args[2].parse::<i64>().unwrap_or(i64::MIN)}else{2};

    let mut galaxy : Vec<Vec<char>> = Vec::new();
    let mut vec_row : Vec<i64> = Vec::new();
    let mut vec_col : Vec<i64> = Vec::new();
    let mut vec_of_planet: Vec<(i64,i64)> = Vec::new();
    let mut cumul :i64 = 0;
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){

        if let Ok(line) = line {
            
            galaxy.push(line.chars().collect::<Vec<char>>());
        }
    }
    // for row in &galaxy {
    //     println!("{}", row.iter().collect::<String>());
    // }
    // println!("\n");

    // getting all the data
    insert_rows( &galaxy, &mut vec_row);
    
    insert_column(&galaxy,&mut vec_col);
    
    collect_coordinates(&mut vec_of_planet, galaxy);
    

    // for row in &galaxy {
    //     println!("{}", row.iter().collect::<String>());
    // }
    

    println!("{:?}", vec_of_planet);
    
    for (i,pair) in vec_of_planet.iter().enumerate(){
        let mut j = i+1;
       // println!("pair {:?}", pair);
        while j < vec_of_planet.len(){
            // println!("pair {:?}", vec_of_planet[j]);
            let x = if vec_of_planet[j].0 > pair.0 {pair.0}else{vec_of_planet[j].0 };
            let y = if vec_of_planet[j].0 <= pair.0 {pair.0}else{vec_of_planet[j].0 };
            let a = if vec_of_planet[j].1 > pair.1 { pair.1} else{vec_of_planet[j].1};
            let b = if vec_of_planet[j].1 <= pair.1 { pair.1} else{vec_of_planet[j].1};
            let number_of_empty_rows = count_in_interval(&vec_row,x,y);
            let number_of_empty_col = count_in_interval(&vec_col,a,b);
            cumul += y-x +  (number_of_empty_rows * (skip_size - 1) )+ b-a +(number_of_empty_col * (skip_size - 1));
            j+=1;
        }
    }
    println!("cumul {:?}", cumul);
    
}