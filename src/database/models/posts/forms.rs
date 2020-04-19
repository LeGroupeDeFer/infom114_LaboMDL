use crate::database::schema::posts;


#[derive(FromForm, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: Option<String>,
    pub content: String,
    pub authorid: u32
}
