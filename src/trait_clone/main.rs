// src/trait_clone/main.rs
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title.clone(),
            author: self.author.clone(),
            published: self.published,
        }
    }
}

//fn print_info(item: &dyn Clone) {
//    println!("item implements Clone trait");
//}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Kalbnik and Carol Nochols"),
        published: 20241205
    };

    let author = Book {
        title: String::from("The another book"),
        author: String::from("Unkown"),
        published: 20211111,
    };

    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
    book_clone.clone_from(&author);
    println!("{:?}", book_clone);

    //print_info(&book);
}