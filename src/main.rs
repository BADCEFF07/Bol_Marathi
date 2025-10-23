use std::env;
use std::fs;
use std::process::exit;
use std::result::Result::Err;
use std::io;


fn run_file(path : &str) -> Result<(), String>{

    match fs::read_to_string(path){
      Err(msg) => return Err(msg.to_string()),
      Ok(contents ) => return run(&contents),
    }
      
}
// function to run the script contents
fn run(_contents: &str) -> Result<(), String>{
  // Placeholder implementation
  return  Err("Not implemented yet".to_string());
}
//function to run the interactive prompt
fn run_prompt( )-> Result<(), String>{
  // Placeholder implementation
  println!("> ");
  let mut buffer = String::new();
  let stdln = io::stdin();
  stdln.read_line (&mut buffer){
    Ok(_) => (),
    Err(_) => return Err("Could not read line".to_string()),
  }
  println!("You wrote {}", buffer);
}
// Main function
fn main(){
  // Collect command line arguments
  let args: Vec<String> = env::args().collect();
    // Check the number of arguments
    if args.len() > 2 {
      println!("Usage: bol marathi [script]");
      exit(64); 
    } 
    // If a file path is provided, run the file
    else if args.len() == 2 {
      // Run the file
      match run_file(&args[1]){
        Ok(_) => exit(0),
        //print error message and exit with error code
        Err(msg) => {
        println!("ERROR : \n {}", msg);
        exit(1);
      
     },
     }
    }
    // If no file path is provided, run the prompt 
    else {
      match run_prompt() {
        Ok(_) => exit(0),
        Err(msg) => {
          println!("ERROR : \n {}", msg);
          exit(1);
        },  
      }
    }
  
  
}
