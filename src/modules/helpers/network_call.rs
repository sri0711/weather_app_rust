use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryOptionValues {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryGeneratorOptions {
    pub url: String,
    pub query: Vec<QueryOptionValues>,
}

pub fn query_generator(options: QueryGeneratorOptions) -> String {
    // Parse the base URL
    let mut url = match Url::parse(&options.url) {
        Ok(url) => url,
        Err(_) => return "Invalid URL".to_string(),
    };

    // Construct query parameters
    let query_params: Vec<(String, String)> = options.query
        .iter()
        .filter_map(|q| {
            match (&q.key, &q.value) {
                (Some(key), Some(value)) => Some((key.clone(), value.clone())),
                _ => None,
            }
        })
        .collect();

    // Add query parameters to the URL
    let query_pairs: Vec<String> = query_params
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();

    if !query_pairs.is_empty() {
        let query_string = query_pairs.join("&");
        url.set_query(Some(&query_string));
    }

    url.to_string()
}
