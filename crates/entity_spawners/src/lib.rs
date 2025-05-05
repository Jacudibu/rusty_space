pub mod spawn_asteroid;
mod spawn_celestial;
pub mod spawn_gates;
pub mod spawn_sector;
pub mod spawn_ship;
pub mod spawn_station;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
