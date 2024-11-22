// src/map/main.rs
/**
 *  map 메소드
 *  
 */ 

fn fizzbuzz_2(max: i32) {
	for i in 1..=max {
		match (i % 3, i % 5) {
			(0, 0) => println!("{} - FizzBuzz", i),
			(0, _) => println!("{} - Fizz", i),
			(_, 0) => println!("{} - Buzz", i),
			(_, _) => (),
		}
	}
}

/**
 * 이터레이터의 map 메소드를 호출, map 메소드의 인자로 이터레이터가 값을 반환할 때마다 그 값을 인자로 받아서
 * 실행되는 함수가 들어간다.
 * 반드시 마지막에 collect 메소드를 호출해야한다.
 * map 매소드는 반환값으로 이터레이터를 반환한다. 즉 이터레이터를 받아서 처리하고 또 다른 이테러이터를 반환하는
 * 것이 map이 하는 일이다.
 * 이터레이터의 대표적인 메소드 : next, collect
 */
fn fizzbuzz_3(max: i32) {
	let ret = (1..max)
		.into_iter()
		.map(|i| match (i % 3, i % 5) {
			(0, 0) => format!("{} - FizzBuzz\n", i),
			(0, _) => format!("{} = Fizz\n", i),
			(_, 0) => format!("{} - Buzz\n", i),
			(_, _) => "".to_string(),
		})
		.collect::<Vec<String>>()
		.join("");
	
	println!("{}", ret);
}

fn main() {
	fizzbuzz_2(37);
	fizzbuzz_3(41);
}