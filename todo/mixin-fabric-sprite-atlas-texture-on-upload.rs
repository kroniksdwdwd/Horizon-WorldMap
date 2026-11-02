// Not ported yet: the Java @Inject(RETURN) on TextureAtlas.upload (MixinFabricSpriteAtlasTexture.onUpload).
// Why it's off: No Horizon function mapped for TextureAtlas.upload (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for TextureAtlas.upload: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   TextureAtlas.upload -> (not mapped yet)
//
// Decompiled Java, xaero/map/mixin/MixinFabricSpriteAtlasTexture.java:13:
// | @Inject(at = @At("RETURN"), method = "upload")
// | public void onUpload(Preparations spriteAtlasTexture$Data_1, CallbackInfo info) {
// |    if (WorldMap.loaded) {
// |       WorldMap.events.handleTextureStitchEventPost((TextureAtlas)this);
// |    }
// | }
