// Not ported yet: the Java @Inject(HEAD) on Minecraft.runTick (MixinFabricMinecraftClient.onRunTickStart).
// Why it's off: No Horizon function mapped for Minecraft.runTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.runTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.disconnect -> (not mapped yet)
//   Minecraft.renderFrame -> (not mapped yet)
//   Minecraft.runTick -> (not mapped yet)
//   Minecraft.setLevel -> (not mapped yet)
//   Minecraft.tick -> hook `tick`. Also runs in the server's Game: check native_window().
//
// Decompiled Java, xaero/map/mixin/MixinFabricMinecraftClient.java:29:
// | @Inject(at = @At("HEAD"), method = "runTick")
// | public void onRunTickStart(CallbackInfo info) {
// |    XaeroWorldMapCoreFabric.onMinecraftRunTick();
// | }
