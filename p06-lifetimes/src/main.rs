fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];


    let result = next_language(&languages, "go");

    println!("{:?}", result);
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
