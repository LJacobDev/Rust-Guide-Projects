#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    // Podcast { episode_number: u32 },
    Podcast(u32), //this is syntax that works like above, but where 'episode_number' is implied and takes less typing to work with
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
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
