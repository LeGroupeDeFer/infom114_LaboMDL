use rocket::Request;

#[derive(Debug)]
pub enum RequestType {
    Vanilla,
    Json,
    Unknown,
}

impl RequestType {
    pub fn guess(req: &Request) -> Self {
        // check if the url path begins with /api/
        let (begin, _) = req.uri().path().split_at(5);
        if begin == "/api/" {
            RequestType::Json
        } else {
            RequestType::Vanilla
        }
    }
}
