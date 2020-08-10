pub fn sort_a_vector_of_integers() {
  let mut vec = vec![1, 5, 10, 2, 15];
  vec.sort();
  println!("{:?}", vec);
}

pub fn sort_a_vector_of_floats() {
  let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
  vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
  println!("{:?}", vec);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
  name: String,
  age: u32,
}

impl Person {
  pub fn new(name: String, age: u32) -> Person {
    Person {
      name,
      age
    }
  }
}

pub fn sort_a_vector_of_struct() {
  let mut people = vec![
    Person::new("Zoe".to_string(), 25),
    Person::new("Al".to_string(), 60),
    Person::new("Al".to_string(), 59),
    Person::new("John".to_string(), 1),
  ];
  people.sort();
  println!("{:?}", people);
  people.sort_by(|a, b| b.age.cmp(&a.age));
  println!("{:?}", people);
}

