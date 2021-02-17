struct Inspector<'a>(&'a u8);

struct World<'a> {
    inspector: Option<Inspector<'a>>,
    days: Box<u8>,
}

//实现Drop trait:对于泛型参数，一定要严格遵守，被引用的内容的生命周期比引用本身要长（严格遵守）
//如果不是泛型，那么只能用static
impl<'a> Drop for Inspector<'a> {
	fn drop(&mut self) {
		println!("self.0 = {}", self.0);
	}
}

fn main() {
    let mut world = World {
        inspector: None,
        days: Box::new(1),   //---------------------'days start
    };						 //--------------- 'days
    world.inspector = Some(Inspector(&world.days)); //-'days end   --'inspector start --'inspector end
	//如果使用Drop，要求days的生命周期一定比inspector要长(严格要求)
}
//引用：引用的生命周期小于被引用内容的生命周期
