# Horizon port report: Xaero's World Map

`xaeroworldmap` 1.47.2 · xaeroworldmap-fabric-26.3-1.47.2.jar · 389 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 6 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 2 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 26 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 17 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: xaero.map.WorldMapFabric
- `server`: xaero.map.WorldMapFabric
- `modmenu`: xaero.map.mods.XaeroWorldMapModMenu

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| KeyMappingHelper.registerKeyMapping | `WorldMapFabric` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on ChatListener.handleDisguisedChatMessage | `MixinChatListener.onHandleDisguisedChatMessag` | HEAD | needs-mapping | No Horizon function known for ChatListener.handleDisguisedChatMessage: run `match`. |
| @Inject(INVOKE) on ChatListener.handleSystemMessage at Lnet/minecraft/client/gui/components/ChatComponent;addServerSystemMessage(Lnet/minecraft/network/chat/Component;)V | `MixinChatListener.onHandleSystemChat` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ChatListener.handleSystemMessage: run `match`. The call it hooks (ChatComponent.addServerSystemMessage) isn't mapped to a Rust function yet. |
| mixin into ClientboundSectionBlocksUpdatePacket with no injectors (interfaces or @Shadow only) | `MixinChunkDeltaUpdateS2CPacketAccessor` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on ClientPacketListener.handleChunkBlocksUpdate at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnChunkDeltaUpdate` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleChunkBlocksUpdate: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleLevelChunkWithLight at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onHandleLevelChunkWithLight` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLevelChunkWithLight: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleLightUpdatePacket at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onHandleLightUpdatePacket` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLightUpdatePacket: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ClientPacketListener.queueLightRemoval | `MixinClientPlayNetworkHandler.onQueueLightRemoval` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.queueLightRemoval: run `match`. |
| @Inject(INVOKE) on ClientPacketListener.handleBlockUpdate at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnBlockUpdate` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleBlockUpdate: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleSetSpawn at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinClientPlayNetworkHandler.onOnPlayerSpawnPosition` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleSetSpawn: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ClientPacketListener.close | `MixinClientPlayNetworkHandler.onCleanup` | HEAD | needs-mapping | No Horizon function known for ClientPacketListener.close: run `match`. |
| mixin into ClientLevel with no injectors (interfaces or @Shadow only) | `MixinClientWorld` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into AbstractClientPlayer with no injectors (interfaces or @Shadow only) | `MixinFabricAbstractClientPlayerEntity` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on ChatListener.showMessageToPlayer at Lnet/minecraft/client/gui/components/ChatComponent;addPlayerMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/client/multiplayer/chat/GuiMessageTag;)V | `MixinFabricChatListener.onShowMessageToPlayer` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for ChatListener.showMessageToPlayer: run `match`. The call it hooks (ChatComponent.addPlayerMessage) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on ClientPacketListener.handleLogin at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinFabricClientPacketListener.onOnGameJoin` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLogin: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(RETURN) on GameRenderer.extract | `MixinFabricGameRenderer.onRenderEnd` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for GameRenderer.extract: run `match`. |
| @Inject(RETURN) on Gui.extractRenderState | `MixinFabricGui.onRenderGuiEnd` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for Gui.extractRenderState: run `match`. |
| @Inject(HEAD) on Gui.setScreen | `MixinFabricGui.onOpenScreen` | HEAD | needs-mapping | No Horizon function known for Gui.setScreen: run `match`. |
| @Inject(HEAD) on Minecraft.tick | `MixinFabricMinecraftClient.onTickStart` | HEAD on hook `tick` | auto |  |
| @Inject(HEAD) on Minecraft.runTick | `MixinFabricMinecraftClient.onRunTickStart` | HEAD | needs-mapping | No Horizon function known for Minecraft.runTick: run `match`. |
| @Inject(HEAD) on Minecraft.disconnect | `MixinFabricMinecraftClient.onDisconnect` | HEAD | needs-mapping | No Horizon function known for Minecraft.disconnect: run `match`. |
| @Inject(HEAD) on Minecraft.setLevel | `MixinFabricMinecraftClient.onJoinWorld` | HEAD | needs-mapping | No Horizon function known for Minecraft.setLevel: run `match`. |
| @ModifyArg(INVOKE) on Minecraft.runTick at Lnet/minecraft/client/Minecraft;renderFrame(Z)V | `MixinFabricMinecraftClient.onRenderCall` | MODIFY_ARG | needs-mapping | No Horizon function known for Minecraft.runTick: run `match`. The call it hooks (Minecraft.renderFrame) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on MinecraftServer.tickServer | `MixinFabricMinecraftServer.onTick` | HEAD on horizon_client::game::levels::Game<'_>::tick_server | auto |  |
| mixin into Player with no injectors (interfaces or @Shadow only) | `MixinFabricPlayerClient` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on Player.tick | `MixinFabricPlayerEntity.onTickStart` | HEAD on horizon_client::attack_strength::AttackStrength::tick | auto |  |
| @Inject(TAIL) on PlayerList.placeNewPlayer | `MixinFabricPlayerList.onPlaceNewPlayer` | RETURN on horizon_client::lan::hub::Hub::place_new_player | auto | Horizon has no TAIL: RETURN runs at every exit, early ones too. |
| @Inject(RETURN) on TextureAtlas.upload | `MixinFabricSpriteAtlasTexture.onUpload` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for TextureAtlas.upload: run `match`. |
| @Inject(HEAD) on Level.close | `MixinFabricWorld.onClose` | HEAD | needs-mapping | No Horizon function known for Level.close: run `match`. |
| @Inject(INVOKE) on ClientPacketListener.handleLogin at Lnet/minecraft/network/protocol/PacketUtils;ensureRunningOnSameThread(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/PacketProcessor;)V | `MixinForgeClientPacketListener.onOnGameJoin` | AFTER_CALL | needs-mapping | No Horizon function known for ClientPacketListener.handleLogin: run `match`. The call it hooks (PacketUtils.ensureRunningOnSameThread) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on Minecraft.runTick | `MixinForgeMinecraftClient.onRunTickStart` | HEAD | needs-mapping | No Horizon function known for Minecraft.runTick: run `match`. |
| @ModifyArg(INVOKE) on Minecraft.runTick at Lnet/minecraft/client/Minecraft;renderFrame(Z)V | `MixinForgeMinecraftClient.onRenderCall` | MODIFY_ARG | needs-mapping | No Horizon function known for Minecraft.runTick: run `match`. The call it hooks (Minecraft.renderFrame) isn't mapped to a Rust function yet. |
| mixin into GlBuffer with no injectors (interfaces or @Shadow only) | `MixinGlBuffer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into GuiRenderer with no injectors (interfaces or @Shadow only) | `MixinGuiRenderer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(RETURN) on LevelStorageAccess.deleteLevel | `MixinLevelStorageAccess.onDeleteLevel` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for LevelStorageAccess.deleteLevel: run `match`. |
| @ModifyVariable(HEAD) on Minecraft.runTick | `MixinMinecraftClient.onRunTick` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for Minecraft.runTick: run `match`. |
| mixin into MinecraftServer with no injectors (interfaces or @Shadow only) | `MixinMinecraftServer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on Hud.extractCrosshair | `MixinOptionalGui.onRenderCrosshair` | HEAD | needs-mapping | No Horizon function known for Hud.extractCrosshair: run `match`. |
| @Inject(HEAD) on PlayerList.sendLevelInfo | `MixinPlayerList.onSendWorldInfo` | HEAD on horizon_client::lan::hub::Hub::send_level_info | auto |  |
| mixin into ServerPlayer with no injectors (interfaces or @Shadow only) | `MixinServerPlayer` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into ServerLevel with no injectors (interfaces or @Shadow only) | `MixinServerWorld` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into LevelChunk with no injectors (interfaces or @Shadow only) | `MixinWorldChunk` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends MobEffect | `WorldMapStatusEffect` | needs-translation | No mapping for this superclass yet. |
| extends BlockableEventLoop | `Executor` | needs-translation | No mapping for this superclass yet. |
| extends TextureTarget | `ImprovedFramebuffer` | needs-translation | No mapping for this superclass yet. |
| extends ConfirmScreen | `ConfirmScreenBase` | needs-translation | No mapping for this superclass yet. |
| extends Plain | `TooltipButton` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSliderButton | `TooltipSlider` | needs-translation | No mapping for this superclass yet. |
| extends BlockState | `UnknownBlockState` | needs-translation | No mapping for this superclass yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| registers blocks or items in code | `WorldMapFabric` | content | Registration and properties: horizon-port-pack carries them as a content mod. |
| 10 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

36 of 562 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/Minecraft#tick` | hook `tick` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/multiplayer/ClientLevel#getHeight` | horizon_world::level::feature_world::<LevelFeatureWorld<'_, '_, L> as PlacementContext>::height (suggested) |
| `net/minecraft/client/multiplayer/ClientLevel#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/client/multiplayer/ClientLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendChat` | horizon_client::game::remote::Game<'_>::send_chat (suggested) |
| `net/minecraft/client/resources/sounds/SimpleSoundInstance#forUI` | horizon_client_render::sound_engine::SoundInstance::for_ui (suggested) |
| `net/minecraft/core/Registry#registerForHolder` | content |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/server/MinecraftServer#tickServer` | horizon_client::game::levels::Game<'_>::tick_server (suggested) |
| `net/minecraft/server/level/ServerLevel#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/server/level/ServerLevel#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/server/players/PlayerList#placeNewPlayer` | horizon_client::lan::hub::Hub::place_new_player (suggested) |
| `net/minecraft/server/players/PlayerList#sendLevelInfo` | horizon_client::lan::hub::Hub::send_level_info (suggested) |
| `net/minecraft/util/ARGB#colorFromFloat` | horizon_client_render::hud_layout::color_from_float (suggested) |
| `net/minecraft/util/BitStorage#getBits` | horizon_core::paletted::PalettedContainer::bits (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getYRot` | `self.native_player()?.yaw` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/Player#tick` | horizon_client::attack_strength::AttackStrength::tick (suggested) |
| `net/minecraft/world/level/Level#getMaxY` | horizon_client::game::Dim::max_y (suggested) |
| `net/minecraft/world/level/Level#getMinY` | horizon_client::game::Dim::min_y (suggested) |
| `net/minecraft/world/level/block/state/BlockState#canBeReplaced` | horizon_world::level::place::Sim<'_, L>::can_be_replaced (suggested) |
| `net/minecraft/world/level/dimension/DimensionType#ambientLight` | horizon_client::mobs::spawn_selection::ambient_light (suggested) |
| `net/minecraft/world/level/dimension/DimensionType#logicalHeight` | horizon_client::game::Dim::logical_height (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/Blaze3D#openPath` | method | 1 |
| `com/mojang/blaze3d/ProjectionType#ORTHOGRAPHIC` | field | 5 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getColorTextureView` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#getDepthTextureView` | method | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#hasDepth` | method | 1 |
| `com/mojang/blaze3d/pipeline/TextureTarget#<init>` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#MOUSE` | field | 1 |
| `com/mojang/blaze3d/platform/Lighting#setupFor` | method | 1 |
| `com/mojang/blaze3d/platform/Lighting$Entry#ITEMS_FLAT` | field | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScale` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getScreenHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getScreenWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#disableScissorForRenderTypeDraws` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 13 |
| `com/mojang/blaze3d/systems/RenderSystem#getModelViewStack` | method | 5 |
| `com/mojang/blaze3d/systems/RenderSystem#getProjectionMatrixBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getProjectionType` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getSamplerCache` | method | 2 |
| `com/mojang/blaze3d/systems/RenderSystem#getSequentialBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#setProjectionMatrix` | method | 6 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#getBuffer` | method | 1 |
| `com/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer#type` | method | 1 |
| `com/mojang/blaze3d/systems/SamplerCache#getRepeat` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/BufferBuilder#addVertex` | method | 2 |
| `com/mojang/blaze3d/vertex/BufferBuilder#build` | method | 2 |
| `com/mojang/blaze3d/vertex/ByteBufferBuilder#<init>` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#close` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#drawState` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#indexBuffer` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData#vertexBuffer` | method | 1 |
| `com/mojang/blaze3d/vertex/MeshData$DrawState#indexCount` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 3 |
| `com/mojang/blaze3d/vertex/PoseStack#last` | method | 9 |
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 7 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 7 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 8 |
| `com/mojang/blaze3d/vertex/PoseStack#setIdentity` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 11 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#normal` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack$Pose#pose` | method | 9 |
| `com/mojang/blaze3d/vertex/VertexConsumer#addVertex` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setColor` | method | 2 |
| `com/mojang/blaze3d/vertex/VertexConsumer#setUv` | method | 2 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GREEN` | field | 4 |
| `net/minecraft/ChatFormatting#DARK_RED` | field | 1 |
| `net/minecraft/ChatFormatting#GOLD` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 4 |
| `net/minecraft/ChatFormatting#GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 4 |
| `net/minecraft/ChatFormatting#WHITE` | field | 1 |
| `net/minecraft/IdentifierException#getMessage` | method | 1 |
| `net/minecraft/client/ClientClockManager#getInstance` | method | 1 |
| `net/minecraft/client/ClientClockManager$ClientClockInstance#<init>` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 2 |
| `net/minecraft/client/KeyMapping#getTranslatedKeyMessage` | method | 1 |
| `net/minecraft/client/KeyMapping#matches` | method | 1 |
| `net/minecraft/client/KeyMapping#setDown` | method | 2 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/Minecraft#disconnect` | method | 1 |
| `net/minecraft/client/Minecraft#gameMode` | field | 3 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 7 |
| `net/minecraft/client/Minecraft#getAtlasManager` | method | 1 |
| `net/minecraft/client/Minecraft#getBlockColors` | method | 1 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 5 |
| `net/minecraft/client/Minecraft#getConnection` | method | 6 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getFixerUpper` | method | 1 |
| `net/minecraft/client/Minecraft#getModelManager` | method | 3 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 1 |
| `net/minecraft/client/Minecraft#getResourcePackRepository` | method | 1 |
| `net/minecraft/client/Minecraft#getSingleplayerServer` | method | 5 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#getTextureManager` | method | 5 |
| `net/minecraft/client/Minecraft#getWindow` | method | 3 |
| `net/minecraft/client/Minecraft#gui` | field | 19 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#isRunning` | method | 1 |
| `net/minecraft/client/Minecraft#isSameThread` | method | 6 |
| `net/minecraft/client/Minecraft#level` | field | 15 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 1 |
| `net/minecraft/client/Minecraft#options` | field | 2 |
| `net/minecraft/client/Minecraft#renderFrame` | method | 2 |
| `net/minecraft/client/Minecraft#runTick` | method | 5 |
| `net/minecraft/client/Minecraft#setLevel` | method | 1 |
| `net/minecraft/client/Minecraft#submit` | method | 2 |
| `net/minecraft/client/MouseHandler#xpos` | method | 1 |
| `net/minecraft/client/MouseHandler#ypos` | method | 1 |
| `net/minecraft/client/Options#getEffectiveRenderDistance` | method | 1 |
| `net/minecraft/client/Options#keyPlayerList` | field | 1 |
| `net/minecraft/client/color/block/BlockColors#getTintSource` | method | 1 |
| `net/minecraft/client/color/block/BlockTintSource#colorInWorld` | method | 1 |
| `net/minecraft/client/gui/Font$DisplayMode#NORMAL` | field | 1 |
| `net/minecraft/client/gui/Gui#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 3 |
| `net/minecraft/client/gui/Gui#overlay` | method | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 8 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 15 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 8 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 6 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 8 |
| `net/minecraft/client/gui/Hud#extractCrosshair` | method | 1 |
| `net/minecraft/client/gui/Hud#getChat` | method | 2 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 1 |
| `net/minecraft/client/gui/components/AbstractSliderButton#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#isFocused` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setFocused` | method | 1 |
| `net/minecraft/client/gui/components/Button#active` | field | 5 |
| `net/minecraft/client/gui/components/Button#builder` | method | 1 |
| `net/minecraft/client/gui/components/Button#onClick` | method | 1 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 2 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 1 |
| `net/minecraft/client/gui/components/Button$Plain#<init>` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addClientSystemMessage` | method | 2 |
| `net/minecraft/client/gui/components/ChatComponent#addPlayerMessage` | method | 1 |
| `net/minecraft/client/gui/components/ChatComponent#addServerSystemMessage` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#<init>` | method | 6 |
| `net/minecraft/client/gui/components/EditBox#active` | field | 2 |
| `net/minecraft/client/gui/components/EditBox#extractRenderState` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#getValue` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#getY` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#isFocused` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setFocused` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setMaxLength` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#setResponder` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#setSuggestion` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setTextColor` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setValue` | method | 6 |
| `net/minecraft/client/gui/render/TextureSetup#singleTexture` | method | 1 |
| `net/minecraft/client/gui/screens/ConfirmScreen#<init>` | method | 4 |
| `net/minecraft/client/gui/screens/ConfirmScreen#extractBackground` | method | 1 |
| `net/minecraft/client/gui/screens/ConfirmScreen#init` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#extractRenderStateWithTooltipAndSubtitles` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 1 |
| `net/minecraft/client/gui/screens/options/controls/KeyBindsScreen#<init>` | method | 1 |
| `net/minecraft/client/input/KeyEvent#key` | method | 5 |
| `net/minecraft/client/input/MouseButtonEvent#<init>` | method | 3 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 1 |
| `net/minecraft/client/input/MouseButtonInfo#<init>` | method | 3 |
| `net/minecraft/client/multiplayer/ClientChunkCache#getLightEngine` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#cardinalLighting` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#clockManager` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#dimension` | method | 11 |
| `net/minecraft/client/multiplayer/ClientLevel#dimensionType` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getBiome` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockEntity` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getBlockState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getChunkSource` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getFluidState` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getLightEngine` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#getPlayerByUUID` | method | 2 |
| `net/minecraft/client/multiplayer/ClientLevel#holderLookup` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#pollLightUpdates` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#close` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getPlayerInfo` | method | 5 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getServerData` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleBlockUpdate` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleChunkBlocksUpdate` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLevelChunkWithLight` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLightUpdatePacket` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleLogin` | method | 2 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleSetSpawn` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#queueLightRemoval` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendCommand` | method | 3 |
| `net/minecraft/client/multiplayer/MultiPlayerGameMode#canHurtPlayer` | method | 3 |
| `net/minecraft/client/multiplayer/PlayerInfo#getProfile` | method | 3 |
| `net/minecraft/client/multiplayer/PlayerInfo#getSkin` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData#ip` | field | 1 |
| `net/minecraft/client/multiplayer/ServerData#isRealm` | method | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#handleDisguisedChatMessage` | method | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#handleSystemMessage` | method | 1 |
| `net/minecraft/client/multiplayer/chat/ChatListener#showMessageToPlayer` | method | 1 |
| `net/minecraft/client/player/AbstractClientPlayer#getSkin` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#chunkPosition` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#connection` | field | 4 |
| `net/minecraft/client/player/LocalPlayer#getUUID` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getX` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getY` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getZ` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#permissions` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#extract` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#gameRenderState` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#lighting` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#lightmap` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainRenderTarget` | method | 4 |
| `net/minecraft/client/renderer/GameRenderer#overlayTexture` | method | 1 |
| `net/minecraft/client/renderer/Projection#<init>` | method | 4 |
| `net/minecraft/client/renderer/Projection#invertY` | method | 1 |
| `net/minecraft/client/renderer/Projection#setupOrtho` | method | 5 |
| `net/minecraft/client/renderer/Projection#zFar` | method | 1 |
| `net/minecraft/client/renderer/Projection#zNear` | method | 1 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#<init>` | method | 4 |
| `net/minecraft/client/renderer/ProjectionMatrixBuffer#getBuffer` | method | 5 |
| `net/minecraft/client/renderer/RenderPipelines#GUI` | field | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 5 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/block/BlockStateModelSet#getParticleMaterial` | method | 1 |
| `net/minecraft/client/renderer/block/FluidModel#tintSource` | method | 1 |
| `net/minecraft/client/renderer/block/FluidStateModelSet#get` | method | 1 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType$Texture#<init>` | method | 3 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType$Texture#name` | method | 1 |
| `net/minecraft/client/renderer/rendertype/PreparedRenderType$Texture#sampler` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#builder` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup#prepareTextures` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderSetup$RenderSetupBuilder#withTexture` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#format` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#pipeline` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#primitiveTopology` | method | 1 |
| `net/minecraft/client/renderer/state/GameRenderState#guiRenderState` | field | 1 |
| `net/minecraft/client/renderer/state/gui/BlitRenderState#<init>` | method | 1 |
| `net/minecraft/client/renderer/state/gui/GuiRenderState#addGuiElement` | method | 1 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getSampler` | method | 2 |
| `net/minecraft/client/renderer/texture/AbstractTexture#getTextureView` | method | 3 |
| `net/minecraft/client/renderer/texture/MissingTextureAtlasSprite#getLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#getTextureView` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#LOCATION_BLOCKS` | field | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#getSprite` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#location` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlas#upload` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#atlasLocation` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getU1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV0` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureAtlasSprite#getV1` | method | 1 |
| `net/minecraft/client/renderer/texture/TextureManager#getTexture` | method | 4 |
| `net/minecraft/client/resources/DefaultPlayerSkin#get` | method | 1 |
| `net/minecraft/client/resources/language/I18n#get` | method | 11 |
| `net/minecraft/client/resources/model/ModelManager#getBlockStateModelSet` | method | 2 |
| `net/minecraft/client/resources/model/ModelManager#getFluidStateModelSet` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad#materialInfo` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#sprite` | method | 1 |
| `net/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo#tintIndex` | method | 1 |
| `net/minecraft/client/resources/model/sprite/AtlasManager#getAtlasOrThrow` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#getLevel` | method | 2 |
| `net/minecraft/client/server/IntegratedServer#getWorldPath` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 1 |
| `net/minecraft/commands/Commands#LEVEL_GAMEMASTERS` | field | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 2 |
| `net/minecraft/core/BlockPos#equals` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 5 |
| `net/minecraft/core/BlockPos#getY` | method | 3 |
| `net/minecraft/core/BlockPos#getZ` | method | 5 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 7 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 8 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 2 |
| `net/minecraft/core/ClientAsset$Texture#texturePath` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getValue` | method | 1 |
| `net/minecraft/core/Direction#UP` | field | 1 |
| `net/minecraft/core/Direction$Axis#Y` | field | 1 |
| `net/minecraft/core/Holder#direct` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 3 |
| `net/minecraft/core/HolderSet#forEach` | method | 1 |
| `net/minecraft/core/IdMapper#byId` | method | 1 |
| `net/minecraft/core/IdMapper#getId` | method | 1 |
| `net/minecraft/core/NonNullList#get` | method | 1 |
| `net/minecraft/core/NonNullList#size` | method | 1 |
| `net/minecraft/core/Registry#entrySet` | method | 1 |
| `net/minecraft/core/Registry#getKey` | method | 1 |
| `net/minecraft/core/Registry#getResourceKey` | method | 2 |
| `net/minecraft/core/Registry#getValue` | method | 6 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 2 |
| `net/minecraft/core/SectionPos#getX` | method | 1 |
| `net/minecraft/core/SectionPos#getZ` | method | 1 |
| `net/minecraft/core/UUIDUtil#uuidFromIntArray` | method | 1 |
| `net/minecraft/core/UUIDUtil#uuidToIntArray` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MOB_EFFECT` | field | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 5 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 2 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 6 |
| `net/minecraft/core/registries/Registries#DIMENSION_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#FLUID` | field | 1 |
| `net/minecraft/data/AtlasIds#BLOCKS` | field | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 2 |
| `net/minecraft/nbt/CompoundTag#contains` | method | 2 |
| `net/minecraft/nbt/CompoundTag#get` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getBoolean` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getByte` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getByteArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompound` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getCompoundOrEmpty` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getDouble` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getInt` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getIntArray` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getLongArray` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getString` | method | 3 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 2 |
| `net/minecraft/nbt/CompoundTag#isEmpty` | method | 2 |
| `net/minecraft/nbt/CompoundTag#put` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putBoolean` | method | 2 |
| `net/minecraft/nbt/CompoundTag#putDouble` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putIntArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putLongArray` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putString` | method | 2 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 1 |
| `net/minecraft/nbt/CompoundTag#toString` | method | 1 |
| `net/minecraft/nbt/IntArrayTag#<init>` | method | 1 |
| `net/minecraft/nbt/ListTag#forEach` | method | 2 |
| `net/minecraft/nbt/ListTag#get` | method | 1 |
| `net/minecraft/nbt/ListTag#getCompoundOrEmpty` | method | 1 |
| `net/minecraft/nbt/ListTag#getString` | method | 1 |
| `net/minecraft/nbt/ListTag#isEmpty` | method | 2 |
| `net/minecraft/nbt/ListTag#size` | method | 2 |
| `net/minecraft/nbt/NbtAccounter#unlimitedHeap` | method | 1 |
| `net/minecraft/nbt/NbtIo#read` | method | 3 |
| `net/minecraft/nbt/NbtIo#write` | method | 2 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 1 |
| `net/minecraft/nbt/NbtUtils#readBlockState` | method | 2 |
| `net/minecraft/nbt/NbtUtils#writeBlockState` | method | 1 |
| `net/minecraft/nbt/StringTag#value` | method | 1 |
| `net/minecraft/nbt/StringTag#valueOf` | method | 1 |
| `net/minecraft/nbt/Tag#toString` | method | 1 |
| `net/minecraft/network/FriendlyByteBuf#readInt` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#readNbt` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#writeInt` | method | 2 |
| `net/minecraft/network/FriendlyByteBuf#writeNbt` | method | 2 |
| `net/minecraft/network/chat/CommonComponents#optionNameValue` | method | 2 |
| `net/minecraft/network/chat/Component#copy` | method | 1 |
| `net/minecraft/network/chat/Component#getSiblings` | method | 3 |
| `net/minecraft/network/chat/Component#getString` | method | 5 |
| `net/minecraft/network/chat/Component#literal` | method | 17 |
| `net/minecraft/network/chat/Component#translatable` | method | 34 |
| `net/minecraft/network/chat/MutableComponent#getSiblings` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getStyle` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#setStyle` | method | 1 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 7 |
| `net/minecraft/network/chat/Style#withColor` | method | 9 |
| `net/minecraft/network/protocol/PacketUtils#ensureRunningOnSameThread` | method | 7 |
| `net/minecraft/network/protocol/game/ClientboundBlockUpdatePacket#getPos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundForgetLevelChunkPacket#pos` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket#x` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket#z` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLightUpdatePacket#x` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLightUpdatePacket#z` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundLoginPacket#commonPlayerSpawnInfo` | method | 1 |
| `net/minecraft/network/protocol/game/ClientboundSetDefaultSpawnPositionPacket#respawnData` | method | 1 |
| `net/minecraft/network/protocol/game/CommonPlayerSpawnInfo#seed` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 4 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 7 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 3 |
| `net/minecraft/resources/Identifier#getPath` | method | 3 |
| `net/minecraft/resources/Identifier#parse` | method | 11 |
| `net/minecraft/resources/Identifier#toString` | method | 9 |
| `net/minecraft/resources/ResourceKey#create` | method | 10 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 20 |
| `net/minecraft/server/MinecraftServer#getPlayerList` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldPath` | method | 2 |
| `net/minecraft/server/MinecraftServer#submit` | method | 1 |
| `net/minecraft/server/level/ChunkMap#read` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#chunkMap` | field | 1 |
| `net/minecraft/server/level/ServerChunkCache#save` | method | 1 |
| `net/minecraft/server/level/ServerLevel#dimensionType` | method | 2 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 5 |
| `net/minecraft/server/level/ServerPlayer#connection` | field | 1 |
| `net/minecraft/server/level/ServerPlayer#getUUID` | method | 3 |
| `net/minecraft/server/level/ServerPlayer#level` | method | 3 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#disconnect` | method | 1 |
| `net/minecraft/server/packs/repository/Pack#getId` | method | 1 |
| `net/minecraft/server/packs/repository/PackRepository#getSelectedPacks` | method | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 1 |
| `net/minecraft/server/permissions/PermissionCheck#check` | method | 1 |
| `net/minecraft/server/players/PlayerList#getPlayers` | method | 1 |
| `net/minecraft/sounds/SoundEvents#UI_BUTTON_CLICK` | field | 1 |
| `net/minecraft/tags/BlockTags#FLOWERS` | field | 1 |
| `net/minecraft/tags/BlockTags#LEAVES` | field | 2 |
| `net/minecraft/util/ARGB#alpha` | method | 1 |
| `net/minecraft/util/BitStorage#get` | method | 2 |
| `net/minecraft/util/BitStorage#getRaw` | method | 1 |
| `net/minecraft/util/Mth#ceillog2` | method | 2 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/util/SimpleBitStorage#<init>` | method | 2 |
| `net/minecraft/util/datafix/DataFixTypes#CHUNK` | field | 1 |
| `net/minecraft/util/datafix/DataFixTypes#updateToCurrentVersion` | method | 1 |
| `net/minecraft/util/thread/BlockableEventLoop#<init>` | method | 1 |
| `net/minecraft/util/thread/BlockableEventLoop#runAllTasks` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#builder` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#getValue` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#invalidateTickCache` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#addConstantLayer` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#addTimelineLayer` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#build` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#SKY_LIGHT_FACTOR` | field | 1 |
| `net/minecraft/world/effect/MobEffect#<init>` | method | 1 |
| `net/minecraft/world/effect/MobEffectCategory#BENEFICIAL` | field | 1 |
| `net/minecraft/world/effect/MobEffectCategory#HARMFUL` | field | 2 |
| `net/minecraft/world/effect/MobEffectCategory#NEUTRAL` | field | 1 |
| `net/minecraft/world/entity/Entity#getTeam` | method | 1 |
| `net/minecraft/world/entity/Entity#level` | method | 2 |
| `net/minecraft/world/entity/Entity#tickCount` | field | 1 |
| `net/minecraft/world/entity/Entity#xOld` | field | 1 |
| `net/minecraft/world/entity/Entity#yOld` | field | 1 |
| `net/minecraft/world/entity/Entity#zOld` | field | 1 |
| `net/minecraft/world/entity/player/Inventory#getContainerSize` | method | 1 |
| `net/minecraft/world/entity/player/Inventory#getItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 1 |
| `net/minecraft/world/entity/player/Player#getCustomName` | method | 1 |
| `net/minecraft/world/entity/player/Player#getInventory` | method | 1 |
| `net/minecraft/world/entity/player/Player#getUUID` | method | 4 |
| `net/minecraft/world/entity/player/Player#getX` | method | 2 |
| `net/minecraft/world/entity/player/Player#getY` | method | 2 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 2 |
| `net/minecraft/world/entity/player/Player#isModelPartShown` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 2 |
| `net/minecraft/world/entity/player/Player#xo` | field | 1 |
| `net/minecraft/world/entity/player/Player#yo` | field | 1 |
| `net/minecraft/world/entity/player/Player#zo` | field | 1 |
| `net/minecraft/world/entity/player/PlayerModelPart#HAT` | field | 1 |
| `net/minecraft/world/entity/player/PlayerSkin#body` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItemName` | method | 1 |
| `net/minecraft/world/item/Items#AIR` | field | 1 |
| `net/minecraft/world/level/CardinalLighting$Type#get` | method | 1 |
| `net/minecraft/world/level/ChunkPos#<init>` | method | 3 |
| `net/minecraft/world/level/ChunkPos#x` | method | 3 |
| `net/minecraft/world/level/ChunkPos#z` | method | 3 |
| `net/minecraft/world/level/ColorResolver#getColor` | method | 2 |
| `net/minecraft/world/level/Level#END` | field | 4 |
| `net/minecraft/world/level/Level#NETHER` | field | 6 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 5 |
| `net/minecraft/world/level/Level#close` | method | 1 |
| `net/minecraft/world/level/Level#dimension` | method | 4 |
| `net/minecraft/world/level/Level#dimensionType` | method | 2 |
| `net/minecraft/world/level/Level#dimensionTypeRegistration` | method | 1 |
| `net/minecraft/world/level/Level#getBiome` | method | 1 |
| `net/minecraft/world/level/Level#getBlockEntity` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getBrightness` | method | 2 |
| `net/minecraft/world/level/Level#getChunk` | method | 4 |
| `net/minecraft/world/level/Level#getServer` | method | 1 |
| `net/minecraft/world/level/Level#registryAccess` | method | 1 |
| `net/minecraft/world/level/LightLayer#BLOCK` | field | 1 |
| `net/minecraft/world/level/LightLayer#SKY` | field | 2 |
| `net/minecraft/world/level/biome/BiomeManager#<init>` | method | 1 |
| `net/minecraft/world/level/biome/BiomeManager#getBiome` | method | 1 |
| `net/minecraft/world/level/biome/Biomes#PLAINS` | field | 1 |
| `net/minecraft/world/level/biome/Biomes#RIVER` | field | 1 |
| `net/minecraft/world/level/biome/Biomes#THE_VOID` | field | 2 |
| `net/minecraft/world/level/block/Block#BLOCK_STATE_REGISTRY` | field | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 9 |
| `net/minecraft/world/level/block/Block#stateById` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 9 |
| `net/minecraft/world/level/block/Blocks#BARRIER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GLASS_PANE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#GRASS_BLOCK` | field | 2 |
| `net/minecraft/world/level/block/Blocks#SHORT_GRASS` | field | 1 |
| `net/minecraft/world/level/block/Blocks#TORCH` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 2 |
| `net/minecraft/world/level/block/RenderShape#INVISIBLE` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#<init>` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#CODEC` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 11 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 5 |
| `net/minecraft/world/level/block/state/BlockState#getLightDampening` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getLightEmission` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getMapColor` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#getPistonPushReaction` | method | 3 |
| `net/minecraft/world/level/block/state/BlockState#getRenderShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getShape` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#ignitedByLava` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkAccess#getHeight` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getSections` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getHeight` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getMinY` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getPos` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunk#getSections` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#hasOnlyAir` | method | 2 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#BIOMES` | field | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#FEATURES` | field | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#FULL` | field | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#byName` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#getIndex` | method | 1 |
| `net/minecraft/world/level/chunk/storage/RegionFile#getChunkDataInputStream` | method | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#END` | field | 1 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#NETHER` | field | 2 |
| `net/minecraft/world/level/dimension/BuiltinDimensionTypes#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/dimension/DimensionType#attributes` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#cardinalLightType` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#coordinateScale` | method | 3 |
| `net/minecraft/world/level/dimension/DimensionType#getStorageFolder` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#hasEndFlashes` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#hasSkyLight` | method | 3 |
| `net/minecraft/world/level/dimension/DimensionType#timelines` | method | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE` | field | 2 |
| `net/minecraft/world/level/lighting/LevelLightEngine#runLightUpdates` | method | 1 |
| `net/minecraft/world/level/material/Fluid#defaultFluidState` | method | 1 |
| `net/minecraft/world/level/material/FluidState#createLegacyBlock` | method | 2 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 3 |
| `net/minecraft/world/level/material/MapColor#col` | field | 2 |
| `net/minecraft/world/level/material/PushReaction#POPPED` | field | 3 |
| `net/minecraft/world/level/storage/LevelData$RespawnData#pos` | method | 1 |
| `net/minecraft/world/level/storage/LevelResource#LEVEL_DATA_FILE` | field | 1 |
| `net/minecraft/world/level/storage/LevelResource#ROOT` | field | 3 |
| `net/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess#deleteLevel` | method | 1 |
| `net/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess#getLevelPath` | method | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 2 |
| `net/minecraft/world/phys/Vec3#x` | field | 1 |
| `net/minecraft/world/phys/Vec3#z` | field | 1 |
| `net/minecraft/world/phys/shapes/VoxelShape#max` | method | 1 |
| `net/minecraft/world/scores/Team#getColor` | method | 1 |
| `net/minecraft/world/scores/TeamColor#WHITE` | field | 1 |
| `net/minecraft/world/scores/TeamColor#rgb` | method | 1 |

</details>

