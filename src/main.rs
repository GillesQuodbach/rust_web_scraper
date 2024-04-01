fn main() {
    //download the target HTML document
    let response = reqwest::blocking::get("https://scrapeme.live/shop/");

    //get the html content from the request response and print it
    let html_content = response.unwrap().text().unwrap();
    println!("{html_content}");

    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);
}
