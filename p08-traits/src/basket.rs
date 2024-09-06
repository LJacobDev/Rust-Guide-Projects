
pub struct Basket {
    pub item: Option<String>,
}

impl Basket {
    pub fn get(&mut self) -> Option<String> {

        //this .take() is capable of moving the value out of self.item, and leaving a None in its place,
        //where just returning self.item would generate a moving error by itself
        //if there is no value in self.item, .take() will return a None, so this method's implementation is complete with this.
        //.take() is a method of the Option enum, and since item is an Option<String>, it is available to use here
        self.item.take()

    }

    pub fn put(&mut self, item: String) {

        //in the starting case of using strings only, just assign what is given to put into the item
        //when numbers are included, additional functionality will be added to be able to add the numbers together
        self.item = Some(item);
    }

    pub fn is_empty(&self) -> bool {
        match &self.item {
            Some(value) => true,
            None => false
        }
    }
}

