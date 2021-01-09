pub mod melee;
pub mod types;

#[cfg(feature = "console")]
pub mod console;

mod error;
pub use self::error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
