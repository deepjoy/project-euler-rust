fn is_palindromic(num: uint) -> bool {
    let num_str = num.to_string();
    let rev_num_str: String = num_str.as_slice().chars().rev().collect();
    return rev_num_str == num_str;
}
fn main() {
	let mut max_num = 0u;
	for x in range(100u, 1000).rev() {
		for y in range(100u, 1000).rev() {
			let current_val = x * y;
			if  current_val < max_num {
				break;
			}
			if is_palindromic(current_val) {
				max_num = current_val;
			}
		}
	}
	println!("{}", max_num);
}
