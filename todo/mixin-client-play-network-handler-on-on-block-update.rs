// Not ported yet: the Java @Inject(INVOKE) on ClientPacketListener.handleBlockUpdate (MixinClientPlayNetworkHandler.onOnBlockUpdate).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleBlockUpdate (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ClientPacketListener.handleBlockUpdate: run `match`.
// Note: The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet.
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
// Decompiled Java, xaero/map/mixin/MixinClientPlayNetworkHandler.java:67:
// | @Inject(
// |    at = @At(
// |       value = "INVOKE",
// |       shift = Shift.AFTER,
// |       target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V"
// |    ),
// |    method = "handleBlockUpdate"
// | )
// | public void onOnBlockUpdate(ClientboundBlockUpdatePacket packet, CallbackInfo info) {
// |    XaeroWorldMapCore.onBlockChange(packet);
// | }
