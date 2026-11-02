// HEAD on `native_hooks::Game<'_>::native_wheel`: the open map takes the
// wheel steps for its zoom instead of the hotbar.
fn xaeroworldmap_wheel(
    game: &mut crate::game::Game<'_>,
    steps: f32,
    claimed: &mut bool,
) {
    use crate::game::port_xaeroworldmap as m;
    if !game.native_on(m::ID) {
        return;
    }
    crate::game::port_xaeroworldmap_game::ensure_init(game);
    crate::game::port_xaeroworldmap_game::mvp_wheel(game, steps, claimed);
}
