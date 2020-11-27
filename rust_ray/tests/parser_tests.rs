use rust_ray::utils::Parser;
use std::cell::Cell;
use std::collections::HashMap;

#[test]
fn run_commands() {
    let called = Cell::new(false);

    let mut p = Parser::new();
    p.add_cmd("runme", |data| {
        let data: HashMap<String, i32> = data;
        called.set(true);
        assert_eq!(data["val"], 42);

        Ok(())
    });

    p.parse("runme: {val: 42}").expect("dont fail");

    assert!(called.get());
}
