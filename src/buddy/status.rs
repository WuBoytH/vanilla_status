use super::*;

pub mod helper;
mod special_s;
mod special_s_dash;
mod special_s_end;
mod special_s_wall;
mod special_s_fail;

pub fn install(agent: &mut smashline::Agent) {
    special_s::install(agent);
    special_s_dash::install(agent);
    special_s_end::install(agent);
    special_s_wall::install(agent);
    special_s_fail::install(agent);
}