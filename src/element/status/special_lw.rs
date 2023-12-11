use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

unsafe extern "C" fn setup_motion(fighter: &mut L2CFighterCommon, unk: bool) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if unk {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_lw_in"),
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
                Hash40::new("special_air_lw_in"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        if unk {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_lw_in"),
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
                Hash40::new("special_lw_in"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    if unk {
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_lw").into());
    }
    else {
        fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());
    }
}

unsafe extern "C" fn setup_energy(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
        let brake_x = sv_kinetic_energy::get_brake_x(fighter.lua_state_agent);
        if brake_x > 0.0 {
            fighter.clear_lua_stack();
            lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            let unk = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), 0x1e716694dc);
            if speed_x < unk {
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    0.0,
                    0.0
                );
            }
        }
    }
}

pub unsafe extern "C" fn element_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut event = FighterElementLinkEventChange__new_l2c_table();
    event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
    event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event, 0);
    event = lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *ITEM_LINK_NO_HAVE, link_event, 0);
    // event = lua_bind::LinkEvent::store_l2c_table(link_event);
    lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let entry_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(),FighterEntryID(entry_id));
    if lua_bind::FighterInformation::is_rabbit_cap(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT),  true, true);
    }
    if lua_bind::FighterInformation::is_reflector(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_BADGE), true, true);
    }
    if lua_bind::FighterInformation::is_superleaf(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SUPERLEAF), true, true);
    }
    if lua_bind::FighterInformation::is_rocketbelt(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), true, true);
    }
    if lua_bind::FighterInformation::is_screw(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SCREW), true, true);
    }
    if lua_bind::FighterInformation::is_backshield(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_BACKSHIELD), true, true);
    }
    AreaModule::set_whole(fighter.module_accessor,false);
    setup_motion(fighter,true);
    setup_energy(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY.into(), false.into());
        if !fighter.global_table[IS_STOP].get_bool() {
            setup_energy(fighter);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if (fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND)
        || (fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR) {
            setup_motion(fighter,false);
        }
    }
    0.into()
}

pub unsafe extern "C" fn element_special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ELEMENT_STATUS_KIND_SPECIAL_LW_STANDBY {
        let mut event = FighterElementLinkEventChange__new_l2c_table();
        event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
        event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
        LinkModule::send_event_nodes_struct(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event, 0);
        event = lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
        let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
        let link_event = callable();
        lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
        LinkModule::send_event_nodes_struct(fighter.module_accessor, *ITEM_LINK_NO_HAVE, link_event, 0);
        // event = lua_bind::LinkEvent::store_l2c_table(link_event);
        lua_bind::LinkEvent::store_l2c_table(link_event);
        let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
        deleter(link_event);
        AreaModule::set_whole(fighter.module_accessor,false);
    }
    0.into()
}
