#![allow(warnings)]

pub mod GoLx{
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::io;
    use std::io::*;


   pub fn Get_input() -> (i32,i32,String) {

        let mut r = 0;
        let mut c = 0;
        let mut g = 0;
        println!("{}","Enter number of rows: ");
        let mut val1 = String::new();
        stdin().read_line(&mut val1)
            .ok()
            .expect("Failed to read line");
    
            let rows = val1.trim();
            match rows.parse::<i32>() {
                Ok(i) => r = i,
                Err(..) => println!("this was not an integer: {}", rows),
            };
        
    
        println!("{}","Enter number of cols: ");
        let mut val2 = String::new();
            stdin().read_line(&mut val2)
                .ok()
                .expect("Failed to read line");
    
                let cols = val2.trim();
                match cols.parse::<i32>() {
                    Ok(i) => c=i,
                    Err(..) => println!("this was not an integer: {}", cols),
                };
    
        
    
        println!("{}", "Please choose a number indicating selected geometry :");
        println!("{}", "1. Cylinderical Top");
        println!("{}", "2. Cylinderical Side");
        println!("{}", "3. Toroidal");
    
        let mut val3 = String::new();
        stdin().read_line(&mut val3)
            .ok()
            .expect("Failed to read line");
        let Geometry = val3.trim();
            match Geometry.parse::<i32>() {
                Ok(i) => g=i,
                Err(..) => println!("this was not an integer: {}", Geometry),
            };
    
            
        
        
        let mut Geo = String::new();
        if g == 1 {
           
           Geo.push_str("Cylinderical_top");
    
        }
        if g == 2 {
            
            Geo.push_str("Cylinderical_side");
        }
        if g == 3 {
            
            Geo.push_str("Toroidal");
    
        }
        
        (r,c,Geo)
    
    }
    
    /*
    Purpose: Reads an input file and returns a string of the initial state.
    Pre-condition:
      param: name: Name of a file that contains the initial state of GoL.
    Post-condition: None
    return: a string that contains the initial state of GoL
    */
    pub fn read_file(name:String) -> String {
    
        let mut file = File::open(name)
        .expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data)
        .expect("Error while reading file");
        data
        
        }
    
    
    
    
    /*
    Purpose: Converts a string representation of GoL into a 2D Vector.
    Pre-condition:
        param: data: A string representation of GoL.
    Post-condition: None.
    return: A 2D Vector representation of GoL.
    */
    
    pub fn Generate_grid(data:String,rows:i32,cols:i32) -> Vec<Vec<i32>>{
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
    
    
    
    /*
    Purpose: Finds neighbours of a cell within the dimension of the Grid.
    Pre-condition:
        param: state: a 2d Vector representation of GOL.
        param: index: The index of the cell whose neighbours are to be found.
    Post-condition:  None
    return: a Vector containing the indicies of the neighbouring cells.
    */
    
    pub fn Find_neighbours(state: &Vec<Vec<i32>>,index:(i32,i32),Geo: &String) -> Vec<(i32,i32)> {
       //assert index validity
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
        else if col + 1 == col_len && (Geo == "Cylinderical_side" || Geo == "Toroidal") { //Cylendrical side.
    
            let indx = (row, 0);
            Index_nbr.push(indx);
    
           
        }
        //left cell
         if col - 1 >= 0 {
         let indx = (row, col - 1);
         Index_nbr.push(indx);
     
        }
        else if col == 0 && (Geo == "Cylinderical_side" || Geo == "Toroidal") { //Cylendrical side.
            let indx = (row, col + (col_len - 1));
            Index_nbr.push(indx);
    
        }
    
        //Top cell
         if row - 1 >= 0 {
         let indx = (row - 1, col);
         Index_nbr.push(indx);}
    
         else if row == 0 && (Geo == "Cylinderical_top" || Geo == "Toroidal"){
            let indx = (row + (row_len - 1), col);
            Index_nbr.push(indx);
         }
        
        //bottom cell
         if row + 1 < row_len {
         let indx = (row + 1, col);
         Index_nbr.push(indx);}
    
         else if row + 1 == row_len && (Geo == "Cylinderical_top" || Geo == "Toroidal") {
            let indx = (0, col);
            Index_nbr.push(indx);
    
    
         }
     
     
        //bottom right
         if row + 1 < row_len && col + 1 < col_len {
         let indx = (row + 1, col + 1);
         Index_nbr.push(indx);}
    
    
         //Cylinderical_side
         else if row + 1 < row_len && col + 1 == col_len && (Geo == "Cylinderical_side" || Geo == "Toroidal"){
            let indx = (row + 1, 0);
            Index_nbr.push(indx);
            
         }
    
         //Cylinderical_top
         else if row == row_len -1 && col < col_len - 2 && (Geo == "Cylinderical_top" || Geo == "Toroidal"){
            let indx = (0, col + 1);
            Index_nbr.push(indx);
            
         }
     
     
        //bottom Left
         if row + 1 < row_len && col - 1 >= 0  {
         let indx = (row + 1, col - 1);
         Index_nbr.push(indx);} 
    
    
    
         //Cylinderical_side
         else if row + 1 < row_len && col == 0 && (Geo == "Cylinderical_side" || Geo == "Toroidal"){ 
            let indx = (row + 1, col_len -1);
            Index_nbr.push(indx);
            
         }
    
         //Cylinderical_top
         else if row == row_len -1 && col != 0 && (Geo == "Cylinderical_top" || Geo == "Toroidal"){
            let indx = (0, col - 1);
            Index_nbr.push(indx);
            
         }
    
         
     
        //Top right
         if row - 1 >= 0 && col + 1 < col_len{
         let indx = (row - 1, col + 1);
         Index_nbr.push(indx);}
    
         //Cylinderical_side
         else if row - 1 > 0 && col + 1 == col_len && (Geo == "Cylinderical_side" || Geo == "Toroidal"){
            let indx = (row - 1, 0);
            Index_nbr.push(indx);
            
         }
         //Cylinderical_top
         else if row == 0 && col < col_len - 2 && (Geo == "Cylinderical_top" || Geo == "Toroidal"){
            let indx = (row + row_len - 1, col + 1);
            Index_nbr.push(indx);
            
         }
    
    
     
        //Top left
         if row - 1 >= 0 && col - 1 >= 0 {
         let indx = (row - 1, col - 1);
         Index_nbr.push(indx);}
    
    
         //Cylinderical_side
         else if row - 1 > 0 && col == 0 && (Geo == "Cylinderical_side" || Geo == "Toroidal") {
            let indx = (row - 1, col_len - 1);
            Index_nbr.push(indx);
            
         }
    
         //Cylinderical_top
         else if row == 0 && col != 0 && (Geo == "Cylinderical_top" || Geo == "Toroidal") {
            let indx = (row + row_len - 1, col - 1);
            Index_nbr.push(indx);
            
         }
    
    
         //Toroidal
         if row == 0 && col == 0 && Geo == "Toroidal"{
            let indx = (row_len - 1, col_len - 1);
            Index_nbr.push(indx);
    
         }
         if row == row_len - 1 && col == 0 && Geo == "Toroidal"{
            let indx = (0, col_len - 1);
            Index_nbr.push(indx);
            
        }
        if row == row_len - 1 && col == col_len - 1 && Geo == "Toroidal"{
            let indx = (0,0);
            Index_nbr.push(indx);
            
        }
        if row == 0 && col == col_len - 1 && Geo == "Toroidal"{
            let indx = (row_len - 1, 0);
            Index_nbr.push(indx);
            
        }
     
     
     Index_nbr
        
     
    }
    
    /*
    Purpose: Returns number of active neighbours.
    Pre-condition:
        param: state: a 2d Vector representation of GOL.
        param: A vector of tuples containing index of a cell.
    Post-condition: None.
    return: the count of active neighbours
    */
    
    pub fn Alive_nbrs (state: &Vec<Vec<i32>>,nbrs:Vec<(i32,i32)>) -> i32 {
        
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
    
    /*
    Purpose: Returns the index of the cell whose state is to be updated.
    Pre-condition:
        params: state: a 2d Vector representation of GOL.
        params: alive: count of alive neghbours.
        params: update_indx: A tuple containing index as first two field and value as last feild if the cell to be updated.
    Post-condition: None.
    return: A tuple containing index as first two field and value as last feild if the cell to be updated.
    */
    
    
    pub fn Update_rule(state: &mut Vec<Vec<i32>>, alive:i32, index:(i32,i32), update_indx: &mut Vec<(usize,usize,i32)>){
         //assert update index is valid.
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
    
    /*
    Purpose: writes the final state in a file.
    Pre-condition:
        params: state: a 2d Vector representation of GOL.
    Post-condition: None.
    return: None.
    */
    
    pub fn output(state: &Vec<Vec<i32>>){
    
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
     
    


}
