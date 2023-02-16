//run-rustfix
#![warn(clippy::unnecessary_literal_unwrap)]
#![allow(clippy::unnecessary_lazy_evaluations)]

fn unwrap_option() {
    let _val = Some(1).unwrap();
    let _val = Some(1).expect("this never happens");
}

fn unwrap_result() {
    let _val = Ok::<usize, ()>(1).unwrap();
    let _val = Ok::<usize, ()>(1).expect("this never happens");
    // let val = Err(1).unwrap_err();
}

fn unwrap_methods_option() {
    let _val = Some(1).unwrap_or(2);
    let _val = Some(1).unwrap_or_default();
    let _val = Some(1).unwrap_or_else(|| _val);
}

fn unwrap_methods_result() {
    let _val = Ok::<usize, ()>(1).unwrap_or(2);
    let _val = Ok::<usize, ()>(1).unwrap_or_default();
    let _val = Ok::<usize, ()>(1).unwrap_or_else(|()| _val);
}

fn main() {
    unwrap_option();
    unwrap_result();
    unwrap_methods_option();
    unwrap_methods_result();
}
