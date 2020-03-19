#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct Info {
    success: bool,
    message: Option<String>,
}

impl Info {
    pub fn new(success: bool, message: Option<String>) -> Self {
        Info { success, message }
    }

    pub fn success(&self) -> bool {
        self.success
    }
}
