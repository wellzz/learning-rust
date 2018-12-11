pub fn create_box(){
	// 在堆上分配一个整型数据
	let _box1 = Box::new(3i32);

	// `_box1` 在这里销毁，而且内存得到释放
}