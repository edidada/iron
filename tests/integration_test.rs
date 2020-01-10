//use adders;
extern crate adder;
#[macro_use] extern crate log;

use log::Level;


mod common;

#[test]
fn it_adds_two_with_common() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

#[cfg(test)]
mod tests {
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_works() {
        init();

        info!("This record will be captured by `cargo test`");

        assert_eq!(2, 1 + 1);
    }
}
