#### Day 1 Summary

1. ###### Installation

   First of all for the day one I installed rust on my machine. Since I am using a Linux desktop it was as easy as running 2 commands in the terminal and everything was setup by the rust installation script. The script used:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. ###### Hello world

   Every great programming journey or any language I had explored before started with this. So first of all hello world in rust is what we will do atleast me. Hello world just gives the vibe of starting something new for me. So, I just wrote a simple program for hello world in rust and compiled it and ran the program and here we go first program was done. I am including the code snippet for this program as it is very short and simple for the other ones the programs will be linked with their sections.

   ```rust
   fn main(){
       println!("Hello world from Rust!");
   }
   ```

   ###### Compiling and running the progarm:

   ```bash
   rustc main.rs			
   ```

   ###### Output

   ![image-20240514190755426](/home/tyzrex/.config/Typora/typora-user-images/image-20240514190755426.png)

3. ###### Exploring Cargo

   Cargo is the package manager for rust which is used to install various packages with rust. Creating a simple project with cargo to know how it works. 

   ```bash
   cargo new hello_cargo --bin	
   ```

   Using the --bin for making executable as opposed to a libraries. They are often just called binaries.

   ```bash
   cd hello_cargo
   ```

   There is a Cargo.toml and a src directory with along a .gitignore file here. Cargo.toml is for managing the packages just like how package.json is in javascript (an easy way to understand). Now inside the src there is one file with name main.rs.

   ###### Building and running

   ```sh
   cargo build && cargo run
   ```

   ![image-20240514191959930](/home/tyzrex/.config/Typora/typora-user-images/image-20240514191959930.png)

   So this is how to create a very basic project using the cargo package manager and using it to build and run it. Cargo makes life easier by handling much of the complexity of creating and running a project in rust

4. ###### Guessing game from the Rust Programming Book

Below is the code snippet for the guessing game from the Rust Programming Book. The code is very simple and easy to understand. The code is also available in the src directory of the day-1 directory.

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

###### Running the program

```bash
cargo run
```



