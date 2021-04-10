# Rust study (noob)
## My experience learning Rust:

### Expectation:
I don't know much about the language yet. I heard that language is strong for building OS. For now I thought it looked like C.
<br>

# Start:
To install the language I read the documentation. I had no problems.

I used some commands to start the first project:
```
mkdir RUST
cd RUST
cargo new Directory
```
<br>
This provided me with what I needed to do my first hello, world!

Main is an entrypoint. By default there is no 
return:
``` 
fn main() {
    let name =  "your_name";
    println!("Hello, {} World!"); //Macro
} 
```
<br>

To compile and run it was only necessary: ​​``` cargo run ```

It was not necessary to write the type even though it was static. As I understand it, he typifies by context.
<br><br>

### First language challenge:
Print the pi number with onl 3 decimal places.
To solve this problem I had to read the module std :: fmt (Back here if you need to format  strings, work in runtime).

The formatting was simple:
```
    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
    //.* means that this {...} is associated with two format inputs 
```
However, there are other ways, just look at the module.
<br><br>

# Primitives


