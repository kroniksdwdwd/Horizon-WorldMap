// Not ported yet: the Java @Inject(HEAD) on Gui.setScreen (MixinFabricGui.onOpenScreen).
// Why it's off: No Horizon function mapped for Gui.setScreen (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Gui.setScreen: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.gui -> (not mapped yet)
//   Gui.extractRenderState -> (not mapped yet)
//   Gui.overlay -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//   Gui.setScreen -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinFabricGui.java:32:
// | @Inject(at = @At("HEAD"), method = "setScreen", cancellable = true)
// | public void onOpenScreen(Screen screen_1, CallbackInfo info) {
// |    if (WorldMap.loaded) {
// |       Screen resultScreen = WorldMap.events.handleGuiOpen(screen_1);
// |       if (screen_1 != resultScreen) {
// |          this.setScreen(resultScreen);
// |          info.cancel();
// |       }
// |    }
// | }
