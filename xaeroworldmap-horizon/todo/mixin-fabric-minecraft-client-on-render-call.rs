// Not ported yet: the Java @ModifyArg(INVOKE) on Minecraft.runTick (MixinFabricMinecraftClient.onRenderCall).
// Why it's off: No Horizon function mapped for Minecraft.runTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for Minecraft.runTick: run `match`.
// Note: The call it hooks (Minecraft.renderFrame) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.disconnect -> (not mapped yet)
//   Minecraft.renderFrame -> (not mapped yet)
//   Minecraft.runTick -> (not mapped yet)
//   Minecraft.setLevel -> (not mapped yet)
//   Minecraft.tick -> hook `tick`. Also runs in the server's Game: check native_window().
//
// Decompiled Java, xaero/map/mixin/MixinFabricMinecraftClient.java:56:
// | @ModifyArg(method = "runTick", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/Minecraft;renderFrame(Z)V"), index = 0)
// | public boolean onRenderCall(boolean advanceGameTime) {
// |    if (WorldMap.INSTANCE != null) {
// |       ((WorldMapFabric)WorldMap.INSTANCE).tryLoadLater();
// |    }
// |
// |    return XaeroWorldMapCore.onRenderCall(advanceGameTime);
// | }
