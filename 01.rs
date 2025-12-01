use std::io; // the i/o header

fn main() {
    // mut denotes that the variable is mutable. otherwise it is constant.
    let mut current_pos = 50;
    // modulus is constant so no mut here
    let modulus = 100;
    
    
    // construct a new mutable string...
    let mut input = String::new();
    // and read it to input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // construct a new number which is parsed from  the input
    let number: i32 = input.trim().parse().expect("Input not an integer"); // Handle the parse error
        
    println!("N = {}", number); // autoformat!!!
    
    // here we go
    
    let mut res = 0;
    let mut res2 = 0;
    for lineno in 0..number {
        input.clear(); // read_line appends, so clear to reset the string value
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let dir = input.chars().nth(0).unwrap(); // chars gets teh chars of the string. nth returns an OPTIONAL so unwrap to get the value.
        // println!("{}", dir);
        
        let times: i32 = (&input[1..]).trim().parse().expect("not integer"); // slices are references
        // println!("{}", times);
        
        // every goddamn control flow needs to be bracketed in this exact way (with linked else if things on the same line as the closing of the preceding block)
        if (dir == 'L') {
            let metric = (modulus - current_pos) % modulus + times;
            res2 += (metric / modulus);
            current_pos = (current_pos + modulus * modulus - times) % modulus;
            
        } else {
            let metric = current_pos + times;
            res2 += (metric / modulus);
            current_pos = (current_pos + modulus * modulus + times) % modulus;
        }
        
        if (current_pos == 0) {
            res += 1;
        }
        // println!("{0} {1} {2} {3}", lineno, res, res2, current_pos); // {0} is the 0th arg in the format list, {1} the 1st, etc.
    }
    
    println!("1. {}", res);
    println!("2. {}", res2);
}
