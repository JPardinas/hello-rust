#[allow(unused_variables)]
fn main() {
    
    println!("\nBasic Types\n");
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


// Remove something to make it work
fn exercise_1() {
    println!("Exercise 1");
    let x: i32 = 5;
    // let mut y: u32 = 5;
    let mut y: i32 = 5;

    y = x;
    
    let z: i32 = 10; // Type of z ? 

    println!("Success!");
}


// Fill the blank
fn exercise_2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}


// Modify `assert_eq!` to make it work
fn exercise_3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


// Fill the blanks to make it work
fn exercise_4() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

// Fix errors and panics to make it work
fn exercise_5() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }
 

// Modify `assert!` to make it work
fn exercise_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}


// Fill the blank to make it work
fn exercise_7() {
    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}


fn exercise_8() {
    // assert!(0.1+0.2==0.3);
    assert!(0.1_f32+0.2_f32==0.3_f32);

    println!("Success!");
}

fn exercise_9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}
