use ebay_easy_api::{models::Marketplace, EbayApiClient};
use std::env::var;

#[test]
fn empty_search() {
    let client =
        EbayApiClient::new_unchecked(var("EBAY_TOKEN").unwrap(), Marketplace::UnitedStates);

    let browser = client.search();
    let results = browser.search("kjefbasrfbsifebfoubefiuwbfwuf", 10).unwrap();

    assert!(results.items.is_empty());
    assert_eq!(results.total, 0);
    assert_eq!(results.len(), 0);
    assert!(results.is_empty());
}
