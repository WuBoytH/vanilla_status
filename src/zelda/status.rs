use super::*;

mod special_hi;
mod special_hi_2;
mod special_hi_3;

pub fn install(agent: &mut smashline::Agent) {
    special_hi::install(agent);
    special_hi_2::install(agent);
    special_hi_3::install(agent);
}