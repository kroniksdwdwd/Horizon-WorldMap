// Not ported yet: the Java @Inject(HEAD) on Minecraft.runTick (MixinForgeMinecraftClient.onRunTickStart).
// Why it's off: No Horizon function mapped for Minecraft.runTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.runTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.renderFrame -> (not mapped yet)
//   Minecraft.runTick -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinForgeMinecraftClient.java:13:
// | @Inject(at = @At("HEAD"), method = "runTick")
// | public void onRunTickStart(CallbackInfo info) {
// |    XaeroWorldMapCore.onMinecraftRunTick();
// | }
