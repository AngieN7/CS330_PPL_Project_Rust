# CS330 PPL Project Rust

# Intro and History
I chose Rust as my programming language for CS330 PPL project due to its memory safety and speed. 
Rust is relatively new, only first conceptualized in 2006 by Graydon Hoare, as a personal project with the goals of combatting memory bugs that are common in languages like C. It was later released in 2010, and it's an open-source language, so it is constantly developing. Like C, Rust is a multi-purpose language that allows programmers to get close to operating systems, it is often used for operating systems and embedded systems, it is also used for GUI and software development.
I plan to continue to learn about Rust by reading The Rust Programming Language, 2nd Edition By Steve Klabnik and Carol Nichols, referring to the official Rust language repo [here](https://github.com/rust-lang/book), and viewing the tutorials on Geeksforgeeks.org [here](https://www.geeksforgeeks.org/introduction-to-rust-programming-language/) and tutorialpoint.com [here](https://www.tutorialspoint.com/rust).  

# Steps to installing Rust
Rust is a compiled language, which means it needs a compiler to execute programs. Rust programs are compiled with rustc. Given Rust's relative newness, it does not come preinstalled on devices and has to be installed manually, and steps depend on whether you have Linux or Unix. I use Unix, so the steps to installation were relatively simple. **For instructions on installing Rust on Linux machines or alternative ways to install Rust click [here] (https://forge.rust-lang.org/infra/other-installation-methods.html#which)
1. Open terminal
2. Paste into terminal:
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
4. Check if installed correctly.
```
$ rustc --version
```
Once the Rust language and compiler are installed, you can use IDE's that support Rust, like VSCode.
**For instructions on VsCode install click [here](https://code.visualstudio.com/download) **
* To use VSCode you will need to install rust-analyzer extension. [Here](https://code.visualstudio.com/docs/languages/rust)
# Steps to creating Rust program in terminal: Hello World
1. Open terminal
2. Create a directory for rust code
3. Create a file for Rust that follows the Rust naming convention.
* Rust naming convention is file.rs and in the event that there are multiple words in the file, the words should be separated by an underscore.
ex.hello_world.rs
```cat >hello_world.rs```
4. Writing the program
* Like many other programming languages, Rust programs must have a main function that runs first, followed by brackets that can accept parameters, and then curly brackets that surround the code of the function.
*  To create a function, you have to first start with fn.
```
 fn main(){
    println!("Hello, world!");
  }
```
5. Saving program: To exit text hit q 

## How to run  in the terminal
1. Call rustc followed by file name
   ```rustc hello_world.rs ```
2. Then, you display the output.
   ``` ./hello_world ```
* Output will produce "Hello, world!" in the terminal.

### Adding Comments to C++ Code 
To add comments to this code, use two forward slashes followed by the comment. 

``` 
// Example of one-line comment on and hello world program.
fn main(){
    println!("Hello, world!");
  }
```

Or- in cases of multi-lined comments, you will use forward slash+star followed by the comment, another star, and another forward slash. 

```
/* Example of multi-line comment on
hello world program. */
fn main(){
    println!("Hello, world!");
  }

```

# PPL 2 Data Types and Naming Conventions
Rust has most of the common data types, data types in Rust are integers, float numbers, characters,  Booleans, arrays and tuples.
* Statically typed, type declaration needed before the variable is created.
* Variable is bound to their type at the time of declaration
* However, if the type is not declared compiler can often correctly assume the variable's type
* Variables in Rust are immutable by default.
* However, you can specify whether or not you'd like the variable to be mutable by adding mut before the variable name.
## Variable conventions
1. Start with let
2. Add variable name, followed by :
3. Declare type followed by =
4. Finally, add a variable.
## Constants

### Integers
* Represents whole numbers.
* Integer variables can be unsigned or signed.
* Unsigned variables can only ever be positive.
* Signed variable can be both negative and positive.
* Default integer variable type is i32

```
//Example of integer variable that can be negative or positive
let x: i32 = 52;

//Example of integer variable that can only be positive
let x: u32 = 52;
```
### Floats
* Represents numbers with decimals (rational numbers)
* Has single and double precision float types represented by f32 and f64.
* The default float type is f64 which is the Double-precision float type.
```
//Example of float variable syntax
let y: f64 = 52.5;

```
### Character
* Rust supports more than just ASCII characters
* Allows for special characters, emojis, and even characters in other languages.
```
//Example of char variable syntax
let c: char = 'c';

//Example of special character variable syntax
let smile: char = '😄';

```
### Booleans
* Variables that have values that are true or false.
* Can also be represented as integers 1, which means true, and 0, which is false.
* Use logic operations:&& (And), | (or) , !(Not)
```
//Example of Bool variable syntax 
 let t: bool = true;
//Example of bool with operator
 let f: bool = !t
```
### Tuples
* Contains a group of values that can belong to different types
* Once fixed length is declared, it is immutable even if you try and use mut
```
//Example of tuple variable syntax
let u:(i32, f64, u64)=(52, 52.5, -18888)

```
### Arrays
* List containing values that are the same type.
* Static but mutable; values can be updated in the array but not deleted when size is initialized.
* Can declare array variables in many different ways.
```
//Array variable syntax option#1
let a = [1, 5, 28, 7];

//Array variable syntax option#2
let a: [i32, 4] = [1, 5, 28, 7];

```

## Operators
* Both int and float allow for mathematical opporations;  +, -, \*, \/, \**, \%

# PPL 3: Loops and Functions
* Rust offers three types of loops
### Loops
*Loops are labeled and
* Keyword that allows code to be repeated until it is told to stop
* Does not necissarily have to have conditions
* Allows nested loops 
* Ended by break condition.
### While
* Runs while a specific condition is met.
### For Loops
* Iterates through ranges, and list
