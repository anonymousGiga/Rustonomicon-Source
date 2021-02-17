use std::mem;

#[repr(packed)]
struct Foo {
	bar: u8,
}

#[derive(Debug)]
struct Bar {
	a: u8,
}

fn main() {
	let array = [Bar{a: 10}];
	unsafe {
		//let mut f: Foo = mem::transmute::<[Bar; 1], Foo>(array);
		let mut f: Foo = mem::transmute_copy(&array);
		assert_eq!(f.bar, 10);

		f.bar = 20;
		assert_eq!(f.bar, 20);
	}
	println!("array = {:#?}", array);
    println!("Hello, world!");
}
