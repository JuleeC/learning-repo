fn main() {
    primitves();
    literals_and_operators();
    tuples();
}
fn primitves() {
    println!("primitves");
    //rust provides a number of primitive types
    //

    /*   Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    /   Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    /   Floating point: f32, f64
    /   char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    /   bool either true or false
    /  The unit type (), whose only possible value is an empty tuple: ()
    */
    // arrrays like [1,2]and tuples like (1,2)fn main() {
    //
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    //mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    // works as well becuase the length is known at compile time.
    let my_array = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
fn literals_and_operators() {
    //when declaring a varibale using underscores is also allowed to improve readability
    let x = 1_000_000;
    println!("x is {}", x);
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
fn tuples() {
    // Tuples can be used as function arguments and as return values.

    // The following struct is for the activity.
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;
    //return the reversed tuple
    (bool_param, int_param)
}
fn arrays_and_slices() {
    // Arrays are fixed length sequences of elements of the same type.
    // The type of the elements is inferred from the value.
    let arr = [1, 2, 3, 4, 5];
    println!("arr is {:?}", arr);

    // Slices are dynamically sized sequences of elements of the same type.
    let slice = &arr[1..4];
    println!("slice is {:?}", slice);

    // Slices can be created from arrays.
    let slice = &arr[..];
    println!("slice is {:?}", slice);

    // Slices can be used to borrow multiple elements from an array.
    let slice = &arr[1..4];
    println!("slice is {:?}", slice);

    // Slices can be used to initialize an array.
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..4];
    slice.reverse();
    println!("slice is {:?}", slice);
    println!("arr is {:?}", arr);
}
