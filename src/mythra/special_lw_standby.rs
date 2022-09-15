#[status_script(agent = "elight", status = FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_lw_standby_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    HitModule::set_whole(module_accessor,HitStatus(*HIT_STATUS_OFF),0);
    ItemModule::set_have_item_visibility(module_accessor,false,0);
    ItemModule::set_attach_item_visibility(module_accessor,false,-1);
    AreaModule::set_whole(module_accessor,false);
    VisibilityModule::set_whole(module_accessor,false);
    ShadowModule::set_draw_status(module_accessor,false);
    MotionModule::change_motion(module_accessor,Hash40::new("invalid"),0.0,1.0,false,0.0,false,false);
    WorkModule::on_flag(module_accessor,*FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_GENERATE_CHANGER);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_standby_main_loop as *const () as _))
}

unsafe fn special_lw_standby_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xe].get_f32() >= 60.0 {
        fighter.change_status(FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_OUT.into(),false.into());
    }
    return L2CValue::I32(0)
}
