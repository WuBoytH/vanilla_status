mod special_lw;
mod special_lw_out;
mod special_lw_standby;

pub fn install(agent: &mut smashline::Agent) {
    special_lw::install(agent);
    special_lw_out::install(agent);
    special_lw_standby::install(agent);
}