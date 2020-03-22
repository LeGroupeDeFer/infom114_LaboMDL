#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct Info {
    message: Option<String>,
}

impl Info {
    pub fn new(message: Option<String>) -> Self {
        Info { message }
    }
}
