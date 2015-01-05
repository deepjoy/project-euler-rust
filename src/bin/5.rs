fn gcd(a:uint, b:uint) -> uint {
    if b == 0 {
       return a;
		}    else {
       return gcd(b, a % b)
		}
}
fn main() {
	let mut result = 1u;
	for x in range(1u,20) {
		let hcf:uint = gcd(x,result);
		result = result * (x / hcf);
	}
  println!("{}", result);
}
