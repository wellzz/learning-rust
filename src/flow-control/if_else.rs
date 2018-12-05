pub fn if_else(){
	let n = 5;
	if n>0 {
		println!("{} is negative", n)
	}else if n > 0 {
		println!("{} is positive", n)
	}else {
		println!("{} is zero", n)
	}

	let a = 
		if n> 10 {
			n*100
		}else{
			n*1000
		};
		println!("a is {}", a);
}