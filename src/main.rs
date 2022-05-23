// Rust cat(1)
// An implementation of cat in Rust
// Goals:
// * Learn basic I/O

use std::{env, fs, io};

// env::args() gives back an Args iterator, which iterates over the list of arguments passed
// in. Thus, it can be treated like a regular iterator.
fn main() -> io::Result<()> {
    // env::args() gives the arguments passed when calling the application
    // using something like io::stdin would probably give you the ability to write to stdin after
    // calling the executable
    let mut args = env::args();
    args.next(); // The first argument is the path to the executable

    // The source to be catted.
    let source = args.next();

    /* Implementing the first part of cat(1) manual: reading from stdin().
     * If the input is blank or -, then the input will be read from stdin.
     * It would probably be easier to check if it matches these two, rather than a valid filepath.
     */
    if let Some(input) = source {
        // The source exists.
        match input.as_str() {
            "-" => {
                // Read from stdin
                let stdin = io::stdin();
                let mut buffer = String::new();
                // TODO: This should loop
                while let Ok(char) = stdin.read_line(&mut buffer) {
                    if char == 0 {
                        break;
                    } else {
                        print!("{buffer}");
                        buffer.clear();
                    }
                }
            }
            x => {
                let source = fs::read(x)?;
                output(source);
            }
        }
    }

    Ok(())
}

fn output(buf: Vec<u8>) {
    for b in buf {
        print!("{}", b as char);
    }
}
