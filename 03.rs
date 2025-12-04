/*

Welcome back to Electrical Enigma! I'm Mersenne and this is my wonderful cohost Draki. [Hi everyone!]

Today we have a very special line of batteries. It has 14 joltages of the form: [63246551221447]

*/

use std::io;

use std::cmp; // !!!!!!!!!!!!!!!!!1

fn main() {
	// standard input with multiple lines and header of one count line count count count line
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
let number:
	i32 = input.trim().parse().expect("Input not an integer"); // Handle the parse error

	println!("N = {}", number);

	let mut res = 0;
	let mut res2 = 0;
	for lineno in 0..number {
	let mut maxjoltage = 0;
	input.clear(); // read_line appends, so clear to reset the string value
		io::stdin().read_line(&mut input).expect("Failed to read line");

		input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.

		// println!("{}", (input.chars().nth(0).unwrap() as i32)); // cast to int for ascii value

let LEN:
		usize = input.len();
		/*
		let mut clist = input.chars(); // two protobeans are fighting over a stick of RAM. the RAM stick does not remvain constanst.

		println!("{0} : {1}", input, LEN);
		for i in 0..LEN {
		    for j in 0..i {
		        println!("{}", clist.as_str()); // you can use as_str to convert a chars iterator result to string
		        println!("- {} {}", j, i); // without the numbers it goes in order
		        let c1 = (clist.nth(j).unwrap() as i32) - 48;
		        let c2 = (clist.nth(i).unwrap() as i32) - 48;
		        println!("> {} {}", c1, c2);
		    }
		}
		*/

		/*

		HAHAHAH HAHAHAHAHAHAAHAHA!!!11 You thought this was going to be easy???
		Little do you know that the NTH operator actually must first [laughter] SKIP OVER THE FIRST N ELEMENTS!!!

		This means that it is actually [hiccups from laughter] O(N) instead of O(1) as you previously thought!!!!!
		hahahahashasasahahaha

		if you wanted to acakwtual y the nath character of a byte stringsss [hic] you use the as_bytes lmao malmamlmao lmao
		then you can actually use the [bracket] operations!!!! hahaahahahhehehehehehe
		[maniacal laughter as Mersenne walks off into the distance]

		-Mersenne the Protogen

		*/

		let mut clist = input.as_bytes(); // two protobeans are fighting over a stick of RAM. the RAM stick does not remvain constanst.

		// println!("{0} : {1}", input, LEN);
		for i in 0..LEN {
		for j in 0..i {
			// println!("- {} {}", j, i); // without the numbers it goes in order
			let c1 = (clist[j] as i32) - 48;
				let c2 = (clist[i] as i32) - 48;

				let test = c1 * 10 + c2;
				if test > maxjoltage { // you don't need the parens
				maxjoltage = test;
			}
		}
	}

	res += maxjoltage;

	/*

	Mersenne just came back from a very hilarious comedy show, he's a bit drunk on humor sorry about that.
	Let's see ... we need the maximum subsequence right?

	Let's use dynamic programming. Denote max[index][len] to be the maximum subsequence generated with length (len), and ending at [index]
	then max[index][len] = clist[len] + 10 * max(ii : 0 .. index) max[ii][len - 1]

	base cases: max[index < len - 1][len] = 0

	-Draki

	*/

	// Time for arrays

	let MAX_LEN = 12;

    let mut dp:
	Vec<Vec<i64>> = Vec::new();
		for i in 0..(LEN + 1) {
			dp.push(vec![0; MAX_LEN + 1]); // Use vec![val; size] to initialize a vec of size (size) each entry (val)
		}
		
		let mut maxjoltage: i64 = 0;

		// the minimum index for which a length has nonzero value is length - 1, for which its value is literally the substring up until that point
		// println!("{}", input);
		for len in 1..(MAX_LEN + 1) {
			dp[len - 1][len] = (&input[0..len]).trim().parse::<i64>().unwrap(); // For long numbers or whatever parse::<type> works here
			// println!("{} {} {}", len - 1, len, dp[len - 1][len]);
			for index in (len)..LEN {
			    // max[index][len] = clist[len] + 10 * max(ii : 0 .. index) max[ii][len - 1]
			    for ii in (0)..index {
			        dp[index][len] = cmp::max(dp[ii][len - 1], dp[index][len]); // cmp provides the max and mins
			    }
			    dp[index][len] = 10 * dp[index][len] + (clist[index] as i64 - 48);
			    
			    // println!("{} {} {}", index, len, dp[index][len]);
			    
			    if (len == MAX_LEN) {
			        maxjoltage = cmp::max(maxjoltage, dp[index][MAX_LEN]);
			    }
			}
			
		}
		
		res2 += maxjoltage;
		
	}
	println!("[{} {}]",res, res2);
}
