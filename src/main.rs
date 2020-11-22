// Goal: CLI application
// call it with:
// % explain <word>
// and the output will be an explanation of the word.

// [x] 1. get our word from the command line
// [x] 2. find and extract the summary from the API response
// [x] 3. print it.

use reqwest::Url;
use std::env;

fn main() {
    if let Some(word) = env::args().nth(1) {
        wikipedia(&word);
    } else {
        println!("Usage: explain <concept you want explained>");
    }
    
}

fn wikipedia(word: &str) {
    let mut url = Url::parse("https://en.wikipedia.org/api/rest_v1/page/summary").unwrap();
    url.path_segments_mut().unwrap().push(word); 

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json().unwrap();
    
//    println!("{}\n", json.get("extract").unwrap().as_str().unwrap());
    println!("{}\n", json["description"].as_str().unwrap());
    println!("{}", json["content_urls"]["desktop"]["page"].as_str().unwrap())
}
