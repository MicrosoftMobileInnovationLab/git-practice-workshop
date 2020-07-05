fn fact(n:i32) -> i32 {
	let mut result : i32 = 1;
	for i in 2..n+1 {
		result = result * i;
	}
	result
}

fn main(){
	let num : i32 = 5;
	println!("{}",fact(num));
}
