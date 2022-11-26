#[derive(Debug)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(input_title: &str, input_status: &str) -> Base {
        Base {
            title: String::from(input_title),
            status: String::from(input_status),
        }
    }
}
