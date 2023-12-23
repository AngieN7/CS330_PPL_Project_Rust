fn main() {
     // Variables can be type annotated.
    let x: i32 = 5;
    let y: f64 = 52.5;
    //shows that operations can be combined on different types
    let sum=x+y;
    let prod=x*y;
    let diff=y-5;
    let quo=y/5;

    println!("Sum: {sum}, Product:{prod}, Differnce:{diff}, Quotient:{quo}");
    
    

    //More syntex
    let u:(i32, f64, u64)=(52, 52.5, -18888);
  

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default = 22;   // `i32`

    // A type can also be inferred from context.
    let  inferred_type = true; // Type bool is inferred.

    // A mutable variable's value can be changed.
    let mut name = "Angie";
    println({name});
    name= "Faith";
    println({name}); 

    // Variables that have not been declared as mut can still be overwritten with shadowing.
    println({default});
    let default = true;
    println({default});

}
