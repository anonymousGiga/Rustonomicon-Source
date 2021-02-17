struct Closure<F> {
	data: (u8, u16),
	func: F,
}

impl<F> Closure<F>
	where for<'b>F: Fn(&'b (u8, u16)) -> &'b u8,
{
	fn call<'b>(&'b self) -> &'b u8 {
		(self.func)(&self.data)
	}
}

fn do_it<'a>(data: &'a (u8, u16)) -> &'a u8 {
	&data.0
}

fn main() {
	let clo = Closure{data: (0, 1), func: do_it};
	println!("{}", clo.call());
}
