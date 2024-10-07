use super::*;

unsafe extern "C" fn zelda_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn zelda_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(zelda_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let is_changing = StatusModule::is_changing(fighter.module_accessor);

    if !is_changing {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
            return 0.into();
        }
    }

    if is_changing
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_hi_start"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_hi_start"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_ZELDA_SPECIAL_HI_AIR);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_air_hi_start"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_air_hi_start"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
        }
    }

    0.into()
}

unsafe extern "C" fn zelda_special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cliff_check = GroundModule::cliff_check(fighter.module_accessor) as i32;
    WorkModule::set_int(fighter.module_accessor, cliff_check, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, zelda_special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, zelda_special_hi_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, zelda_special_hi_end);
}