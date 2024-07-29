#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        //a verbose way to do type checking to find out what variant of the Media Enum this is
        if let Media::Book { title, author } = self {
            format!("Book: {} by {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} by {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Unknown Media Type")
        }
    }
}

// fn print_media(media: Media) {
//     println!("{:#?}", media);
// }

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
        author: String::from("Booker Authorman"),
    };

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(book);

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", book.description());
}
