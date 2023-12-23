# CS330 PPL Project Rust

# Intro and History
I chose Rust as my programming language for CS330 PPL project due to its memory safety and speed. 
Rust is relatively new, only first conceptualized in 2006 by Graydon Hoare, as a personal project with the goal of combatting memory bugs that are common in languages like C. It was later released in 2010, and it's an open-source language, so it is constantly developing. Like C, Rust is a multi-purpose language that allows programmers to get close to operating systems, it is often used for operating systems and embedded systems, it is also used for GUI and software development.
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
* Rust naming convention is file.rs and if there are multiple words in the file, the words should be separated by an underscore.
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

# How to run  in the terminal
1. Call rustc followed by file name
   ```rustc hello_world.rs ```
2. Then, you display the output.
   ``` ./hello_world ```
* Output will produce "Hello, world!" in the terminal.


### Adding Comments to C++ Code 
To add comments to this code, use two forward slashes followed by the comment. 

```rust 
// Example of one-line comment on and hello world program.
fn main(){
    println!("Hello, world!");
  }
```

Or- in cases of multi-lined comments, you will use forward slash+star followed by the comment, another star, and another forward slash. 

```rust
/* Example of multi-line comment on
Hello World program. */
fn main(){
    println!("Hello, world!");
  }

```
# Data Types, Variable Syntex, Naming Conventions, and Reserved Words
Rust has most of the common data types, data types in Rust are integers, float numbers, characters,  Booleans, arrays, and tuples. Rust is statically typed so the type declaration is needed before creating a variable, and those variables are bound to that type for their lifetimes. That said, if the variable type is not declared the compiler can often assume the variables type, and will compile 
By default, variables in Rust are immutable, but the language does give users an option to create a mutable variable by placing mut in front of it. 

## Naming Conventions and Reserved Words
Rust variable names are typically lowercase, and when they have two parts it's best practice to use snakecase. Rust has a few categories for their reserved words, these words cannot be used as variable names. Reserve words fall under three categories, reserved and in use, reserved for future use, and weakly reserved keywords.

**Reserved and in use:** types that can be called and used in current instances of Rust, 

**Reserved for future use**:Types that will likely be introduced in future versions of Rust, and are already in use in many other languages, 

**Weak keywords**: These reserved words are reserved based on context, but it is still best practice not to use them in your variable names. 

| Reservation Type | Reserved Words |
| --- | --- |
| **Strict and in use** | as, break, const, continue, crate, else, enum, extern, false, fn, for, if, impl, in, let,loop, match, mod, move, mut, pub, ref, return, self, Self, static, struct, super, trait, true, type, try, unsafe, use, where, while |
|**Strict for future use** | abstract, become, box, do, final, macro, override, priv, typeof, unsized, virtual, yield |
| **Weak keywords** | macro_rules, union, 'static, dyn |



## Data Types and Variable Declaration Syntax
1. Start with let
2. Add variable name, followed by :
3. Declare type followed by =
4. Finally, add a variable.
   
### Integers
* Represents whole numbers.
* Integer variables can be unsigned or signed.
* Unsigned variables can only ever be positive.
* Signed variable can be both negative and positive.
* Default integer variable type is i32

```rust
//Example of integer variable that can be negative or positive
let x: i32 = 52;

//Example of integer variable that can only be positive
let x: u32 = 52;
```
### Floats
* Represents numbers with decimals (rational numbers)
* Has single and double precision float types represented by f32 and f64.
* The default float type is f64 which is the Double-precision float type.
```rust
//Example of float variable syntax
let y: f64 = 52.5;

```
### Character
Rust supports more than just ASCII characters, it allows for special characters, emojis, and even characters in other languages.
```rust
//Example of char variable syntax
let c: char = 'c';

//Example of special character variable syntax
let smile: char = '😄';

```
### Booleans
Boolean-type variables are variables that have values that are true or false. They can also be represented as integers 1, which means true, and 0, which is false. They use logic operations: && (And), || (or) , !(Not)
```rust
//Example of Bool variable syntax 
 let t: bool = true;
//Example of bool with operator
 let f: bool = !t
```
### Tuples
A data type that contains a group of values that can belong to different types. Once a fixed length is declared, it is immutable even if you try and use mut
```rust
//Example of tuple variable syntax
let u:(i32, f64, u64)=(52, 52.5, -18888)
```
### Arrays
Arrays are lists containing values that are the same type. They are static but mutable; values can be updated in the array but not deleted when the size is initialized, and they can declared in many different ways.
```rust
//Array variable syntax option#1
let a = [1, 5, 28, 7];
//Array variable syntax option#2
let a: [i32, 4] = [1, 5, 28, 7];

```

## Operators
Both int and float allow for mathematical operations;  +, -, \*, \/, \**, \%
# If/Elses 
Rust used if/else statements similarly to other languages, they execute a code block based on a condition. They are treated as expressions that have to return the same type. Unlike other languages, the condition does not need to be inside of the parenthesis. The condition must be a boolean because Rust will not convert non-booleans to bools.
```rust
let w = 38
if w>2{
    print!("{} is bigger than 2", w);
} else w<2 {
    print!("{} is less than 2", w);
}
```

## Rust performs a different kind of short-circuiting 
Rust will go through all of the statements until it finds the True statement then, it won't bother checking the others. For example, if an if statement is true and it is followed by an else if statement, Rust will not bother running the else if statement even if the else if statement is also true. This is similar to how traditional short-circuiting will not check the second condition of an or statement if the first condition is correct.

```rust
//Examples of typical Short-circuiting that is found in Rust and other languages
let w = 6;
let j = 3
    //doesn't bother to check the second condition since it knows the first is true, and only one of the two has to be true for it to be true
    if w>2 or j % 5==0{
        print!("wow");
    }

    //This would not run since the first statement is false, and both statements since **and** are used.
    if w == 0 and j ==3 {
        println!("{} is even", w);
    }
```

```rust
//Examples of non-traditional Short-circuiting that is found in Rust
let w = 6;

    if w>2{
        print!("{} is bigger than 2", w);
    }
    //This would not run since the previous conditional statement was true
    else if w%2 == 0 {
        println!("{} is even", w);
    }
```

# PPL 4: Loops and Functions
* Rust offers three types of loops
### Loops
Loops are labeled with the keyword "loop", and allow for code to be repeated until it is told to stop with the use of the work break. Loops in Rust do not have to have conditions, and can instead be a block of code that will repeatedly execute until told to break. Allows nested loop. If there was a value that you want to return after the loop, add the variable after the break.

```rust
//this will loop infinitely since there is no break statement or loop condition.
loop{
println("looptie-loop");
}

```
### While
On the other hand, loops have to have a connection within them, and run when the specific condition is met.
```rust
let mut num = 0;
let loopcon = 5;  
while num >=loopcon{
println("looptie-loop");
num+1;
} 
```

### For Loops
For is a constructor that uses various iterators. By default, for construction, uses into_iter on collections like list. Collections can also be converted using functions iter() and iter_mut()
#### For range
Used simply to iterate over ranges. Uses a..b range notation, a is the start, and b is where it ends. B is exclusive, so it will go through value up to b, but it would not include b.
```rust
//this will print 1, 2, 3, 4, 5 will NOT print 6.
for i in 1..6{
    println!("{}", i);
}
```
## Functions
Denoted by fn declaration before the name. Like variables, Function parameters must have type declaration. Not all functions explicitly return. Return types must be declared using an arrow after parameters, before the code block. If they do not declare type, they still return something. Functions that do not declare type and  have return expressions return '()'.

'''rust 
//Function that prints hello 
fn main(){
let a = 2;
let b = 3;
sum(a, b);
}

fn sum(a:int, b:int){
println(a+b);
}

'''
# PPL 5:Objects, Classes and Inheritance
Rust supports Structs that act as objects within the language. Structs are a group of associated keys and fields, where the keys act as variable names while the fields hold the type and values they accept. In addition to not supporting objects, Rust does not support inheritance. Instead, Rust used Bound Parametric Polymorphism by using generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. With Bound Parametric Polymorphism, Rust can take the object-like type, struct, and uses implementation to access its data in a related function. 

### Structs as an Object
The structs act as objects, and their keys act as the things they should have. Main then creates an instance of that struct, sending it values. Methods implement the struct and then do something with its data.

```rust
pub struct Contact{
    name: String,
    cell number: int,
    email: string
}
impl Contact {
    pub fn get_name(&self) {
        println("Name;{}", self.name);
        }
    }
}
fn main {
let my_contact = Contact {
        name: Rusty,
        cell number: 1112223333,
        email: String::from("rusty@gmail.com")
   };
}

```




