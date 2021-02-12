pub mod stone;
pub mod action;
pub mod gostate;
pub mod board;
mod stone_group;
mod cell;
mod constants;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
