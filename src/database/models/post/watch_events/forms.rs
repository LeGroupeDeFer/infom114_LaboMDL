
#[derive(Serialize, Deserialize, Debug)]
pub struct WatchEventData {
    pub event: String,
    pub comment: String,
}