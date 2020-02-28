#[derive(Serialize)]
pub struct ErrorTemplateContext<'a> {
    pub code: &'a str,
    pub message: &'a str,
}
