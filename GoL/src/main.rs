use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let  data = read_file();
    let mut state: Vec<Vec<i32>> = Generate_grid(data,100,100);
  
  
    for _ticks in 0..10{
       let mut update_cells:Vec<(usize,usize,i32)> = Vec::new();
       for row in 0..state.len() as i32{
  
          for col in 0..state[0].len() as i32{
              let indx = (row,col);
              
              let mut nbrs = Find_neighbours(&state, indx);
              
              let alive_count = Alive_nbrs(&state, nbrs);
              
              Update_rule(&mut state, alive_count, indx, &mut update_cells);
              
              
              }
       }
       for i in 0..update_cells.len(){
        state[update_cells[i].0][update_cells[i].1] = update_cells[i].2;
     }
  
    }
  
    //write to file.
    output(&state);
  
  
    
  }

//reads an input file and returns a string of the initial state
fn read_file() -> String {

    let mut file = File::open("input.txt")
    .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
    .expect("Error while reading file");
    data
    
    }


//returns a nested vector 2D that containing the initial state.
fn Generate_grid(data:String,rows:i32,cols:i32) -> Vec<Vec<i32>>{
    let mut str:Vec<char> = Vec::new();
    
    for i in data.chars(){
        
        str.push(i)

    }
    
   let mut state: Vec<Vec<i32>> = Vec::new();
   let mut index = 0;
   for _i in 0..rows {
      let mut Rvec: Vec<i32> = Vec::new();
      for _j in 0..cols{
         
         if str[index] == '*'{
                 
                 Rvec.push(1);
             
                 }
     
         else {
             Rvec.push(0)
                         };
         index = index + 1;

      }
      state.push(Rvec);

};

   state

}


fn Find_neighbours(state: &Vec<Vec<i32>>,index:(i32,i32)) -> Vec<(i32,i32)> {
    let row_len = state.len() as i32;
    let col_len = state[0].len() as i32;
    let mut Index_nbr:Vec<(i32,i32)> = Vec::new();
    let mut row = index.0;
    let mut col = index.1;
    
    //right cell
    if col + 1 < col_len {
       let indx = (row, col + 1);
       Index_nbr.push(indx);
 
    }
    //left cell
     if col - 1 >= 0 {
     let indx = (row, col - 1);
     Index_nbr.push(indx);
 
    }
    //bottom cell
     if row - 1 >= 0 {
     let indx = (row - 1, col);
     Index_nbr.push(indx);}
    
    //Top cell
     if row + 1 < row_len {
     let indx = (row + 1, col);
     Index_nbr.push(indx);}
 
 
    //Top right
     if row + 1 < row_len && col + 1 < col_len {
     let indx = (row + 1, col + 1);
     Index_nbr.push(indx);}
 
 
    //Top Left
     if row + 1 < row_len && col - 1 >= 0 {
     let indx = (row + 1, col - 1);
     Index_nbr.push(indx);}
 
    //bottom right
     if row - 1 >= 0 && col + 1 < col_len{
     let indx = (row - 1, col + 1);
     Index_nbr.push(indx);}
 
    //bottom left
     if row - 1 >= 0 && col - 1 >= 0 {
     let indx = (row - 1, col - 1);
     Index_nbr.push(indx);}
 
 
 Index_nbr
    
 
}

//returns number of active neighbours.
fn Alive_nbrs (state: &Vec<Vec<i32>>,nbrs:Vec<(i32,i32)>) -> i32 {
    let mut count = 0;
    for indx in nbrs{
       let x = indx.0 as usize;
       let y = indx.1 as usize;
       if state[x][y] == 1{
         count = count + 1
       }
 
    }
    count
 
 
 }


 fn Update_rule(state: &mut Vec<Vec<i32>>, alive:i32, index:(i32,i32), update_indx: &mut Vec<(usize,usize,i32)>){

     let x = index.0 as usize;
     let y = index.1 as usize;
     
    if state[x][y] == 1{
 
         if alive < 2 || alive > 3{
            update_indx.push((x,y,0));
            
         }
 
         }
 
    if state[x][y] == 0{
 
         if alive == 3{
          update_indx.push((x,y,1));
          
 
         }
 
 
     }
    
 
    
 
 }



 fn output(state: &Vec<Vec<i32>>){

    let mut state_str = String::new();
   
    for r in 0..state.len(){
 
       for c in 0..state[0].len(){
 
          if &state[r][c].to_string() == "1"{
             state_str.push_str("*");
 
          }
          else{
             state_str.push_str(" ");
          }
           
           
           if c == state[0].len()-1 {
             state_str.push_str("\n");
           }
 
       }
    }
 
     let mut f = File::create("./output.txt").expect("Unable to create file");
     f.write_all(state_str.as_bytes()).expect("Unable to write data");
    
 
 
 }
 