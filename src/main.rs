use submillisecond::{response::Html, router, Application};

fn main() -> std::io::Result<()> {
    Application::new(router! {
        GET "/" => index
    })
    .serve("0.0.0.0:3000")
}

fn index() -> Html<&'static [u8]> {
    Html(include_bytes!("../static/index.html"))
}
