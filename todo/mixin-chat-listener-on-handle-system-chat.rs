// Not ported yet: the Java @Inject(INVOKE) on ChatListener.handleSystemMessage (MixinChatListener.onHandleSystemChat).
// Why it's off: No Horizon function mapped for ChatListener.handleSystemMessage (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ChatListener.handleSystemMessage: run `match`.
// Note: The call it hooks (ChatComponent.addServerSystemMessage) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChatComponent.addServerSystemMessage -> (not mapped yet)
//   ChatListener.handleDisguisedChatMessage -> (not mapped yet)
//   ChatListener.handleSystemMessage -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinChatListener.java:21:
// | @Inject(
// |    method = "handleSystemMessage",
// |    cancellable = true,
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/components/ChatComponent;addServerSystemMessage(Lnet/minecraft/network/chat/Component;)V")
// | )
// | public void onHandleSystemChat(Component component, boolean bl, CallbackInfo info) {
// |    if (XaeroWorldMapCore.onSystemChat(component)) {
// |       info.cancel();
// |    }
// | }
