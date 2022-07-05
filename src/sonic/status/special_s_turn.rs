use {
    smash::{
        lua2cpp::{L2CFighterCommon},
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::table_const::*
};

#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn sonic_special_s_turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_TURN_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_TURN_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_S_TURN_FLOAT,
        0
    );
    let log_mask;
    let power_up_bit;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
        log_mask = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;
        power_up_bit = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S;
    }
    else {
        log_mask = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK;
        power_up_bit = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW;
    };
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        log_mask as u64,
        0,
        power_up_bit as u32,
        0
    );
    0.into()
}

#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_special_s_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s_turn"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(0x197f2e7624), // uncracked Motion Kind
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        sonic_special_s_turn_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(sonic_special_s_turn_substatus as *const () as _));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_TURN);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
        effect!(
            fighter,
            MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
            Hash40::new("sonic_spinblur"),
            true,
            true
        );
        effect!(
            fighter,
            MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
            Hash40::new("sonic_spinblur_middle"),
            true,
            true
        );
        effect!(
            fighter,
            MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
            Hash40::new("sonic_spinblur_max"),
            true,
            true
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_turn_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_turn_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_TURN_WORK_INT_ADVANCE_COUNTER);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_TURN_WORK_INT_ADVANCE_COUNTER) < 0 {
        fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
        fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_TURN_WORK_INT_ADVANCE_COUNTER);
    if counter >= 0 {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                return 0.into();
            }
        }
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
    }
    else {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }
    1.into()
}

#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn sonic_special_s_turn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
        if status != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH {
            sonic_special_s_turn_end_disable_eff(fighter);
        }
        else {
            sonic_special_s_dash_call_script(fighter);
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && status != *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_turn_end_disable_eff(fighter: &mut L2CFighterCommon) {
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spinblur"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spinblur_middle"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spinblur_max"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spintrace"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spintrace_middle"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spintrace_max"),
        true,
        true
    );
}

unsafe extern "C" fn sonic_special_s_dash_call_script(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD) {
        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_WORK_INT_SPECIAL_LW_CHARGE_LEVEL);
        let script = if charge == 0 {
            0x1c48dff86a
        }
        else if charge == 1 {
            0x20ab29b2f3
        }
        else {
            0x1cd6bc000d
        };
        MotionAnimcmdModule::call_script_single(
            fighter.module_accessor,
            *FIGHTER_ANIMCMD_EFFECT,
            Hash40::new_raw(script),
            -1
        );
    }
}

pub fn install() {
    install_status_scripts!(
        sonic_special_s_turn_pre, sonic_special_s_turn_main, sonic_special_s_turn_end
    );
}
