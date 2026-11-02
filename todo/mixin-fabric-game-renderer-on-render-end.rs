// Not ported yet: the Java @Inject(RETURN) on GameRenderer.extract (MixinFabricGameRenderer.onRenderEnd).
// Why it's off: No Horizon function mapped for GameRenderer.extract (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for GameRenderer.extract: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GameRenderer.extract -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinFabricGameRenderer.java:13:
// | @Inject(at = @At("RETURN"), method = "extract")
// | public void onRenderEnd(DeltaTracker deltaTracker, boolean boolean_1, CallbackInfo info) {
// |    if (WorldMap.loaded) {
// |       WorldMap.events.handleRenderTick(false);
// |    }
// | }
