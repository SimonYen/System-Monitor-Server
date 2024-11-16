use salvo::prelude::*;
use tera::Context;
use crate::TEMPLATES;


//主页
#[handler]
pub async fn home(res: &mut Response) {
    res.render(Text::Html(TEMPLATES.render("home.html",&Context::new()).expect("模板渲染失败!")));
}