mod special_n;
mod special_n_hold;
mod special_n_shoot;
pub mod helper;

pub fn install() {
    special_n::install();
    special_n_hold::install();
    special_n_shoot::install();
}