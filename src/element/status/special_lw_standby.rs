use super::*;

pub unsafe extern "C" fn element_special_lw_standby_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0xff);
    AreaModule::set_whole(fighter.module_accessor, false);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    ShadowModule::set_draw_status(fighter.module_accessor, false);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("invalid"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_GENERATE_CHANGER);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_standby_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_standby_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[MOTION_FRAME].get_f32() >= 60.0 {
        fighter.change_status(FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_OUT.into(), false.into());
    }
    0.into()
}
