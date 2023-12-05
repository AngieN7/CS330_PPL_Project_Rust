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
* Use logic operations:&& (And), || (or) , !(Not)
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
* Both int and float allow for mathematical operations;  +, -, \*, \/, \**, \%
# If/else
* Rust used if/else statements similarly to other languages
* They execute a code block based on a condition
* They are treated as expressions that have to return the same type.
* Unlike other languages, the condition does not need to be inside of the parenthesis
* The condition must be a boolean because Rust will not convert nonbooleans to bools.
  
'''
let w = 38
if w>2{
    print!("{} is bigger than 2", w);
} else w<2 {
    print!("{} is less than 2", w);
}
'''

## Rust performs a different kind of short-circuiting 
* It will go through all of the statements until it finds the True statement then, it won't bother checking the others.
* For example, if an if statement is true and it is followed by an else if statement, Rust will not bother running the else if statement even if the else if statement is also true. 
* This is similar to how traditional short-circuiting will not check the second condition of an or statement if the first condition is correct.
* 
'''
let w = 6;

    if w>2{
        print!("{} is bigger than 2", w);
    }
    //This would not run since the previous conditional statement was true
    else if w%2 == 0 {
        println!("{} is even", w);
    }
'''
### If let and let else

# PPL 4: Loops and Functions
* Rust offers three types of loops
### Loops
*Loops are labeled
* Keyword that allows code to be repeated until it is told to stop.
* Does not necessarily have to have conditions.
* Allows nested loop.
* Ended by break condition.
* If there was a value that you want to return after the loop, add the variable after the break.
### While
* Runs while a specific condition is met.
#### While Let
### For Loops
* For in is a constructor that uses various iterators
* By default, for construction, uses into_iter on collections like list.
* Collections can also be converted using functions iter() and iter_mut()
#### For range
* Can be used simply to iterate over ranges.
* Represented by for n syntax, and n represents the values being iterated over
* Uses a..b range notation, a is the start, b is where it ends
* B is exclusive, so it will go through value up to b, but it would not include b.
'''
//this will print 1, 2, 3, 4, 5 will NOT print 6.
for i in 1..6{
    println!("{}", i);
}
'''
## Functions
* Denoted by fn declaration before the name
* Function parameters must have type declaration.
* Not all functions explicitly return.
* Return types must be declared using an arrow after parameters, before the code block.
* If they do not declare type, they still return something.
* Functions that do not declare type and  have return expressions return '()'.


### Nested Loops
# PPL 5:Objects, Classes and Inheritance
* Rust supports Structs
## Structs
* A group of key's and fields.
* The keys act as variable names while the fields hold the type and values they accept.
* Very similar to instance variables.
*  First, you define the struct with key and type.
*  Then, a struct instance is created that calls upon the struct and applies values to the keys.
*  Dot notation is used to access the values of a struct and even change them if an instance of the struct is mutable 
*  For keys to be mutable, the entire instance must be mutable
### Structs as an Object
* The structs act as objects, and their keys act as the things they should have.
* Main then creates an instance of that struct, sending it values
* Methods implement the struct and then do something with it's data.
* Then, the main calls the method sending in the specific instances of the strcut.
* Rust does not allow for multiple inheritance.
## Enum

## Standard methods 


