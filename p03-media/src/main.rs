#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    // Podcast { episode_number: u32 },
    Podcast(u32), //this is syntax that works like above, but where 'episode_number' is implied and takes less typing to work with
    Placeholder,
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

        //This is called Pattern Matching
        //'self' is being checked against the type pattern mentioned on the left side of the => arrow,
        //if self matches the structure of any of the given Enum variants, then the code on the right of the => is executed
        //which in this case, looks to be an implicit return statement of the format!() macro

        //it is also possible to wrap curly braces around the statement after the => arrow, which allows multiline blocks of statements
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            //above, Podcast(u32) is how the enum variant is defined,
            //so an arbitrary name like "episode_number" can be made up here
            //to be able to use the u32 value in this match arm
            Media::Podcast(episode_number) => {
                format!("Podcast episode number: {}", episode_number)
            }
            //above, Placeholder had no curly braces or fields at all
            //so it pattern matches in this way here
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add_media(&mut self, media: Media) {
        self.items.push(media);
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
    //this enum variant is created in a similar syntax to how it was defined in the Enum block
    let podcast = Media::Podcast(1);
    //this enum variant is created with no curly braces nor parentheses like how it was defined in the enum block
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add_media(audiobook);
    catalog.add_media(book);
    catalog.add_media(good_movie);
    catalog.add_media(podcast);
    catalog.add_media(placeholder);

    println!("{:#?}", catalog);

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", book.description());

    println!("Catalog item descriptions:");
    //this needs to use an immutable reference to catalog.items otherwise it moves them
    for media in &catalog.items {
        println!("{}", media.description());
    }

    //experimenting with Vec::get and index slicing

    //using catalog.items.get(index) doesn't move the value, and it also wraps the result in 'Some()'
    println!("Index 1: {:#?}", catalog.items.get(1));
    println!("Index 1: {:#?}", catalog.items.get(1));

    //using catalog.items.get(100) for an index that is not in the collection, it returns a None value
    println!("Index 100: {:#?}", catalog.items.get(100));

    //if you want to get all the items rather than one index or a subset, use ..
    println!("All items: {:#?}", catalog.items.get(..));

    //to get the 3rd item to the last item, use 2..
    println!("Item index 2 to the end: {:#?}", catalog.items.get(2..));

    //there isn't a 'stride' option like in python, but similar functionality would work like this:
    println!("Every second item: {:#?}", catalog.items.iter().step_by(2).collect::<Vec<_>>());
    //using either <Vec<_>> or <Vec<&Media>> both work, though I don't yet know what the meaning of the former is
    //I checked and it means it's a type placeholder that allows the compiler to infer the type based on the context, which it could tell was &Media
    //however, using just .collect() without <Vec<_>> wasn't enough to be able to infer that it was needing <Vec<&Media>> and it needed some help with it
    println!("Every second item: {:#?}", catalog.items.iter().step_by(2).collect::<Vec<&Media>>());


}
