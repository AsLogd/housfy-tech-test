use std::io::Read;
use std::io;

const PARSE_ERROR: &str = "Parsing error. Please, use the follwing input format: 
(0-9)+, (0-9)+ 
(N|W|E|S)
(F|L|R)+
";

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut data = match htt::parse_input(&buffer) {
    	Some(d) => d,
    	None => {
    		println!("{}", PARSE_ERROR);
    		return ();
    	}
    };

    match htt::solve(&mut data) {
    	Ok(p) => println!("No collision detected. Current position: {:?}", p),
    	Err(p) =>println!("Would collide at {:?}", p)
    }
}










