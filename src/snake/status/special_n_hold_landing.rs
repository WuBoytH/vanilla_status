use super::*;

unsafe extern "C" fn special_n_hold_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ITEM_SHOOT_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ITEM_SHOOT_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ITEM_SHOOT_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_hold_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ItemShootLanding_Common(true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_landing_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_hold_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if special_n_main_loop_common(fighter).get_bool() {
        return 1.into();
    }

    if fighter.sub_ItemShootLanding_Common_Main().get_bool() {
        return 1.into();
    }

    fighter.sub_ftStatusUniqProcessItemShoot_execFixPos_Common();

    0.into()
}

unsafe extern "C" fn special_n_hold_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_end_common(fighter);

    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, special_n_hold_landing_pre);
    agent.status(Init, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, special_n_init_common);
    agent.status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, special_n_hold_landing_main);
    agent.status(Exec, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, special_n_exec_common);
    agent.status(End, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, special_n_hold_landing_end);
}