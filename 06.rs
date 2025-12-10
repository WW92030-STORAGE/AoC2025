use std::io;
use std::cmp;

/*

The first part of this was done on day 6, the second part on day 10 after first doing it in python.
The purpose of aoc2025 is to learn rust and to solve problems. nobody ever said they had to be done simultaneously.

*/

fn main() {
	// standard input with multiple lines and header of one count line count count count line
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
let NLINE:
	usize = input.trim().parse().expect("Input not an integer"); // Handle the parse error

	println!("N = {}", NLINE);
	let mut numbers: Vec<Vec<i64>> = Vec::new();
	let mut ops: Vec<char> = Vec::new();
	let mut lines: Vec<Vec<char>> = Vec::new(); // 6B: it's slice time!!! 

	let mut res = 0;
	let mut res2 = 0;
	
	let mut NUM_COLS: usize = 0;
	for lineno in 0..(NLINE - 1) {
	    input.clear();
		io::stdin().read_line(&mut input).expect("Failed to read line");
        lines.push(input.chars().collect()); // 6B
		input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.
		
		
		
		println!("{}: {}", lineno, input);
		
		let values = input.split(" ");
		
		let mut line: Vec<i64> = Vec::new();
		
		for value in values {
		    if (value == "") { // some whitespaces are not even, so we must deal with those
		        continue;
		    }
		    line.push(value.trim().parse().expect("not int"));
		}
		numbers.push(line); // This MOVES the Vec<i64> line to the ownership of numbers and thus cannot be used anymore normally.
	}
	
	// Final line
	
	input.clear();
	io::stdin().read_line(&mut input).expect("Failed to read line");
    lines.push(input.chars().collect()); // 6B
    
	input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.
	
	let values = input.split(" ");
	for value in values {
	    if (value == "") {
	        continue;
	    }
	    ops.push(value.chars().nth(0).unwrap());
	}
	
	NUM_COLS = ops.len();
	
	for col in 0..NUM_COLS {
	    let mut temp: i64 = 0;
	    if (ops[col] == '*') {
	        temp = 1;
	    }
	    for row in 0..(NLINE - 1) {
	        if (ops[col] == '+') {
	            temp += numbers[row as usize][col];
	        } else {
	            temp *= numbers[row as usize][col];
	        }
	    }
	    res += temp;
	}
	
	// everything below here before the final print is 6B
	
	NUM_COLS = 0;
	for lineno in 0..lines.len() {
	    NUM_COLS = cmp::max(NUM_COLS, lines[lineno].len());
	}
	
	let mut numbers: Vec<i64> = Vec::new();
	
	// Just like python, you can't "reverse iterate" on a descendin range. 
	// But unlike python there exists no option to set the step value.
	// Instead you use rev() to reverse the range.
	for col in (0..NUM_COLS).rev() {
	    let mut number: i64 = 0;
	    let mut hasn: bool = false;
	    
	    for row in 0..(NLINE - 1) {
	        if (lines[row][col] == ' ') {
	            continue;
	        }
	        number = 10 * number + ((lines[row][col] as i64) - 48);
	        hasn = true;
	    }
	    if (hasn) {
	        numbers.push(number);
	    }
	    if (lines[NLINE - 1][col] == '+') {
	        let mut val: i64 = 0;
	        for index in 0..numbers.len() {
	            val += numbers[index];
	        }
	        res2 += val;
	        numbers.clear();
	    } else if (lines[NLINE - 1][col] == '*') {
	        let mut val: i64 = 1;
	        for index in 0..numbers.len() {
	            val *= numbers[index];
	        }
	        res2 += val;
	        numbers.clear();
	    } 
	}

	println!("[{} {}]",res, res2);
}
