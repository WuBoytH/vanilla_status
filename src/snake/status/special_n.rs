use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        ItemModule::drop_item(fighter.module_accessor, 90.0, 0.0, 0);
    }

    if !ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE) {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW.into(), false.into());
        return 1.into();
    }

    WorkModule::set_float(fighter.module_accessor, -1.0, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLOAT_THROW_RATE);

    if !StopModule::is_stop(fighter.module_accessor) {
        special_n_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_n_substatus as *const () as _));

    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start"), *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);

    special_n_motion_helper(fighter, true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_motion_helper(fighter: &mut L2CFighterCommon, param_2: L2CValue) {
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
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN,
                Hash40::new_raw(motion),
                true,
                -1.0
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
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN,
                Hash40::new_raw(motion),
                false,
                -1.0
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
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN,
                Hash40::new_raw(motion),
                true,
                -1.0
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
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN,
                Hash40::new_raw(motion),
                false,
                -1.0
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLAG_FIRST);
        }
    }
}

unsafe extern "C" fn special_n_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if !param_2.get_bool() {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_FLAG_BUTTON_SPECIAL_OFF);
            fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
            fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
        }
    }

    0.into()
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_FLAG_BUTTON_SPECIAL_OFF) {
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW.into(), false.into());
            return 1.into();
        }

        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT
        }
        else {
            FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }

    special_n_motion_helper(fighter, false.into());

    0.into()
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        WorkModule::set_float(fighter.module_accessor, stick_x * lr, *FIGHTER_SNAKE_STATUS_SPECIAL_N_HOLD_WAIT_WORK_FLOAT_THROW_RATE);
    }

    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, ArticleOperationTarget(0));

    special_n_end_common(fighter);

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
}