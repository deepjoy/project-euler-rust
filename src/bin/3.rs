fn main() {
	let mut num = 600851475143u;
	let mut counter = 1u;
	let mut largest_prime = 1u;
	loop {
		if counter > num {
			break;
		}
		if num % counter ==0 {
			num /=counter;
			largest_prime = counter;
		}
		counter = counter + 1;
	}
  println!("{}", largest_prime );
}
