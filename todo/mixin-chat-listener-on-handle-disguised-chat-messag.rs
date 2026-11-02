// Not ported yet: the Java @Inject(HEAD) on ChatListener.handleDisguisedChatMessage (MixinChatListener.onHandleDisguisedChatMessag).
// Why it's off: No Horizon function mapped for ChatListener.handleDisguisedChatMessage (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChatListener.handleDisguisedChatMessage: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChatComponent.addServerSystemMessage -> (not mapped yet)
//   ChatListener.handleDisguisedChatMessage -> (not mapped yet)
//   ChatListener.handleSystemMessage -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinChatListener.java:14:
// | @Inject(method = "handleDisguisedChatMessage", cancellable = true, at = @At("HEAD"))
// | public void onHandleDisguisedChatMessag(Component component, Bound bound, CallbackInfo info) {
// |    if (!XaeroWorldMapCore.onHandleDisguisedChatMessage(bound, component)) {
// |       info.cancel();
// |    }
// | }
