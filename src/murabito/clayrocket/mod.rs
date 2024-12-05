use super::*;

mod vtable_hook;

pub fn install() {
    vtable_hook::install();
}