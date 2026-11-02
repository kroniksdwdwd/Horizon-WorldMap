// Not ported yet: the Java @Inject(HEAD) on ClientPacketListener.close (MixinClientPlayNetworkHandler.onCleanup).
// Why it's off: No Horizon function mapped for ClientPacketListener.close (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientPacketListener.close: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.close -> (not mapped yet)
//   ClientPacketListener.handleBlockUpdate -> (not mapped yet)
//   ClientPacketListener.handleChunkBlocksUpdate -> (not mapped yet)
//   ClientPacketListener.handleLevelChunkWithLight -> (not mapped yet)
//   ClientPacketListener.handleLightUpdatePacket -> (not mapped yet)
//   ClientPacketListener.handleSetSpawn -> (not mapped yet)
//   ClientPacketListener.queueLightRemoval -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinClientPlayNetworkHandler.java:101:
// | @Inject(at = @At("HEAD"), method = "close")
// | public void onCleanup(CallbackInfo info) {
// |    XaeroWorldMapCore.onPlayNetHandlerCleanup((ClientPacketListener)this);
// | }
