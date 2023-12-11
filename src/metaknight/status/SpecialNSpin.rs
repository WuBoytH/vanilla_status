use crate::imports::status_imports::*;

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_metaknight_special_n_spin_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_N, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_metaknight_special_n_spin_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_ON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_ADD_ATTACK_ON);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_ATTACK_ID_FRAME)
    0.into()
}

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe fn status_metaknight_special_n_spin_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let button_unable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("button_unable_frame"));
    let start_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_rot_speed"));
    WorkModule::set_int(fighter.module_accessor, button_unable_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_UNABLE_COUNTER);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_spin"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::set_rate(fighter.module_accessor, start_rot_speed);
    if !StopModule::is_stop(fighter.module_accessor) {
        metaknight_special_n_spin_sound_handler(fighter, false.into());
    }
    metaknight_special_n_spin_handler(fighter);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(metaknight_special_n_spin_sound_handler as *const () as _));
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let add_speed_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("add_speed_stick"));
    let start_stick_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("start_stick_speed"));
    if stick_x.abs() >= add_speed_stick {
        let lr = PostureModule::lr(fighter.module_accessor);
        let speed_x = start_stick_speed * stick_x * lr;
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: speed_x, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_special_n_spin_loop as *const () as _))
}

unsafe fn metaknight_special_n_spin_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_N);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_N);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
    }
    return;
}

unsafe fn metaknight_special_n_spin_sound_handler(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_BUTTON_HOP_COUNT);
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE) <= 0 {
            WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE);
            let start_se_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);
            let sound = match start_se_counter {
                0 => { if motion_kind == hash40("special_n_spin_c3") { 0x1896dcd23e } else { hash40("se_metaknight_swish07") } },
                1 => { if motion_kind == hash40("special_n_spin_c3") { 0x187603a50d } else { hash40("se_metaknight_swish09") } },
                2 => { if motion_kind == hash40("special_n_spin_c3") { 0x188ed7a452 } else { hash40("se_metaknight_swish11") } },
                3 => { if motion_kind == hash40("special_n_spin_c3") { 0x188ed7a452 } else { hash40("se_metaknight_swish11") } },
                4 => hash40("se_metaknight_swish10"),
                5 => { if motion_kind == hash40("special_n_spin_c3") { 0x187603a50d } else { hash40("se_metaknight_swish09") } },
                6 => { if motion_kind == hash40("special_n_spin_c3") { 0x1896dcd23e } else { hash40("se_metaknight_swish07") } },
                7 => hash40("se_metaknight_swish06"),
                8 => hash40("se_metaknight_swish06"),
                9 => hash40("se_metaknight_swish05"),
                _ => hash40("se_metaknight_swish05"),
            };
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(sound), true, false, false, false, enSEType(0));
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_START_SE_COUNTER);                           
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let ground_effect_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
            let rate = MotionModule::rate(fighter.module_accessor);
            let counter_value = ground_effect_counter - rate;
            WorkModule::set_float(fighter.module_accessor, counter_value, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER);
            if counter_value <= 0.0 {
                let FIGHTER_PTR = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
                FighterSpecializer_Metaknight::set_special_n_ground_effect(FIGHTER_PTR);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn metaknight_special_n_spin_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let end_rot_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("end_rot_speed"));
    if MotionModule::rate(fighter.module_accessor) <= end_rot_speed {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END.into(), false.into())
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        metaknight_special_n_spin_handler(fighter);
        return 0.into();
    }
    return 0.into()
}   

/*MISSING EXEC STATUS*/

#[status_script( agent = "metaknight", status = FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END )]
unsafe fn status_metaknight_special_n_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x1230d89b8b), false, false);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_metaknight_special_n_spin_pre,
        status_metaknight_special_n_spin_init,
        status_metaknight_special_n_spin_main,
        status_metaknight_special_n_spin_end
    );
}