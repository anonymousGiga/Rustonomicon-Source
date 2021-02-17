fn print(x: &i32) { //传入进入的时候会发生转换   &mut i32 -> &i32
	println!("x = {}", x);
}

fn main() {
	let x: &mut i32 = &mut 10;
	print(x);
	print(x as &i32);  //转换也可以通过显式的方式主动触发

	let a: i8 = 1;
	let b: i32 = a as i32;
	println!("b = {}", b);
    println!("Hello, world!");
}
