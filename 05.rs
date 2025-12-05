use std::io;
use std::cmp;

/*

Prepend the following:

Number of lines in the first part
Number of lines in the second part

*/

/*

Alright here's the flavor text

AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

*/

fn main() {
	// standard input with multiple lines and header of one count line count count count line
	let mut input = String::new();
	
	// oho! there are TWO meta-parameters this time!!
	
	input.clear();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	let N_RANGES: usize = input.trim().parse().expect("Input not an integer"); // Handle the parse error
	
	input.clear();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	let N_DATA: usize = input.trim().parse().expect("Input not an integer"); // Handle the parse error
	
	println!("N_RANGES: {} | N_DATA: {}", N_RANGES, N_DATA);
	
	
	let mut res = 0;
	let mut res2 = 0;
	
	let mut ranges: Vec<(i64, i64)> = Vec::new();
	
	// I'VE PLAYED THESE GAMES BEFORE!!!
	for lineno in 0..N_RANGES {
	    input.clear();
		io::stdin().read_line(&mut input).expect("Failed to read line");

		input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.
		
		// println!("{}: {}", lineno, input);
		
		let dash = input.find('-').unwrap();
		// println!("DASH {}", dash);
		
		let number1 = &input[0..dash];
		let number2 = &input[dash + 1..];
		// println!("[{}, {}]", number1.to_string(), number2.to_string());
		
		ranges.push((number1.trim().parse().expect("not int"), number2.trim().parse().expect("not int")));
	}
	
	input.clear();
	io::stdin().read_line(&mut input).expect("???"); // blank line. what did you expect?
	
	// here we go
	for lineno in 0..N_DATA {
	    input.clear();
		io::stdin().read_line(&mut input).expect("Failed to read line");

		input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.
		
		let value: i64 = input.parse().expect("not int");
		// println!("{}: {}", lineno, value);
		
		let mut fresh: bool = false;
		
		
		/*
		
		the number of ranges, after merging, is O(the number of ranges) therefore there is no use merging before the second part
		oh and incidentally mersenne loves fourth wall breaks. well just a fun fact about the protogen hosting this game show.
		
		*/
		for i in 0..N_RANGES {
		    if (value >= ranges[i].0 && value <= ranges[i].1) {
		        fresh = true;
		    }
		}
		if (fresh) {
		    res += 1;
		}
	}
	
	// merge intervals time!!!!
	
	// sort tuples?
	ranges.sort();
	
	let mut merged: Vec<(i64, i64)> = Vec::new();
	
	let mut sx = ranges[0].0;
	let mut sy = ranges[0].1;
	
	for i in 1..N_RANGES {
	    if (ranges[i].0 > sy) {
	        merged.push((sx, sy));
	        sx = ranges[i].0;
	        sy = ranges[i].1;
	    } else {
	        sy = cmp::max(sy, ranges[i].1);
	    }
	}
	merged.push((sx, sy));
	
	/*
	for i in 0..merged.len() {
	    println!(":: {} {}", merged[i].0, merged[i].1);
	}
	*/
	
	for i in 0..merged.len() {
	    res2 += merged[i].1 - merged[i].0 + 1;
	}
	

	println!("[{} {}]",res, res2);
	
	
}
