use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add_media(&mut self, media: Media) {
        self.items.push(media);
    }

    //a method that has no error checking and can cause panic at runtime if given out of bounds index
    pub fn get_by_index(&self, index: usize) -> &Media {
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
    pub fn get_by_index_custom_option_enum(&self, index: usize) -> Option<&Media> {
        if index < self.items.len() {
            //we have something to return
            Some(&self.items[index])
        } else {
            //there is no item at this index
            None
        }
    }


}
