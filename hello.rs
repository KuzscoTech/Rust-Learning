use std::fmt::{self, Formatter, Display};
use std::mem;

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

fn main() {
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // test_case_one_debug_frmt();
    // test_case_two_struct_list_write();
    // test_case_three_custom_format();
    test_case_four_matrix_manipulation();
}

