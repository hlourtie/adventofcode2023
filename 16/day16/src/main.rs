use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;

fn shine(plan: Vec<Vec<char>>, start_position:((i16,i16),(i16,i16)) ) -> usize{

    let mut vec_visited : Vec<Vec<bool>> = vec![vec![false;plan[0].len()];plan.len()];
    let mut vec_of_beams : Vec<((i16,i16), (i16,i16))> = Vec::new();
    vec_of_beams.push(start_position);
    let mut vec_of_visited_coordinates : Vec<((i16,i16), (i16,i16))> = Vec::new();
    let mut laps = 0;
    while vec_of_beams.len() > 0 && laps < 10000 {
        let beamslenght = vec_of_beams.len();
        let mut index_to_remove : Vec<usize> = Vec::new();
        for index in 0..beamslenght{
            let beam = vec_of_beams[index];
            let position = beam.0;
            let direction = beam.1;
            let mut new_direction = (0,0);
            //println!("position {:?} direction:{:?}",position, direction);
            match plan[position.0 as usize][position.1 as usize]{
                '/'=> {
                    new_direction = ( -1 * direction.1, -1 * direction.0)
                },
                '\\' =>{ 
                    
                    new_direction = (direction.1, direction.0);
                },
                '|' =>{
                    
                    if direction.1 != 0 {
                        new_direction = (direction.1, direction.0);
                        let other_direction = (-1* direction.1, direction.0);
                        if position.0 + other_direction.0 > -1 && position.0 + other_direction.0 < plan.len() as i16 &&
                            position.1 + other_direction.1 > -1 && position.1 + other_direction.1 < plan[0].len() as i16{
                                vec_of_beams.push(((position.0 + other_direction.0 , position.1 + other_direction.1), other_direction));
                            }
                      
                    }else{
                        new_direction = (direction.0,direction.1);
                    }
                    
                },
                '-' =>{
                    if direction.0 != 0{
                        new_direction = (direction.1, direction.0);
                        let other_direction = (direction.1, -1 * direction.0);
                        if position.0 + other_direction.0 > -1 && position.0 + other_direction.0 < plan.len() as i16 &&
                            position.1 + other_direction.1 > -1 && position.1 + other_direction.1 < plan[0].len() as i16{
                                vec_of_beams.push(((position.0 + other_direction.0 , position.1 + other_direction.1), other_direction));
                            }
                    }else{
                        new_direction = (direction.0,direction.1);
                    }
                },
                '.' =>{
                    new_direction = (direction.0,direction.1);
                },
                _=>panic!("something went wrong")
            }
            vec_visited[position.0 as usize][position.1 as usize] = true;
            if position.0 + new_direction.0 > -1 && position.0 + new_direction.0 < plan.len() as i16 &&
                position.1 + new_direction.1 > -1 && position.1 + new_direction.1 < plan[0].len() as i16 && !vec_of_beams.contains(&((position.0 + new_direction.0 , position.1 + new_direction.1),new_direction)) && !vec_of_visited_coordinates.contains(&((position.0 + new_direction.0 , position.1 + new_direction.1),new_direction)){
                    vec_of_visited_coordinates.push(((position.0 + new_direction.0 , position.1 + new_direction.1),new_direction));
                vec_of_beams[index] = ((position.0 + new_direction.0 , position.1 + new_direction.1),new_direction);
                            }else{
                                index_to_remove.push(index);
                            }     
        }

        index_to_remove.reverse();
        //println!("{:?}", index_to_remove);
        for index in index_to_remove{
            vec_of_beams.remove(index);
        }
        //println!("{:?}", vec_of_beams); 
        laps+=1;
    } 
    let true_count = vec_visited.iter()
                           .flat_map(|row| row.iter())  // Flatten the 2D Vec into a single iterator
                           .filter(|&&value| value)     // Filter to keep only 'true' values
                           .count(); 
     true_count as usize         
}


fn main(){
   // env::set_var("RUST_BACKTRACE", "FULL");
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut plan : Vec<Vec<char>> = Vec::new();
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            plan.push(line.chars().collect());
        }
    }
    //part 1

    println!("Energized : {:?}",shine(plan.clone(), ((0 ,0),(0,1))));

    //part 2
    let mut highest = 0;
   
    for y in 0..plan.len(){
        let mut temp = shine(plan.clone(), ((y as i16 ,0),(0,1)));
        if temp > highest{
            highest = temp;
        }
        temp = shine(plan.clone(),  ((y as i16 ,(plan[0].len() - 1) as i16),(0,-1)));
        if temp > highest{
            highest = temp;
        }
    }

    for x in 0..plan[0].len()
    {
        let mut temp = shine(plan.clone(), ((0 ,x as i16),(1,0)));
        if temp > highest{
            highest = temp;
        }
        temp = shine(plan.clone(),  ((plan.len()as i16 - 1 as i16,x as i16),(-1,0)));
        if temp > highest{
            highest = temp;
        }
    }
    println!("highest: {:?}", highest);
}
