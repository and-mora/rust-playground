struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library {
            books: vec!()
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        println!("books:");
        for book in self.books.iter().enumerate() {
            println!("{}. {}", book.0 + 1, book.1);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        // stream style, reduce like
        return self.books.iter().fold(self.books.first(), |mut state, x| {
            // each iteration, we check the oldest between the state and the current
            if state.unwrap().year > x.year {
                state = Some(x);
            }
            state
        });

        // imperative style
        // if self.books.is_empty() {
        //     return None;
        // }
        //
        // let mut oldest_book = self.books.first();
        // for book in self.books.iter() {
        //     if book.year < oldest_book.unwrap().year {
        //         oldest_book = Some(book);
        //     }
        // }
        //
        // return oldest_book;

        // official (and best) solution
        //self.books.iter().min_by_key(|book| book.year)
    }
}

fn main() {
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());

    let favorite_book = Book::new("Lord of the Rings", 1954);
    println!("Our favorite book {favorite_book} should go in the library");
    library.add_book(favorite_book);
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }

    println!("Our library has {} books", library.len());
    for book in &library.books {
        println!("{book}");
    }
}