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