use num_cpus;

pub fn count_cpu_cores() {
  println!("Number of logical cores is {}", num_cpus::get());
}