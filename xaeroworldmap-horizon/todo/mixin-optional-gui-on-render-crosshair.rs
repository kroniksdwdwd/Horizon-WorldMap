// Not ported yet: the Java @Inject(HEAD) on Hud.extractCrosshair (MixinOptionalGui.onRenderCrosshair).
// Why it's off: No Horizon function mapped for Hud.extractCrosshair (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Hud.extractCrosshair: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractCrosshair -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinOptionalGui.java:14:
// | @Inject(at = @At("HEAD"), method = "extractCrosshair", cancellable = true)
// | public void onRenderCrosshair(GuiGraphicsExtractor guiGraphics, DeltaTracker deltaTracker, CallbackInfo info) {
// |    if (XaeroWorldMapCore.onRenderCrosshair(guiGraphics)) {
// |       info.cancel();
// |    }
// | }
