use std::cell::RefCell

thread_local! {
    static msg: RefCell<String> = RefCell::new(String::new());
}
fn get() {
    let a =String::from("123");
    let b =a.clone();
    let c=a;
    msg.with(|msg| (*msg.borrow()).clone())
}

#[ic_cdk::query]
fn get_msg() -> String {
    msg.with(|msg| (*msg.borrow()).clone())
}

#[ic_cdk::update]
fn set_msg(new_msg: String) {
    msg.with(|msg| (*msg.borrow_mut).clone())
}
fn set(n: Nat) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    COUNTER.with(|count| *count.borrow_mut() = n);
}

