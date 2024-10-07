use super::*;

unsafe extern "C" fn zelda_special_hi_3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn zelda_special_hi_3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let wrap_xy_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_xy_speed"));
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    KineticModule::clear_speed_all(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            speed_x * wrap_xy_speed,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP_X_NORMAL_MAX);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            speed_x * wrap_xy_speed,
            speed_y * wrap_xy_speed
        );
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let limit_speed_x = sv_kinetic_energy::get_limit_speed_x(fighter.lua_state_agent);
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            limit_speed_x,
            -1.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    VisibilityModule::set_whole(fighter.module_accessor, true);
    0.into()
}

unsafe extern "C" fn zelda_special_hi_3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        let fall_x_mull_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mull_value"));
        WorkModule::set_float(fighter.module_accessor, fall_x_mull_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(zelda_special_hi_3_main_loop as *const () as _))
}

unsafe extern "C" fn zelda_special_hi_3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_NONE {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
    if start_situation == *SITUATION_KIND_GROUND {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE) {
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let flick_y = fighter.global_table[FLICK_Y].get_i32();
        let dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
        let dive_flick_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
        let dive_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("dive_speed_y"), 0);
        if sum_speed_y < 0.0
        && stick_y <= dive_cont_value
        && flick_y <= dive_flick_frame_value {
            if -dive_speed_y < sum_speed_y {
                let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
                let fighter_information = lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), FighterEntryID(entry_id));
                let gravity = lua_bind::FighterInformation::gravity(fighter_information);
                let gravity_speed_cooeficient = lua_bind::BattleObjectWorld::gravity_speed_coefficient(singletons::BattleObjectWorld());
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -dive_speed_y * gravity * gravity_speed_cooeficient
                );
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -dive_speed_y * gravity * gravity_speed_cooeficient
                );
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    0.0
                );
                fighter.check_mach_stamp();
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE);
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        sv_kinetic_energy!(
            add_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            stick_x * air_accel_x_mul,
            0.0
        );
    }

    if !fighter.global_table[IS_STOP].get_bool() {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
                let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
                let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    speed_x,
                    speed_y - (speed_y / 10.0)
                );
            }
            else {
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    speed_y
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

                let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
                let fall_x_mull_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mull_value"));
                let speed_max = air_speed_x_stable * fall_x_mull_value;
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    speed_x.clamp(-speed_max, speed_max),
                    0.0
                );
            }
        }
    }

    0.into()
}

unsafe extern "C" fn zelda_special_hi_3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_whole(fighter.module_accessor, true);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Pre, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, zelda_special_hi_3_pre);
    agent.status(Init, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, zelda_special_hi_3_init);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, zelda_special_hi_3_main);
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, zelda_special_hi_3_end);
}