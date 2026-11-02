// Not ported yet: the Java @Inject(HEAD) on ClientPacketListener.queueLightRemoval (MixinClientPlayNetworkHandler.onQueueLightRemoval).
// Why it's off: No Horizon function mapped for ClientPacketListener.queueLightRemoval (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientPacketListener.queueLightRemoval: run `match`.
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
// Decompiled Java, xaero/map/mixin/MixinClientPlayNetworkHandler.java:62:
// | @Inject(at = @At("HEAD"), method = "queueLightRemoval")
// | public void onQueueLightRemoval(ClientboundForgetLevelChunkPacket packet, CallbackInfo info) {
// |    XaeroWorldMapCore.onQueueLightRemoval(packet);
// | }
