use super::*;

unsafe extern "C" fn special_n_throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    if FighterSpecializer_Snake::is_constraint_article(
        fighta,
        *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE,
        ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST)
    ) != 1 {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_throw_fail"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_throw_fail"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    }
    else {
        let rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_THROW_WORK_FLOAT_THROW_RATE);
        let throw_hi_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("throw_hi_threshold"));
        let throw_lw_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("throw_lw_threshold"));
        if rate >= throw_hi_threshold {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_n_throw_hi"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_throw_hi"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        }
        else if rate <= throw_lw_threshold {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_n_throw_lw"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_throw_lw"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_n_throw_m"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_throw_m"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        }
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_THROW_WORK_FLAG_FIRST);
    special_n_throw_motion_helper(fighter, true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_throw_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_throw_motion_helper(fighter: &mut L2CFighterCommon, param_2: L2CValue) {
    if !param_2.get_bool() {
        if !StatusModule::is_situation_changed(fighter.module_accessor) {
            return;
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        special_n_set_ground(fighter);
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(motion),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(motion),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLAG_FIRST);
        }
    }
    else {
        special_n_set_air(fighter);
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(motion),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(motion),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLAG_FIRST);
        }
    }
}

unsafe extern "C" fn special_n_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }

    special_n_throw_motion_helper(fighter, false.into());

    0.into()
}

unsafe extern "C" fn special_n_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_end_common(fighter);

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW, special_n_throw_pre);
    agent.status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW, special_n_throw_main);
    agent.status(End, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW, special_n_throw_end);
}