use error_chain::error_chain;
use std::io::Read;
use serde::Deserialize;
use reqwest::header::{USER_AGENT, CONTENT_LENGTH, RANGE, HeaderValue};
use std::time::Duration;
use reqwest::{Client, ClientBuilder, StatusCode};
use serde_json::json;
use std::env;
use tempfile::Builder;
use std::io::copy;
use std::fs::File;
use std::str::FromStr;

error_chain! {
  foreign_links {
    Io(std::io::Error);
    HttpRequest(reqwest::Error);
    EnvVar(env::VarError);
    Header(reqwest::header::ToStrError);
  }
}

pub fn make_get_request() -> Result<()> {
  let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
  let mut body = String::new();
  res.read_to_string(&mut body)?;
  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());
  println!("Body:\n{}", body);
  Ok(())
}

#[tokio::main]
pub async fn async_get() -> Result<()> {
  let res = reqwest::get("http://httpbin.org/get").await?;
  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());
  let body = res.text().await?;
  println!("Body:\n{}", body);
  Ok(())
}

#[derive(Deserialize, Debug)]
struct User {
  login: String,
  id: u32,
}

#[tokio::main]
pub async fn query_github_api() -> Result<()> {
  let request_url = format!(
    "https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner = "rust-lang-nursery",
    repo = "rust-cookbook"
  );
  println!("{}", request_url);
  let client = reqwest::Client::new().get(&request_url).header(USER_AGENT, "my-rust");
  let response = client.send().await?;
  let users: Vec<User> = response.json().await?;
  println!("{:?}", users);
  Ok(())
}

#[tokio::main]
pub async fn check_resource_exits() -> Result<()> {
  let user = "ferris_the-crab";
  let request_url = format!("https://api.github.com/users/{}", user);
  println!("{}", request_url);
  let timeout = Duration ::new(5, 0);
  let client = ClientBuilder::new().timeout(timeout).build()?;
  let response = client.head(&request_url).header(USER_AGENT, "my-rust").send().await?;

  if response.status().is_success() {
    println!("{} is a user!", user);
  } else {
    println!("{} is not a user", user);
  }
  Ok(())
}

#[tokio::main]
pub async fn handle_gist() -> Result<()> {
  let gh_user = env::var("GH_USER")?;
  let gh_pass = env::var("GH_PASS")?;
  let gist_body = json!({
    "description": "the description for this gist",
    "public": true,
    "files": {
      "main.rs": {
        "content": r#"fn main() { println!("hello world"); }"#
      }
    }
  });

  let request_url = "https://api.github.com/gists";
  let response = Client::new()
    .post(request_url)
    .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
    .json(&gist_body)
    .header(USER_AGENT, "my-rust")
    .send()
    .await?;

  let gist: Gist = response.json().await?;
  println!("Created {:?}", gist);

  let request_url = format!("{}/{}", request_url, gist.id);
  let response = Client::new()
    .delete(&request_url)
    .basic_auth(gh_user, Some(gh_pass))
    .send()
    .await?;
  println!("Gist {} deleted! Status code: {}", gist.id, response.status());
  Ok(())
}

#[derive(Deserialize, Debug)]
struct Gist {
  id: String,
  html_url: String,
}

pub fn iterate_page() -> Result<()> {
  for dep in ReverseDependencies::of("serde")? {
    println!("reverse dependencies: {:#?}", dep);
  }
  Ok(())
}

struct ReverseDependencies {
  crate_id: String,
  dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
  client: reqwest::blocking::Client,
  page: u32,
  per_page: u32,
  total: u32,
}

#[derive(Debug, Deserialize)]
struct Dependency {
  crate_id: String
}

impl ReverseDependencies {
  fn of(crate_id: &str) -> Result<Self> {
    Ok(ReverseDependencies {
      crate_id: crate_id.to_owned(),
      dependencies: vec![].into_iter(),
      client: reqwest::blocking::Client::new(),
      page: 0,
      per_page: 100,
      total: 0
    })
  }

  fn try_next(&mut self) -> Result<Option<Dependency>> {
    if let Some(dep) = self.dependencies.next() {
      return Ok(Some(dep));
    }

    if self.page > 0 && self.page * self.per_page >= self.total {
      return Ok(None);
    }

    self.page += 1;
    let url = format!(
      "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
      self.crate_id,
      self.page,
      self.per_page
    );
    let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
    self.dependencies = response.dependencies.into_iter();
    self.total = response.meta.total;
    Ok(self.dependencies.next())
  }
}

impl Iterator for ReverseDependencies {
  type Item = Result<Dependency>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.try_next() {
      Ok(Some(dep)) => Some(Ok(dep)),
      Ok(None) => None,
      Err(err) => Some(Err(err)),
    }
  }
}

#[derive(Deserialize)]
struct ApiResponse {
  dependencies: Vec<Dependency>,
  meta: Meta,
}

#[derive(Deserialize)]
struct Meta {
  total: u32,
}

#[tokio::main]
pub async fn download_file() -> Result<()> {
  let tmp_dir = Builder::new().prefix("example").tempdir()?;
  let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
  let response = reqwest::get(target).await?;
  let mut dest = {
    let fname = response
      .url()
      .path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| if name.is_empty() { None } else { Some(name) } )
      .unwrap_or("tmp.bin");

    println!("file to download: '{}'", fname);
    let fname = tmp_dir.path().join(fname);
    println!("will be located under: '{:?}'", fname);
    File::create(fname)?
  };
  let content = response.text().await?;
  copy(&mut content.as_bytes(), &mut dest)?;
  Ok(())
}

#[tokio::main]
pub async fn post_file() -> Result<()> {
  let paste_api = "https://paste.rs";
  let mut file = File::open("message")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let client = reqwest::Client::new();
  let res = client.post(paste_api)
    .body(contents)
    .send()
    .await?;
  let response_text = res.text().await?;
  println!("Your paste is located at: {}", response_text);
  Ok(())
}

pub fn partial_download() -> Result<()> {
  let url = "https://httpbin.org/range/102400?duration=2";
  const CHUNK_SIZE: u32 = 10240;

  let client = reqwest::blocking::Client::new();
  let response = client.head(url).send()?;
  let length = response.headers().get(CONTENT_LENGTH).ok_or("response doesn't include the content length")?;
  let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
  let mut output_file = File::create("download.bin")?;

  println!("Starting download...");
  for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
    println!("range {:?}", range);
    let mut response = client.get(url).header(RANGE, range).send()?;
    let status = response.status();
    if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
      error_chain::bail!("Unexpected server response: {}", status)
    }
    copy(&mut response, &mut output_file)?;
  }

  let content = response.text()?;
  copy(&mut content.as_bytes(), &mut output_file)?;
  println!("Finished with success!");
  Ok(())
}

struct PartialRangeIter {
  start: u64,
  end: u64,
  buffer_size: u32
}

impl PartialRangeIter {
  pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
    if buffer_size == 0 {
      Err("invalid buffer_size, give a value greater than zero.")?;
    }
    Ok(PartialRangeIter {
      start,
      end,
      buffer_size,
    })
  }
}

impl Iterator for PartialRangeIter {
  type Item = HeaderValue;
  fn next(&mut self) -> Option<Self::Item> {
    if self.start > self.end {
      None
    } else {
      let prev_start = self.start;
      self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
      Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
    }
  }
}