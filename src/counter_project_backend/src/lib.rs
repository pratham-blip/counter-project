use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
}

#[query]
fn get_counter() -> u64 {
    COUNTER.with(|counter| *counter.borrow())
}

#[update]
fn increment_counter() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}

#[update]
fn decrement_counter() {
    COUNTER.with(|counter| {
        let mut value = counter.borrow_mut();
        if *value > 0 {
            *value -= 1;
        }
    });
}

#[update]
fn reset_counter() {
    COUNTER.with(|counter| *counter.borrow_mut() = 0);
}
