use std::marker;

struct _MyIter<'a, T: 'a> {
	ptr: *const T,
	end: *const T,
	_usemarker: marker::PhantomData<&'a T>	,
}

fn main() {
    println!("Hello, world!");
}
