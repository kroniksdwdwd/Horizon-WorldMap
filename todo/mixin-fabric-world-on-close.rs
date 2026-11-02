// Not ported yet: the Java @Inject(HEAD) on Level.close (MixinFabricWorld.onClose).
// Why it's off: No Horizon function mapped for Level.close (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Level.close: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Level.close -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinFabricWorld.java:13:
// | @Inject(at = @At("HEAD"), method = "close")
// | public void onClose(CallbackInfo info) {
// |    if (this instanceof ServerLevel) {
// |       if (!WorldMap.loaded) {
// |          return;
// |       }
// |
// |       WorldMap.events.handleWorldUnload((ServerLevel)this);
// |    }
// | }
