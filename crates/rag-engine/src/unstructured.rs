use std::error::Error;

use reqwest::{multipart, Client};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MetaData {
    pub filename: String,
    pub filetype: String,
    pub page_number: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Unstructured {
    #[serde(rename(deserialize = "type"))]
    pub type_of: String,
    pub element_id: String,
    pub metadata: MetaData,
    pub text: String,
}

/***
 * Additional Parameters:
 *
 * multipage_sections If True, sections can span multiple pages. Defaults to True.
 *
 * combine_under_n_chars Combines elements (for example a series of titles)
 * until a section reaches a length of n characters. Unstructured Defaults to 500.
 *
 * new_after_n_chars Cuts off new sections once they reach a length of "n" characters.
 * Unstructured Defaults to 1500.
 */
pub async fn document_to_chunks(
    file: Vec<u8>,
    file_name: &str,
    combine_under_n_chars: u32,
    new_after_n_chars: u32,
    multipage_sections: bool,
    unstructured_endpoint: &str,
) -> Result<Vec<Unstructured>, Box<dyn Error>> {
    let client = Client::new();

    let url = format!("{}/general/v0/general", unstructured_endpoint);

    //make form part of file
    let some_file = multipart::Part::bytes(file).file_name(file_name.to_string());
    let title = multipart::Part::text("by_title");
    let combine_under_n_chars = multipart::Part::text(combine_under_n_chars.to_string());
    let new_after_n_chars = multipart::Part::text(new_after_n_chars.to_string());
    let multipage_sections = multipart::Part::text(multipage_sections.to_string());

    //create the multipart form
    let form = multipart::Form::new()
        .part("files", some_file)
        .part("combine_under_n_chars", combine_under_n_chars)
        .part("new_after_n_chars", new_after_n_chars)
        .part("multipage_sections", multipage_sections)
        .part("chunking_strategy", title);

    //send request
    let response = client.post(url).multipart(form).send().await?;
    let text_result = response.text().await?;
    let result = serde_json::from_str(&text_result);

    match result {
        Ok(result) => Ok(result),
        Err(_) => Err(text_result.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_form_file() {
        let file = "Hello World\nline1".as_bytes().to_vec();
        let get_json = document_to_chunks(
            file,
            "hello.txt",
            500,
            1000,
            true,
            "http://chunking-engine:8000",
        )
        .await
        .unwrap();

        println!("{:#?} ", get_json);
    }
}
