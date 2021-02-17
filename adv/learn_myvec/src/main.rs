use std::marker::PhantomData;

//定义一个Unique类型，满足如下条件：
//对T可变;
//拥有类型T的值
//如果T是Send/Sync的，那么Unique也是Send/Sync的;
//指针永远不为null
struct Unique<T> {
	ptr: *const T, //指针不可变，指针只想的内容可变
	_marker: PhantomData<T>,  //用于drop检查，如果不加，编译器会认为结构体没有拥有类型T
}

unsafe impl<T: Send> Send for Unique<T> {}
unsafe impl<T: Sync> Sync for Unique<T> {}

impl<T> Unique<T> {
	const unsafe fn new_unchecked(ptr: *mut T) -> Self {
		unsafe {
			Unique {
				ptr: ptr, 
				_marker: PhantomData
			}
		}
	}

	fn new (ptr: *mut T) -> Option<Self> {
		if !ptr.is_null() {
			Some(unsafe { 
					Unique { 
						ptr: ptr, 
						_marker: PhantomData
					}
			})
		} else {
			None
		}
	}

	fn as_ptr(&self) -> *mut T {
		self.ptr as *mut T
	}
}

struct MyVec<T> {
	//ptr: *mut T,
	ptr: Unique<T>,
	cap: usize,
	len: usize,
}

fn main() {
    println!("Hello, world!");
}
