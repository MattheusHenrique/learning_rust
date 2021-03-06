# Rust study (noob)
## My experience learning Rust:

Some examples (almost all) were taken from https://doc.rust-lang.org/rust-by-example/types/cast.html. Others I forgot.

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

### A little exercise
I did some exercises in "Exercism". I had difficulty understanding the difference between & str and String. When I get to that part of the documentation, I will pay special attention.
<br><br>

# Primitives
In general, the types are similar to C.

A brief explanation about each of them:

### Scalar Types:
- signed integers: ```i8```, ```i16```, ```i32```, ```i64```, ```i128``` and ```iseze```(a pointer size).
- unsigned integers: equal to signed integers but with ```u``` instead of ```i```.
- float: ```f32```, ```f64```.
- char: Unicode(4 bytes each)
- bool: The legendary true and false as usual.
- unit type (): This is different. Its only possible value is an empty tuple ```()```.
<br>
<br>
### Compound Types
- arrays like: ```[1, 2, 3]```
- tuples like: ```(1, false)```

Intergers default to i32 and floats to f64.

### Ways to declare variables: 
By context:
```
let logical = true //is a bool
let a_float = 3.0 //'f64'
```

Writing down the type. (like C):
```
let year: i32 = 2021
```
Using as suffix:
```
let an_integer = 5i32
```
Let variables by default are immutable, but there is ```mut```.
```
let mut days = 21;
days = 22;
```
The variable can only receive the same type.
```
let mut integer = 31;
integer = true; //Compile error
```
Variables can have different types if they are declared:
```
let mut happy = true;
happy = 3; //does not work
let happy = 3; //work
```

### Literals and operators:
Integers can be represented with notation binary, hexadecimal and octal: ``` 0b, 0x, 0o ```

Underscores are used to improve readability:
```100_000_000``` is same ```100000000```
<br>

## Tuples:
The tuple is a collection of different types. Tuples are useful for functions that want to return several values of different types.
```
let user = (10, "mattheus"); 
```

## Arrays: 
Arrays are collections of the same type. They are stored sequentially at compile time. Your signature and the type and its length: ```[<T>; length]```
```
let numbers: [i32, 5] = [1, 2, 3, 4, 5] ; 
```
<br>

## Slices:
Slices are similar to array. but they do not have a defined size at compilation time. Your signature and the first word is a pointer to the data and the second word is the length of the slice: ```&[<T>]```

Similar to the dynamic allocation of a vector in C ? I do not know. but if you don't have to give free () as I understand it. I already love Rust.
<br>

# Custom Types:
It consists of two keywords
- struct: define a structure (like C??)
- enum: define an enumeration
Constants can also be create via the const and static keywords.
<br>

## Structures:
- Tuples structs
- The classic C structs (this I know.)
- Unit structs, which are field-less. (wtf?)

EXAMPLES:
```
//like C
struct Car{
    name: String,
    age: u8;
}

//A unit struct (wtf two)
struct Unit;

//A tuple struct
struct Pair(i32, f32);

//Structs can be reused as fields of another struct
like c 

```
<br>

## Enums:
TODO
<br>

## use:
TODO
<br>

## Constants
There are two types of constants in rust. They can be declared in any scope.
- ```const```: Unchanging 
- ```static```: Possibly changeable. With static lifetime.
- Are declared in uppercase
<br><br>

# Variable Bindings:
I had a hard time understanding the concept. For me variables and variable bindings were the same thing.
But from what I understand, variable bindings are related to the declaration of the variable. (which by default is not variable: "mutable"). I didn't like the explanation of the documentation. I think it should be clearer, since the concept is quite different from other languages.

ex: 
```
let number = 20; //Bindings
```

## Mutability:
Not much to talk about. ```mut``` Mut in building to make mutable.

ex:
```
let mut age = 10;

println!("I am {} years old", age); //age = 10
age+=1;
println!("I am {} years old", age); //age = 11
```

## Scope and Shadowing:
Let declarations are block-scoped: ```{}```.
Allowing shading of variables.
<br>

## Declare first:
It is possible to declare the variable without passing value. But this variable must receive value at compile time. Or the copiler will shoot you. Declare first, trust me I have suffered.

## Freezing:
I did not understand. //TODO

# Types

## Casting:
Rust cannot convert types implicitly, however it is possible to make explicit casting by the keyword ```as```

In general, conversions follow the c pattern. However, cases in which the casting is undefined in c, have well-defined values.

ex: 
```
//Implicit conversion
let decimal: f32 = 5.0;
let integer: u8 = decimal; 
//Error in your face
```

```
//Explicit conversion
let integer = decimal as u8;
let character = integer as char;
//Everything worked out
```

## Literals:
The types of natural numbers can be defined as a suffix or before ```=```
```
let number = 32i8;
let age: u8 = 22;
```
If there are no limitations, the compiler will interpret integers as i32 and floats as f64


