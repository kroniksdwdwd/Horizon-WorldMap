// Not ported yet: the Java @Inject(INVOKE) on ChatListener.showMessageToPlayer (MixinFabricChatListener.onShowMessageToPlayer).
// Why it's off: No Horizon function mapped for ChatListener.showMessageToPlayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ChatListener.showMessageToPlayer: run `match`.
// Note: The call it hooks (ChatComponent.addPlayerMessage) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChatComponent.addPlayerMessage -> (not mapped yet)
//   ChatListener.showMessageToPlayer -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinFabricChatListener.java:17:
// | @Inject(
// |    method = "showMessageToPlayer",
// |    cancellable = true,
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/gui/components/ChatComponent;addPlayerMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/client/multiplayer/chat/GuiMessageTag;)V"
// |    )
// | )
// | public void onShowMessageToPlayer(
// |    Bound bound,
// |    PlayerChatMessage playerChatMessage,
// |    Component component,
// |    GameProfile gameProfile,
// |    boolean bl,
// |    Instant instant,
// |    CallbackInfoReturnable<Boolean> info
// | ) {
// |    if (WorldMap.loaded) {
// |       if (WorldMap.events.handleClientPlayerChatReceivedEvent(bound, component, gameProfile)) {
// |          info.setReturnValue(false);
// |       }
// |    }
// | }
