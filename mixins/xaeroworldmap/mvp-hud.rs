// HEAD on `native_hooks::Game<'_>::native_hud`: the full-screen map overlay,
// after Horizon's minimap and mod panels.
fn xaeroworldmap_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::port_xaeroworldmap as m;
    if !game.native_on(m::ID) {
        return;
    }
    crate::game::port_xaeroworldmap_game::ensure_init(game);
    crate::game::port_xaeroworldmap_game::mvp_hud(game, batch, corners);
}
