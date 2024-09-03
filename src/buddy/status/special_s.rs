use super::*;
use super::helper::*;

unsafe extern "C" fn buddy_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN) == 0 {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL);
        return 1.into();
    }
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn buddy_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());

    let special_s_remain = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
    WorkModule::set_flag(fighter.module_accessor, special_s_remain <= 0, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);

    fighter.sub_change_motion_by_situation(
        hash40("special_s").into(),
        hash40("special_air_s").into(),
        false.into()
    );
    fighter.sub_set_ground_correct_by_situation(true.into());

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);

    fighter.sub_shift_status_main(L2CValue::Ptr(buddy_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn buddy_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    buddy_special_s_set_armor(fighter);

    fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL) {
            FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL
        }
        else {
            FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    fighter.sub_change_motion_by_situation(
        hash40("special_s").into(),
        hash40("special_air_s").into(),
        true.into()
    );
    fighter.sub_set_ground_correct_by_situation(true.into());

    0.into()
}

unsafe extern "C" fn buddy_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
        buddy_special_s_remove_armor(fighter);
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, buddy_special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, buddy_special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, buddy_special_s_end);
}