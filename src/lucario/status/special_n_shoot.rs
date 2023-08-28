use {
    crate::imports::status_imports::*,
    super::helper::*
};

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_shoot") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_shoot") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    if max_charge_frame < charge as f32 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
    }
    lucario_special_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor){
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_set_kinetic(fighter);
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucario_special_n_shoot_main
    );
}
