// Not ported yet: the Java @Inject(INVOKE) on ClientPacketListener.handleLogin (MixinForgeClientPacketListener.onOnGameJoin).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleLogin (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ClientPacketListener.handleLogin: run `match`.
// Note: The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.handleLogin -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinForgeClientPacketListener.java:14:
// | @Inject(
// |    at = @At(
// |       value = "INVOKE",
// |       shift = Shift.AFTER,
// |       target = "Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V"
// |    ),
// |    method = "handleLogin"
// | )
// | public void onOnGameJoin(ClientboundLoginPacket packet, CallbackInfo info) {
// |    XaeroWorldMapCore.onPlayNetHandler((ClientPacketListener)this, packet);
// | }
