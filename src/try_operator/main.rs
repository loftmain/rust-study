// src/try_operator/main.rs
/**
 * ? 연산자는 값이 None이거나 Err 타입이면 바로 현재 함수의 반환값으로 반환한다.
 * 아니면 Ok나 Some안에 저장된 원래 값을 꺼내 반환해서 계속 처리는 진행하게 해준다.
 * 에러처리 + unwrap
let r = match expr {
	Ok(value) => value,
	Err(err) => return Err(err),
}
 
 */
fn foo() -> Result<i32, String> {
	let r = bar()?;
	println!("Do something with {}", r);
	return Ok(1);
}

fn bar() -> Result<i32, String> {
	let r = foobar()?;
	println!("Do something with {}", r);
	return Ok(1);
}

fn foobar() -> Result<i32, String> {
	let r = "foobar error".to_string();
	Err(r)
}

fn main() {
	let r = foo();
	match r {
		Ok(n) => println!("Do something with {}", n),
		Err(s) => println!("Do error handling with {}", s),
	}
}