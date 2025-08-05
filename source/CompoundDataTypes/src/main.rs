//Compound DataTypes
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
    // let human:(&str, i32, bool) = ("Alice", 30, false);
    let human:(String, i32, bool) = ("Alice".to_string(), 30, false);
    // let human = ("Alice", 30, false); // also the same
    println!("Human Tuples: {:?}", human);


}