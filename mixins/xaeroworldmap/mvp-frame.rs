// HEAD on `native_hooks::Game<'_>::native_frame`: Xaero's World Map's map is
// composed here, once a frame, after Horizon's own minimap update.
fn xaeroworldmap_frame(
    game: &mut crate::game::Game<'_>,
    elapsed: std::time::Duration,
) {
    use crate::game::port_xaeroworldmap as m;
    let _ = elapsed;
    if !game.native_on(m::ID) {
        return;
    }
    crate::game::port_xaeroworldmap_game::ensure_init(game);
    crate::game::port_xaeroworldmap_game::mvp_frame(game);
}
