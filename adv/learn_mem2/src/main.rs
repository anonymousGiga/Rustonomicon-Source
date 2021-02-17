fn main() {
	//如果值从变量中移动出来，并且变量的类型不是Copy，那么变量就会处于未初始化的状态
	let x = 0;
	let mut y = Box::new(0); //引用类型
	//let z1 = x;
	let z2 = y; //把值从y中移动出来，绑定到z2,那么y此时处于未初始化的状态

	println!("x = {}", x);
	y = Box::new(1);
	println!("y = {}", y); //y未被初始化，所以此处不可用
	//println!("z1 = {}", z1);
	println!("z2 = {}", z2);
}
