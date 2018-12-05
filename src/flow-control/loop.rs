fn loop_control() -> i32 {
	let mut i = 0;
	let result = loop {
		if i > 5 {
			break i * 2;
		}
		println!("loop");
		i += 1;
	};
	result
}
