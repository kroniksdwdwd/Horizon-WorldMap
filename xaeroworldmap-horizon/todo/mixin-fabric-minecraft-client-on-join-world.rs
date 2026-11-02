// Not ported yet: the Java @Inject(HEAD) on Minecraft.setLevel (MixinFabricMinecraftClient.onJoinWorld).
// Why it's off: No Horizon function mapped for Minecraft.setLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.setLevel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.disconnect -> (not mapped yet)
//   Minecraft.renderFrame -> (not mapped yet)
//   Minecraft.runTick -> (not mapped yet)
//   Minecraft.setLevel -> (not mapped yet)
//   Minecraft.tick -> hook `tick`. Also runs in the server's Game: check native_window().
//
// Decompiled Java, xaero/map/mixin/MixinFabricMinecraftClient.java:45:
// | @Inject(at = @At("HEAD"), method = "setLevel")
// | public void onJoinWorld(ClientLevel newWorld, CallbackInfo info) {
// |    if (this.level != null) {
// |       if (!WorldMap.loaded) {
// |          return;
// |       }
// |
// |       WorldMap.events.handleWorldUnload(this.level);
// |    }
// | }
