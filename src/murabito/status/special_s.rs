use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    let disable_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_FRAME);
    if disable_frame <= 0 {
        if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_EXIST);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        }
    }

    special_s_situation_helper(fighter, true.into());

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_EXIST) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET, false, -1);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_situation_helper(fighter: &mut L2CFighterCommon, is_changing: L2CValue) {
    if !is_changing.get_bool() && !StatusModule::is_situation_changed(fighter.module_accessor) {
        return;
    }

    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if situation != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_EXIST)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_FALL) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }

        let motion = MotionModule::motion_kind(fighter.module_accessor);

        if motion != hash40("special_s") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }

        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MURABITO_STATUS_SPECIAL_S_INT_SITUATION);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);

        let motion = MotionModule::motion_kind(fighter.module_accessor);

        if motion != hash40("special_air_s") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }

        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MURABITO_STATUS_SPECIAL_S_INT_SITUATION);
    }
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IS_STOP].get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET) {

            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET);

            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_EXIST) {

                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_SHOOT);

                if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    special_s_shoot_clayrocket(fighter, WEAPON_MURABITO_CLAYROCKET_SHOOT_SHOOT.into());

                    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_BURST) {
                        let burst_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("burst_motion_rate"));
                        if burst_motion_rate > 0.0 {
                            MotionModule::set_whole_rate(fighter.module_accessor, 1.0 / burst_motion_rate);
                        }
                        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                        }
                    }
                    special_s_shoot_clayrocket(fighter, WEAPON_MURABITO_CLAYROCKET_SHOOT_RIDE.into());
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_BURST) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_RIDE);
                        fighter.change_status(FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE.into(), true.into());
                        return 1.into();
                    }
                }
            }
        }
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_RIDE) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                FIGHTER_STATUS_KIND_FALL
            }
            else {
                FIGHTER_STATUS_KIND_WAIT
            }
        }
        else {
            FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE
        };
        fighter.change_status(status.into(), (status == FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE).into());
        return 1.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        special_s_situation_helper(fighter, false.into());
    }
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_FALL)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_RESET {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }

    0.into()
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_EXIST) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET_SHOOT) {
            special_s_shoot_clayrocket(fighter, WEAPON_MURABITO_CLAYROCKET_SHOOT_SHOOT.into());
        }
        special_s_end_common(fighter);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
}