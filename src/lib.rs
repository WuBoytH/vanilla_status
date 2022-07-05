#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod table_const;
mod sonic;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    sonic::install();
}