#![allow(warnings)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::io::*;
mod lib;

fn main() {
    
    let input = lib::GoLx::Get_input();
    println!("rows: {}",input.0);
    println!("cols: {}",input.1);
    println!("Geometry: {}",input.2);

    
    let name:String = "input.txt".to_string();
    let  data = lib::GoLx::read_file(name);
    let mut state: Vec<Vec<i32>> = lib::GoLx::Generate_grid(data,100,100);
  
  
    for _ticks in 0..100{ //simulate ticks.
       let mut update_cells:Vec<(usize,usize,i32)> = Vec::new();
       for row in 0..state.len() as i32{
  
          for col in 0..state[0].len() as i32{
              let indx = (row,col);
              
              let mut nbrs = lib::GoLx::Find_neighbours(&state, indx,&input.2);
              
              let alive_count = lib::GoLx::Alive_nbrs(&state, nbrs);
              
              lib::GoLx::Update_rule(&mut state, alive_count, indx, &mut update_cells);
              
              
              }
       }
       //update cell values.
       for i in 0..update_cells.len(){
        state[update_cells[i].0][update_cells[i].1] = update_cells[i].2;
     }
  
    }
  
    //write to file.
    lib::GoLx::output(&state);
  
  
    
  }





