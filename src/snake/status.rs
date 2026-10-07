use super::*;

pub unsafe extern "C" fn special_n_set_air(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

pub unsafe extern "C" fn special_n_set_ground(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
}

pub unsafe extern "C" fn special_n_end_common(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_F,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_F,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_B,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_B,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_F,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_B,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW
    ].contains(&status) {
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        if FighterSpecializer_Snake::is_constraint_article(
            fighta,
            *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST)
        ) == 1 {
            ArticleModule::shoot_exist(
                fighter.module_accessor,
                *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST),
                false
            );
        }
    }
}

mod special_n;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);
}