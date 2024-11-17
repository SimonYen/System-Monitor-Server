use salvo::prelude::*;
use crate::handler::gross::Gross;
use crate::TEMPLATES;


//主页
#[handler]
pub async fn home(res: &mut Response) {
    let gross=Gross::new();
    res.render(Text::Html(TEMPLATES.render("home.html", &gross.to_context()).expect("模板渲染失败!")));
}