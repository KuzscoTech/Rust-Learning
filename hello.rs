use std::fmt;
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
    struct List(Vec<i32>, Vec<f64>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[")?;
            for (index , item) in &self.iter() {
                //{ vec[1,2,3,4,5], vec[2.3,4.4,5.6] } 
                //must make a tuple index
                let vec = item;
                write!(f, "(")?;
                
                //Iterating over 'v' in 'vec' while enumerating the iteration
                for (count, v) in vec.iter().enumerate() {
                    if count != 0 { write! (f, ", ")?; }
                        write!(f, "{}", v)?;
                    if (count == (item.len() - 1) && index != (mem::size_of(&self.len()) - 1)) {
                        write!(f, "), ")?;
                    } else {
                        write!(f, ") ")?;
                    }
                }
            }
            write!(f, "]")
        }
    }

    let v = List(vec![4,5,6,7,8,9], vec![3.4, 2.3, 9.2]);
    println!("{}", v);
}


fn main() {
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // test_case_one_debug_frmt();
    test_case_two_struct_list_write();
}

