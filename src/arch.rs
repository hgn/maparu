extern crate num_cpus;

#[allow(dead_code)]
pub fn cpus() -> usize {
    return num_cpus::get_physical();
}
