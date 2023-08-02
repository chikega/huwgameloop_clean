/* Gary Chike                     04/02/2023  */
// Edited 05/21/2023   SublimeText on Lubuntu
// Edited 08/02/2023   reviewing with Git  

use std::io::stdin;
use std::io::stdout;
use std::io::Write; // flush()
use std::time::Duration;
use std::thread::sleep;

fn main() {
println!("> press <q> to exit..");
loop {
    print!("> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
            .expect("> failed");
    if input.trim().is_empty() {}
    else if input.trim().to_lowercase() == "q" {break;}
    else {println!("> You wrote '{}'", input.trim() );}
}
println!("> Goodbye!");
sleep(Duration::from_secs(2));
}