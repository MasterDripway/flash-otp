mod backend;

use core::panic;

use backend::
{
    engine::{random_vec, encode, decode}, 
    matrix::{Matrix}, 
    loader::{open_file}
};



fn execute(settings : Vec<String>) {
    let mode: &str = &settings[1];
    let file_path: &str = &settings[2];
    let output_path: &str = &settings[3];
    let key_path: &str = &settings[4];
    
    let data = Matrix::from(open_file(file_path).expect("Error opening file"));
    match mode {
        "encode" => {
            
            let key = Matrix::from(random_vec(data.array.len()));
            let result = encode(&data, &key);
            match result.dump(output_path) {
                Ok(()) => println!("Saved data successfully"),
                Err(e) => {
                    data.dump(output_path).unwrap();
                    panic!("Data save panicked with {e}. Reverting changes");
                },
            };
            key.dump(key_path).expect("Error while saving key. Reverted changes.")
        },
        "decode" => {
            let key = Matrix::from(open_file(key_path).expect("Error opening key"));
            let result = decode(&data, &key);
            match result.dump(output_path) {
                Ok(()) => println!("Saved data successfully"),
                Err(e) => {
                    data.dump(output_path).unwrap();
                    panic!("Data save panicked with {e}. Reverting changes");
                },
            };
        },
        _ => panic!("Invalid mode"),
    };


}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Insufficient arguments.\nReference:\n[call] [mode] [read_path] [out_path] [key_path]");
    } 
    
    else {
        execute(args);
    }
    Ok(())
    
}




/* 
TODO
// ! FIX ENCODE AND DECODE PROBLEMS - DONE
CHECK ADD AND CHECK SUB NEED FIXING 

*/