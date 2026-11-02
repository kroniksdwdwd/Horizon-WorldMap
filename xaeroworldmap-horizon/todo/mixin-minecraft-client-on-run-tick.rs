// Not ported yet: the Java @ModifyVariable(HEAD) on Minecraft.runTick (MixinMinecraftClient.onRunTick).
// Why it's off: No Horizon function mapped for Minecraft.runTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for Minecraft.runTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.runTick -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinMinecraftClient.java:21:
// | @ModifyVariable(method = "runTick", at = @At("HEAD"), index = 1)
// | public boolean onRunTick(boolean unused) {
// |    WorldMap.events.onMinecraftRunTickStart();
// |    return unused;
// | }
