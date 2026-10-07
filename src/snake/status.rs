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

unsafe extern "C" fn special_n_common_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if !param_2.get_bool() {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            WorkModule::set_float(fighter.module_accessor, stick_x * lr, *FIGHTER_SNAKE_STATUS_SPECIAL_N_HOLD_WAIT_WORK_FLOAT_THROW_RATE);

            fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
            fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));

            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW.into(), true.into());
        }
    }

    0.into()
}

unsafe extern "C" fn special_n_main_loop_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    0.into()
}

unsafe extern "C" fn special_n_init_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_WAIT
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_F,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_WALK_F
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_B,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_WALK_B
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_F,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_WALK_BRAKE_F
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_B,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_WALK_BRAKE_B
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_F,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_DASH_F
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_B,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_DASH_B
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_AIR
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_JUMP
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_JUMP_SQUAT
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_JUMP_AERIAL
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_LANDING
    );
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL,
        *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_FLY
    );

    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL {
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_n_jump_aerial"),
            0.0,
            rate,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        fighter.sub_ftStatusUniqProcessShoot_initShoot_Common(
            Hash40::new("special_n_hold").into(),
            Hash40::new("special_air_n_hold").into()
        );
    }
    0.into()
}

unsafe extern "C" fn special_n_exec_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessShoot_execShoot_Common();
    0.into()
}

mod special_n;
mod special_n_hold_wait;
mod special_n_hold_walk_f;
mod special_n_hold_walk_b;
mod special_n_hold_walk_brake_f;
mod special_n_hold_walk_brake_b;
mod special_n_hold_dash_f;
mod special_n_hold_dash_b;
mod special_n_hold_jump_squat;
mod special_n_hold_jump;
mod special_n_hold_jump_aerial;
mod special_n_hold_air;
mod special_n_hold_landing;
mod special_n_throw;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);
    special_n_hold_wait::install(agent);
    special_n_hold_walk_f::install(agent);
    special_n_hold_walk_b::install(agent);
    special_n_hold_walk_brake_f::install(agent);
    special_n_hold_walk_brake_b::install(agent);
    special_n_hold_dash_f::install(agent);
    special_n_hold_dash_b::install(agent);
    special_n_hold_jump_squat::install(agent);
    special_n_hold_jump::install(agent);
    special_n_hold_jump_aerial::install(agent);
    special_n_hold_air::install(agent);
    special_n_hold_landing::install(agent);
    special_n_throw::install(agent);
}