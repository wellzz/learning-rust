#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
	// `&'static str` 是一个指向分配在只读内存区的字符串的引用
	author: &'static str,
	title: &'static str,
	year: u32,
}

// 此函数接受一个指向图书 Book 的引用
fn borrow_book(book: &Book) {
	println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// 此函数接受一个指向可变的图书 Book 的引用，同时把年份 `year` 改为 2004 年
fn new_edition(book: &mut Book) {
	book.year = 2014;
	println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn borrow_mut() {
	// 创建一个名为 `immutabook` 的不可变的图书 Book
	let immutabook = Book {
		// 字符串字面量拥有 `&'static str` 类型
		author: "Douglas Hofstadter",
		title: "Gödel, Escher, Bach",
		year: 1979,
	};

	// 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
	let mut mutabook = immutabook;
	
	// 不可变地借用一个不可变对象
	borrow_book(&immutabook);

	// 不可变地借用一个可变对象
	borrow_book(&mutabook);
	
	// 借用一个可变对象作为可变类型
	new_edition(&mut mutabook);
	
	// 报错！不能借用一个不可变对象来充当可变类型
	new_edition(&mut immutabook);
	// 改正 ^ 注释掉此行
}