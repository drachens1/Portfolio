use EasyWebsiteBuilder2::page::Page;
use EasyWebsiteBuilder2::website::Website;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut website = Website::new();
    website
      .add_page(Page::new_from_html_str("/", include_str!("../pages/landing.html"), 0))
      .add_page(Page::new_from_html_str("/404", include_str!("../pages/404.html"), 0))
      .start([0, 0, 0, 0], 8080).await;
}
