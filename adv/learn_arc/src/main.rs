use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time;

fn main() {
	let data = vec![1, 2, 3, 4];  //创建了一个变量，在堆上分配内存
	let idx = Arc::new(AtomicUsize::new(0)); //创建原子变量
	let other_idx = idx.clone(); //获取引用

	thread::spawn(move || {
		other_idx.fetch_add(10, Ordering::SeqCst);//对这个变量加10: idx += 10;
	});

	//let ten_millis = time::Duration::from_millis(10);
	//thread::sleep(ten_millis);
	
	println!("{}", data[idx.load(Ordering::SeqCst)]);  //idx = 0, data[idx] = 1
													   //idx = 10, data[idx]越界
}
