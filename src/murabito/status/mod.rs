use super::*;

pub unsafe extern "C" fn special_s_shoot_clayrocket(fighter: &mut L2CFighterCommon, status: L2CValue) {
    WorkModule::set_int(fighter.module_accessor, status.get_i32(), *FIGHTER_MURABITO_STATUS_SPECIAL_S_INT_SHOOT_STATUS);

    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
}

pub unsafe extern "C" fn special_s_end_common(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP
    ].contains(&status)
    && LinkModule::is_link(fighter.module_accessor, *FIGHTER_MURABITO_LINK_NO_CLAYROCKET) {
        LinkModule::unlink(fighter.module_accessor, *FIGHTER_MURABITO_LINK_NO_CLAYROCKET);
    }
}

mod special_s;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
}