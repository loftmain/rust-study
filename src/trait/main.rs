// /src/trait/main.rs
trait Printable {
	type Age;
	fn print(&self);
	fn get_age(&self) -> Self::Age;
}

struct Person {
	name: String,
	age: u32,
}

impl Person {
	fn new(name: &str, age: u32) -> Self {
		Person {
			name: name.to_string(),
			age: age,
		}
	}
}

impl Printable for Person {
	type Age = u32;
	fn print(&self) {
		println!("name : {}, {} years old", self.name, self.get_age());
	}
	fn get_age(&self) -> Self::Age {
		self.age
	}
}

struct Book {
	title: String,
	author: String,
	published: u32,
}

impl Printable for Book {
	type Age = u32;
	fn print(&self) {
		println!(
			"Title: {}\nAuthor: {}\nPublished: {}",
			self.title,
			self.author,
			self.get_age()
		);
	}
	fn get_age(&self) -> Self::Age {
		self.published
	}
}

fn print_info(item: &dyn Printable<Age = u32>) {
	item.print();
}

fn main() {
	let person = Person::new("Alice", 22);
	let book = Book {
		title: String::from("The Rust Programming Language"),
		author: String::from("Steve Klabnik and Carol Nichols"),
		published: 20230228,
	};
	
	print_info(&person);
	print_info(&book);
}