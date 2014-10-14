fn main() {
    vectors();
}

fn vectors() {
	let mut nums  = vec![1i, 2i, 3i];
	let nums2 = [1i, ..20];

	nums.push(4i); // works

	let nums_arr = [1i, 2i, 3i];
	// nums_arr.push(4i); <-- won't work

	let slice = nums.as_slice();

	for i in nums2.iter() {
		println!("{}", i);
	}

	let names = ["Grayson", "Brian", "Niko"];
	println!("The second name is: {}", names[1]);
}
