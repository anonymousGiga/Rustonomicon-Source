use std::panic;

fn main() {
	//let ret = panic::catch_unwind(|| {
	//	println!("hello!");
	//});
	//assert!(ret.is_ok());

	//let ret = panic::catch_unwind(|| {
	//	panic!("this is a panic!");
	//});
	//assert!(ret.is_err());

	panic!("============================");
	panic!("this is a panic!");
	panic!("============================");

    println!("Hello, world!");
}
