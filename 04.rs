/*

What kind of algorithm is this?
This is brute force. Useful for trivial scenarios.

What kind of algorithm is this?
This is recursion. Useful for making things readable, and for problems with many layers.

What kind of algorithm is this?
This is dynamic programming. It's like recursion but it remembers things to avoid repetition.

What kind of algorithm is this?
This is [Hyperlink Blocked]. It's used in [indiscernible]

*/

use std::io;

use std::cmp; // !!!!!!!!!!!!!!!!!1

// use the arrow provider to specify the type of return
fn inBounds(grid: &Vec<Vec<bool>>, r: i32, c: i32) -> bool {
    if (r < 0 || c < 0) {
        return false;
    } else if (r >= grid.len() as i32) {
        return false;
    } else if (c >= grid[r as usize].len() as i32) {
        return false;
    }
    return true;
}

fn main() {
	// standard input with multiple lines and header of one count line count count count line
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
let number:
	i32 = input.trim().parse().expect("Input not an integer"); // Handle the parse error

	println!("N = {}", number);
	
	let R: i32 = number;

	let mut res = 0;
	let mut res2 = 0;
	
	let mut grid: Vec<Vec<bool>> = Vec::new();
	for lineno in 0..number {
	    input.clear();
		io::stdin().read_line(&mut input).expect("Failed to read line");

		input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.
		
		println!("{} {}", lineno, input);
		
		grid.push(vec![false; input.len()]);
		
		let input_bytes = input.as_bytes();
		for i in 0..input.len() { // the type of this kind of range thing is given by the types of the endpoints
		    grid[lineno as usize][i] = input_bytes[i] == '@' as u8; // this isn't C anymore, this is RUST! where there is no such thing as auto typecasting
		}
	}
	
	let dx: Vec<i32> = vec![1, 1, 0, -1, -1, -1, 0, 1];
	let dy: Vec<i32> = vec![0, 1, 1, 1, 0, -1, -1, -1];
	
	// println!("R {}", R);
	
	for r in 0..R {
	    let C = grid[r as usize].len() as i32; // i love the typecasting lmao
	    // println!("C {}", C);
	    for c in 0..C {
	        // println!("{} {} {}", r, c, grid[r as usize][c as usize]);
	        if (grid[r as usize][c as usize]) {
	            let mut dcount = 0;
	            for d in 0..8 {
	                let rprime = r + dx[d];
	                let cprime = c + dy[d];
	                if (!inBounds(&grid, rprime, cprime)) {
	                    dcount += 1;
	                } else if (!grid[rprime as usize][cprime as usize]) {
	                    dcount += 1;
	                }
	            }
	            // println!("{} {} {}", r, c, dcount);
	            
	            if (dcount > 4) {
	                res += 1;
	            }
	        }
	    }
	}
	
	let mut total_cells: usize = 0;
	let mut selected: Vec<Vec<bool>> = Vec::new();
	for r in 0..R {
	    total_cells += grid[r as usize].len();
	    selected.push(vec![false; grid[r as usize].len()]);
	}
	
	for iter in 0..total_cells {
	    for r in 0..R {
	        for c in 0..selected[r as usize].len() {
	            selected[r as usize][c as usize] = false;
	        }
	    }
	    
	    let mut count = 0;
	    for r in 0..R {
	        let C = grid[r as usize].len() as i32;
	        // println!("C {}", C);
	        for c in 0..C {
	            // println!("{} {} {}", r, c, grid[r as usize][c as usize]);
	            if (grid[r as usize][c as usize]) {
	                let mut dcount = 0;
	                for d in 0..8 {
	                    let rprime = r + dx[d];
	                    let cprime = c + dy[d];
	                    if (!inBounds(&grid, rprime, cprime)) {
	                        dcount += 1;
	                    } else if (!grid[rprime as usize][cprime as usize]) {
	                        dcount += 1;
	                    }
	                }
	                // println!("{} {} {}", r, c, dcount);
	            
	                if (dcount > 4) {
	                    selected[r as usize][c as usize] = true;
	                    count += 1;
	                }
	            }
	        }
	    }
	    
	    res2 += count;
	    for r in 0..R {
	        for c in 0..selected[r as usize].len() {
	            if (selected[r as usize][c as usize]) {
	                grid[r as usize][c as usize] = false;
	            }
	        }
	    }
	    if (count <= 0) {
	        break;
	    }
	}
	

	println!("[{} {}]",res, res2);
}
