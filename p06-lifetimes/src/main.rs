fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];


    let result = next_language(&languages, "go");

    println!("{:?}", result);

    let last_language = last_language(&languages);

    println!("{:?}", last_language);


    println!("longest language name: {}", longest_language_name("typescript", "go"));
}

///Takes a vector of languages, and a string slice to look for in the vector
///If the language is found in the vector, the next language in the vector will be found
///If the function were to return a String, there would be no issue
///However, the instructor wants it to return a &str in order to demonstrate lifetime annotations
fn next_language_string_version(languages: &[String], current: &str) -> String {
    
    let mut found = false;
    
    for lang in languages {

        if found {
            return lang.to_string();
        }

        if lang == current {
            found = true;
        }
    }

    //if the sought for language doesn't exist in the vector, or is the last one, return the last one
    //assume that the vector will always have a Some() value,
    //since this is just being done as an example of lifetimes, so use .unwrap() here
    return languages.last().unwrap().to_string();
}

///Takes a vector of languages, and a string slice to look for in the vector
///If the language is found in the vector, the next language in the vector will be found
///Returns a string slice because the instructor wants it to in order to demonstrate lifetime annotations
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    
    let mut found = false;
    
    for lang in languages {

        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    //if the sought for language doesn't exist in the vector, or is the last one, return the last one
    //assume that the vector will always have a Some() value,
    //since this is just being done as an example of lifetimes, so use .unwrap() here
    return languages.last().unwrap();
}



///An example of a corner case where lifetime annotations are not necessary because Rust will assume that the returned reference will be related to the referenced input data
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}


///This one generates an issue where it says it wants to know whether the returned reference is borrowed from input parameter 1 or input parameter 2
///So to fix that, lifetime annotations are added to all the parameters to show that they could relate to any of them
///Again, the point of these seems to be about making it easy to tell by looking at the function signature how the return value could relate to the input arguments, so that people using the function know what bindings have to be kept in scope to use the function successfully
fn longest_language_name<'a>(name1: &'a str, name2: &'a str) -> &'a str {
    if name1.len() >= name2.len() { name1 } else { name2 }
}