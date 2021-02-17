fn main() {
	let mut array = [0x78, 0x56, 0x34, 0x12];
	let num: u32 = unsafe {
		std::mem::transmute::<[u8; 4], u32>(array) //实际上把原值中的值复制到了目标值中
	};
	println!("num = {}", num);
	//1、大端字节序：0x78563412: 2018915346
	//2、小端字节序：0x12345678: 305419896

	assert_eq!(num, u32::from_ne_bytes(array));
	assert_eq!(num, u32::from_le_bytes(array));
	//assert_eq!(num, u32::from_be_bytes(array)); //应该报错
    println!("Hello, world!");
}
