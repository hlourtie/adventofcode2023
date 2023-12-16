use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;

fn push_north(plan: &mut Vec<Vec<char>>){

    let mut i :usize = 1;
    
    let col = plan.len();

    while i < col{
        let mut j :usize = 0;
        while j < plan[0].len(){
            
            if plan[i][j] == 'O' {
                let mut h = 1;
                
                while plan[i - h][j] == '.'{
                    plan[i + 1 - h][j] = '.';
                    h+=1;
                    if (i as i32) - (h as i32) < 0{
                        break;
                    }
                }
                plan[i + 1 - h][j] = 'O';
            }
            j+=1;
        }
        i+=1;
    }
}
fn push_south(plan: &mut Vec<Vec<char>>){

    
    let mut i = plan.len() - 2;

    loop {
        let mut j :usize = 0;
        while j < plan[0].len(){
            
            if plan[i][j] == 'O' {
                let mut h = 1;
                
                while plan[i + h][j] == '.'{
                    plan[i  + h - 1][j] = '.';
                    h+=1;
                    if i  + h  ==  plan.len() {
                        break;
                    }
                }
                plan[i  + h - 1][j] = 'O';
            }
            j+=1;
        }
        if i == 0 {
            break;
        }
        i-=1;
    }
}

fn push_west(plan: &mut Vec<Vec<char>>){

    let mut j :usize = 1;
    
    let col = plan[0].len();

    while j < col{
        let mut i :usize = 0;
        while i < plan.len(){
            
            if plan[i][j] == 'O' {
                let mut h = 1;
                
                while plan[i ][j- h] == '.'{
                    plan[i][j + 1 - h] = '.';
                    h+=1;
                    if (j as i32) - (h as i32) < 0{
                        break;
                    }
                }
                plan[i][j + 1 - h] = 'O';
            }
            i+=1;
        }
        j+=1;
    }

}

fn push_east(plan: &mut Vec<Vec<char>>){
  
    
    let mut j = plan[0].len() -2;

    loop{
        let mut i :usize = 0;
        while i < plan.len(){
            
            if plan[i][j] == 'O' {
                let mut h = 1;
                
                while plan[i ][j + h] == '.'{
                    plan[i][j + h - 1] = '.';
                    h+=1;
                    if j + h == plan[0].len(){
                        break;
                    }
                }
                plan[i][j + h - 1] = 'O';
            }
            i+=1;
        }
        if j == 0{
            break;
        }
        j-=1;
    }

}


fn main(){
    let args : Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut plan : Vec<Vec<char>> = Vec::new();
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            plan.push(line.chars().collect::<Vec<char>>());
        }
    }
    let mut cycle_count:usize = 0;
    let mut vec_of_pos : Vec<Vec<Vec<char>>> = Vec::new();
   
    loop
        {
        
            push_north(&mut plan);
            push_west(&mut plan);
            push_south(&mut plan);
            push_east(&mut plan);
            
            
           
            if !vec_of_pos.contains(&plan) {
                vec_of_pos.push(plan.clone());
            }else{
                break;
            }
            cycle_count += 1;
            
            
        }
    println!("cycle_count {:?}", cycle_count);
    println!("end plan");
    println!("\n\n");
    
// for row in plan{
//     println!("{:?}", row);
// }
// println!("\n\n");
// for planned in &vec_of_pos{
//     for row in planned{
//         println!("{:?}", row);

//     }
//     println!("\n\n");
// }
let start_of_cycle = vec_of_pos.iter().position(|x| x == &plan).unwrap_or(0);
println!("start of cycle: {:?}", start_of_cycle);
    let point = (1000000000 - start_of_cycle ) % (cycle_count- start_of_cycle );
    println!("point {:?}", point);
    let plan_to_weight: &Vec<Vec<char>> = &vec_of_pos[point + start_of_cycle-1];
     let mut vec_of_weight : Vec <usize> = Vec::new();
     let max_weight =  plan_to_weight.len();
     for (i, row) in plan_to_weight.iter().enumerate(){
       let count = row.iter().filter(|&&x| x == 'O').count();
       vec_of_weight.push((max_weight - i) * count as usize);
    }
    let tot = vec_of_weight.iter().fold(0, |acc, x| acc+x);
    println!("tot {:?}", tot);
}