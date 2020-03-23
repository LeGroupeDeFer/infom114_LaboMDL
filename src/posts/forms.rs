use crate::schema::posts;
// use crate::models::post::Post;


#[derive(FromForm, Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: Option<String>,
    pub content: String,
    pub authorid: u32
}
