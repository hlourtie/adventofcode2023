use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main(){
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&Path::new(&args[1])).unwrap_or_else(|why| panic!("{:?}", why));
    let commands : Vec<Vec<char>> = content.split(',').collect::<Vec<&str>>().iter().map(|x| (*x).to_string().chars().collect::<Vec<char>>()).collect();
    let mut vec_of_vals : Vec<u64> = Vec::new();
    for command in &commands{
        let mut tot: u64 = 0;
        for el in command{
            // let ascii_val =  el as u8;
            tot = ((tot + (*el as u64) ) * 17) % 256;
        }
        vec_of_vals.push(tot);
    }
    
    
    let total = &vec_of_vals.iter().fold(0,|acc, x| acc + x);
   
    println!("total: {:?}", total);
    // part 2 bits start here but we are using part 1 heavily.

    let mut mirror_boxes : HashMap < u64, Vec<(String, u8)>> = HashMap::new();

    for command in commands{
        let mut label = String::new();
        let mut  i = 0; 
        let mut lens_box_number : u64 = 0;
        while command[i] != '=' && command[i] != '-'{
       
            
            lens_box_number = ((lens_box_number + (command[i] as u64) ) * 17) % 256;
            label.push(command[i]);
            i+=1;
        }

        if command[i] == '-'{
            if mirror_boxes.contains_key(&lens_box_number){
                let lens_box = mirror_boxes.get_mut(&lens_box_number).unwrap();

              if let Some((index, _)) = lens_box.iter().enumerate().find(|(_index, &(ref val1, _))| *val1 == label){
                   lens_box.remove(index);
                }
                
            }
        }else{
            let lens_number: u8 = command[i+1] as u8 - '0' as u8;
            if mirror_boxes.contains_key(&lens_box_number){

                let lens_box = mirror_boxes.get_mut(&lens_box_number).unwrap();
                if let Some((index, _)) = lens_box.iter().enumerate().find(|(_index, &(ref val1, _))| *val1 == label){
                    lens_box[index].1 = lens_number;
                }else{
                    lens_box.push((label , lens_number));
                }
            }else{
                let mut vec: Vec<(String, u8)> = Vec::new();
                vec.push((label , lens_number));
                mirror_boxes.insert(lens_box_number, vec);
            }
        }
    }
    let mut tot:u64 = 0;
    for (key, value) in mirror_boxes.iter(){
        
        for i in 0..value.len(){
            
            tot += (*key + 1) * value[i].1 as u64 *(i as u64 + 1);
            
        }

    }
    println!("{:?}", tot);
}