//赋值的时候（绑定变量），如果之前被初始化过，那么需要先析构，再覆盖;如果没有被初始化过，那么直接覆盖
#[derive(Debug)]
struct A{
	name: &'static str,
}

impl Drop for A {
	fn drop(&mut self) {
		println!("Dropping {}", self.name);
	}
}

fn main() {
	{
		let a = A{ name: "aa" }; //a未初始化，直接覆盖
		println!("1----------------");
		let b = a;              //b未被初始化，直接覆盖

		println!("2----------------");
		let mut c = A{name: "cc"};   //c未被初始化，直接覆盖
		println!("c == {:?}", c);

		println!("3----------------");
		c = b;                 //c之前已经被初始化了，需要先回收，然后再用b绑定的值去覆盖
							   //此处应该调用cc的drop函数,c绑定的就是之前aa
		println!("c == {:?}", c);
		println!("4----------------");
	    //调用aa的析构函数	
	}
	println!("At end of main");
}
