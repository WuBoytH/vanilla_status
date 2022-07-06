#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null
)]

mod table_const;
mod sonic;
mod lucario;

#[skyline::main(name = "vanilla_status")]
pub fn main() {
    sonic::install();
    lucario::install();
}