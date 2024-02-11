
use super::*;


#[acmd_script( agent = "ryu", scripts = ["game_speciallwinstall"] , category = ACMD_GAME , low_priority)]
unsafe fn game_speciallwinstall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 11.0, 5.0);
    frame(lua_state, 11.0);
    FT_MOTION_RATE_RANGE(fighter, 11.0, 24.0, 13.0);
    frame(lua_state, 24.0);
    FT_MOTION_RATE_RANGE(fighter, 24.0, 44.0, 2.0);
    frame(lua_state, 44.0);
    FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        game_speciallwinstall
    );
}
