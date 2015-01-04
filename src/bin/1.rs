fn main() {
	let mut sum = 0u;
	for x in range(0u, 1000u) {
		if  x %3 ==0 || x%5 ==0  {
			sum += x;
		}
	}
  println!("{}", sum );
}
