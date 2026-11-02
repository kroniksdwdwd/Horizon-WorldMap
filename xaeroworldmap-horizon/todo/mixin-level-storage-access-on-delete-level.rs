// Not ported yet: the Java @Inject(RETURN) on LevelStorageAccess.deleteLevel (MixinLevelStorageAccess.onDeleteLevel).
// Why it's off: No Horizon function mapped for LevelStorageAccess.deleteLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for LevelStorageAccess.deleteLevel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelStorageAccess.deleteLevel -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinLevelStorageAccess.java:12:
// | @Inject(at = @At("RETURN"), method = "deleteLevel", cancellable = false)
// | public void onDeleteLevel(CallbackInfo info) {
// |    XaeroWorldMapCore.onDeleteWorld((LevelStorageAccess)this);
// | }
