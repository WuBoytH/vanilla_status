use crate::imports::acmd_imports::*;

unsafe extern "C" fn lucario_hadou_handl(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou"),
            Hash40::new("handl"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

unsafe extern "C" fn lucario_hadou_handr(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou"),
            Hash40::new("handr"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

unsafe extern "C" fn lucario_hadoum_handl(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_m"),
            Hash40::new("handl"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

unsafe extern "C" fn lucario_hadoum_handr(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_m"),
            Hash40::new("handr"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

unsafe extern "C" fn lucario_hadoul_handl(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_l_l"),
            Hash40::new("handl"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

unsafe extern "C" fn lucario_hadoul_handr(fighter: &mut L2CAgentBase) {
    let rot = if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        180.0
    }
    else { 0.0 };
    if macros::is_excute(fighter) {
        FighterSpecializer_Lucario::req_aura_effect(
            fighter.battle_object as *mut Fighter,
            Hash40::new("lucario_hadou_l_r"),
            Hash40::new("handr"),
            1.0,
            0.0,
            0.0,
            0.0,
            rot,
            0.0,
            1.0,
            true
        );
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_hadou_l", lucario_hadou_handl);

    agent.effect_acmd("effect_hadou_r", lucario_hadou_handr);

    agent.effect_acmd("effect_hadoum_l", lucario_hadoum_handl);

    agent.effect_acmd("effect_hadoum_r", lucario_hadoum_handr);

    agent.effect_acmd("effect_hadoul_l", lucario_hadoul_handl);

    agent.effect_acmd("effect_hadoul_r", lucario_hadoul_handr);
}