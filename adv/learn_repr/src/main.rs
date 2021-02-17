use std::mem;

struct A {
	_a: u8,//1 bytes
	//padding 3 bytes
	_b: i32, //4 bytes
	_c: u8, //1 bytes
	//padding 3 bytes
}

#[repr(C)]
struct B {
	_a: u8,//1 bytes
	//padding 3 bytes
	_b: i32, //4 bytes
	_c: u8, //1 bytes
	//padding 3 bytes
}

//演示零尺寸大小类型
struct C;
struct D;
#[repr(C)]
struct E {
	_c: C,
	_d: D,
}


//枚举类型
#[repr(C)]
enum MyType{
	First,
	_Second,
}

//enum MyType{
//	First(String),
//	Second(int),
//}

fn main() {
	let _a = MyType::First;
	//let mut b: i32 = 0;
	//b = a;

	println!("size of A is {}", mem::size_of::<A>());
	println!("size of B is {}", mem::size_of::<B>());
	println!("size of E is {}", mem::size_of::<E>());
    println!("Hello, world!");
}
