 
/*

(in character)
Hello ladies, gentlemen, humans, furries, protogens, and one and all, Welcome back to another episode of Mersenne Fails at Rust. 
Today we will be looking at string operations and doing horrifying brute force iteration...

*/

use std::io;

fn is_invalid(value: i64) -> bool {
    // format!(...) is like sprintf the same way println! is like printf.
    let int_str = format!("{}", value);
    
    let len = int_str.len();
    if (len % 2 != 0) {
        return false;
    }
    
    // oh and because i forgor :skull: last time: A..B is [A, B).
    for i in 0..(len / 2) {
        if (int_str.chars().nth(i).unwrap() != int_str.chars().nth(i + len / 2).unwrap()) {
            return false;
        }
    }
    return true;
}

fn i2_invalid(value: i64) -> bool {
    let int_str = format!("{}", value);
    // println!("begin {}", value);
    
    // this time we don't return false instantly, we return true if nothing happens in an iteration.
    let len = int_str.len();
    for freq in 2..(len + 1) {
        if (len % freq != 0) {
            continue;
        }
    
        // oh and because i forgor :skull: last time: A..B is [A, B).
        let period = len / freq;
        let mut found = true;
        for i in 0..period {
            
            for k in 1..freq {
                // println!("!! {0} {1}", i, i + k * period);
                if (int_str.chars().nth(i).unwrap() != int_str.chars().nth(i + k * period).unwrap()) {
                    found = false;
                }
            }
        }
        if (found) {
            return true;
        }
    }
    return false;
}

fn main() {
    println!("Hello World");
    // construct a new mutable string...
    let mut input = String::new();
    // and read it to input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<&str> = input.split(',').collect(); 
    // The split() method returns an iterator. To create a usable array of all the values use the collect operation.
    
    let mut invalid_sum = 0; // part 1
    let mut invalid_sun = 0; // part 2 
    
    for slice in values.iter() { // and use the iter operation to get an iterator of an array. kinda redundant this was yeah?
        println!("{}", slice);
        
        let dash = slice.find('-').unwrap(); // find a char. returns the index. again, it's optional, so unwrap because the presence of a dash is a precondition.
        println!("{}", dash);
        
        let sep = dash as usize; // use the as to do typecasting. technically dash is already usize but let's just make sure it is.
        
        // this expands on the one slice thing we did in problem 1. 
        // because slices, strings, whatnot don't usually have a known length they must be put into reference so that the pointers can allocate to stack.
        // therefore here we see that we must enclose everything into pointers
        // this is kinda analogous to how C has char* (or char[]) strings.
        // ...what, you thought this was going to be easy? this isn't Furry Art 101 ... oh wait that's not easy either.
        let slice1: &str = &slice[0..sep];
        let slice2: &str = &slice[(sep + 1)..];
        println!("{0} {1}", slice1, slice2);
        
        let val1: i64 = slice1.trim().parse().expect("not int");
        let val2: i64 = slice2.trim().parse().expect("not int");
        
        for id in val1..(val2 + 1) { // remember ranges are open on the right. don't be like me that thought that they were both closed in problem 1.
            if (is_invalid(id as i64)) { // again, typecasting for completeness
                invalid_sum += id;
            }
            if (i2_invalid(id as i64)) {
                // println!("> {}", id);
                invalid_sun += id;
            }
        }
    }
    
    println!("1. {}", invalid_sum);
    println!("2. {}", invalid_sun);
}
