extern crate hyper;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate commercetools;

use std::str::FromStr;
use clap::App;
use commercetools::region::Region;
use commercetools::client::CtpClient;

fn main() {
    env_logger::init().unwrap();
    let matches = App::new("sphere")
        .version("1.0")
        .author("Yann Simon <yann.simon@commercetools.com>")
        .args_from_usage(
            "<PROJECT_KEY> 'project key'
             <CLIENT_ID> 'client ID'
             <CLIENT_SECRET> 'client secret'
             --region=[Europe|NorthAmerica] 'region to use (default to Europe)'")
        .get_matches();

    let project_key = matches.value_of("PROJECT_KEY").unwrap();
    let client_id = matches.value_of("CLIENT_ID").unwrap();
    let client_secret = matches.value_of("CLIENT_SECRET").unwrap();
    let region = matches.value_of("region").map(|s| Region::from_str(s).unwrap()).unwrap_or(Region::Europe);

    let ctp_client = CtpClient::new(&region, project_key, client_id, client_secret);

    let products = ctp_client.get("/products?limit=1").unwrap();
    println!("\nProducts: {}", products);

    let reviews = ctp_client.get("/reviews?limit=1").unwrap();
    println!("\nReviews: {}", reviews);

    let create = false;
    if create {
        let create_review = r#"
        {
          "text": "my review"
        }
        "#;
        let review = ctp_client.post("/reviews", create_review).unwrap();
        println!("\nNew Review: {}", review);
    }
}
