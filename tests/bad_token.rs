use ebay_easy_api::{error::Error, models::Marketplace, EbayApiClient};

#[test]
fn bad_token() {
    let client = EbayApiClient::new("0", Marketplace::UnitedStates);

    let Err(error) = client else {
        panic!("Did not get an error");
    };

    let Error::Api(api_err) = error else {
        panic!("Did not get an API error")
    };

    assert_eq!(api_err.id, 1001);
    assert_eq!(api_err.domain.as_ref(), "OAuth");
    assert_eq!(api_err.category.as_ref(), "REQUEST");
    assert_eq!(api_err.message.as_ref(), "Invalid access token");
    assert_eq!(
        api_err.long_message.as_ref(),
        "Invalid access token. Check the value of the Authorization HTTP request header."
    );
}
