use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{Url, Position};
use reqwest::StatusCode;
use lazy_static::lazy_static;
use std::borrow::Cow;
use regex::Regex;

error_chain! {
  foreign_links {
    ReqError(reqwest::Error);
    IoError(std::io::Error);
    UrlParseError(url::ParseError);
    JoinError(tokio::task::JoinError);
  }
}
 
#[tokio::main] 
pub async fn extract_url() -> Result<()> {
  let res = reqwest::get("https://www.rust-lang.org/en-US/")
    .await?
    .text()
    .await?;

  Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| println!("{}", x));

  Ok(())
}

#[tokio::main]
pub async fn check_broken_links() -> Result<()> {
  let url = Url::parse("https://www.rust-lang.org/en-US/")?;
  let res = reqwest::get(url.as_ref()).await?.text().await?;
  let document = Document::from(res.as_str());
  let base_url = get_base_url(&url, &document).await?;
  let base_parser = Url::options().base_url(Some(&base_url));
  let links: HashSet<Url> = document
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .filter_map(|link| base_parser.parse(link).ok())
    .collect();
  let mut tasks = vec![];

  for link in links {
    tasks.push(tokio::spawn(async move {
      if check_link(&link).await.unwrap() {
        println!("{} is OK", link);
      } else {
        println!("{} is Broken", link);
      }
    }));
  }

  for task in tasks {
    task.await?
  }
  Ok(())
}

async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
  let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
  let base_url = base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
  Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool> {
  let res = reqwest::get(url.as_ref()).await?;
  Ok(res.status() != StatusCode::NOT_FOUND)
}

#[tokio::main]
pub async fn extract_all_links() -> Result<()> {
  let content = reqwest::get("https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw")
    .await?
    .text()
    .await?;
  println!("{:#?}", extract_links(content.as_str()));
  Ok(())
}

fn extract_links(content: &str) -> HashSet<Cow<str>> {
  lazy_static! {
    static ref WIKI_REGEX: Regex = Regex::new(
      r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
    ).unwrap();
  }

  let links: HashSet<_> = WIKI_REGEX
    .captures_iter(content)
    .map(|c| match (c.name("internal"), c.name("external")) {
      (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
      (None, Some(val)) => Cow::from(val.as_str()),
      _ => unreachable!(),
    }).collect();
  links
}