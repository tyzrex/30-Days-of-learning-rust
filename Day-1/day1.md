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