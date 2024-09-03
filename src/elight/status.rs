use super::*;

mod special_s;
mod special_s_forward;
mod special_s_end;
mod special_hi_jump;
mod special_lw;
mod special_lw_out;
mod special_lw_standby;

pub fn install(agent: &mut smashline::Agent) {
    special_s::install(agent);
    special_s_forward::install(agent);
    special_s_end::install(agent);
    special_hi_jump::install(agent);
    special_lw::install(agent);
    special_lw_out::install(agent);
    special_lw_standby::install(agent);
}