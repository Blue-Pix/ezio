use rayon::prelude::*;
use rand::Rng;
use rand::distributions::Alphanumeric;
use image::imageops::FilterType;
use image::ImageError;
use error_chain::{error_chain, ChainedError};
use glob::{glob_with, MatchOptions};
use std::fs::create_dir_all;
use std::path::Path;

error_chain! {
  foreign_links {
    Image(ImageError);
    Io(std::io::Error);
    Glob(glob::PatternError);
  }
}

pub fn parallel() {
  let mut arr = [0, 7, 9, 11];
  arr.par_iter_mut().for_each(|p| *p -= 1);
  println!("{:?}", arr);
}

pub fn test_in_parallel() {
  let mut vec = vec![2, 4, 6, 8];
  assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
  assert!(vec.par_iter().all(|n| (*n % 2) == 0));
  assert!(!vec.par_iter().any(|n| *n > 8));
  assert!(vec.par_iter().all(|n| *n <= 8));

  vec.push(9);

  assert!(vec.par_iter().any(|n| (*n % 2) != 0));
  assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
  assert!(vec.par_iter().any(|n| *n > 8));
  assert!(!vec.par_iter().all(|n| *n <= 8));
}

pub fn parallel_find() {
  let v = vec![6, 2, 1, 9, 3, 8, 11];
  let f1 = v.par_iter().find_any(|&&x| x == 9);
  let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
  let f3 = v.par_iter().find_any(|&&x| x > 8);
  println!("{}", f1.unwrap());
  println!("{}", f2.unwrap());
  println!("{}", f3.unwrap());
}

pub fn parallel_sort() {
  let mut vec = vec![String::new(); 100_000];
  vec.par_iter_mut().for_each(|p| {
    let mut rng = rand::thread_rng();
    *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect();
  });
  vec.par_sort_unstable();
  println!("{:?}", vec);
}

struct Person {
  age: u32,
}

pub fn parallel_map_reduce() { 
  let v: Vec<Person> = vec![
    Person { age: 23 },
    Person { age: 19 },
    Person { age: 42 },
    Person { age: 17 },
    Person { age: 17 },
    Person { age: 31 },
    Person { age: 30 },
  ];

  let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
  let sum_over_30 = v.par_iter()
                     .map(|x| x.age)
                     .filter(|&x| x > 30)
                     .reduce(|| 0, |x, y| x + y);
  let alt_sum_30: u32 = v.par_iter()
                         .map(|x| x.age)
                         .filter(|&x| x > 30)
                         .sum();
  println!("{}", sum_over_30 as f32 / num_over_30);
  println!("{}", alt_sum_30 as f32 / num_over_30);
}

pub fn parallel_gen_thumbnail() -> Result<()> {
  let options: MatchOptions = Default::default();
  let files: Vec<_> = glob_with("*.jpg", options)?
                          .filter_map(|x| x.ok())
                          .collect();

  if files.len() == 0 {
    error_chain::bail!("No .jpg files found in current directory");
  }

  let thumb_dir = "thumbnails";
  create_dir_all(thumb_dir)?;

  let image_failures: Vec<_> = files
    .par_iter()
    .map(|path| {
      make_thumbnail(path, thumb_dir, 300)
        .map_err(|e| e.chain_err(|| path.display().to_string()))
    })
    .filter_map(|x| x.err())
    .collect();

  image_failures.iter().for_each(|x| println!("{}", x.display_chain()));
  println!("{} thumbnails saved successfully", files.len() - image_failures.len());
  Ok(())
}

fn make_thumbnail<PA, PB>(
  original: PA, thumb_dir: PB, longest_edge: u32
) -> Result<()>
where 
  PA: AsRef<Path>,
  PB: AsRef<Path>
{
  let image = image::open(original.as_ref())?;
  let file_path = thumb_dir.as_ref().join(original);
  Ok(image.resize(longest_edge, longest_edge, FilterType::Nearest).save(file_path)?)
}

