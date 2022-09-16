mod special_s;
mod special_s_forward;
mod special_s_end;
mod special_hi_jump;
mod special_lw;
mod special_lw_out;
mod special_lw_standby;

pub fn install() {
    special_s::install();
    special_s_forward::install();
    special_s_end::install();
    special_hi_jump::install();
    special_lw::install();
    special_lw_out::install();
    special_lw_standby::install();
}