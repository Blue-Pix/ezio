use rand::Rng;
use rand::distributions::{Distribution, Uniform, Standard, Alphanumeric};
use rand_distr;

pub fn random_numbers() {
  let mut rng = rand::thread_rng();
  let n1: u8 = rng.gen();
  let n2: u16 = rng.gen();
  println!("Random u8: {}", n1);
  println!("Random u16: {}", n2);
  println!("Random u32: {}", rng.gen::<u32>());
  println!("Random i32: {}", rng.gen::<i32>());
  println!("Random float: {}", rng.gen::<f64>());
}

pub fn random_numbers_within_range() {
  let mut rng = rand::thread_rng();
  println!("Integer: {}", rng.gen_range(0, 10));
  println!("Float: {}", rng.gen_range(0.0, 10.0));
}

pub fn random_numbers_using_uniform_distribution() {
  let mut rng = rand::thread_rng();
  let die = Uniform::from(1..7);
  loop {
    let throw = die.sample(&mut rng);
    println!("Roll the die: {}", throw);
    if throw == 6 {
      break;
    }
  }
}

pub fn random_numbers_using_normal_distribution() -> Result<(), rand_distr::NormalError> {
  let mut rng = rand::thread_rng();
  let normal = rand_distr::Normal::new(2.0, 3.0)?;
  let v = normal.sample(&mut rng);
  println!("{} is from a N(2, 9) distribution", v);
  Ok(())
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl Distribution<Point> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
    let (rand_x, rand_y) = rng.gen();
    Point {
      x: rand_x,
      y: rand_y,
    }
  }
}

pub fn random_values_with_custom_type() {
  let mut rng = rand::thread_rng();
  let rand_tuple = rng.gen::<(i32, bool, f64)>();
  let rand_point: Point = rng.gen();
  println!("Random tuple: {:?}", rand_tuple);
  println!("Random point: {:?}", rand_point);
}

pub fn random_password() {
  let rand_string: String = rand::thread_rng().sample_iter(&Alphanumeric).take(30).collect();
  println!("Password: {}", rand_string);
}

pub fn random_password_custom() {
  const CHARSET: &[u8] = b"ABCDE123456@-&%";
  const PASSWORD_LEN: usize = 20;
  let mut rng = rand::thread_rng();
  let password: String = (0..PASSWORD_LEN).map(|_| {
      let idx = rng.gen_range(0, CHARSET.len());
      CHARSET[idx] as char
    }).collect();
  println!("Password: {}", password);
}