extern crate num_cpus;

pub fn cpus() -> usize {
   return num_cpus::get_physical();
}
