// src/map_option/main.rs
/**
 * Option의 메소드인 map은 타입이 Some일때는 Some안에 있는 값을 꺼내서 클로저의 인자로 넘겨주고,
 * 클로저의 결과값을 Option 타입으로 반환한다. None은 그대로 None으로 반환한다.
 */
fn main() {
	let some_number = Some(5);
	let none_number: Option<i32> = None;
	
	let double_some = some_number.map(|x| x * 2);
	let double_none = none_number.map(|x| x * 2);
	
	println!("Double Some: {:?}", double_some); //Double some: Some(10)
	println!("Double none: {:?}", double_none); //Double None: None
}
