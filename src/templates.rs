use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "dm_01.stpl")]
pub struct Dm01<'a> {
    pub username: &'a str
}
