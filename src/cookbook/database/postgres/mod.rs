use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

pub fn create_table() -> Result<(), Error> {
  let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;
  client.batch_execute("
    CREATE TABLE IF NOT EXISTS author (
      id SERIAL PRIMARY KEY,
      name VARCHAR NOT NULL,
      country VARCHAR NOT NULL
    )
  ")?;

  client.batch_execute("
    CREATE TABLE IF NOT EXISTS book (
      id SERIAL PRIMARY KEY,
      title VARCHAR NOT NULL,
      author_id INTEGER NOT NULL REFERENCES author
    )
  ")?;
  Ok(())
}

struct Author {
  _id: i32,
  name: String,
  country: String,
}

pub fn insert_and_select() -> Result<(), Error> {
  let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;
  let mut authors = HashMap::new();
  authors.insert(String::from("Chinua Archebe"), "Nigeria");
  authors.insert(String::from("Rabindranath Tagore"), "India");
  authors.insert(String::from("Anita Nair"), "India");

  for (key, val) in &authors {
    let author = Author {
      _id: 0,
      name: key.to_string(),
      country: val.to_string(),
    };
    client.execute(
      "INSERT INTO author (name, country) VALUES ($1, $2)",
      &[&author.name, &author.country]
    )?;
  }

  for row in client.query("SELECT id, name, country FROM author", &[])? {
    let author = Author {
      _id: row.get(0),
      name: row.get(1),
      country: row.get(2),
    };
    println!("Author {} is from {}", author.name, author.country);
  }
  Ok(())
}

pub fn aggregate() -> Result<(), Error> {
  let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;
  for row in client.query(
    "SELECT country, COUNT(country) AS count FROM author GROUP BY country ORDER BY count DESC", &[])? {
    
    let (country, count): (Option<String>, Option<i64>) = (row.get(0), row.get(1));
    if country.is_some() && count.is_some() {
      println!("{}: {}", country.unwrap(), count.unwrap());
    }
  }
  Ok(())
}