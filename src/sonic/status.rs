mod special_s_turn;
mod special_lw;

pub fn install(agent: &mut smashline::Agent) {
    special_s_turn::install(agent);
    special_lw::install(agent);
}