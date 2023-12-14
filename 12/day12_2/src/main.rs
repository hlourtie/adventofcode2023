use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

// implement some patern recognition if we have 
//already seen a state we can directly return it no need to go 
//and ty it a bunch of times
fn count_arrangements(input:String) ->usize{
    let input_vec :Vec<&str> = input.split(" ").collect();
    let repeated_string_of_num = (input_vec[1].to_owned() + ",").repeat(5);
    let final_string_of_num= &repeated_string_of_num[..repeated_string_of_num.len() - 1];
    let  group_sizes: Vec<usize> = final_string_of_num.split(",").map(|x| x.parse::<usize>().unwrap_or(usize::MIN)).collect::<Vec<usize>>();
  
    let repeated_string = (input_vec[0].to_owned() + "?").repeat(5);
    let final_string = &repeated_string[..repeated_string.len() - 1];
    let mut string_line : Vec<char> = final_string.chars().collect();
    let mut solution_map : HashMap<(usize,usize,usize), usize> = HashMap::new();
    //trim all useless characters
    while string_line[0] == '.'{
        string_line.remove(0);
    }
    return count_arrangements_helper(&string_line, 0, &group_sizes , 0,0, &mut solution_map)
}
fn count_arrangements_helper(spring_line:&Vec<char>, index:usize, group_sizes: &Vec<usize>, group_index : usize, current_group: usize,solution_map : &mut HashMap<(usize,usize,usize), usize>  ) -> usize{
    
    let patern: (usize,usize,usize) = (index, group_index, current_group);

    if solution_map.contains_key(&patern){
        return *solution_map.get(&patern).unwrap();
    }
    if index == spring_line.len() {
        if current_group == 0  && group_index >= group_sizes.len(){
            return 1
        }else if  group_index == group_sizes.len() - 1 && current_group == group_sizes[group_index]{
            return 1
        }else{
            return 0 
        };
    }
    let mut count = 0;


    for c  in vec!['.','#'].iter(){
        // println!("c is :            {:?}", c);
        // println!("spring[index]:    {:?}", c);
        if spring_line[index] == *c || spring_line[index]=='?'{
            if *c =='.' && current_group == 0{
                count += count_arrangements_helper(spring_line,index+1, group_sizes,group_index,0, solution_map);
            }else if *c == '.' && current_group > 0 && group_index < group_sizes.len() && group_sizes[group_index] == current_group{
                count += count_arrangements_helper(spring_line, index+1, group_sizes, group_index+1, 0, solution_map);
            }else if *c == '#'{
                count += count_arrangements_helper(spring_line, index+1, group_sizes, group_index, current_group+1, solution_map);
            }
        }
    }
    solution_map.insert(patern, count);
    count
}
    


fn main(){
    let args: Vec<String> = env::args().collect();
    let file = File::open(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let mut vec_count : Vec<usize> = Vec::new();
    for (_line_num, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            //println!("---------------------------------------------------");
            vec_count.push(count_arrangements(line));
            //println!("---------------------------------------------------");
        }
    }
   println!("vec count  {:?}", vec_count);
    let tot_amount =  vec_count.iter().fold(0,|acc,x| acc+x);
    println!("total {:?}", tot_amount);
}
