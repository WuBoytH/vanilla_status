unsafe fn setup_motion(fighter: &mut L2CFighterCommon, unk: bool) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        if unk {
            MotionModule::change_motion(module_accessor,Hash40::new("special_air_lw_in"),0.0,1.0,false,0.0,false,false);
        }
        else {
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_air_lw_in"),-1.0,1.0,0.0,false,false);
        }
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        if unk {
            MotionModule::change_motion(module_accessor,Hash40::new("special_lw_in"),0.0,1.0,false,0.0,false,false);
        }
        else {
            MotionModule::change_motion_inherit_frame(module_accessor,Hash40::new("special_lw_in"),-1.0,1.0,0.0,false,false);
        }
        GroundModule::correct(module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    if unk {
        fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_lw")));
    }
    else {
        fighter.sub_set_special_start_common_kinetic_setting(L2CValue::new_int(hash40("param_special_lw")));
    }
}

unsafe fn setup_energy(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        let brake_x = sv_kinetic_energy::get_brake_x(fighter.lua_state_agent);
        if brake_x > 0.0 {
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            let unk = WorkModule::get_param_float(module_accessor,hash40("param_special_lw"),0x1e716694dcu64);
            if speed_x < unk {
                fighter.clear_lua_stack();
                lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP,0.0,0.0);
                sv_kinetic_energy::set_brake(fighter.lua_state_agent);
            }
        }
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let mut event = FighterElementLinkEventChange__new_l2c_table();
    event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
    event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    smash::app::lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
    LinkModule::send_event_nodes_struct(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,link_event,0);
    event = smash::app::lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    smash::app::lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
    LinkModule::send_event_nodes_struct(module_accessor,*ITEM_LINK_NO_HAVE,link_event,0);
    event = smash::app::lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager,FighterEntryID(entry_id));
    if smash::app::lua_bind::FighterInformation::is_rabbit_cap(fighter_info) {
        ItemModule::eject_attach(module_accessor,ItemKind(*ITEM_KIND_USAGIHAT),true,true);
    }
    if smash::app::lua_bind::FighterInformation::is_reflector(fighter_info) {
        ItemModule::eject_attach(module_accessor,ItemKind(*ITEM_KIND_BADGE),true,true);
    }
    if smash::app::lua_bind::FighterInformation::is_super_leaf(fighter_info) {
        ItemModule::eject_attach(module_accessor,ItemKind(*ITEM_KIND_SUPERLEAF),true,true);
    }
    if smash::app::lua_bind::FighterInformation::is_rocketbelt(fighter_info) {
        ItemModule::eject_attach(module_accessor,ItemKind(*ITEM_KIND_ROCKETBELT),true,true);
    }
    if smash::app::lua_bind::FighterInformation::is_screw(fighter_info) {
        ItemModule::eject_attach(module_accessor,ItemKind(*ITEM_KIND_SCREW),true,true);
    }
    if smash::app::lua_bind::FighterInformation::is_backshield(fighter_info) {
        ItemModule::eject_attach(module_accessor,ItemKind(*ITEM_KIND_BACKSHIELD),true,true);
    }
    AreaModule::set_whole(module_accessor,false);
    setup_motion(fighter,true);
    setup_energy(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if MotionModule::is_end(module_accessor) {
        fighter.change_status(FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY.into(),false.into());
        if fighter.global_table[0x8].get_bool() == false {
            setup_energy(fighter);
        }
    }
    if StatusModule::is_changing(module_accessor) == false {
        if (fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND
        && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND)
        || (fighter.global_table[0x17].get_i32() != *SITUATION_KIND_AIR
        && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR) {
            setup_motion(fighter,false);
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if fighter.global_table[0xb].get_i32() != *FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY {
        let mut event = FighterElementLinkEventChange__new_l2c_table();
        event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
        event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        smash::app::lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
        LinkModule::send_event_nodes_struct(module_accessor,*WEAPON_LINK_NO_CONSTRAINT,link_event,0);
        event = smash::app::lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        smash::app::lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
        LinkModule::send_event_nodes_struct(module_accessor,*ITEM_LINK_NO_HAVE,link_event,0);
        event = smash::app::lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
        AreaModule::set_whole(module_accessor,false);
    }
    return L2CValue::I32(0)
}
