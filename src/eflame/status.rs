mod special_lw;
mod special_lw_out;
mod special_lw_standby;

pub fn install() {
    special_lw::install();
    special_lw_out::install();
    special_lw_standby::install();
}