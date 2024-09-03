use super::*;

unsafe extern "C" fn buddy_special_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
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
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn buddy_special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mul = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_end_speed_x_mul_air"))
    }
    else {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_end_speed_x_mul_air"))
    };
    sv_kinetic_energy!(
        mul_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        mul,
        1.0,
        1.0
    );
    
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());

    buddy_special_s_end_kinetic_handler(fighter);

    fighter.sub_change_motion_by_situation(
        hash40("special_s_end").into(),
        hash40("special_air_s_end").into(),
        false.into()
    );

    fighter.sub_set_ground_correct_by_situation(true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(buddy_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn buddy_special_s_end_kinetic_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );

        let dash_end_accel_x_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_end_accel_x_air"));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            dash_end_accel_x_air,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );

        let dash_end_accel_x_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_end_accel_x_ground"));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            dash_end_accel_x_ground,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_STOP_RESET_TYPE_AIR,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );

        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let aceel_y = sv_kinetic_energy::get_accel_y(fighter.lua_state_agent);
        let dash_end_gravity_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("dash_end_gravity_mul"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            aceel_y * dash_end_gravity_mul
        );
    }

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
}

unsafe extern "C" fn buddy_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
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
        return 0.into();
    }

    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s_end") {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }

    fighter.sub_change_motion_by_situation(
        hash40("special_s_end").into(),
        hash40("special_air_s_end").into(),
        true.into()
    );
    if fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into()).get_bool() {
        buddy_special_s_end_kinetic_handler(fighter);
    }

    fighter.sub_set_ground_correct_by_situation(true.into());

    0.into()
}

unsafe extern "C" fn buddy_special_s_end_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, buddy_special_s_end_pre);
    agent.status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, buddy_special_s_end_main);
    agent.status(End, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, buddy_special_s_end_end);
}