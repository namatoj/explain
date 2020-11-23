// Goal: CLI application
// call it with:
// % explain <word>
// and the output will be an explanation of the word.

// [x] 1. get our word from the command line
// [x] 2. find and extract the summary from the API response
// [x] 3. print it.

use reqwest::Url;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug)]
struct WikipediaSummary {
    title: String,
    summary: String,
    url: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, help = "flag for a more verbose explanation.")]
    more: bool,

    #[structopt(help = "the query to be explained")]
    query: Vec<String>,
}

impl fmt::Display for WikipediaSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1b[1m\x1b[4m{}\x1b[0m: {} \n\n{}",
            self.title, self.summary, self.url
        )
    }
}

fn main() {
    let opt = Opt::from_args();

    if !opt.query.is_empty() {
        let word = opt.query.join("_");
        wikipedia(&word, opt.more);
    } else {
        println!("Usage: explain <concept you want explained>");
    }
}

fn wikipedia(query: &str, long_summary: bool) {
    let title = get_wikipedia_title(query);
    let summary = get_wikipedia_summary(&title, long_summary);

    println!("{}", summary);
}

fn get_wikipedia_title(query: &str) -> String {
    let url_string = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&utf8=&format=json",
        query
    );

    let resp = reqwest::blocking::get(&url_string).unwrap();
    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json().unwrap();

    json["query"]["search"][0]["title"]
        .as_str()
        .unwrap()
        .to_string()
}

fn get_wikipedia_summary(title: &str, long_summary: bool) -> WikipediaSummary {
    let underscore_title = title.replace(" ", "_");

    let mut url = Url::parse("https://en.wikipedia.org/api/rest_v1/page/summary").unwrap();
    url.path_segments_mut().unwrap().push(&underscore_title);

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json().unwrap();

    println!("{:?}", long_summary);
    let summary = if long_summary || json["description"].is_null() {
        json["extract"].as_str().unwrap().to_string()
    } else {
        json["description"].as_str().unwrap().to_string()
    };

    WikipediaSummary {
        title: title.to_string(),
        summary,
        url: json["content_urls"]["desktop"]["page"]
            .as_str()
            .unwrap()
            .to_string(),
    }
}
