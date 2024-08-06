use ebay_easy_api::{models::marketplace::Marketplace, EbayApiClient};
use std::env;

fn main() {
    let token = env::var("EBAY_TOKEN").expect("Missing ebay API token");
    let marketplace = Marketplace::UnitedStates;
    let client = EbayApiClient::new(token, marketplace).unwrap();

    let searcher = client.search();
    let results = searcher.search("omnron plc", 3).unwrap();

    println!("{results:#?}");
}
