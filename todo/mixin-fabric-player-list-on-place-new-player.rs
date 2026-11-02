// Not ported yet: the Java @Inject(TAIL) on PlayerList.placeNewPlayer (MixinFabricPlayerList.onPlaceNewPlayer).
// Why it's off: `horizon_client::lan::hub::Hub::place_new_player` has a closure in its body: horizon-mixins doesn't rewrite such bodies with RETURN. A HEAD hook on it, or a RETURN on a function it calls, may work instead.
// horizon-mixins kind: RETURN on `horizon_client::lan::hub::Hub::place_new_player`
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerList.placeNewPlayer -> horizon_client::lan::hub::Hub::place_new_player (suggested, high)
//
// Decompiled Java, xaero/map/mixin/MixinFabricPlayerList.java:15:
// | @Inject(at = @At("TAIL"), method = "placeNewPlayer")
// | public void onPlaceNewPlayer(Connection connection, ServerPlayer serverPlayer, CommonListenerCookie commonListenerCookie, CallbackInfo info) {
// |    if (WorldMap.loaded) {
// |       WorldMap.commonEvents.onPlayerLogIn(serverPlayer);
// |    }
// | }
