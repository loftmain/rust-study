// src/closure/main.rs
/**
 *	클로저 
 *	|와 | 사이에 클로저의 인자를 넣고, 그 다음에 클로저의 실행 코드를 적으면 된다.
 *	예제와 같이 클로저를 다른 함수의 인자로 전달할 수도 있다.
 *
 *  Fn 타입은 외부 변수를 사용할 수 있는데, 제약 조건이 외부 변수를 수정하지 않는 불변 참조를 해서 사용한다는 것.
 *  만약 클로저에서 외부 변수를 수정해야한다면 FnMut 타입을 사용한다.
 */

fn fizzbuzz_fn<FA, FB>(fizzfn: FA, buzzfn: FB) 
where 
	FA: Fn(i32) -> bool,
	FB: Fn(i32) -> bool,
{
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

fn main() {
	let fizz = 3;
	let buzz = 5;
	fizzbuzz_fn(|x| x % fizz == 0, |y| y % buzz == 0);
}