// 此函数取倒堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
	println!("Destroying a box that contains {}", c);

	// `c` 被销毁且内存得到释放
}

fn ownership() {
	// 栈分配的整型
	let x = 5u32;

	// 将 `x` **复制**（*Copy*）到 `y`——不存在资源移动
	let y = x;

	// 两个值都可以独立地使用
	println!("x is {}, and y is {}", x, y);

	// `a` 是一个指向堆分配的整型的指针
	let a = Box::new(5i32);

	println!("a contains: {}", a);

	// **移动**（*Move*) `a` 到 `b`
	let b = a;
	// 把 `a` 的指针地址（非数据）复制到 `b`。现在两者都是指向
	// 同一个堆分配的数据，但是现在是 `b` 占有它。
	
	// 报错！`a` 再也不能访问数据，因为它不再拥有堆上的内存。
	//println!("a contains: {}", a);
	// 试一试 ^ 将此行注释去掉

	// 此函数从 `b` 中取得栈分配的内存的所有权
	destroy_box(b);

	// 此时堆上的内存已经释放掉，而这个操作会导致解引用已释放的内存，
	// 但这种情况会被编译器会禁止。
	// 报错！和前面出错的原因一样。
	//println!("b contains: {}", b);
	// 试一试 ^ 将此行注释去掉
}
