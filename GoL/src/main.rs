#![allow(warnings)]


use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod lib;




fn main() {
    let name:String = "input.txt".to_string();
    let  data = lib::GoL::read_file(name);
    let mut state: Vec<Vec<i32>> = lib::GoL::Generate_grid(data,100,100);
  
  
    for _ticks in 0..100{ //simulate ticks.
       let mut update_cells:Vec<(usize,usize,i32)> = Vec::new();
       for row in 0..state.len() as i32{
  
          for col in 0..state[0].len() as i32{
              let indx = (row,col);
              
              let mut nbrs = lib::GoL::Find_neighbours(&state, indx);
              
              let alive_count = lib::GoL::Alive_nbrs(&state, nbrs);
              
              lib::GoL::Update_rule(&mut state, alive_count, indx, &mut update_cells);
              
              
              }
       }
       //update cell values.
       for i in 0..update_cells.len(){
        state[update_cells[i].0][update_cells[i].1] = update_cells[i].2;
     }
  
    }
  
    //write to file.
    lib::GoL::output(&state);
    println!("{}", "Execution Sucessfull, please check output.txt.");
  
  
    
  }

