// src/function_pointer/main.rs
/**
	함수 포인터
	fn이 하나의 키워드로서 함수 포인터를 나타낸다.
	
*/
fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
	for i in 1..=100 {
		if fizzfn(i) && buzzfn(i) {
			println!("FizzBizz");
		} else if fizzfn(i) {
			println!("Fizz");
		} else if buzzfn(i) {
			println!("Buzz");
		}
	}
}

fn fizzcheck(n: i32) -> bool {
	n % 3 == 0
}

fn buzzcheck(n: i32) -> bool {
	n % 5 == 0
}

fn main() {
	fizzbuzz_fn(fizzcheck, buzzcheck);
	fizzbuzz_fn(|x| x % 3 == 0, |y| y % 5 == 0);
}