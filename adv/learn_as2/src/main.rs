//显式类型转换是强制类型转换的超集，所有的强制类型转换都可以通过显式转换的方式主动触发。某些场景只适用于显式类型转换。
//显式类型转换的方式是使用关键字as，如：expr as Type。
//显式类型转换必须在类型层面是合法的，否则在编译时会报错。
//显式类型转换本身不属于非安全行为。
//显式类型转换不可传递，例子：x as y as z合法，不代表x as z合法。
//其它注意事项。


fn print(x: &i32) {     //类型转换：&mut i32 -> &i32
	println!("x = {}", x);
}

fn main() {
	let x: &mut i32 = &mut 10;
	print(x);
	print(x as &i32);

	let s: String = "hello".to_string();
	//let _i = s as i8;
    println!("Hello, world!");
}
