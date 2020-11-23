// Goal: CLI application
// call it with:
// % explain <word>
// and the output will be an explanation of the word.

// [x] 1. get our word from the command line
// [x] 2. find and extract the summary from the API response
// [x] 3. print it.

use reqwest::Url;
use std::{env, fmt};


#[derive(Debug)]
struct WikipediaSummary {
    title: String,
    summary: String,
    url: String,
}

impl fmt::Display for WikipediaSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[1m\x1b[4m{}\x1b[0m: {} \n\n{}", self.title, self.summary, self.url)
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 0 {
        let word = args.join("_");
        wikipedia(&word);
    } else {
        println!("Usage: explain <concept you want explained>");
    }
    
}

fn wikipedia(query: &str) {

    let title = get_wikipedia_title(query);
    let summary = get_wikipedia_summary(&title);

    println!("{}", summary);
}

fn get_wikipedia_title(query: &str) -> String {
    let url_string = format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&utf8=&format=json", query);

    let resp = reqwest::blocking::get(&url_string).unwrap();
    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json().unwrap();
   
    json["query"]["search"][0]["title"].as_str().unwrap().to_string()
}


fn get_wikipedia_summary(title: &str) -> WikipediaSummary {
    let underscore_title = title.replace(" ", "_");
        
    let mut url = Url::parse("https://en.wikipedia.org/api/rest_v1/page/summary").unwrap();
    url.path_segments_mut().unwrap().push(&underscore_title); 

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json().unwrap();
   
    let summary;
    match json["description"].as_str() {
        Some(value) => summary = value.to_string(),
        None => summary = json["extract"].as_str().unwrap().to_string()
    }


    WikipediaSummary {
        title: title.to_string(),
        summary, 
        url: json["content_urls"]["desktop"]["page"].as_str().unwrap().to_string(),
    }
    
}

