pub fn add_three_times_four(x: int) -> int {
	times_four(add_three(x))
}

fn add_three(x: int) -> int {
	x + 3
}

fn times_four(x: int) -> int {
	x * 4
}

#[cfg(test)]
mod test {
	use super::add_three;
	use super::times_four;

	#[test]
	fn add_three_five_is_eight() {
		let result = add_three(5i);

		assert_eq!(8i, result);
	}

	#[test]
	fn times_four_five_is_twenty() {
		let result = times_four(5i);

		assert_eq!(20i, result);
	}
}