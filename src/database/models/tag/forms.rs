#[derive(Serialize, Deserialize, Debug)]
pub struct TagData {
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagReport {
    pub tag: String,
    pub info: u64,
    pub idea: u64,
    pub poll: u64,
}
