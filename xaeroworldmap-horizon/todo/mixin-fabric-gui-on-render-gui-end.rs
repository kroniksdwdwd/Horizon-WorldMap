// Not ported yet: the Java @Inject(RETURN) on Gui.extractRenderState (MixinFabricGui.onRenderGuiEnd).
// Why it's off: No Horizon function mapped for Gui.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Gui.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.gui -> (not mapped yet)
//   Gui.extractRenderState -> (not mapped yet)
//   Gui.overlay -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//   Gui.setScreen -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinFabricGui.java:23:
// | @Inject(at = @At("RETURN"), method = "extractRenderState")
// | public void onRenderGuiEnd(DeltaTracker deltaTracker, boolean shouldRenderLevel, boolean resourcesLoaded, CallbackInfo info) {
// |    if (WorldMap.loaded) {
// |       if (shouldRenderLevel && this.minecraft.gui.overlay() == null && this.minecraft.gui.screen() != null) {
// |          WorldMap.events.handleDrawScreen(this.minecraft.gui.screen());
// |       }
// |    }
// | }
