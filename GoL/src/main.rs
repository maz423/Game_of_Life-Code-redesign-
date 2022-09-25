use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
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