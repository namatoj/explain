use std::convert::From;
use std::error::Error as ErrorTrait;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug)]
struct WikipediaSummary {
    title: String,
    summary: String,
    url: String,
}

#[derive(Debug)]
enum Error {
    ArticleNotFound,
    UnsuccessfulResponse,
    UrlError,
    ParseError,
}

impl ErrorTrait for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ArticleNotFound => write!(f, "Article not found."),
            Error::UnsuccessfulResponse => write!(f, "Unsuccessful response."),
            Error::UrlError => write!(f, "Url error, can't reach the resource."),
            Error::ParseError => write!(f, "Parse error, couldn't parse url."),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(_err: reqwest::Error) -> Self {
        Error::UrlError
    }
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
        match wikipedia(&word, opt.more) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Usage: explain <concept you want explained>");
    }
}

fn wikipedia(query: &str, long_summary: bool) -> Result<(), Error> {
    let title = get_wikipedia_title(query)?;
    let summary = get_wikipedia_summary(&title, long_summary)?;

    println!("{}", summary);

    Ok(())
}

fn get_wikipedia_title(query: &str) -> Result<String, Error> {
    let url_string = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&utf8=&format=json",
        query
    );

    let resp = reqwest::blocking::get(&url_string)?;

    if !resp.status().is_success() {
        return Err(Error::UnsuccessfulResponse);
    }

    let json: serde_json::Value = resp.json()?;

    Ok(json["query"]["search"][0]["title"]
        .as_str()
        .ok_or_else(|| Error::ArticleNotFound)?
        .to_string())
}

fn get_wikipedia_summary(title: &str, long_summary: bool) -> Result<WikipediaSummary, Error> {
    let underscore_title = title.replace(" ", "_");

    let url = format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        underscore_title
    );

    let resp = reqwest::blocking::get(&url)?;
    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json()?;

    let summary = if long_summary || json["description"].is_null() {
        json["extract"]
            .as_str()
            .ok_or_else(|| Error::ParseError)?
            .to_string()
    } else {
        json["description"]
            .as_str()
            .ok_or_else(|| Error::ParseError)?
            .to_string()
    };

    let url = json["content_urls"]["desktop"]["page"]
        .as_str()
        .ok_or_else(|| Error::ParseError)?
        .to_string();

    Ok(WikipediaSummary {
        title: title.to_string(),
        summary,
        url,
    })
}
