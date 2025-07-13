//    format!: write formatted text to String
//    print!: same as format! but the text is printed to the console (io::stdout).
//    println!: same as print! but a newline is appended.
//    eprint!: same as print! but the text is printed to the standard error (io::stderr).
//    eprintln!: same as eprint! but a newline is appended.
fn main() {
    hello_world();
    debugging();
    display();
    formatting();
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
    // Define a structure for which `fmt::Display` will be implemented. This is
    // a tuple struct named `Structure` that contains an `i32`.
    struct Structure(i32);
    let x = Structure(3);
    println!("{}", x);
    impl fmt::Display for Structure {
        //this trait requires fmt with this exact signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Structure({})", self.0)
        }
    }
    // A structure holding two numbers. `Debug` will be derived so the results can
    // be contrasted with `Display`.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`.
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }
}
fn formatting() {
    println!("formatting");
    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }

    use std::fmt::{Display, Formatter};
    impl Display for City {
        // `f` is a buffer, and this method must write the formatted string into it.
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument).
            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
}
