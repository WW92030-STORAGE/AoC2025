use std::io;
use std::cmp;

/*

Suppose there are N protogens (sentient cyborg anthropomorphic animal hybrids with visor-like faces) in an enclosed space. Protogens performing the same task form groups, however initially all protogens are performing different tasks. Now, let’s say whenever a protogen (or group) completes a task, you assign the aforementioned group (or individual) to do a different one, combining the two aforementioned parties into a larger group. 

This raises several problems: How do we efficiently combine two groups? How do we efficiently calculate, say, at some arbitrary time, the number of protogens working on a specific task? How do we efficiently check if two protogens are in the same group? 

One way to do this, is to loop through the entire array, and check the values. But this takes O(n) time. What we can do, however, is speed this up to O(log n) time. The way to do this is by using a Disjoint Set Union (DSU). 

The Disjoint Set Union is similar to the way a hashmap works: Each “disjoint set” has a representative, which can always be calculated for all elements. Additionally, we also track the size of each group. Technically, all elements, including non-representatives, have a size parameter, but subset sizes are stored at the representatives.

Each element has two values tied to it: the size and the parent value. By doing so, we create a tree structure of the subsets. The parent of a representative is itself.

To find the representative of some element, we continually go to the parent of our current location, until our parent is equal to our location. When that happens, we return the current element as the representative.

To test whether two elements are in the same set, we simply check if their representatives are equal.

When we first create the DSU, each element is their own representative, and all group sizes are equal to one.

To merge two sets, we simply combine them by the representatives. If the two subsets were of equal size, then we can do in any order. Otherwise, we take the smaller subset, and make the parent of that set’s representative equal to the representative of the larger subset. We also increase the size of the larger subset.

Due to the way these operations are performed, the runtime of queries thus becomes O(log n), since the maximum length of the path from the representative to an element is O(log n).

*/

// oh boy time for structs. Like in C, you can't put methods directly in structs.
// But unlike C, you can use methods easily, except defined outside of structs in an IMPL section.

struct DSU {
    N: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
}


impl DSU {
    // Use &self or &mut self as a parameter for referencing, and then self as the victim.
    
    fn init(&mut self, n: usize) {
        self.N = n;
        self.parent = vec![0; self.N];
        self.size = vec![1; self.N];
        for i in 0..self.N {
            self.parent[i] = i;
        }
    }
    
    // Func parameters can also be mutable!
    fn find(&self, mut val: usize) -> usize {
        while (self.parent[val] != val) {
            val = self.parent[val];
        }
        return val;
    }
    
    // merges the sets containing (a) and (b) together. returns success status and final root of the merged trees
    fn merge(&mut self, a: usize, b: usize) -> (bool, usize) {
        let r1: usize = self.find(a);
        let r2: usize = self.find(b);
        
        if (r1 == r2) {
            return (false, 0);
        }
        
        let s1: usize = self.size[r1];
        let s2: usize = self.size[r2];
        
        // merge r2 into r1
        if (s1 > s2) {
            self.parent[r2] = r1;
            self.size[r1] += s2;
            return (true, r1);
        } else { // merge r1 into r2
            self.parent[r1] = r2;
            self.size[r2] += s1;
            return (true, r2);
        }
    }
}

fn main() {
	// standard input with multiple lines and header of one count line count count count line
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
let NLINE:
	usize = input.trim().parse().expect("Input not an integer"); // Handle the parse error

	println!("N = {}", NLINE);

	let mut res = 0;
	let mut res2 = 0;
	
	let mut points: Vec<(i64, i64, i64)> = Vec::new();
	
	for lineno in 0..NLINE {
	    input.clear();
		io::stdin().read_line(&mut input).expect("Failed to read line");

		input = input.trim_end().to_string(); // trim the bad stuff. unfortunately it returns a reference so must convert again.
		
		// println!("{}: {}", lineno, input);
		
		let values: Vec<&str> = input.split(",").collect();
		
		let x0: i64 = values[0].parse().expect("not int");
		let x1: i64 = values[1].parse().expect("not int");
		let x2: i64 = values[2].parse().expect("not int");
		points.push((x0, x1, x2));
	}
	
	// Sorted list of edges. Weight comes first as primary key
	let mut edges: Vec<(i64, usize, usize)> = Vec::new();
	
	for i in 0..NLINE {
	    for j in 0..i {
	        let dx: i64 = points[i].0 - points[j].0;
	        let dy: i64 = points[i].1 - points[j].1;
	        let dz: i64 = points[i].2 - points[j].2;
	        let rsq: i64 = dx * dx + dy * dy + dz * dz;
	        
	        edges.push((rsq, i, j));
	    }
	}
	
	edges.sort();
	
	// DSU!!!
	 
	let mut dsu = DSU {N: 0, parent: Vec::new(), size: Vec::new()};
	dsu.init(NLINE);
	
	// Compute the first 1000 (or whatever) collisions
	let N_CONNECTIONS = 1000;
	
	for i in 0..N_CONNECTIONS {
	    if (i >= edges.len()) {
	        break;
	    }
	    dsu.merge(edges[i].1, edges[i].2);
	}
	
	/*
	for i in 0..dsu.size.len() {
	    println!("{}: {}", i, dsu.size[i]);
	}
	*/
	
	// print out the top 3 sizes
	dsu.size.sort();
	res = 1;
	for i in 0..3 {
	    res = res * dsu.size[dsu.size.len() - i - 1];
	}
	
	// PART II
	dsu.init(NLINE);
	
	for i in 0..edges.len() {
	    let status: (bool, usize) = dsu.merge(edges[i].1, edges[i].2);
	    if (dsu.size[status.1] == NLINE) {
	        res2 = (points[edges[i].1].0 * points[edges[i].2].0);
	        break;
	    }
	}
	
	println!("[{} {}]",res, res2);
}


/*
	
In the universe known as Zenith's Outer Reach there is a certain interplanetary network containing N planets. A government wishes to build a series of N - 1 portals between the planets such that all the planets are connected together. 

However, due to economic and political reasons, the cost to connect planets might vary for different planets. It is known the finite cost of connecting any two planets together. So how can the government efficiently construct the N - 1 portals between the planets?

We can represent the planets as nodes and pairs of planets as edges in a graph, where edge weight is the cost of connecting two planets. Our goal is thus to find a SPANNING TREE of minimum total weight cost. 

What is a spanning tree? Well, a tree is a type of graph that connects N nodes such that all nodes are connected to each other and there are no cycles. It is also known that such a graph contains N - 1 edges, and that any graph with N connected nodes and N - 1 edges is a tree.

Now, for a collection of N nodes and the edges E between them, a spanning tree T is a subset T ⊂ E of N - 1 edges that form a tree between the N nodes. There can exist multiple spanning trees. However, we are trying to find one with minimum cost.

So how can we do that? Well, this relies on knowledge of one key property: if you take a spanning tree, then if you add an edge e ∈ E - T (i.e. not originally in T) you get a cycle in T ∪ {e} that contains e. 

Now, suppose you took any edge e' in this cycle and removed it to create a new graph T' of N - 1 edges. We claim that T' is also a spanning tree.

In a tree, there are no cycles, so between any two nodes is a single unique simple path (one that does not repeat edges). Now, from T to T ∪ {e}, either the path has edges on the cycle or it does not. 

When we remove the edge e', if the original path did not have cycle edges then the new path does not either, and there is still a single path between the two nodes. Otherwise, creating the cycle created two paths between the nodes, traversing either "side" of the cycle.

So if we remove e' one of those paths is removed however the other one still remains. A path remains between the two nodes.

And we see that all the nodes are still connected in T' and T' has N - 1 edges so T' is still a spanning tree.

Now, what does this mean? Suppose you had a spanning tree T ⊂ E. Now consider a cycle C ∈ E where all but a single edge of C are present in T. Now, suppose that we sorted the edges by weight and found the maximum weight edge X.

If X ∈ C then if we take this edge, remove it, and put in the missing edge (Y ∈ C - T) to create T', we get another spanning tree with lower weight sum than T since we removed X and added Y which has a lower weight cost. Therefore this gives rise to the following proposition:

Cycle property: if C is a cycle in G, and X is the highest weight edge in C, then the minimum spanning tree (MST) cannot contain X. We saw how a cycle missing one edge is affected by this property, and this generalizes to cycles with multiple edges not in the tree.

So how do we exploit this? We sort the edges by weight, then look at the edges. Edges are added one at a time to a growing FOREST (collection of trees). 

If an edge has endpoints that are already connected in our forest, thus forming a cycle with that edge being the highest weighted edge in said cycle, we skip over it and move on. Otherwise, we add the edge to the forest, connecting two existing trees into one larger tree.

Obviously, this calls for a DSU because that allows to easily check if two elements are in the same set (tree) and combine two sets (trees) together.

So our implementation goes as follows (once again N = |V| and M = |E|):

KRUSKAL(G = (V, E), w : E → R):
  Let DSU contain N nodes, each a separate singleton set.
  Let T = ∅ 
  Sort E ascending by weight.
  FOR e = (u, v) ∈ E:
    if FIND(DSU, u) == FIND(DSU, v): continue
    else: T = T ∪ {e} and UNION(DSU, u, v)
    if |T| ≥ N - 1: break
  return T

What's the runtime? There are M edges to consider, and each iteration of the FOR loop takes O(log N) time for the DSU operations. Therefore our runtime is O(M log N) = O(E log V). There exist other algorithms that have different runtimes however they work in other ways.

	
*/
