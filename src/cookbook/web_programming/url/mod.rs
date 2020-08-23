use url::{Url, ParseError, Origin, Host, Position};
use error_chain::error_chain;

error_chain! {
  foreign_links {
    UrlParse(ParseError);
  }
  errors {
    CannotBeABase
  }
}

pub fn parse_url() -> Result<()> {
  let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
  let parsed = Url::parse(s)?;
  println!("{:#?}", parsed);
  println!("The path part of the URL is: {}", parsed.path());
  Ok(())
}

pub fn create_base_url() -> Result<()> {
  let full = "https://github.com/rust-lang/cargo?asdf";
  let url = Url::parse(full)?;
  let base = base_url(url)?;
  println!("{}", base);
  Ok(())
}

fn base_url(mut url: Url) -> Result<Url> {
  match url.path_segments_mut() {
    Ok(mut path) => {
      path.clear();
    },
    Err(_) => {
      return Err(Error::from_kind(ErrorKind::CannotBeABase));
    }
  }
  url.set_query(None);
  Ok(url)
}

pub fn create_url_from_base() -> Result<()> {
  let path = "/rust-lang/cargo";
  let gh = build_github_url(path)?;
  println!("{}", gh);
  Ok(())
}

fn build_github_url(path: &str) -> Result<Url> {
  const GITHUB: &'static str = "https://github.com";
  let base = Url::parse(GITHUB)?;
  let joined = base.join(path)?;
  Ok(joined)
}

pub fn url_struct() -> Result<()> {
  let s = "ftp://rust-lang.org/examples";
  let url = Url::parse(s)?;
  println!("{}", url.scheme());
  println!("{:?}", url.host());
  println!("{:?}", url.port_or_known_default());
  Ok(())
}

pub fn origin_url() -> Result<()> {
  let s = "ftp://rust-lang.org/examples";
  let url = Url::parse(s)?;
  let scheme = "ftp".to_owned();
  let host = Host::Domain("rust-lang.org".to_owned());
  let port = 21;
  let expected = Origin::Tuple(scheme, host, port);
  let origin = url.origin();
  println!("{:?}", expected);
  println!("{:?}", origin);
  Ok(())
}

pub fn remove_fragment() -> Result<()> {
  let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
  let cleaned: &str = &parsed[..Position::AfterPath];
  println!("{}", cleaned);
  Ok(())
}