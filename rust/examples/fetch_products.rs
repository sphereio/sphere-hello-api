extern crate hyper;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate commercetools;

use std::str::FromStr;
use clap::App;
use commercetools::region::Region;

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

    let ctp_client = commercetools::client::CtpClient::new(&region, project_key, client_id, client_secret);
    let products = ctp_client.request("/products?limit=1");
    println!("\nProducts: {}", products);

    let categories = ctp_client.request("/categories?limit=1");
    println!("\nCategories: {}", categories);

}
