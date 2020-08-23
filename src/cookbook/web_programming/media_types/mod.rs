use mime::{Mime, APPLICATION_OCTET_STREAM};
use error_chain::error_chain;
use std::str::FromStr;
use reqwest::header::CONTENT_TYPE;

error_chain! {
  foreign_links {
    Reqwest(reqwest::Error);
    Header(reqwest::header::ToStrError);
    Mime(mime::FromStrError);
  }
}

pub fn get_mime_from_string() {
  let invalid_mime_type = "invalid";
  let default_mime = invalid_mime_type
    .parse::<Mime>()
    .unwrap_or(APPLICATION_OCTET_STREAM);
  println!("MIME for {:?} used default value {:?}", invalid_mime_type, default_mime);

  let valid_mime_type = "TEXT/PLAIN";
  let parsed_mime = valid_mime_type.parse::<Mime>().unwrap_or(APPLICATION_OCTET_STREAM);
  println!("MIME for {:?} was parsed as {:?}", valid_mime_type, parsed_mime);
}

pub fn get_mime_from_file_name() {
    let filename = vec!("foobar.jpg", "foo.bar", "foobar.png");
    for file in filename {
      let mime = find_mimetype(&file.to_owned());
      println!("MIME for {}: {}", file, mime);
    }
}

fn find_mimetype(filename: &String) -> Mime {
  let parts: Vec<&str> = filename.split(".").collect();
  let res = match parts.last() {
    Some(v) => 
      match *v {
        "png" => mime::IMAGE_PNG,
        "jpg" => mime::IMAGE_JPEG,
        "json" => mime::APPLICATION_JSON,
        &_ => mime::TEXT_PLAIN,
      },
    None => mime::TEXT_PLAIN,
  };
  res
}

#[tokio::main]
pub async fn get_mime_from_http_response() -> Result<()> {
  let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png").await?;
  let headers = response.headers();
  match headers.get(CONTENT_TYPE) {
    None => {
      println!("The response does not contain a Content-Type header.");
    },
    Some(content_type) => {
      let content_type = Mime::from_str(content_type.to_str()?)?;
      let media_type = match (content_type.type_(), content_type.subtype()) {
        (mime::TEXT, mime::HTML) => "a HTML Document",
        (mime::TEXT, _) => "a text document",
        (mime::IMAGE, mime::PNG) => "a PNG image",
        (mime::IMAGE, _) => "an image",
        _ => "neither text nor image",
      };
      println!("The response contains {}", media_type);
    }
  }
  Ok(())
}