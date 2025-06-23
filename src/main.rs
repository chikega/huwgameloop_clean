/* Gary Chike                     04/02/2023  */
// Edited 05/21/2023   SublimeText on Lubuntu
// Edited 08/02/2023   reviewing with Git, added else if, else 
// Edited 03/02/2024   added sleep, Duration, thread::sleep
// Edited 09/22/2024   added const Q, to_uppercase()
// Edited: 06/23/2025  did away w/ if-else block and created
// two if statements instead, added 'continue' back in

use std::io::{stdin, stdout, Write}; // 03/02/2024

use std::time::Duration;
use std::thread::sleep;

fn main() {
    const Q: &str = "Q"; // explicit type annotation required with 'const'
    
    println!("> press <Q> to exit..");
    loop {
        print!("> "); stdout().flush().unwrap();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
                .expect("> Failed to read line");
        if input.trim().is_empty() {continue;} // 08/02/2023 deleted 'continue', Rust expects at minimum an empty 'block' - {}
        if input.trim().to_uppercase() == Q {break;} // 09/22/2024 changed to_uppercase() and added Q
        println!("> You wrote '{}'", input.trim() );
    }
    println!("> Goodbye!");
    sleep(Duration::from_secs(2));
}