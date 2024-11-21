// src/enum_basic/main.rs

enum WEEK {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
}

fn main() {
	let today: WEEK = WEEK::Sunday;
	let tomorrow: WEEK = 1; // compile error !!!
	
	match today {
		WEEK::Sunday => println!("Sunday: Sleep for 10 hours"),
		WEEK::Monday => println!("Monday: Work"),
		WEEK::Tuesday => println!("Tuesday: Work"),
		WEEK::Wednesday => println!("Wednesday: work"),
		WEEK::Thursday => println!("Thursday: work"),
		WEEK::Friday => println!("Friday: Work"),
		WEEK::Saturday => {
			println!("Saturday: Party at Club from 22")
		}
	}
}