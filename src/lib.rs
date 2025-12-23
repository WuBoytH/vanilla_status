#![feature(
    proc_macro_hygiene,
    simd_ffi
)]
#![allow(
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::missing_safety_doc
)]

use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    table_const::*
};

mod zelda;
mod ike;
mod sonic;
mod lucario;
mod murabito;
mod buddy;
mod edge;
pub mod element;
mod eflame;
mod elight;
mod purin;
mod link;

#[skyline::main(name = "vanilla_status")]
pub fn main() {
    zelda::install();
    ike::install();
    sonic::install();
    lucario::install();
    murabito::install();
    buddy::install();
    edge::install();
    eflame::install();
    elight::install();
    purin::install();
    link::install();
}