#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use commercetools::client::CtpClient;
use commercetools::region::Region;
use std::collections::HashMap;
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Reference {
    pub typeId: String,
    pub id: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ProductVariant {
    pub id: u64,
    pub sku: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ProductData {
    pub name: HashMap<String, String>,
    pub categories: Vec<Reference>,
    pub description: Option<HashMap<String, String>>,
    pub slug: HashMap<String, String>,
    pub metaTitle: Option<HashMap<String, String>>,
    pub metaDescription: Option<HashMap<String, String>>,
    pub metaKeywords: Option<HashMap<String, String>>,
    pub masterVariant: ProductVariant,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ProductCatalogData {
    pub published: bool,
    pub hasStagedChanges: bool,
    pub current: ProductData,
    pub staged: ProductData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: String,
    pub version: u64,
    pub createdAt: String,
    pub lastModifiedAt: String,
    pub masterData: ProductCatalogData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Review {
    pub id: String,
    pub version: u64,
    pub createdAt: String,
    pub lastModifiedAt: String,
    pub text: Option<String>,
}

fn main() {
    env_logger::init().unwrap();
    let matches = App::new("sphere")
        .version("1.0")
        .author("Yann Simon <yann.simon@commercetools.com>")
        .args_from_usage(
            "<PROJECT_KEY> 'project key' \n\
             <CLIENT_ID> 'client ID' \n\
             <CLIENT_SECRET> 'client secret' \n\
             --region=[Europe|NorthAmerica] 'region to use (default to Europe)'",
        )
        .arg(
            Arg::with_name("permissions")
                .short("p")
                .long("permission")
                .help("permissions (default to manage_project)")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let project_key = matches.value_of("PROJECT_KEY").unwrap();
    let client_id = matches.value_of("CLIENT_ID").unwrap();
    let client_secret = matches.value_of("CLIENT_SECRET").unwrap();
    let region = matches
        .value_of("region")
        .map(|s| Region::from_str(s).unwrap())
        .unwrap_or(Region::Europe);
    let permissions: Vec<&str> = if matches.is_present("permissions") {
        matches.values_of("permissions").unwrap().collect()
    } else {
        vec!["manage_project"]
    };

    let mut ctp_client = CtpClient::new(&region, project_key, client_id, client_secret)
        .with_permissions(&permissions);

    // simple GET call
    let mut products = ctp_client.get("/products?limit=1").unwrap();
    println!("\nProducts: {}", products.body_as_string().unwrap());

    // paged result of products
    let products2 = ctp_client.list::<Product>("products").unwrap();
    println!(
        "\nList of product ids: {:?}",
        products2
            .results
            .iter()
            .map(|p| &p.id)
            .collect::<Vec<&String>>()
    );

    println!("\nFirst product: {:?}", &products2.results.first());

    // read reviews
    let mut reviews = ctp_client.get("/reviews?limit=1").unwrap();
    println!("\nReviews: {}", reviews.body_as_string().unwrap());

    let create = permissions
        .iter()
        .any(|&p| p == "manage_project" || p == "manage_products");
    if create {
        // create and delete a review
        let create_review = r#"
        {
          "text": "my review"
        }
        "#;
        let mut review_call = ctp_client.post("/reviews", create_review).unwrap();
        let review = review_call.body_as::<Review>().unwrap();
        println!("\n[{}] New Review: {:?}", &review_call.status(), review);

        let url = format!("/reviews/{}?version={}", review.id, review.version);
        let mut deleted_review = ctp_client.delete(&url).unwrap();
        println!(
            "\n[{}] Deleted Review: {:?}",
            &deleted_review.status(),
            deleted_review.body_as_string().unwrap()
        );
    }

    // read products IDs with a Graph QL query
    let query = r#"
    {
      products {
        results {
          id
        }
      }
    }
    "#;
    let mut graphql = ctp_client.graphql(query).unwrap();
    println!("\nGraphQL: {}", graphql.body_as_string().unwrap());
}
