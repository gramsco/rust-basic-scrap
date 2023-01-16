use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // make an HTTP GET request to the Wikipedia page with information about current conflicts
    let body = reqwest::get("https://en.wikipedia.org/wiki/List_of_ongoing_armed_conflicts")
        .await?
        .text()
        .await?;

    // parse the response body as HTML
    let document = Document::from(body.as_str());

    // find the table with the class "wikitable"
    let table = document.find(Class("wikitable")).next().unwrap();

    // find the rows in the table
    let rows = table.find(Name("tr"));

    // iterate over the rows and print the conflict information
    for row in rows {
        // find the columns in the row
        let mut columns = row.find(Name("td"));

        // get the text of the first and third columns
        let conflict = columns.nth(0).map(|node| node.text()).unwrap_or_default();
        let location = columns.nth(2).map(|node| node.text()).unwrap_or_default();

        // print the conflict and location
        println!("{} - {}", conflict, location);
    }

    Ok(())
}
