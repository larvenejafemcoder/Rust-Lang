// Primitive data types
// which are int, float, bool and char


//Integer
// Rust has signed (+ and -) and usinged integer (only+) types of different sizes.
// which are i8, i16, i32, i64 and lastly i128 (Signed integers). =>>  bigger the number the bigger the size
// which are u8, u16, u32, u64 and lastly u128 (unSigned integers).

// All of Rust Vars are inmutable by default

fn main() {
    let x: i32 = -42;
    let y: u64 = 100; // cannot put a negative data with unsigned integers

    println!("Singed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    // the different between the i32 and i64 is the (bits of both) so i32 has 32bits and i64 has 64bits
    // range : i32 - 2147483647
    //         i64 - 9223337203664775807
    
    let e: i32 = 2147483647;
    let i: i64 = 9223337203664775807;

    println!("The Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    //Float [Floating Point Types]
    //f32, f64 
    let pi: f64 = 3.14;
    println!("Value of pi is: {}", pi);

    //Boolean Values : True or False
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    //Character Type - Char

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);

}
