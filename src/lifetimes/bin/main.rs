fn add_with_lifetimes<'a, 'b>(x: &'a i32, y: &'b i32) -> i32 {
    *x + *y
}

fn longer_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// These annotations are not required, the compiler can infer it
fn first_word<'a>(phrase: &'a str) -> &'a str {
    let mut i = 0;
    for ch in phrase.chars() {
        if ch == ' ' {
            break;
        }
        i+=1;
    }
    &phrase[0..i]
}

struct Book<'a> {
    title: &'a str
}
impl<'a> Book<'a> {
    fn title(&self) -> &str {
        self.title
    }
}

fn main() {
    println!("Lifetimes!");
    println!("Add with lifetimes: {}", add_with_lifetimes(&10, &25));
    println!("Longer string: {}", longer_string(&String::from("Foo"), &String::from("Barz")));
    let phrase = String::from("Ok, it's comprehensive");
    println!("First word in '{}' is '{}'", phrase, first_word(&phrase));

    let book_title = String::from("That's a great book");
    let book = Book { title: &book_title };
    println!("Title of the inner book: {}", book.title());
}