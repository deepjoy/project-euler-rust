use std::mem;

struct Fibonacci {
    curr: uint,
    next: uint,
}

// Implement 'Iterator' for 'Fibonacci'
impl Iterator<uint> for Fibonacci {
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = mem::replace(&mut self.next, new_next);

        // 'Some' is always returned, this is an infinite value generator
        Some(mem::replace(&mut self.curr, new_curr))
    }
}

// Returns a fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
		let limit = 4000000u;
		let mut gen = fibonacci();
		let mut sum = 0u;
		loop {
			let val = gen.next().unwrap_or_default();
			if val > limit {
				break;
			}
			if val %2 ==0 {
				sum += val;
			}
		}
		  println!("{}", sum);
}