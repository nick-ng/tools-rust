use std::collections::HashMap;
use std::env;

fn jira_request(route: &str, method: &str) -> HashMap<String, String> {
    println!("{}", method);

    let base_url = env::var("JIRA_URL").unwrap();
    let atlassian_user = env::var("ATLASSIAN_USER").unwrap();
    let atlassian_api_token = env::var("ATLASSIAN_API_TOKEN").unwrap();

    let full_url = format!("{}{}", base_url, route);

    println!("full_url: {}", full_url);

    let resp = match method {
        "get" | _ => reqwest::blocking::get(full_url)
            .unwrap()
            .json::<HashMap<String, String>>()
            .unwrap(),
    };

    return resp;
}

fn main() {
    let resp = jira_request("/api/markdown-document/uri/cv", "get");

    println!("Full response: {:?}", resp);
    println!("Keys: {:?}", resp.keys());
    println!("title: {:?}", resp.get("title").unwrap());

    let a = env::var("JIRA_URL").unwrap();

    println!("{:?}", a);
}
