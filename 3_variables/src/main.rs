#[allow(unused_variables)]
fn main() {
    
    println!("\nVariables\n");
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
    exercise_9();
}


// immutable variables
fn exercise_1() {
    // Fix the error below with least amount of modification to the code
    println!("Exercise 1");
    let x: i32 = 5;
    // let y: i32; Warning if variable not used

    assert_eq!(x, 5);
    println!("Suceess!\n")
}

// mutable variables
fn exercise_2() {
    // Fill the blanks in the code to make it compile
    println!("Exercise 2");
    let mut x: i32 = 1;

    x += 2;
    assert_eq!(x, 3);
    println!("Suceess!\n")
}

// scope
fn exercise_3() {
    // Fix the error below with least amount of modification
    println!("Exercise 3");
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and the value of y is {}", x, y);
    }
    
    // println!("The value of x is {} and the value of y is {}", x, y); // Error
    println!("Suceess!\n")
}

fn exercise_4() {
    // Fix the error with the use of define_x
    println!("Exercise 4");
    // println!("{}, world", x); 
    define_x();
    println!("Suceess!\n")
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x)
}

// shadowing
fn exercise_5() {
    // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
    println!("Exercise 5");
    let x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("{}", x); // Prints "42".
    println!("Suceess!\n")
}

fn exercise_6() {
    // Remove a line in the code to make it compile
    println!("Exercise 6");
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!\n");
}

// unused variables
fn exercise_7() {
    // Remove a line in the code to make it compile
    println!("Exercise 7");
    // Fix the warning below
    let x: i32 = 1;
    // #[allow(unused_variables)]
    println!("Success!\n")
}

// destructuring
fn exercise_8() {
    // Fix the error below with least amount of modification
    println!("Exercise 8");
    let (mut x, y): (i32, i32) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!\n");
}

// destructuring assignments
fn exercise_9() {
    println!("Exercise 9");
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}