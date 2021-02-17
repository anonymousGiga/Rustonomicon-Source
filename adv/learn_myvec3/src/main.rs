#![feature(ptr_internals)]
use std::ptr::{Unique, self};
use std::mem;
use std::alloc::{alloc, realloc, dealloc, Layout, handle_alloc_error};

struct RawVec<T> {
	ptr: Unique<T>,
	cap: usize,
}

impl<T> RawVec<T> {
	fn new() -> Self {
		//assert!(mem::size_of::<T>() != 0, "先不处理零尺寸类型");
		let cap = if mem::size_of::<T>() == 0 {!0} else {0};
		RawVec {
			ptr: Unique::dangling(),
			cap: cap,
		}
	}

	fn grow(&mut self)	{
		unsafe {
			let elem_size = mem::size_of::<T>();
			assert!(elem_size != 0, "capacity overflow");

			let align = mem::align_of::<T>();
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
}

impl<T> Drop for RawVec<T> {
	fn drop(&mut self) {
		let elem_size = mem::size_of::<T>();

		if self.cap != 0 && elem_size != 0 {
			let align = mem::align_of::<T>();
			let elem_size = mem::size_of::<T>();
			let num_bytes = elem_size * self.cap;

			unsafe {
				let layout: Layout = Layout::from_size_align_unchecked(num_bytes, align)				;
				dealloc(self.ptr.as_ptr() as *mut _, layout);
			}
		}
	}	
}

struct MyVec<T> {
	buf: RawVec<T>,
	len: usize,
}

impl<T> MyVec<T> {
	fn ptr(&self) -> *mut T { self.buf.ptr.as_ptr() }
	
	fn cap(&self) -> usize { self.buf.cap }

	fn new() -> Self {
		MyVec {
			buf: RawVec::new(),
			len: 0,
		}
	}

	//push
	fn push(&mut self, elem: T) {
		if self.len == self.cap() {
			self.buf.grow();
		} 

		unsafe {
			//ptr = ptr + offset
			//*ptr = elem	
			ptr::write(self.ptr().offset(self.len as isize), elem);
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
				Some(ptr::read(self.ptr().offset(self.len as isize)))
			}
		}
	}

	//insert
	fn insert(&mut self, index: usize, elem: T) {
		assert!(index <= self.len, "out of bound");
		if self.cap() == self.len {
			self.buf.grow();
		}	
		
		unsafe {
			if index < self.len {
				ptr::copy(self.ptr().offset(index as isize),
						self.ptr().offset(index as isize + 1),
						self.len - index);
			}
			ptr::write(self.ptr().offset(index as isize), elem);
			self.len += 1;
		}
	}
	
	//remove
	fn remove(&mut self, index: usize) -> T {
		assert!(index <= self.len, "out of bound");
		unsafe {
			self.len -= 1;
			let result = ptr::read(self.ptr().offset(index as isize));
			ptr::copy(self.ptr().offset(index as isize + 1),
					self.ptr().offset(index as isize),
					self.len - index);
			result
		}
	}

	//into_iter
	fn into_iter(self) -> IntoIter<T> {
		unsafe{
			let iter = RawValIter::new(&self);
			let buf = ptr::read(&self.buf);
			mem::forget(self);

			IntoIter {
				iter: iter,
				_buf: buf,
			}
		}
	}
}

struct IntoIter<T> {
	_buf: RawVec<T>,
	iter: RawValIter<T>,
}

impl<T> Drop for IntoIter<T> {
	fn drop(&mut self) {
		for _ in &mut *self {}
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.iter.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<T> DoubleEndedIterator for IntoIter<T> {
	fn next_back(&mut self) -> Option<T> {
		self.iter.next_back()
	}
}

impl<T> Drop for MyVec<T> {
	fn drop(&mut self) {
		while let Some(_) = self.pop() {}
	}
}

use std::slice;
use std::ops::{Deref, DerefMut};

impl<T> Deref for MyVec<T> {
	type Target = [T];
	fn deref(&self) -> &[T] {
		unsafe {
			slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len)	
		}
	}
}

impl<T> DerefMut for MyVec<T> {
	fn deref_mut(&mut self) -> &mut [T] {
		unsafe {
			slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len)	
		}
	}
}

struct RawValIter<T> {
	start: *const T,
	end: *const T,
}

impl<T> RawValIter<T> {
	unsafe fn new(slice: &[T]) -> Self {
		RawValIter {
			start: slice.as_ptr(),
			end: if mem::size_of::<T>() == 0 {
				((slice.as_ptr() as usize) + slice.len()) as *const _
			} else if slice.len() == 0 {
				slice.as_ptr()
			} else {
				slice.as_ptr().offset(slice.len() as isize)
			}
		}
	}
}

impl<T> Iterator for RawValIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		if self.start == self.end {
			None
		} else {
			unsafe {
				let result = ptr::read(self.start);
				self.start = if mem::size_of::<T>() == 0 {
					(self.start as usize + 1) as *const _
				} else {
					self.start.offset(1)
				};

				Some(result)
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let elem_size = mem::size_of::<T>();
		let len = (self.end as usize - self.start as usize) 
				/ if elem_size == 0 {1} else { elem_size };
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for RawValIter<T> {
	fn next_back(&mut self) -> Option<T> {
		if self.start == self.end {
			None
		} else {
			unsafe {
				self.end = if mem::size_of::<T>() == 0 {
					(self.end as usize - 1) as *const _
				} else {
					self.end.offset(-1)
				};

				Some(ptr::read(self.end))
			}
		}
	}
}


//drain 是一个集合 API，它将容器内的数据所有权移出，却不占有容器本身
use std::marker::PhantomData;
struct Drain<'a, T: 'a> {
	vec: PhantomData<&'a mut MyVec<T>>,
	iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.iter.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
	fn next_back(&mut self) -> Option<T> {
		self.iter.next_back()
	}
}

impl<'a, T> Drop for Drain<'a, T> {
	fn drop(&mut self) {
		for _ in &mut self.iter {}
	}
}

impl<T> MyVec<T> {
	fn drain(&mut self) -> Drain<T> {
		unsafe {
			let iter = RawValIter::new(&self);
			self.len = 0;
			Drain {
				iter: iter,
				vec: PhantomData,
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
	
	println!("==================================");
	let mut vec4: MyVec<i32> = MyVec::new();
	vec4.push(1);
	vec4.push(2);
	vec4.push(3);
	vec4.push(4);

	let mut drain = vec4.drain();
	let a = drain.next().unwrap();
	println!("drain: {}", a);//1

	while let Some(val) = drain.next_back() {
		println!("v === {}", val); //4\3\2
	}

    println!("Hello, world!");
}
