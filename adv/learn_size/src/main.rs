use std::mem;

struct A<'a> {
	_a: i32, //4 bytes;
	_b: &'a [u8],
}

trait MyTrait {
	fn test();
}

struct Foo;

struct Baz {
	_foo: Foo,
	_qux: (),
	_baz: [u8; 0],
}

enum Void {}

fn main() {
	let array: [u8; 10] = [1; 10];
	let s = &array[..];

	println!("s size = {}", mem::size_of_val(s));
	println!("&s size = {}", mem::size_of_val(&s));
	println!("&i32 size = {}", mem::size_of::<&i32>());
	println!("&i64 size = {}", mem::size_of::<&i64>());
	println!("i32 size = {}", mem::size_of::<i32>());
	println!("i64 size = {}", mem::size_of::<i64>());
	println!("A size = {}", mem::size_of::<A>());
	println!("&A size = {}", mem::size_of::<&A>());

	//println!("[u8] size = {}", mem::size_of::<[u8]>());
	println!("&[u8] size = {}", mem::size_of::<&[u8]>());

	//println!("MyTrait size = {}", mem::size_of::<MyTrait>());
	//println!("&MyTrait size = {}", mem::size_of::<&MyTrait>());
	
	println!("Foo size = {}", mem::size_of::<Foo>());
	println!("Baz size = {}", mem::size_of::<Baz>());

	println!("Void size = {}", mem::size_of::<Void>());

    println!("Hello, world!");
}
