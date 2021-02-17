struct Foo {
	a: i32,
	b: i32,
	c: i32,
}

fn main() {
	let mut x = Foo {a: 1, b: 2, c: 3};
	let a = &mut x.a;
	let b = &mut x.b;
	let c = &x.c;
	*a += 1;	
	*b += 1;
	let c1 = &x.c;
	println!("a = {}, b= {}, c= {}, c1 = {}", a, b, c, c1);


	//slice 或者数组
	//为什么报错：Rust的mut检查实现的很简单，对于struct的每个成员单独借用是可以的，但是对数组slice的每个元素借用，就算多次
	//如果要分别借用，解决方法：使用unsafe
	let mut x = [1, 2, 3];
	let a = &mut x[0];
	let b = &mut x[1];
	println!("a = {}, b = {}", a, b);
    println!("Hello, world!");
}
