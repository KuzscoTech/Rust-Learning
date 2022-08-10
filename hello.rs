#![allow(dead_code)]
use std::fmt::{self, Formatter, Display};
use std::mem;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn test_case_one_debug_frmt() {
    #[derive(Debug)]
    struct Testing(f32, f32, f32);

    //This is pretty much the same as toString() -> for arrays
    impl fmt::Display for Testing {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }

    let test = Testing(1.2, 2.3, 3.4);
    println!("{}", test);
}

fn test_case_two_struct_list_write() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                //{ vec[1,2,3,4,5], vec[2.3,4.4,5.6] } 
                //must make a tuple index
                let vec = &self.0;
                write!(f, "[")?;
                
                //Iterating over 'v' in 'vec' while enumerating the iteration
                for (count, v) in vec.iter().enumerate() {
                    if count != 0 { write! (f, ", ")?; }
                    write!(f, "{}", v)?;
                }
            write!(f, "]")
        }
    }

    let v = List(vec![4,5,6,7,8,9]);
    println!("{}", v);
}

fn test_case_three_custom_format() {
    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }
    
    impl Display for City {
        // `f` is a buffer, and this method must write the formatted string into it
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
    
            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument)
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }
    
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let hex = format!("{:0<2X}{:0<2X}{:0<2X}", self.red, self.green, self.blue);
            write!(f, "RGB ({}, {}, {}) 0x{}", self.red, self.green, self.blue, hex)
        }
    }

    
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        //*city -> unwrapping the data
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }
}

fn test_case_four_matrix_manipulation() {

    fn transpose(mut matrix: Matrix) -> Matrix {
        let temp = matrix.2;
        matrix.2 = matrix.1;
        matrix.1 = temp;
        matrix
    }

    struct Matrix(f32, f32, f32, f32);


    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n", self.0, self.1)?;
            write!(f, "( {} {} )", self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

fn test_case_five_array_manipulation() {
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }
    
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    
}

fn test_case_six_structs(){

    fn rect_area(rectangle: Rectangle) -> f32{
        let w: f32 = rectangle.bottom_right.y - rectangle.top_left.y;
        println!("{}", w);
        let l: f32 = rectangle.bottom_right.x - rectangle.top_left.x;
        println!("{}", l);

        (w*l)
    }

    fn square(point: Point, arg: f32)-> Rectangle {
        let point_two = Point {..point};
        let rectangle = Rectangle {
            top_left: point,
            bottom_right: Point {x: (point_two.x + arg), y: (point_two.y + arg)},
        };
        rectangle
    }

    struct Person {
        name: String,
        age: u8,
    }
    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    let name = String::from("Kushal");
    let age = 23;
    let kushal = Person{name, age}; 

    let point: Point = Point { x: 5.2, y: 0.0 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    let bottom_right = Point { x: 10.3, y: 0.4 };

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        //                 x: 5.2        y: 0
        top_left: Point { x: left_edge, y: top_edge },
        //              x: 10.3          y:0.4   
        bottom_right: bottom_right,
    };
    println!("Rectangle: {}{}", _rectangle.top_left.x, _rectangle.top_left.y);
    println!("Area of Rectangle: {}", rect_area(_rectangle));
    let _square = square(point, 5.0);
    println!("Square : ({}, {})", _square.bottom_right.x, -_square.bottom_right.y);
    println!("Area of Square: {}", rect_area(_square));
}

fn test_case_seven_enum(){
    enum WebEvent {
        // An `enum` may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and
    // returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            },
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);


    //Type Alias
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    // Creates a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let x = Operations::Add;

    // Explicitly `use` each name so they are available without
    // manual scoping.
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use Work::*;
    
    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    // println!("{} {}", status, work);
    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
}

fn test_case_eight_linkedlist(){
    enum List {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }

    use  List::*;
    // Methods can be attached to an enum
    impl List {
        // Create an empty list
        fn new() -> List {
            // `Nil` has type `List`
            Nil
        }

        // Consume a list, and return the same list with a new element at its front
        fn prepend(self, elem: u32) -> List {
            // `Cons` also has type List
            Cons(elem, Box::new(self))
        }

        // Return the length of the list
        fn len(&self) -> u32 {
            // `self` has to be matched, because the behavior of this method
            // depends on the variant of `self`
            // `self` has type `&List`, and `*self` has type `List`, matching on a
            // concrete type `T` is preferred over a match on a reference `&T`
            // after Rust 2018 you can use self here and tail (with no ref) below as well,
            // rust will infer &s and ref tail. 
            // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
            match *self {
                // Can't take ownership of the tail, because `self` is borrowed;
                // instead take a reference to the tail
                Cons(_, ref tail) => 1 + tail.len(),
                // Base Case: An empty list has zero length
                Nil => 0
            }
        }

        // Return representation of the list as a (heap allocated) string
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // `format!` is similar to `print!`, but returns a heap
                    // allocated string instead of printing to the console
                    format!("{}, {}", head, tail.stringify())
                },
                Nil => {
                    format!("Nil")
                },
            }
        }
    }

    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn main() {
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // test_case_one_debug_frmt();
    // test_case_two_struct_list_write();
    // test_case_three_custom_format();
    // test_case_four_matrix_manipulation();
    // test_case_five_array_manipulation();
    // test_case_six_structs();
    // test_case_seven_enum();
    test_case_eight_linkedlist();
}

