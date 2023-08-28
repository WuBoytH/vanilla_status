use crate::imports::status_imports::*;

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn elight_special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_end").into(), Hash40::new("special_air_s_end").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_end_main_loop as *const () as _))
}

unsafe fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let frame = MotionModule::frame(fighter.module_accessor);
            let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_end"), true);
            if frame >=  cancel_frame {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
            SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_elight_landing01"));
        }
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_end").into(), Hash40::new("special_air_s_end").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
    }
    else {
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        elight_special_s_end_main
    );
}
