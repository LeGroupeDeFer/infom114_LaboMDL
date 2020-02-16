#[derive(Serialize)]
pub struct LoginTemplateContext {
    pub title: &'static str,
    pub parent: &'static str,
}
