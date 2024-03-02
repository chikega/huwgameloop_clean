/* Gary Chike                     04/02/2023  */
// Edited 05/21/2023   SublimeText on Lubuntu
// Edited 08/02/2023   reviewing with Git, added else if, else 

use std::io::{stdin, stdout, Write}; // 03/02/2024

use std::time::Duration;
use std::thread::sleep;

fn main() {
println!("> press <q> to exit..");
loop {
    print!("> "); stdout().flush().unwrap();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
            .expect("> Failed to read line");
    if input.trim().is_empty() {} // 08/02/2023 deleted 'continue', Rust expects a 'block' - {}
    else if input.trim().to_lowercase() == "q" {break;}
    else {println!("> You wrote '{}'", input.trim() );}
}
println!("> Goodbye!");
sleep(Duration::from_secs(2));
}