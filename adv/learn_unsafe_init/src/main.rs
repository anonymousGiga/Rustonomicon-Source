use std::mem;
use std::ptr;

fn main() {
	//let x :String;
	//x = mem::uninitialized();

	//引用类型，如果变量是初始化过的，那么需要先调用drop，然后再覆盖
	//所以，此时对x重新绑定的话，需要用以下三个函数：
	//ptr::write(ptr, val)
	//ptr::copy(src, dest, count)
	//ptr::copy_nonoverlapping(src, dest, count)
	
	const SIZE: usize = 3;
	let mut x: [Box<u32>; SIZE];
	////x = [Box::new(0), Box::new(1), Box::new(2)]; //对x初始化
	//let y = [Box::new(0), Box::new(1), Box::new(2)];
	////x = y; //先对x之前的内容销毁，然后再用y对应内容去覆盖
	
	unsafe {
		x = mem::uninitialized(); //骗过编译器，x被初始化了
		//x = y; //先对x之前的内容销毁，然后再用y对应内容去覆盖

		for i in 0..SIZE {
			ptr::write(&mut x[i], Box::new(i as u32));//直接去覆盖，不用先回收
		}
	}

	println!("x = {:?}", x);
	
}

	//let x = [1; 3];
	//let y = [1, 2, 3];

	//// c语言中
	//int a[2];
	//for (int i = 0; i < 2; i++) {
	//	a[i] = i;
	//}

	//mem::uninitialized
	//创建变量之后，没有初始化，然后用mem::uninitialized可以骗过编译器，让编译器认为我们已经对应变量初始化了，这样编译就能通过
