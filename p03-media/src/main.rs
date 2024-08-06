use std::option;

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

    //a method that has no error checking and can cause panic at runtime if given out of bounds index
    fn get_by_index(&self, index: usize) -> &Media {
        &self.items[index]
    }

    //OLD WAY THAT RETURNED 'MIGHTHAVEAVALUE' AS EXAMPLE OF HOW OPTION WORKS
    //a method that has a custom Enum returned to show how Option::Some() and Option::None work
    // fn get_by_index_custom_option_enum(&self, index: usize) -> MightHaveAValue {
    //     if index < self.items.len() {
    //         //we have something to return
    //         MightHaveAValue::ThereIsAValue(&self.items[index])
    //     } else {
    //         //there is no item at this index
    //         MightHaveAValue::NoValueAvailable
    //     }
    // }

    //a method that has a custom Enum returned to show how Option::Some() and Option::None work
    fn get_by_index_custom_option_enum(&self, index: usize) -> Option<&Media> {
        if index < self.items.len() {
            //we have something to return
            Some(&self.items[index])
        } else {
            //there is no item at this index
            None
        }
    }


}

///Custom 'Option' Enum as demonstrated by guide
///This one requires a lifetime annotation added to the syntax, 'a ,
///but the guide doesn't explain it much yet,
///as they say it will be covered in a later topic of the course

//commenting this enum out and modifying 'get_by_index_custom_option_enum to no longer return 'mighthaveavalue' but return 'Option' instead

// #[derive(Debug)]
// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable,
// }

//I made this function first as my own guess at how to make a custom Vec::get, before watching the instructor build the function,
//to see how close my guess would be to the real thing.  The instructor's version of this is done by adding a method onto the Catalog struct
///Making a custom version of Vec::get to further understand the reason why Vec::get gives Option::Some(value) and Option::None returns
fn get_by_index_version1(vector: &Vec<Media>, index: usize) -> &Media {
    &vector[index]
}

/*

///A custom made enum to demonstrate what is going on when Option::Some() and Option::None are being returned by methods in Rust
enum CustomOptionVersion2 {

    //This one is something I'm trying to make by guessing how it would be done before seeing how the guide does it
    //My guess appeared to look about right, but it ran into a problem where the type specified for Some wasn't able to be Some(_)
    //and using Some(Media) to make it expect the exact type for the practice experiment revealed other issues about it preferring something with a Copy trait
    //while using Some(&Media) was causing errors talking about how the lifetime might not be appropriate somehow
    //so it didn't become fully functional and I want to see what the guide does here
    //UPDATE:  It seemed to have to do with this enum needing a "lifetime annotation" added to it in order to clear some errors talking about lifetime issues
    Some(Media),
    None,
}

fn get_by_index_version2(vector: &Vec<Media>, index: usize) -> CustomOptionVersion2 {
    if vector.len() > index {
        CustomOptionVersion2::Some(vector[index])
    }
    else {
        CustomOptionVersion2::None
    }
}

*/

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

    println!("Catalog object: {:#?}\n", catalog);

    println!("Catalog item descriptions:");
    //this needs to use an immutable reference to catalog.items otherwise it moves them
    for media in &catalog.items {
        println!("{}", media.description());
    }

    println!(
        "\nUsing catalog.items.get() to show match checking the returned Some() and None Options:"
    );
    //Example of using Some() and None match checking when using items.get()
    match catalog.items.get(1) {
        Option::Some(value) => {
            println!("Item found: {:#?}", value)
        }
        Option::None => {
            println!("No item at that index")
        }
    }

    //it is possible to omit the "Option::" part of the enum variant checking
    match catalog.items.get(8) {
        Some(value) => println!("Value found: {:#?}", value),
        None => println!("No value found at that index"),
    }

    //testing the function I made on my own before seeing the instructor's explanations

    //these function calls return the value and print it nicely
    // println!("Get_By_Index 1: {:#?}", get_by_index_version1(&catalog.items, 1));
    // println!("Get_By_Index 1: {:#?}", get_by_index_version1(&catalog.items, 1));

    //this function call causes a panic at runtime due to the index being out of bounds
    // println!("Get_By_Index 1: {:#?}", get_by_index_version1(&catalog.items, 100));

    //this demonstrates why having an Option return type can be helpful to avoid a panic at runtime

    //using the instructor's example method calls:
    // println!("Index 1: {:#?}", catalog.get_by_index(1));     //returns &Media
    // println!("Index 4: {:#?}", catalog.get_by_index(4));     //returns &Media
    // println!("Index 40: {:#?}", catalog.get_by_index(40));   //causes a panic due to out of bounds index

    println!("\nUsing custom made 'Vec::get' type method with custom made 'Option' type enums:");
    //using the intructor's example of a method call that returns a custom made enum called MightHaveAValue:
    println!("Index 1: {:#?}", catalog.get_by_index_custom_option_enum(1)); //returns ThereIsAValue(&'a Media)
    println!("Index 4: {:#?}", catalog.get_by_index_custom_option_enum(4)); //returns ThereIsAValue(&'a Media)
    println!("Index 40: {:#?}", catalog.get_by_index_custom_option_enum(40)); //returns NoValueAvailable, doesn't panic


    // old block of code that demonstrated the custom enum of MightHaveAValue before replacing it to use Option again
    // println!("\nGet the actual item with match statement rather than getting the enum:");
    // match catalog.get_by_index_custom_option_enum(1) {
    //     MightHaveAValue::ThereIsAValue(value) => println!("Value found: {:#?}", value),
    //     MightHaveAValue::NoValueAvailable => println!("No value was available at that index")
    // }


    // println!("\nGet the actual item using 'if let' pattern matching:");
    // if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index_custom_option_enum(1) {
    //     println!("Value found: {:#?}", value)
    // }
    // else {
    //     println!("No value found")
    // }
    

    //new versions of the above block of code that use the Option type for its matching
    println!("\nGet the actual item with match statement rather than getting the enum:");
    match catalog.get_by_index_custom_option_enum(1) {
        Some(value) => println!("Value found: {:#?}", value),
        None => println!("No value was available at that index")
    }


    println!("\nGet the actual item using 'if let' pattern matching:");
    if let Some(value) = catalog.get_by_index_custom_option_enum(1) {
        println!("Value found: {:#?}", value)
    }
    else {
        println!("No value found")
    }




    // ADDITIONAL WAYS TO CHECK OPTION TYPES:  .unwrap(), .expect(""), .unwrap_or(&placeholder)


    //using .unwrap()
    //this gets the value if it is a Some(), and panics if it is a None

    
    let mut item = catalog.get_by_index_custom_option_enum(1);
    
    //this prints the item
    println!("Item at index 1 unwrapped: {:#?}", item.unwrap());
    
    item = catalog.get_by_index_custom_option_enum(10);
    
    //this panics
    // println!("Item at index 10 unwrapped: {:#?}", item.unwrap());



    //using .expect()
    //this gets the value if there is Some, and if None, it prints a message and panics

    item = catalog.get_by_index_custom_option_enum(1);

    //this prints the item
    println!("Item at 1 using .expect(): {:#?}", item.expect(".expect() error message: There was no value at the index selected"));

    item = catalog.get_by_index_custom_option_enum(10);

    //this prints a message and then panics
    // println!("Item at 1 using .expect(): {:#?}", item.expect(".expect() error message: There was no value at the index selected"));


    //using .unwrap_or()
    //this gets the value in the Some() if there is one, or else gives another value if there is a None, and does not panic

    item = catalog.get_by_index_custom_option_enum(1);

    //this gives the value in the Some()
    //this needed to be given a default value that was of type &Media, so I used a reference to a newly created placeholder variant 
    println!("Index 1 using the .unwrap_or() method: {:#?}", item.unwrap_or(&Media::Placeholder));
    item = catalog.get_by_index_custom_option_enum(10);

    //this has a None Option type so it gives 'Placeholder' in the output AND DOES NOT PANIC / CRASH THE PROGRAM
    //this needed to be given a default value that was of type &Media, so I used a reference to a newly created placeholder variant 
    println!("Index 10 using the .unwrap_or() method: {:#?}", item.unwrap_or(&Media::Placeholder));


    /*

    //--------------------experimenting with checking all values with a loop

        //got unexpected results when using 'for i in [0..5]' vs 'for i in [0,1,2,3,4,5]'

    //for i in [0..5] was intended to produce "value found:" 5 times and "No item.." one time,
    //but instead it said "Value found" once and showed all items
    //compiler sees this as Range<usize>
    for i in [0..5] {
        match catalog.items.get(i) {
            Some(value) => println!("Value found: {:#?}", value),
            None => println!("No item found at that index"),
        }
    }

    //writing the for loop in this way did result in either "Value Found" or "No item found.." appearing a total of 6 times
    //compiler sees this as usize
    for i in [0,1,2,3,4,5] {
        match catalog.items.get(i) {
            Some(value) => println!("Value found: {:#?}", value),
            None => println!("No item found at index of {}", i)
        }
    }

    */

    /*

    //--------------------experimenting with Vec::get and index slicing

    //using catalog.items.get(index) doesn't move the value, and it also wraps the result in 'Some()'
    println!("Index 1: {:#?}", catalog.items.get(1));
    println!("Index 1: {:#?}", catalog.items.get(1));

    //using catalog.items.get(100) for an index that is not in the collection, it returns a None value
    println!("Index 100: {:#?}", catalog.items.get(100));

    //if you want to get all the items rather than one index or a subset, use ..
    println!("All items: {:#?}", catalog.items.get(..));

    //to get the 3rd item to the last item, use 2..
    println!("Item index 2 to the end: {:#?}", catalog.items.get(2..));


    //there isn't a 'stride' option like in python, but similar functionality would work like this, using iter().step_by()
    //this example of step_by().collect()::<Vec<_>> no longer has the Option Some() or None appear with it as was seen when using Vec::get

    println!("Every second item: {:#?}", catalog.items.iter().step_by(2).collect::<Vec<_>>());

    //using either <Vec<_>> or <Vec<&Media>> both work, though I don't yet know what the meaning of the former is

    //I checked and <_> means it's a type placeholder that allows the compiler to infer the type based on the context, which it could tell was &Media
    //however, using just .collect() wasn't enough to be able to infer that it was needing <Vec<&Media>>, and it needed <Vec<_>> to help it infer

    println!("Every second item: {:#?}", catalog.items.iter().step_by(2).collect::<Vec<&Media>>());

    */
}
