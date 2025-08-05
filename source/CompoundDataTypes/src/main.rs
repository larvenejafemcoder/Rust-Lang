//Compound DataTypes - // All of Rust Vars are inmutable by default
//There are 4 groups of compound datatypes 
//Which are arrays, tuples(much like python), slices and (finally) strings(slice string)
fn main() {

    // ////////////////////////////////////////////////////////
    //Declaring an array
    //an array needs to have elements of the same datatypes
    //ex: [1,2,5,6] or ["foo", "bar"] and cannot be mixed with multple datatypes
    let numbers: [ i32; 5 ]= [ 1,2,3,4,5 ];
    println!("Number Arrays: {:?}", numbers);

    
    // let mixed = [1,2,"apple", true];
    // println!("Mixed array: {:?}", mixed)
    
    let fruits: [ &str; 3] = ["Apple", "Banana", "Pineapple"];
    println!("Fruit Arrays: {:?}", fruits);
    println!("Fruit Arrays 1st element: {}", fruits[0]);
    println!("Fruit Arrays 2nd element: {}", fruits[1]);
    println!("Fruit Arrays 3rd element: {}", fruits[2]);
    // ////////////////////////////////////////////////////////


    //Tuples
    let human:(&str, i32, bool) = ("Alice", 30, false);
    // let human:(String, i32, bool) = ("Alice".to_string(), 30, false); // the same as former code
    // let human = ("Alice", 30, false); // also the same
    println!("Human Tuples: {:?}", human);

    let my_mixed_tuple = ("Kratos", 45, true, [1,2,3,4,5]);
    println!("My Mixed Tuple: {:?}", my_mixed_tuple);

    // ////////////////////////////////////////////////////////

    //Slices in Rust are views into a sequence (like an array or string) — 
    //they reference part of the data without owning it, using &[T] or &str. 

    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices:&[&String] = &[&"IT".to_string(), &"Maths".to_string(), &"Algebra".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // ////////////////////////////////////////////////////////
    // String     vs  String Slices (&str)
    // is Mutable  -  is inmutable
    // String (growable, mutable and owned string type)
    let mut stone_cold:String = String::from("Hell, " ); //stored on the heap memory and not the stack memory
    println!("Stone cold said: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold said: {}", stone_cold);

    // whereas &str (non-growable, inmutable and borrowed string type)

    let string: String = String::from("Hello World");
    let slice: &str = &string[0..5]; //stored on the stack not the heap
    println!("String Slice: {}", slice);
    print();
}

fn print() {
    // println!("SLICE: {}", slice);
    println!("How YOU DECLARE A FunCTION");
}