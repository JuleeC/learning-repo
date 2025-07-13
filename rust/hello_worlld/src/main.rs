//    format!: write formatted text to String
//    print!: same as format! but the text is printed to the console (io::stdout).
//    println!: same as print! but a newline is appended.
//    eprint!: same as print! but the text is printed to the standard error (io::stderr).
//    eprintln!: same as eprint! but a newline is appended.
fn main() {
    hello_world();
    debugging();
    display();
}
fn debugging() {
    println!("debugging");
    //struct UnPrintableStruct(i32);
    //we cant print this struct because it doesnt implement the Display trait
    //println!("{:?}", UnPrintableStruct(3));

    #[derive(Debug)]
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct PrintableStruct(i32);
    let x = PrintableStruct(3);
    //the same as println!("{:?}", x);
    println!("{:?}", PrintableStruct(3));
    println!("{:?}", x);

    //    #[derive(Debug)]
    //    struct Person<'a> {
    //        name: &'a str,
    //        age: u8,
    //    }
    //
    //  let name = "Peter";
    // let age = 27;
    //let peter = Person { name, age };

    // Pretty print
    // println!("{:#?}", peter);
    // println!("{0}", name);
    //  println!("{0}", age);
}

fn hello_world() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    //println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    println!("Hello World!");
}
//need to import the trait
use std::fmt;
fn display() {
    println!("display");
}
