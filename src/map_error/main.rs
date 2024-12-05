/**
 * map 메서드 사용시 객체의 소유권이 이동되므로 더이상 사용불가
 * 레퍼런스를 전달 .as_ref()
 */
fn main() {
	let mut maybe_some_string = Some(String::from("Hello, World!"));
	let maybe_some_len = maybe_some_string.as_ref().map(|s| s.len());
	assert_eq!(maybe_some_len, Some(13));
	println!("{:?}", maybe_some_string);
	
	maybe_some_string.as_mut().map(|s| s.push_str(" Again!"));
	println!("{:?}", maybe_some_string);
}