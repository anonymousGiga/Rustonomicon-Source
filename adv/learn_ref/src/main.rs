//在Rust中，存在两种引用类型，分别是：
//1、引用
//2、借用（也就是可变引用）

//遵循规则：
//
//1、引用的生命周期不能超过被引用的内容（原因：Rust中内存在拥有它的变量离开作用域后就被自动释放）
//2、可变引用不能存在别名

fn main() {
	//1、引用的生命周期不能超过被引用的内容
	let a = String::from("This is a !");
	let mut b = &a;
	{
		let c = String::from("This is c !");
		b = &c;
		println!("reference b is c: {}", b);
		//println!("reference b is c: {}", *b);   //b <=> *b
	}
	//println!("reference b is c: {}", b);
	
    println!("Hello, world!");
}

////考虑如下函数
//fn compute(input: &u32, output: &mut u32) {
//	if *input > 10 {
//		*ouput = 1;
//	}
//	
//	if *input > 5 {
//		*ouput = 2;
//	}
//}
//
////编译器优化，会怎么优化？
////可能的方式：
//*output = 0;
//fn compute(input: &u32, output: &mut u32) {
//	//input == output == 0xabd1233
//	//*input == *output == 15
//	let cached_input = *input;
//	if cached_input > 10 { //true
//		*output = 1;	//*input == *ouput == 1
//	} 
//	if cached_input > 5 {//false
//		*output = 2;
//	}
//}
////上述优化正确的条件：input和output不能是同一个东西
////上出函数正常的期望，希望给定一个input，返回一个output，而不是将我们的input也修改了
//
//
