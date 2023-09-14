use reqwest::{multipart, Client};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct MetaData {
    pub filename: String,
    pub filetype: String,
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

pub async fn call_unstructured_api(
    file: Vec<u8>,
    file_name: &str,
) -> Result<Vec<Unstructured>, reqwest::Error> {
    let client = Client::new();

    let unstructured_endpoint = if let Ok(domain) = std::env::var("UNSTRUCTURED_ENDPOINT") {
        domain
    } else {
        "http://unstructured:8000".to_string()
    };

    let url = format!("{}/general/v0/general", unstructured_endpoint);

    //make form part of file
    let some_file = multipart::Part::bytes(file).file_name(file_name.to_string());

    //create the multipart form
    let form = multipart::Form::new().part("files", some_file);

    //send request
    let response = client.post(url).multipart(form).send().await?;
    let result = response.json::<Vec<Unstructured>>().await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_form_file() {
        let file = "Hello World\nline1".as_bytes().to_vec();
        let get_json = call_unstructured_api(file, "hello.txt").await.unwrap();

        println!("{:#?}", get_json);
    }
}
