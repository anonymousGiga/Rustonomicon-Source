#![feature(ptr_internals)]
use std::ptr::{Unique, self};
use std::mem;
use std::alloc::{alloc, realloc, dealloc, Layout, handle_alloc_error};

struct MyVec<T> {
	ptr: Unique<T>,
	cap: usize,
	len: usize,
}

impl<T> MyVec<T> {
	fn new() -> Self {
		assert!(mem::size_of::<T>() != 0, "先不处理零尺寸类型");
		MyVec {
			ptr: Unique::dangling(),
			len: 0,
			cap: 0,
		}
	}

	//if cap == 0:
	//	allocate()
	//	cap = 1
	//else:
	//  reallocate()
	//  cap = cap*2
	fn grow(&mut self)	{
		unsafe {
			let align = mem::align_of::<T>();
			let elem_size = mem::size_of::<T>();
			let layout: Layout;

			let (new_cap, ptr) = if self.cap == 0 {
				layout = Layout::from_size_align_unchecked(elem_size, align);
				let ptr = alloc(layout);
				(1, ptr)
			} else {
				let new_cap = self.cap * 2;
				let old_num_bytes = self.cap * elem_size;
				
				assert!(old_num_bytes <= (isize::MAX as usize)/2,
						"capacity overflow");
				let new_num_bytes = old_num_bytes * 2;
				layout = Layout::from_size_align_unchecked(new_num_bytes, align);
				let ptr = realloc(self.ptr.as_ptr() as *mut _,
								layout,
								new_num_bytes);
				(new_cap, ptr)
			};

			if ptr.is_null() {
				handle_alloc_error(layout);
			}

			if let Some(ptr) = Unique::new(ptr as *mut _) {
				self.ptr = ptr;
			} else {
				panic!("error");
			}

			self.cap = new_cap;
		}
	}

	//push
	fn push(&mut self, elem: T) {
		if self.len == self.cap {
			self.grow();
		} 

		unsafe {
			//ptr = ptr + offset
			//*ptr = elem	
			ptr::write(self.ptr.as_ptr().offset(self.len as isize), elem);
		}

		self.len += 1;
	}

	//pop
	fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			None
		} else {
			self.len -= 1;
			unsafe {
				Some(ptr::read(self.ptr.as_ptr().offset(self.len as isize)))
			}
		}
	}

	//insert
	fn insert(&mut self, index: usize, elem: T) {
		assert!(index <= self.len, "out of bound");
		if self.cap == self.len {
			self.grow();
		}	
		
		unsafe {
			if index < self.len {
				ptr::copy(self.ptr.as_ptr().offset(index as isize),
						self.ptr.as_ptr().offset(index as isize + 1),
						self.len - index);
			}
			ptr::write(self.ptr.as_ptr().offset(index as isize), elem);
			self.len += 1;
		}
	}
	
	//remove
	fn remove(&mut self, index: usize) -> T {
		assert!(index <= self.len, "out of bound");
		unsafe {
			self.len -= 1;
			let result = ptr::read(self.ptr.as_ptr().offset(index as isize));
			ptr::copy(self.ptr.as_ptr().offset(index as isize + 1),
					self.ptr.as_ptr().offset(index as isize),
					self.len - index);
			result
		}
	}

	//into_iter
	fn into_iter(self) -> IntoIter<T> {
		let ptr = self.ptr;
		let cap = self.cap;
		let len = self.len;

		//确保Vec不会被drop，因为那样会释放内存空间
		mem::forget(self);

		unsafe{
			IntoIter {
				buf: ptr,
				cap: cap,	
				start: ptr.as_ptr(),
				end: if cap == 0 {
					ptr.as_ptr()
				} else {
					ptr.as_ptr().offset(len as isize)
				}
			}
		}
	}
}

impl<T> Drop for MyVec<T> {
	fn drop(&mut self) {
		if self.cap != 0 {
			while let Some(_) = self.pop() {}

			let align = mem::align_of::<T>();
			let elem_size = mem::size_of::<T>();
			let num_bytes = elem_size * self.cap;

			unsafe {
				let layout: Layout = Layout::from_size_align_unchecked(num_bytes, align)				;
				dealloc(self.ptr.as_ptr() as *mut _, layout);
			}
		}
		println!("release memory in drop function!");
	}
}

use std::slice;
use std::ops::{Deref, DerefMut};

impl<T> Deref for MyVec<T> {
	type Target = [T];
	fn deref(&self) -> &[T] {
		unsafe {
			slice::from_raw_parts(self.ptr.as_ptr(), self.len)	
		}
	}
}

impl<T> DerefMut for MyVec<T> {
	fn deref_mut(&mut self) -> &mut [T] {
		unsafe {
			slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)	
		}
	}
}


struct IntoIter<T> {
	buf: Unique<T>,
	cap: usize,
	start: *const T,
	end: *const T,
}

impl<T> Drop for IntoIter<T> {
	fn drop(&mut self) {
		if self.cap != 0 {
			for _ in &mut *self {}
			
			let align = mem::align_of::<T>();
			let elem_size = mem::size_of::<T>();
			let num_bytes = elem_size * self.cap;

			unsafe {
				let layout: Layout = Layout::from_size_align_unchecked(num_bytes, align);
				dealloc(self.buf.as_ptr() as *mut _, layout);
			}
		}
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		if self.start == self.end {
			None
		} else {
			unsafe {
				let result = ptr::read(self.start);
				self.start = self.start.offset(1);
				Some(result)
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = (self.end as usize - self.start as usize) 
				/ mem::size_of::<T>();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IntoIter<T> {
	fn next_back(&mut self) -> Option<T> {
		if self.start == self.end {
			None
		} else {
			unsafe {
				self.end = self.end.offset(-1);
				Some(ptr::read(self.end))
			}
		}
	}
}

fn main() {
	let mut vec: MyVec<i32> = MyVec::new();
	vec.push(1);
	if let Some(v) = vec.pop() {
		println!("v == {}", v);
	}
	
	println!("==================================");
	
	{
		let mut vec1: MyVec<i32> = MyVec::new();
		vec1.push(1);
		vec1.push(2);
		
		//let s = &vec1[1..];
		//println!("s[0] = {}", s[0]);

		vec1.insert(0, 11);
		while let Some(v) = vec1.pop() {
			println!("v === {}", v);
		}

		println!("==================================");
		let mut vec2: MyVec<i32> = MyVec::new();
		vec2.push(1);
		vec2.push(2);
		let ret = vec2.remove(0);
		println!("remove {}", ret);
		//while let Some(v) = vec2.pop() {
		//	println!("v === {}", v);
		//}

		let iter = vec2.iter();
		for val in iter {
			println!("v === {}", val);
		}
		
	}

	println!("==================================");
	let mut vec3: MyVec<i32> = MyVec::new();
	vec3.push(1);
	vec3.push(2);

	let iter = vec3.iter();
	for val in iter {
		println!("get val: {}", val);
	}

	let iter3: IntoIter<i32> = vec3.into_iter();
	for val in iter3 {
		//println!("get val: {}", val);
		//val = 111;
		println!("after modify val: {}", val);
	}
	
    println!("Hello, world!");
}
