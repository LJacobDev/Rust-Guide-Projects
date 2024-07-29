#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        //Rust will not allow you to process the properties of self, like self.title, until the type has been first determined,
        //even though all three variants have a .title property

        //a verbose way to do type checking to find out what variant of the Media Enum this is
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} by {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} by {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Unknown Media Type")
        // }

        //the less verbose match statement way of doing the type checking for which variant it is:
        // match self {
        //     Media::Book { title, author } => format!("Book: {} by {}", title, author),
        //     Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
        //     Media::Audiobook { title } => format!("Audiobook: {}", title),
        // }

        //self is being checked against the type pattern mentioned on the left side of the => arrow,
        //if self matches the structure of any of the given Enum variants, then the code on the right of the => is executed
        //which in this case, looks to be an implicit return statement of the format!() macro

        //it is also possible to wrap curly braces around the statement after the => arrow, which allows multiline blocks of statements
        match self {
            Media::Book { title, author } => {format!("Book: {} by {}", title, author)},
            Media::Movie { title, director } => {format!("Movie: {} by {}", title, director)},
            Media::Audiobook { title } => {format!("Audiobook: {}", title)},
        }

    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let book = Media::Book {
        title: String::from("A Book"),
        author: String::from("Author Authorman"),
    };

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", book.description());
}
