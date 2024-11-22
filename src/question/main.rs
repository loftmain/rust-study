// src/question/main.rs
fn foo() -> Result<i32, String> {
	let r = bar();
	match r {
		Ok(n) => {
			println!("Do Something with {}", n);
			return Ok(1);
		}
		Err(s) => {
			println!("Do error handing with {}", s);
			return Err(s);
		}
	}
}

fn bar() -> Result<i32, String> {
	let r = foobar();
	match r {
		Ok(n) => {
			println!("Do something with {}", n);
			return Ok(1);
		}
		Err(s) => {
			println!("Do error handling with {}", s);
			return Err(s);
		}
	}
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