use {
    smash::{
        lua2cpp::{L2CFighterCommon},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    crate::singletons::FIGHTER_MANAGER,
    crate::table_const::*
};

unsafe fn setup_motion_out(fighter: &mut L2CFighterCommon, unk: bool) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if unk {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_lw_out"),
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
                Hash40::new("special_air_lw_out"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let out_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("out_speed_x_mul"));
        let out_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("out_accel_x_mul"));
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0) * out_speed_x_mul;
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0) * out_accel_x_mul;
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0) * out_accel_x_mul;
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            air_accel_x_mul
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            air_accel_x_add
        );
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            let out_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("out_speed_y_mul"));
            let out_accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("out_accel_y_mul"));
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0) * out_speed_y_mul * -1.0;
            let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * out_accel_y_mul * -1.0;
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y
            );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_speed_y_stable
            );
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                speed_x,
                0.0
            );
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    else {
        if unk {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_lw_out"),
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
                Hash40::new("special_lw_out"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    let out_brake_x_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("out_brake_x_ground"));
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        out_brake_x_ground,
        0.0
    );
}

pub unsafe fn element_special_lw_out_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) <= 0 {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
    let cond = lua_bind::FighterManager::is_result_mode(FIGHTER_MANAGER);
    WorkModule::set_flag(fighter.module_accessor,cond, *FIGHTER_ELEMENT_STATUS_SPECIAL_LW_IS_RESULT);
    setup_motion_out(fighter, true);
    MotionAnimcmdModule::flush(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_out_main_loop as *const () as _))
}

unsafe fn special_lw_out_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    || (!fighter.sub_wait_ground_check_common(L2CValue::new_bool(false)).get_bool()
    && !fighter.sub_air_check_fall_common().get_bool()) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if !StatusModule::is_changing(fighter.module_accessor) {
                if (fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
                && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND)
                || (fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
                && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR) {
                    setup_motion_out(fighter, false);
                }
            }
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                if !KineticModule::is_enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
                    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0) * -1.0;
                    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * -1.0;
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        air_accel_y
                    );
                    sv_kinetic_energy!(
                        set_stable_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        air_speed_y_stable
                    );
                    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                }
            }
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    0.into()
}
