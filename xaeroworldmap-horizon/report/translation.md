# Java translated to Rust

Translated 106 of 2596 methods (1 more is done another way in Horizon) and 140 of 303 static fields to Rust (21 of those call the game, through Horizon's helpers); 0 of 3 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 2150 | `SinAnimation.SinAnimation(3)`: its class isn't translated (it extends `Animation`) |
| Other Java this translator doesn't handle yet | 188 | `IGuiRenderer.xaero_wm_getGuiProjection(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 82 | `CapabilityGetter.getServerWorldCapabilities(1)`: it uses Minecraft's `ServerLevel` |
| Uses other libraries | 38 | `LegacyCommonConfigIO.load(0)`: it uses `ConfigProfile` (xaero.lib.common.config.profile) |
| Uses null | 8 | `LegacyCommonConfigIO.readLine(1)`: a nullable object kept in a changing static |
| Shares one object between names (Java references) | 6 | `MapElementRenderHandler.Builder.setPoseStack(1)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Uses files (java.nio / java.io) | 6 | `IOHelper.readToBuffer(3)`: it uses `DataInputStream` (java.io) |
| Uses try/catch | 5 | `XaeroWorldMapCore.cleanupCurrentSession(0)`: a try/finally |
| Calls the game where Horizon has no helper yet | 3 | `Misc.getMouseX(2)`: it uses Minecraft's Minecraft.mouseHandler (no Horizon helper for it yet) |
| Uses Java collections other than List (Map, Set...) | 2 | `TextureUploadBenchmark.getTotalsString(0)`: it calls `Arrays` (java.util).toString |
| Done another way in Horizon | 1 | `XaeroWorldMapModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses inheritance (extends, implements, instanceof) | 1 | `MapProcessor.shouldSkipWorldRender(0)`: `instanceof` |

## Close, not the same

Where the Rust does a little less than the Java, because Horizon doesn't have something:

- WorldMapConfigConstants.: the text gui.xaero_off isn't in the mod's en_us.json, so the key itself shows

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| xaero/map/mixin/MixinFabricClientPacketListener.java:14 | `join` | not translated: it calls XaeroWorldMapCoreFabric.onPlayNetHandler, which isn't translated |
| xaero/map/mixin/MixinForgeClientPacketListener.java:14 | `join` | not translated: it calls XaeroWorldMapCore.onPlayNetHandler, which isn't translated |
| xaero/map/WorldMapFabric.java:40 | `init` | not translated: a catch block that uses its exception |

## Every class


### `Animation`

- [x] `Animation.Animation(3)` (xaero/map/animation/Animation.java:9)
- [x] `Animation.getCurrent(0)` (xaero/map/animation/Animation.java:16)
- [x] `Animation.getDestination(0)` (xaero/map/animation/Animation.java:20)
- [x] `Animation.getPassed(0)` (xaero/map/animation/Animation.java:24)

### `SinAnimation`: it extends `Animation`

- [ ] `SinAnimation.SinAnimation(3)`: its class isn't translated (it extends `Animation`) (xaero/map/animation/SinAnimation.java:4)
- [ ] `SinAnimation.getCurrent(0)`: its class isn't translated (it extends `Animation`) (xaero/map/animation/SinAnimation.java:8)

### `SlowingAnimation`: it extends `Animation`

- [x] `SlowingAnimation.animationThing` (xaero/map/animation/SlowingAnimation.java:4)
- [ ] `SlowingAnimation.SlowingAnimation(4)`: its class isn't translated (it extends `Animation`) (xaero/map/animation/SlowingAnimation.java:9)
- [ ] `SlowingAnimation.getCurrent(0)`: its class isn't translated (it extends `Animation`) (xaero/map/animation/SlowingAnimation.java:16)
- [ ] `SlowingAnimation.getDestination(0)`: its class isn't translated (it extends `Animation`) (xaero/map/animation/SlowingAnimation.java:23)

### `BiomeBlendCalculator`: it implements Minecraft's `BlockAndTintGetter`

- [ ] `BiomeBlendCalculator.setWorld(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:19)
- [ ] `BiomeBlendCalculator.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:23)
- [ ] `BiomeBlendCalculator.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:27)
- [ ] `BiomeBlendCalculator.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:31)
- [ ] `BiomeBlendCalculator.getLightEngine(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:35)
- [ ] `BiomeBlendCalculator.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:39)
- [ ] `BiomeBlendCalculator.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:43)
- [ ] `BiomeBlendCalculator.cardinalLighting(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:47)
- [ ] `BiomeBlendCalculator.getBlockTint(2)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BiomeBlendCalculator.java:51)

### `BiomeColorCalculator`: it uses Minecraft's `ResourceKey`

- [ ] `BiomeColorCalculator.prepare(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/biome/BiomeColorCalculator.java:23)
- [ ] `BiomeColorCalculator.getBiomeColor(7)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/biome/BiomeColorCalculator.java:33)
- [ ] `BiomeColorCalculator.getBiomeAtPos(4)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/biome/BiomeColorCalculator.java:88)

### `BiomeGetter`: it uses Minecraft's `ResourceKey`

- [ ] `BiomeGetter.BiomeGetter(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/biome/BiomeGetter.java:16)
- [ ] `BiomeGetter.getBiome(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/biome/BiomeGetter.java:20)

### `BlockTintProvider`: it implements Minecraft's `BlockAndTintGetter`

- [ ] `BlockTintProvider.BlockTintProvider(5)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:38)
- [ ] `BlockTintProvider.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:49)
- [ ] `BlockTintProvider.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:53)
- [ ] `BlockTintProvider.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:57)
- [ ] `BlockTintProvider.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:61)
- [ ] `BlockTintProvider.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:65)
- [ ] `BlockTintProvider.getLightEngine(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:69)
- [ ] `BlockTintProvider.cardinalLighting(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:73)
- [ ] `BlockTintProvider.getBlockTint(2)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:77)
- [ ] `BlockTintProvider.getBiomeColor(6)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/biome/BlockTintProvider.java:82)

### `SingleBlockAndTintGetter`: it implements Minecraft's `BlockAndTintGetter`

- [ ] `SingleBlockAndTintGetter.SingleBlockAndTintGetter(3)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:20)
- [ ] `SingleBlockAndTintGetter.getLightEngine(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:26)
- [ ] `SingleBlockAndTintGetter.cardinalLighting(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:30)
- [ ] `SingleBlockAndTintGetter.getBlockTint(2)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:34)
- [ ] `SingleBlockAndTintGetter.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:38)
- [ ] `SingleBlockAndTintGetter.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:43)
- [ ] `SingleBlockAndTintGetter.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:47)
- [ ] `SingleBlockAndTintGetter.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:51)
- [ ] `SingleBlockAndTintGetter.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockAndTintGetter`) (xaero/map/block/SingleBlockAndTintGetter.java:55)

### `BlockStateShortShapeCache`: it uses Minecraft's `BlockState`

- [ ] `BlockStateShortShapeCache.BlockStateShortShapeCache(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BlockStateShortShapeCache.java:25)
- [ ] `BlockStateShortShapeCache.scheduleAndGet(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BlockStateShortShapeCache.java:30)
- [ ] `BlockStateShortShapeCache.isShort(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BlockStateShortShapeCache.java:52)
- [ ] `BlockStateShortShapeCache.reset(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BlockStateShortShapeCache.java:90)
- [ ] `BlockStateShortShapeCache.supplyForIOThread(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BlockStateShortShapeCache.java:98)

### `BrokenBlockTintCache`: it uses Minecraft's `BlockState`

- [ ] `BrokenBlockTintCache.BrokenBlockTintCache(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BrokenBlockTintCache.java:9)
- [ ] `BrokenBlockTintCache.setBroken(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BrokenBlockTintCache.java:13)
- [ ] `BrokenBlockTintCache.isBroken(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BrokenBlockTintCache.java:17)
- [ ] `BrokenBlockTintCache.getSize(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/BrokenBlockTintCache.java:21)

### `PlaceholderBlockGetter`: it implements Minecraft's `BlockGetter`

- [ ] `PlaceholderBlockGetter.setPlaceholderState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/map/cache/placeholder/PlaceholderBlockGetter.java:12)
- [ ] `PlaceholderBlockGetter.getBlockEntity(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/map/cache/placeholder/PlaceholderBlockGetter.java:16)
- [ ] `PlaceholderBlockGetter.getBlockState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/map/cache/placeholder/PlaceholderBlockGetter.java:20)
- [ ] `PlaceholderBlockGetter.getFluidState(1)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/map/cache/placeholder/PlaceholderBlockGetter.java:24)
- [ ] `PlaceholderBlockGetter.getHeight(0)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/map/cache/placeholder/PlaceholderBlockGetter.java:28)
- [ ] `PlaceholderBlockGetter.getMinY(0)`: its class isn't translated (it implements Minecraft's `BlockGetter`) (xaero/map/cache/placeholder/PlaceholderBlockGetter.java:32)

### `UnknownBlockStateCache`: it uses Minecraft's `BlockState`

- [ ] `UnknownBlockStateCache.getBlockStateFromNBT(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/cache/UnknownBlockStateCache.java:16)

### `CapabilityGetter`

- [ ] `CapabilityGetter.getServerWorldCapabilities(1)`: it uses Minecraft's `ServerLevel` (xaero/map/capabilities/CapabilityGetter.java:7)

### `ServerWorldCapabilities`


### `WorldMapChannelCommonRegistryHandler`: it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)

- [ ] `WorldMapChannelCommonRegistryHandler.registerPrimaryCommonOptions(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/map/common/config/channel/register/handler/WorldMapChannelCommonRegistryHandler.java:13)
- [ ] `WorldMapChannelCommonRegistryHandler.registerProfiledOptions(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/map/common/config/channel/register/handler/WorldMapChannelCommonRegistryHandler.java:17)
- [ ] `WorldMapChannelCommonRegistryHandler.registerServerOptionChangeHandlers(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/map/common/config/channel/register/handler/WorldMapChannelCommonRegistryHandler.java:21)
- [ ] `WorldMapChannelCommonRegistryHandler.registerOptionServerRedirectors(1)`: its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)) (xaero/map/common/config/channel/register/handler/WorldMapChannelCommonRegistryHandler.java:25)

### `LegacyCommonConfigInit`

- [ ] `LegacyCommonConfigInit.init(1)`: it uses IPlatformHelper, which isn't translated (it's an interface) (xaero/map/common/config/LegacyCommonConfigInit.java:9)

### `LegacyCommonConfigIO`

- [x] `LegacyCommonConfigIO.LegacyCommonConfigIO(1)` (xaero/map/common/config/LegacyCommonConfigIO.java:24)
- [ ] `LegacyCommonConfigIO.load(0)`: it uses `ConfigProfile` (xaero.lib.common.config.profile) (xaero/map/common/config/LegacyCommonConfigIO.java:28)
- [ ] `LegacyCommonConfigIO.readLine(1)`: a nullable object kept in a changing static (xaero/map/common/config/LegacyCommonConfigIO.java:66)
- [x] `LegacyCommonConfigIO.shouldEnableEveryoneTracksEveryone(0)` (xaero/map/common/config/LegacyCommonConfigIO.java:88)

### `WorldMapConfigOptionServerHandlers`

- [ ] `WorldMapConfigOptionServerHandlers.registerAll(1)`: it uses `HandlerBasedConfigChangeListener` (xaero.lib.common.config.listener.handler) (xaero/map/common/config/listener/handler/WorldMapConfigOptionServerHandlers.java:6)

### `WorldMapConfigOptionServerRedirectors`

- [ ] `WorldMapConfigOptionServerRedirectors.registerAll(1)`: it uses `OptionValueRedirectorManager` (xaero.lib.common.config.option.value.redirect) (xaero/map/common/config/option/value/redirect/WorldMapConfigOptionServerRedirectors.java:6)

### `WorldMapProfiledConfigOptions`

- [ ] `WorldMapProfiledConfigOptions.ALL`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:23)
- [ ] `WorldMapProfiledConfigOptions.LIGHTING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:24)
- [ ] `WorldMapProfiledConfigOptions.BLOCK_COLORS`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:27)
- [ ] `WorldMapProfiledConfigOptions.LOAD_NEW_CHUNKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:35)
- [ ] `WorldMapProfiledConfigOptions.UPDATE_CHUNKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:38)
- [ ] `WorldMapProfiledConfigOptions.TERRAIN_DEPTH`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:41)
- [ ] `WorldMapProfiledConfigOptions.TERRAIN_SLOPES`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:44)
- [ ] `WorldMapProfiledConfigOptions.FOOTSTEPS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:52)
- [ ] `WorldMapProfiledConfigOptions.COORDINATES`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:55)
- [ ] `WorldMapProfiledConfigOptions.WAYPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:58)
- [ ] `WorldMapProfiledConfigOptions.RENDER_WAYPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:61)
- [ ] `WorldMapProfiledConfigOptions.WAYPOINT_BACKGROUNDS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:64)
- [ ] `WorldMapProfiledConfigOptions.WAYPOINT_SCALE`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:68)
- [ ] `WorldMapProfiledConfigOptions.BIOME_BLENDING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:77)
- [ ] `WorldMapProfiledConfigOptions.BIOME_COLORS_IN_VANILLA`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:82)
- [ ] `WorldMapProfiledConfigOptions.MIN_ZOOM_LOCAL_WAYPOINTS`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:86)
- [ ] `WorldMapProfiledConfigOptions.ADJUST_HEIGHT_FOR_SHORT_BLOCKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:95)
- [ ] `WorldMapProfiledConfigOptions.FLOWERS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:101)
- [ ] `WorldMapProfiledConfigOptions.STAINED_GLASS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:104)
- [ ] `WorldMapProfiledConfigOptions.CAVE_MODE_ALLOWED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:107)
- [ ] `WorldMapProfiledConfigOptions.CAVE_MODE_ALLOWED_DIMENSIONS`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:112)
- [ ] `WorldMapProfiledConfigOptions.CAVE_MODE_DEPTH`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:125)
- [ ] `WorldMapProfiledConfigOptions.LEGIBLE_CAVE_MAPS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:132)
- [ ] `WorldMapProfiledConfigOptions.AUTO_CAVE_MODE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:137)
- [ ] `WorldMapProfiledConfigOptions.CAVE_MODE_TOGGLE_TIMER`: it uses `SteppedConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:157)
- [ ] `WorldMapProfiledConfigOptions.DEFAULT_CAVE_MODE_TYPE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:167)
- [ ] `WorldMapProfiledConfigOptions.DISPLAY_CAVE_MODE_START`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:176)
- [ ] `WorldMapProfiledConfigOptions.WRITING_DISTANCE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:180)
- [ ] `WorldMapProfiledConfigOptions.ARROW`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:189)
- [ ] `WorldMapProfiledConfigOptions.OPENING_ANIMATION`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:192)
- [ ] `WorldMapProfiledConfigOptions.ARROW_COLOR`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:195)
- [ ] `WorldMapProfiledConfigOptions.DISPLAY_ZOOM`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:204)
- [ ] `WorldMapProfiledConfigOptions.DISPLAY_HOVERED_BIOME`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:207)
- [ ] `WorldMapProfiledConfigOptions.ZOOM_BUTTONS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:211)
- [ ] `WorldMapProfiledConfigOptions.DETECT_AMBIGUOUS_Y`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:214)
- [ ] `WorldMapProfiledConfigOptions.DEFAULT_MAP_TELEPORT_FORMAT`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:219)
- [ ] `WorldMapProfiledConfigOptions.DEFAULT_MAP_TELEPORT_DIMENSION_FORMAT`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:225)
- [ ] `WorldMapProfiledConfigOptions.DEFAULT_PLAYER_TELEPORT_FORMAT`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:231)
- [ ] `WorldMapProfiledConfigOptions.MAP_TELEPORT_ALLOWED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:237)
- [ ] `WorldMapProfiledConfigOptions.PARTIAL_Y_TELEPORT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:242)
- [ ] `WorldMapProfiledConfigOptions.OPAC_CLAIMS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:247)
- [ ] `WorldMapProfiledConfigOptions.OPAC_CLAIMS_BORDER_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:252)
- [ ] `WorldMapProfiledConfigOptions.OPAC_CLAIMS_FILL_OPACITY`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:260)
- [ ] `WorldMapProfiledConfigOptions.MAP_ITEM`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:268)
- [ ] `WorldMapProfiledConfigOptions.MINIMAP_RADAR`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:276)
- [ ] `WorldMapProfiledConfigOptions.DISPLAY_TRACKED_PLAYERS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:281)
- [ ] `WorldMapProfiledConfigOptions.UNLIMITED_ZOOM_IN`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:285)
- [ ] `WorldMapProfiledConfigOptions.UNLIMITED_ZOOM_OUT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:290)
- [ ] `WorldMapProfiledConfigOptions.SKIP_WORLD_RENDER`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:295)
- [ ] `WorldMapProfiledConfigOptions.SERVER_CHUNK_RADIUS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:300)
- [ ] `WorldMapProfiledConfigOptions.FORCE_FAST_WRITING`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:305)
- [ ] `WorldMapProfiledConfigOptions.DISTANCES`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:310)
- [ ] `WorldMapProfiledConfigOptions.DISTANCE_TO_KM_AT`: it uses `IndexedConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:315)
- [ ] `WorldMapProfiledConfigOptions.DISTANCE_PRECISION`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:325)
- [ ] `WorldMapProfiledConfigOptions.registerAll(1)`: it uses `ConfigOptionManager` (xaero.lib.common.config.option) (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java:333)

### `WorldMapPrimaryCommonConfigOptions`

- [ ] `WorldMapPrimaryCommonConfigOptions.ALL`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/primary/option/WorldMapPrimaryCommonConfigOptions.java:11)
- [ ] `WorldMapPrimaryCommonConfigOptions.REGISTER_EFFECTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/common/config/primary/option/WorldMapPrimaryCommonConfigOptions.java:12)
- [ ] `WorldMapPrimaryCommonConfigOptions.registerAll(1)`: it uses `ConfigOptionManager` (xaero.lib.common.config.option) (xaero/map/common/config/primary/option/WorldMapPrimaryCommonConfigOptions.java:15)

### `WorldMapConfigUtils`


### `WorldMapConfigConstants`

- [x] `WorldMapConfigConstants.INGAME_TOOLTIP` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:6)
- [x] `WorldMapConfigConstants.LEGACY_PLUGIN_TOOLTIP` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:7)
- [x] `WorldMapConfigConstants.FAIRPLAY_TOOLTIP` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:8)
- [x] `WorldMapConfigConstants.EFFECT_TOOLTIP` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:9)
- [x] `WorldMapConfigConstants.MINIMAP_TOOLTIP` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:10)
- [x] `WorldMapConfigConstants.BLOCK_COLORS_NAMES` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:11)
- [x] `WorldMapConfigConstants.TERRAIN_SLOPES_NAMES` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:12)
- [x] `WorldMapConfigConstants.SEC` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:18)
- [x] `WorldMapConfigConstants.MILLISEC` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:19)
- [x] `WorldMapConfigConstants.DEFAULT_CAVE_MODE_TYPE_NAMES` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:20)
- [x] `WorldMapConfigConstants.ARROW_COLOR_NAMES` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:25)
- [x] `WorldMapConfigConstants.ARROW_COLORS` (xaero/map/common/config/WorldMapConfigConstants.java:37)
- [x] `WorldMapConfigConstants.CONVERT_DISTANCE_NEVER_COMPONENT` in game_side.rs (xaero/map/common/config/WorldMapConfigConstants.java:47)

### `WorldMapChannelClientRegistryHandler`: it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)

- [ ] `WorldMapChannelClientRegistryHandler.registerPrimaryClientOptions(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/map/config/channel/register/handler/WorldMapChannelClientRegistryHandler.java:14)
- [ ] `WorldMapChannelClientRegistryHandler.registerConfigOptionUITypes(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/map/config/channel/register/handler/WorldMapChannelClientRegistryHandler.java:18)
- [ ] `WorldMapChannelClientRegistryHandler.registerClientOptionChangeHandlers(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/map/config/channel/register/handler/WorldMapChannelClientRegistryHandler.java:22)
- [ ] `WorldMapChannelClientRegistryHandler.registerOptionClientRedirectors(1)`: its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)) (xaero/map/config/channel/register/handler/WorldMapChannelClientRegistryHandler.java:26)

### `WorldMapConfigOptionClientHandlers`

- [ ] `WorldMapConfigOptionClientHandlers.handleBlockColors(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:12)
- [ ] `WorldMapConfigOptionClientHandlers.handleTerrainDepth(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:16)
- [ ] `WorldMapConfigOptionClientHandlers.handleTerrainSlopes(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:20)
- [ ] `WorldMapConfigOptionClientHandlers.handleBiomeBlending(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:24)
- [ ] `WorldMapConfigOptionClientHandlers.handleBiomesInVanilla(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:28)
- [ ] `WorldMapConfigOptionClientHandlers.handleAdjustShortBlockHeight(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:32)
- [ ] `WorldMapConfigOptionClientHandlers.handleStainedGlass(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:36)
- [ ] `WorldMapConfigOptionClientHandlers.handleLegibleCaveMaps(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:40)
- [ ] `WorldMapConfigOptionClientHandlers.handleFlowers(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:44)
- [ ] `WorldMapConfigOptionClientHandlers.clearAllCachedHighlightHashes(0)`: it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:54)
- [ ] `WorldMapConfigOptionClientHandlers.handleOpacClaims(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:63)
- [ ] `WorldMapConfigOptionClientHandlers.handleOpacClaimFillOpacity(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:67)
- [ ] `WorldMapConfigOptionClientHandlers.handleOpacClaimBorderOpacity(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:71)
- [ ] `WorldMapConfigOptionClientHandlers.handleReloadViewed(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:75)
- [ ] `WorldMapConfigOptionClientHandlers.handleMapItem(1)`: it uses `Config` (xaero.lib.common.config) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:81)
- [ ] `WorldMapConfigOptionClientHandlers.registerAll(1)`: it uses `ClientConfigChangeListener` (xaero.lib.client.config.listener) (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java:88)

### `WorldMapConfigOptionUITypes`

- [ ] `WorldMapConfigOptionUITypes.DEFAULT_MAP_TP_COMMAND`: it uses `ConfigOptionUIType` (xaero.lib.client.config.option.ui.type) (xaero/map/config/option/ui/type/WorldMapConfigOptionUITypes.java:10)

### `WorldMapConfigOptionUIRegister`

- [ ] `WorldMapConfigOptionUIRegister.registerAll(1)`: it uses `ConfigOptionUITypeManager` (xaero.lib.client.config.option.ui) (xaero/map/config/option/ui/WorldMapConfigOptionUIRegister.java:10)

### `WorldMapConfigOptionClientRedirectors`

- [ ] `WorldMapConfigOptionClientRedirectors.registerAll(1)`: it uses `ClientOptionValueRedirectorManager` (xaero.lib.client.config.option.value.redirect) (xaero/map/config/option/value/redirect/WorldMapConfigOptionClientRedirectors.java:13)

### `WorldMapPrimaryClientConfigOptions`

- [ ] `WorldMapPrimaryClientConfigOptions.ALL`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:16)
- [ ] `WorldMapPrimaryClientConfigOptions.IGNORED_UPDATE`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:17)
- [ ] `WorldMapPrimaryClientConfigOptions.UPDATE_NOTIFICATIONS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:23)
- [ ] `WorldMapPrimaryClientConfigOptions.RELOAD_VIEWED`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:27)
- [ ] `WorldMapPrimaryClientConfigOptions.RELOAD_VIEWED_VERSION`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:32)
- [ ] `WorldMapPrimaryClientConfigOptions.DEBUG`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:37)
- [ ] `WorldMapPrimaryClientConfigOptions.CAVE_MODE_START`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:40)
- [ ] `WorldMapPrimaryClientConfigOptions.EXPORT_MULTIPLE_IMAGES`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:46)
- [ ] `WorldMapPrimaryClientConfigOptions.NIGHT_EXPORT`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:52)
- [ ] `WorldMapPrimaryClientConfigOptions.EXPORT_SCALE_DOWN_SQUARE`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:57)
- [ ] `WorldMapPrimaryClientConfigOptions.EXPORT_HIGHLIGHTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:70)
- [ ] `WorldMapPrimaryClientConfigOptions.DIFFERENTIATE_BY_SERVER_ADDRESS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:75)
- [ ] `WorldMapPrimaryClientConfigOptions.DISPLAY_DISABLED_WAYPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:78)
- [ ] `WorldMapPrimaryClientConfigOptions.CLOSE_WAYPOINTS_AFTER_HOP`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:81)
- [ ] `WorldMapPrimaryClientConfigOptions.ONLY_CURRENT_MAP_WAYPOINTS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:84)
- [ ] `WorldMapPrimaryClientConfigOptions.GLOBAL_VERSION`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:87)
- [ ] `WorldMapPrimaryClientConfigOptions.MAX_LOADED_REGIONS`: it uses `RangeConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:91)
- [ ] `WorldMapPrimaryClientConfigOptions.IGNORE_MAP_FILE_LOCKS`: it uses `BooleanConfigOption` (xaero.lib.common.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:99)
- [ ] `WorldMapPrimaryClientConfigOptions.registerAll(1)`: it uses `ClientConfigOptionManager` (xaero.lib.client.config.option) (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java:105)

### `WorldMapClientConfigUtils`

- [ ] `WorldMapClientConfigUtils.isFairPlay(0)`: a nullable object kept in a changing static (xaero/map/config/util/WorldMapClientConfigUtils.java:22)
- [ ] `WorldMapClientConfigUtils.isCaveModeDisabledLegacy(0)`: a nullable object kept in a changing static (xaero/map/config/util/WorldMapClientConfigUtils.java:39)
- [ ] `WorldMapClientConfigUtils.getEffectiveCaveModeAllowed(0)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/map/config/util/WorldMapClientConfigUtils.java:53)
- [ ] `WorldMapClientConfigUtils.getDebug(0)`: it uses `ClientConfigManager` (xaero.lib.client.config) (xaero/map/config/util/WorldMapClientConfigUtils.java:84)
- [ ] `WorldMapClientConfigUtils.togglePrimaryOption(1)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/util/WorldMapClientConfigUtils.java:95)
- [ ] `WorldMapClientConfigUtils.tryTogglingCurrentProfileOption(1)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/util/WorldMapClientConfigUtils.java:102)
- [ ] `WorldMapClientConfigUtils.isOptionServerEnforced(1)`: it uses `ConfigOption` (xaero.lib.common.config.option) (xaero/map/config/util/WorldMapClientConfigUtils.java:111)

### `ControlsHandler`: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))

- [ ] `ControlsHandler.ControlsHandler(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/controls/ControlsHandler.java:23)
- [ ] `ControlsHandler.eventExists(1)`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsHandler.java:27)
- [ ] `ControlsHandler.oldEventExists(1)`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsHandler.java:37)
- [ ] `ControlsHandler.isKeyRepeat(1)`: Minecraft's KeyMapping kept in a variable or field (xaero/map/controls/ControlsHandler.java:47)
- [ ] `ControlsHandler.keyDown(3)`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsHandler.java:54)
- [ ] `ControlsHandler.keyUp(2)`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsHandler.java:84)
- [ ] `ControlsHandler.handleKeyEvents(0)`: it uses Minecraft's `Minecraft` (xaero/map/controls/ControlsHandler.java:89)
- [ ] `ControlsHandler.onKeyInput(1)`: it uses Minecraft's `Minecraft` (xaero/map/controls/ControlsHandler.java:123)

### `ControlsRegister`: it uses Minecraft's `KeyMapping`

- [ ] `ControlsRegister.CATEGORY`: it uses Minecraft's `Category` (xaero/map/controls/ControlsRegister.java:12)
- [ ] `ControlsRegister.keyOpenMap`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:13)
- [ ] `ControlsRegister.keyOpenSettings`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:14)
- [ ] `ControlsRegister.keyOpenServerSettings`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:15)
- [ ] `ControlsRegister.keyZoomIn`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:16)
- [ ] `ControlsRegister.keyZoomOut`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:17)
- [ ] `ControlsRegister.keyQuickConfirm`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:18)
- [ ] `ControlsRegister.keyToggleDimension`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:19)
- [ ] `ControlsRegister.keyToggleTrackedPlayers`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:20)
- [ ] `ControlsRegister.keyTogglePacChunkClaims`: it uses Minecraft's `KeyMapping` (xaero/map/controls/ControlsRegister.java:21)
- [ ] `ControlsRegister.register(2)`: its class isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/controls/ControlsRegister.java:26)

### `KeyEvent`: it uses Minecraft's `KeyMapping`

- [ ] `KeyEvent.KeyEvent(4)`: its class isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/controls/KeyEvent.java:11)
- [ ] `KeyEvent.getKb(0)`: its class isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/controls/KeyEvent.java:18)
- [ ] `KeyEvent.isTickEnd(0)`: its class isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/controls/KeyEvent.java:22)
- [ ] `KeyEvent.isRepeat(0)`: its class isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/controls/KeyEvent.java:26)
- [ ] `KeyEvent.isKeyDown(0)`: its class isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/controls/KeyEvent.java:30)

### `IGuiRenderer`: it's an interface

- [ ] `IGuiRenderer.xaero_wm_getGuiProjection(0)`: it has no body (xaero/map/core/IGuiRenderer.java:7)
- [ ] `IGuiRenderer.xaero_wm_getGuiProjectionMatrixBuffer(0)`: it has no body (xaero/map/core/IGuiRenderer.java:9)

### `IWorldMapClientPlayNetHandler`: it's an interface

- [ ] `IWorldMapClientPlayNetHandler.getXaero_worldmapSession(0)`: it has no body (xaero/map/core/IWorldMapClientPlayNetHandler.java:6)
- [ ] `IWorldMapClientPlayNetHandler.setXaero_worldmapSession(1)`: it has no body (xaero/map/core/IWorldMapClientPlayNetHandler.java:8)
- [ ] `IWorldMapClientPlayNetHandler.getXaero_serverChunkRadius(0)`: it has no body (xaero/map/core/IWorldMapClientPlayNetHandler.java:10)

### `IWorldMapGlBuffer`: it's an interface

- [ ] `IWorldMapGlBuffer.xaero_wm_getHandle(0)`: it has no body (xaero/map/core/IWorldMapGlBuffer.java:4)

### `IWorldMapMinecraftClient`: it's an interface

- [ ] `IWorldMapMinecraftClient.getXaeroWorldMap_fps(0)`: it has no body (xaero/map/core/IWorldMapMinecraftClient.java:4)

### `IWorldMapServerLevel`: it's an interface

- [ ] `IWorldMapServerLevel.getXaero_wm_capabilities(0)`: it has no body (xaero/map/core/IWorldMapServerLevel.java:6)
- [ ] `IWorldMapServerLevel.setXaero_wm_capabilities(1)`: it has no body (xaero/map/core/IWorldMapServerLevel.java:8)

### `IWorldMapSMultiBlockChangePacket`: it's an interface

- [ ] `IWorldMapSMultiBlockChangePacket.xaero_wm_getSectionPos(0)`: it has no body (xaero/map/core/IWorldMapSMultiBlockChangePacket.java:6)

### `XaeroWorldMapCore`

- [ ] `XaeroWorldMapCore.chunkCleanField`: it uses `Field` (java.lang.reflect) (xaero/map/core/XaeroWorldMapCore.java:29)
- [ ] `XaeroWorldMapCore.currentSession`: it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/core/XaeroWorldMapCore.java:30)
- [ ] `XaeroWorldMapCore.ensureField(0)`: it uses XaeroWorldMapCore.chunkCleanField, which isn't translated (it uses `Field` (java.lang.reflect)) (xaero/map/core/XaeroWorldMapCore.java:32)
- [ ] `XaeroWorldMapCore.chunkUpdateCallback(2)`: it calls XaeroWorldMapCore.ensureField, which isn't translated (xaero/map/core/XaeroWorldMapCore.java:42)
- [ ] `XaeroWorldMapCore.onHandleLevelChunkWithLight(1)`: it uses Minecraft's `ClientboundLevelChunkWithLightPacket` (xaero/map/core/XaeroWorldMapCore.java:61)
- [ ] `XaeroWorldMapCore.onHandleLightUpdatePacket(1)`: it uses Minecraft's `ClientboundLightUpdatePacket` (xaero/map/core/XaeroWorldMapCore.java:65)
- [ ] `XaeroWorldMapCore.onQueueLightRemoval(1)`: it uses Minecraft's `ClientboundForgetLevelChunkPacket` (xaero/map/core/XaeroWorldMapCore.java:69)
- [ ] `XaeroWorldMapCore.onBlockChange(1)`: it uses Minecraft's `ClientboundBlockUpdatePacket` (xaero/map/core/XaeroWorldMapCore.java:73)
- [ ] `XaeroWorldMapCore.onMultiBlockChange(1)`: it uses Minecraft's `ClientboundSectionBlocksUpdatePacket` (xaero/map/core/XaeroWorldMapCore.java:77)
- [ ] `XaeroWorldMapCore.onPlayNetHandler(2)`: it uses Minecraft's `ClientPacketListener` (xaero/map/core/XaeroWorldMapCore.java:82)
- [ ] `XaeroWorldMapCore.cleanupCurrentSession(0)`: a try/finally (xaero/map/core/XaeroWorldMapCore.java:111)
- [ ] `XaeroWorldMapCore.onPlayNetHandlerCleanup(1)`: it uses Minecraft's `ClientPacketListener` (xaero/map/core/XaeroWorldMapCore.java:121)
- [ ] `XaeroWorldMapCore.onDeleteWorld(1)`: it uses Minecraft's `LevelStorageAccess` (xaero/map/core/XaeroWorldMapCore.java:145)
- [x] `XaeroWorldMapCore.onMinecraftRunTick(0)` (xaero/map/core/XaeroWorldMapCore.java:166)
- [ ] `XaeroWorldMapCore.onSystemChat(1)`: it uses WorldMap.events, which isn't translated (it uses ClientEvents, which isn't translated (it uses Minecraft's `RealmsServer`)) (xaero/map/core/XaeroWorldMapCore.java:172)
- [ ] `XaeroWorldMapCore.onHandleDisguisedChatMessage(2)`: it uses Minecraft's `Bound` (xaero/map/core/XaeroWorldMapCore.java:176)
- [ ] `XaeroWorldMapCore.onRenderCall(1)`: a nullable object kept in a changing static (xaero/map/core/XaeroWorldMapCore.java:180)
- [ ] `XaeroWorldMapCore.handlePlayerSetSpawnPacket(1)`: it uses Minecraft's `ClientboundSetDefaultSpawnPositionPacket` (xaero/map/core/XaeroWorldMapCore.java:188)
- [ ] `XaeroWorldMapCore.onRenderCrosshair(1)`: it uses WorldMap.events, which isn't translated (it uses ClientEvents, which isn't translated (it uses Minecraft's `RealmsServer`)) (xaero/map/core/XaeroWorldMapCore.java:194)

### `XaeroWorldMapCoreFabric`

- [ ] `XaeroWorldMapCoreFabric.onPlayNetHandler(2)`: it uses Minecraft's `ClientPacketListener` (xaero/map/core/XaeroWorldMapCoreFabric.java:9)
- [ ] `XaeroWorldMapCoreFabric.onMinecraftRunTick(0)`: it uses WorldMap.INSTANCE, which isn't translated (it uses WorldMap, which isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel))) (xaero/map/core/XaeroWorldMapCoreFabric.java:21)

### `CrashHandler`: it uses `Throwable`

- [ ] `CrashHandler.checkForCrashes(0)`: its class isn't translated (it uses `Throwable`) (xaero/map/CrashHandler.java:6)
- [ ] `CrashHandler.getCrashedBy(0)`: its class isn't translated (it uses `Throwable`) (xaero/map/CrashHandler.java:14)
- [ ] `CrashHandler.setCrashedBy(1)`: its class isn't translated (it uses `Throwable`) (xaero/map/CrashHandler.java:18)

### `ByteBufferDeallocator`: it uses `Object`

- [ ] `ByteBufferDeallocator.ByteBufferDeallocator(0)`: its class isn't translated (it uses `Object`) (xaero/map/deallocator/ByteBufferDeallocator.java:17)
- [ ] `ByteBufferDeallocator.deallocate(2)`: its class isn't translated (it uses `Object`) (xaero/map/deallocator/ByteBufferDeallocator.java:38)
- [ ] `ByteBufferDeallocator.reportException(1)`: its class isn't translated (it uses `Object`) (xaero/map/deallocator/ByteBufferDeallocator.java:77)

### `Effects`

- [ ] `Effects.NO_WORLD_MAP_UNHELD`: it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/Effects.java:8)
- [ ] `Effects.NO_WORLD_MAP_HARMFUL_UNHELD`: it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/Effects.java:9)
- [ ] `Effects.NO_CAVE_MAPS_UNHELD`: it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/Effects.java:10)
- [ ] `Effects.NO_CAVE_MAPS_HARMFUL_UNHELD`: it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/Effects.java:11)
- [ ] `Effects.NO_WORLD_MAP`: it uses Minecraft's `Holder` (xaero/map/effects/Effects.java:12)
- [ ] `Effects.NO_WORLD_MAP_HARMFUL`: it uses Minecraft's `Holder` (xaero/map/effects/Effects.java:13)
- [ ] `Effects.NO_CAVE_MAPS`: it uses Minecraft's `Holder` (xaero/map/effects/Effects.java:14)
- [ ] `Effects.NO_CAVE_MAPS_HARMFUL`: it uses Minecraft's `Holder` (xaero/map/effects/Effects.java:15)
- [ ] `Effects.init(0)`: it uses Effects.NO_WORLD_MAP, which isn't translated (it uses Minecraft's `Holder`) (xaero/map/effects/Effects.java:17)

### `EffectsRegister`

- [ ] `EffectsRegister.registerEffects(1)`: it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/EffectsRegister.java:8)

### `NoCaveMapsEffect`: it extends `WorldMapStatusEffect`

- [ ] `NoCaveMapsEffect.NoCaveMapsEffect(1)`: its class isn't translated (it extends `WorldMapStatusEffect`) (xaero/map/effects/NoCaveMapsEffect.java:6)

### `NoWorldMapEffect`: it extends `WorldMapStatusEffect`

- [ ] `NoWorldMapEffect.NoWorldMapEffect(1)`: its class isn't translated (it extends `WorldMapStatusEffect`) (xaero/map/effects/NoWorldMapEffect.java:6)

### `WorldMapStatusEffect`: it extends Minecraft's `MobEffect`

- [ ] `WorldMapStatusEffect.WorldMapStatusEffect(3)`: its class isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/WorldMapStatusEffect.java:10)
- [ ] `WorldMapStatusEffect.setRegistryName(1)`: its class isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/WorldMapStatusEffect.java:16)
- [ ] `WorldMapStatusEffect.getRegistryName(0)`: its class isn't translated (it extends Minecraft's `MobEffect`) (xaero/map/effects/WorldMapStatusEffect.java:20)

### `HoveredMapElementHolder`: it's generic

- [ ] `HoveredMapElementHolder.HoveredMapElementHolder(2)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:16)
- [ ] `HoveredMapElementHolder.getRightClickOptions(0)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:21)
- [ ] `HoveredMapElementHolder.isRightClickValid(0)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:40)
- [ ] `HoveredMapElementHolder.getRightClickTitleBackgroundColor(0)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:45)
- [ ] `HoveredMapElementHolder.getElement(0)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:50)
- [ ] `HoveredMapElementHolder.getRenderer(0)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:54)
- [ ] `HoveredMapElementHolder.is(1)`: its class isn't translated (it's generic) (xaero/map/element/HoveredMapElementHolder.java:58)

### `MapElementGraphics`: it uses `XaeroBufferProvider` (xaero.lib.client.graphics)

- [ ] `MapElementGraphics.MapElementGraphics(1)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:29)
- [ ] `MapElementGraphics.fill(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:33)
- [ ] `MapElementGraphics.fill(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:37)
- [ ] `MapElementGraphics.fillGradient(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:54)
- [ ] `MapElementGraphics.blit(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:59)
- [ ] `MapElementGraphics.blit(9)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:80)
- [ ] `MapElementGraphics.blit(11)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:84)
- [ ] `MapElementGraphics.blit(11)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:107)
- [ ] `MapElementGraphics.drawCenteredString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:115)
- [ ] `MapElementGraphics.drawCenteredString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:119)
- [ ] `MapElementGraphics.drawCenteredString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:123)
- [ ] `MapElementGraphics.drawString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:127)
- [ ] `MapElementGraphics.drawString(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:131)
- [ ] `MapElementGraphics.drawString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:139)
- [ ] `MapElementGraphics.drawString(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:143)
- [ ] `MapElementGraphics.drawString(5)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:151)
- [ ] `MapElementGraphics.drawString(6)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:155)
- [ ] `MapElementGraphics.getBufferSource(0)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:163)
- [ ] `MapElementGraphics.pose(0)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:167)
- [ ] `MapElementGraphics.flush(0)`: its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/element/MapElementGraphics.java:171)

### `MapElementMenuHitbox`

- [x] `MapElementMenuHitbox.MapElementMenuHitbox(4)` (xaero/map/element/MapElementMenuHitbox.java:9)
- [x] `MapElementMenuHitbox.getX(0)` (xaero/map/element/MapElementMenuHitbox.java:16)
- [x] `MapElementMenuHitbox.getY(0)` (xaero/map/element/MapElementMenuHitbox.java:20)
- [x] `MapElementMenuHitbox.getW(0)` (xaero/map/element/MapElementMenuHitbox.java:24)
- [x] `MapElementMenuHitbox.getH(0)` (xaero/map/element/MapElementMenuHitbox.java:28)
- [x] `MapElementMenuHitbox.setY(1)` (xaero/map/element/MapElementMenuHitbox.java:32)
- [x] `MapElementMenuHitbox.setH(1)` (xaero/map/element/MapElementMenuHitbox.java:36)

### `MapElementMenuRenderer`: it's generic

- [x] `MapElementMenuRenderer.MENU_RIGHT_PADDING` (xaero/map/element/MapElementMenuRenderer.java:24)
- [x] `MapElementMenuRenderer.MAX_MENU_SIZE` (xaero/map/element/MapElementMenuRenderer.java:25)
- [x] `MapElementMenuRenderer.MIN_MENU_SIZE` (xaero/map/element/MapElementMenuRenderer.java:26)
- [ ] `MapElementMenuRenderer.MapElementMenuRenderer(2)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:41)
- [ ] `MapElementMenuRenderer.onMapInit(4)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:52)
- [ ] `MapElementMenuRenderer.renderMenu(11)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:62)
- [ ] `MapElementMenuRenderer.beforeMenuRender(0)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:138)
- [ ] `MapElementMenuRenderer.afterMenuRender(0)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:140)
- [ ] `MapElementMenuRenderer.postMapRender(7)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:142)
- [x] `MapElementMenuRenderer.getMenuElementCount(1)` (xaero/map/element/MapElementMenuRenderer.java:161)
- [ ] `MapElementMenuRenderer.renderMenuElement(11)`: it's generic (xaero/map/element/MapElementMenuRenderer.java:165)
- [ ] `MapElementMenuRenderer.renderMenuElement(13)`: it's generic (xaero/map/element/MapElementMenuRenderer.java:194)
- [ ] `MapElementMenuRenderer.onMapMouseRelease(1)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:234)
- [ ] `MapElementMenuRenderer.releaseScroll(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:238)
- [ ] `MapElementMenuRenderer.renderScroll(10)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:243)
- [ ] `MapElementMenuRenderer.updateSearch(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:265)
- [ ] `MapElementMenuRenderer.charTyped(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:286)
- [ ] `MapElementMenuRenderer.keyPressed(2)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:290)
- [ ] `MapElementMenuRenderer.mouseScrolled(1)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:302)
- [ ] `MapElementMenuRenderer.tick(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:306)
- [ ] `MapElementMenuRenderer.unfocusAll(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:309)
- [ ] `MapElementMenuRenderer.onMenuClosed(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:315)
- [ ] `MapElementMenuRenderer.scroll(1)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:323)
- [ ] `MapElementMenuRenderer.getSearchPattern(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:327)
- [ ] `MapElementMenuRenderer.getSearchStartPattern(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:331)
- [ ] `MapElementMenuRenderer.updateFilteredList(0)`: its class isn't translated (it's generic) (xaero/map/element/MapElementMenuRenderer.java:335)
- [ ] `MapElementMenuRenderer.getAnyRenderer(1)`: it's generic (xaero/map/element/MapElementMenuRenderer.java:371)
- [ ] `MapElementMenuRenderer.getRenderer(1)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:375)
- [ ] `MapElementMenuRenderer.menuStartPos(1)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:377)
- [ ] `MapElementMenuRenderer.menuSearchPadding(0)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:379)
- [ ] `MapElementMenuRenderer.renderInMenu(11)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:381)
- [ ] `MapElementMenuRenderer.getFilterPlaceholder(0)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:385)
- [ ] `MapElementMenuRenderer.beforeFiltering(0)`: it has no body (xaero/map/element/MapElementMenuRenderer.java:387)

### `MapElementMenuScroll`

- [x] `MapElementMenuScroll.MapElementMenuScroll(3)` (xaero/map/element/MapElementMenuScroll.java:9)
- [x] `MapElementMenuScroll.getDirection(0)` (xaero/map/element/MapElementMenuScroll.java:15)
- [x] `MapElementMenuScroll.scroll(0)` (xaero/map/element/MapElementMenuScroll.java:19)
- [x] `MapElementMenuScroll.onMouseRelease(0)` (xaero/map/element/MapElementMenuScroll.java:29)
- [x] `MapElementMenuScroll.getIcon(0)` (xaero/map/element/MapElementMenuScroll.java:33)
- [x] `MapElementMenuScroll.getName(0)` (xaero/map/element/MapElementMenuScroll.java:37)

### `MapElementRenderHandler`: it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))

- [ ] `MapElementRenderHandler.MapElementRenderHandler(3)`: its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))) (xaero/map/element/MapElementRenderHandler.java:36)
- [ ] `MapElementRenderHandler.add(1)`: its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))) (xaero/map/element/MapElementRenderHandler.java:42)
- [ ] `MapElementRenderHandler.createResult(2)`: it's generic (xaero/map/element/MapElementRenderHandler.java:46)
- [ ] `MapElementRenderHandler.getRenderer(1)`: it's generic (xaero/map/element/MapElementRenderHandler.java:51)
- [ ] `MapElementRenderHandler.render(17)`: its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))) (xaero/map/element/MapElementRenderHandler.java:55)
- [ ] `MapElementRenderHandler.renderHoveredWithRenderer(9)`: it's generic (xaero/map/element/MapElementRenderHandler.java:143)
- [ ] `MapElementRenderHandler.renderWithRenderer(11)`: it's generic (xaero/map/element/MapElementRenderHandler.java:184)
- [ ] `MapElementRenderHandler.transformAndRenderElement(11)`: it's generic (xaero/map/element/MapElementRenderHandler.java:238)
- [ ] `MapElementRenderHandler.getElementIndexDepth(2)`: its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))) (xaero/map/element/MapElementRenderHandler.java:310)

### `MapElementRenderHandler.Builder`

- [x] `MapElementRenderHandler.Builder.Builder(0)` (xaero/map/element/MapElementRenderHandler.java:317)
- [ ] `MapElementRenderHandler.Builder.setDefault(0)`: `null` (Rust has no null: this needs an Option) (xaero/map/element/MapElementRenderHandler.java:320)
- [ ] `MapElementRenderHandler.Builder.setPoseStack(1)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/map/element/MapElementRenderHandler.java:325)
- [ ] `MapElementRenderHandler.Builder.build(0)`: it uses MapElementRenderHandler, which isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))) (xaero/map/element/MapElementRenderHandler.java:330)
- [x] `MapElementRenderHandler.Builder.begin(0)` (xaero/map/element/MapElementRenderHandler.java:349)

### `MapElementRenderLocation`

- [x] `MapElementRenderLocation.UNKNOWN` (xaero/map/element/MapElementRenderLocation.java:4)
- [x] `MapElementRenderLocation.IN_MINIMAP` (xaero/map/element/MapElementRenderLocation.java:5)
- [x] `MapElementRenderLocation.OVER_MINIMAP` (xaero/map/element/MapElementRenderLocation.java:6)
- [x] `MapElementRenderLocation.IN_GAME` (xaero/map/element/MapElementRenderLocation.java:7)
- [x] `MapElementRenderLocation.WORLD_MAP` (xaero/map/element/MapElementRenderLocation.java:8)
- [x] `MapElementRenderLocation.WORLD_MAP_MENU` (xaero/map/element/MapElementRenderLocation.java:9)
- [x] `MapElementRenderLocation.fromMinimap(1)` (xaero/map/element/MapElementRenderLocation.java:11)

### `MenuHitboxReader`: it extends `MenuOnlyElementReader`

- [ ] `MenuHitboxReader.getLeftSideLength(2)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:6)
- [ ] `MenuHitboxReader.isMouseOverMenuElement(6)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:10)
- [ ] `MenuHitboxReader.getMenuName(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:18)
- [ ] `MenuHitboxReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:22)
- [ ] `MenuHitboxReader.getFilterName(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:26)
- [ ] `MenuHitboxReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:30)
- [ ] `MenuHitboxReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuHitboxReader.java:34)

### `MenuOnlyElementReader`: it's generic

- [ ] `MenuOnlyElementReader.isHidden(2)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:6)
- [ ] `MenuOnlyElementReader.getRenderX(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:11)
- [ ] `MenuOnlyElementReader.getRenderZ(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:16)
- [ ] `MenuOnlyElementReader.getInteractionBoxLeft(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:21)
- [ ] `MenuOnlyElementReader.getInteractionBoxRight(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:26)
- [ ] `MenuOnlyElementReader.getInteractionBoxTop(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:31)
- [ ] `MenuOnlyElementReader.getInteractionBoxBottom(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:36)
- [ ] `MenuOnlyElementReader.getRenderBoxLeft(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:41)
- [ ] `MenuOnlyElementReader.getRenderBoxRight(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:46)
- [ ] `MenuOnlyElementReader.getRenderBoxTop(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:51)
- [ ] `MenuOnlyElementReader.getRenderBoxBottom(3)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementReader.java:56)

### `MenuOnlyElementRenderer`: it's generic

- [ ] `MenuOnlyElementRenderer.MenuOnlyElementRenderer(1)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementRenderer.java:10)
- [ ] `MenuOnlyElementRenderer.shouldRender(2)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementRenderer.java:14)
- [ ] `MenuOnlyElementRenderer.preRender(4)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementRenderer.java:19)
- [ ] `MenuOnlyElementRenderer.postRender(4)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementRenderer.java:25)
- [ ] `MenuOnlyElementRenderer.renderElementShadow(9)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementRenderer.java:31)
- [ ] `MenuOnlyElementRenderer.renderElement(10)`: its class isn't translated (it's generic) (xaero/map/element/MenuOnlyElementRenderer.java:45)

### `MenuScrollReader`: it extends `MenuOnlyElementReader`

- [ ] `MenuScrollReader.getLeftSideLength(2)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuScrollReader.java:7)
- [ ] `MenuScrollReader.getMenuName(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuScrollReader.java:11)
- [ ] `MenuScrollReader.getFilterName(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuScrollReader.java:15)
- [ ] `MenuScrollReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuScrollReader.java:19)
- [ ] `MenuScrollReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuScrollReader.java:23)
- [ ] `MenuScrollReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `MenuOnlyElementReader`) (xaero/map/element/MenuScrollReader.java:27)

### `ElementReader`: it's generic

- [ ] `ElementReader.isHidden(2)`: it has no body (xaero/map/element/render/ElementReader.java:10)
- [ ] `ElementReader.getRenderX(3)`: it has no body (xaero/map/element/render/ElementReader.java:12)
- [ ] `ElementReader.getRenderZ(3)`: it has no body (xaero/map/element/render/ElementReader.java:14)
- [ ] `ElementReader.getInteractionBoxLeft(3)`: it has no body (xaero/map/element/render/ElementReader.java:16)
- [ ] `ElementReader.getInteractionBoxRight(3)`: it has no body (xaero/map/element/render/ElementReader.java:18)
- [ ] `ElementReader.getInteractionBoxTop(3)`: it has no body (xaero/map/element/render/ElementReader.java:20)
- [ ] `ElementReader.getInteractionBoxBottom(3)`: it has no body (xaero/map/element/render/ElementReader.java:22)
- [ ] `ElementReader.getRenderBoxLeft(3)`: it has no body (xaero/map/element/render/ElementReader.java:24)
- [ ] `ElementReader.getRenderBoxRight(3)`: it has no body (xaero/map/element/render/ElementReader.java:26)
- [ ] `ElementReader.getRenderBoxTop(3)`: it has no body (xaero/map/element/render/ElementReader.java:28)
- [ ] `ElementReader.getRenderBoxBottom(3)`: it has no body (xaero/map/element/render/ElementReader.java:30)
- [ ] `ElementReader.getLeftSideLength(2)`: it has no body (xaero/map/element/render/ElementReader.java:32)
- [ ] `ElementReader.getMenuName(1)`: it has no body (xaero/map/element/render/ElementReader.java:34)
- [ ] `ElementReader.getFilterName(1)`: it has no body (xaero/map/element/render/ElementReader.java:36)
- [ ] `ElementReader.getMenuTextFillLeftPadding(1)`: it has no body (xaero/map/element/render/ElementReader.java:38)
- [ ] `ElementReader.getRightClickTitleBackgroundColor(1)`: it has no body (xaero/map/element/render/ElementReader.java:40)
- [ ] `ElementReader.shouldScaleBoxWithOptionalScale(0)`: it has no body (xaero/map/element/render/ElementReader.java:42)
- [ ] `ElementReader.isInteractable(2)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:44)
- [ ] `ElementReader.getBoxScale(3)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:48)
- [ ] `ElementReader.isMouseOverMenuElement(6)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:52)
- [ ] `ElementReader.isHoveredOnMap(9)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:72)
- [ ] `ElementReader.isOnScreen(10)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:101)
- [ ] `ElementReader.getRightClickOptions(2)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:139)
- [ ] `ElementReader.isRightClickValid(1)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:143)
- [ ] `ElementReader.getTooltip(3)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:147)
- [ ] `ElementReader.getRenderY(3)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:151)
- [ ] `ElementReader.hasYCoordinate(0)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementReader.java:155)

### `ElementRenderer`: it's generic

- [ ] `ElementRenderer.ElementRenderer(3)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:13)
- [ ] `ElementRenderer.shouldRenderHovered(1)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:19)
- [ ] `ElementRenderer.getReader(0)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:23)
- [ ] `ElementRenderer.getContext(0)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:27)
- [ ] `ElementRenderer.getProvider(0)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:31)
- [ ] `ElementRenderer.getOrder(0)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:35)
- [ ] `ElementRenderer.shouldBeDimScaled(0)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:39)
- [ ] `ElementRenderer.compareTo(1)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderer.java:43)
- [ ] `ElementRenderer.preRender(4)`: it has no body (xaero/map/element/render/ElementRenderer.java:47)
- [ ] `ElementRenderer.postRender(4)`: it has no body (xaero/map/element/render/ElementRenderer.java:49)
- [ ] `ElementRenderer.renderElementShadow(9)`: it has no body (xaero/map/element/render/ElementRenderer.java:51)
- [ ] `ElementRenderer.renderElement(10)`: it has no body (xaero/map/element/render/ElementRenderer.java:63)
- [ ] `ElementRenderer.shouldRender(2)`: it has no body (xaero/map/element/render/ElementRenderer.java:76)

### `ElementRenderInfo`: it uses Minecraft's `Entity`

- [ ] `ElementRenderInfo.ElementRenderInfo(14)`: its class isn't translated (it uses Minecraft's `Entity`) (xaero/map/element/render/ElementRenderInfo.java:31)

### `ElementRenderLocation`

- [ ] `ElementRenderLocation.ALL`: it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints) (xaero/map/element/render/ElementRenderLocation.java:7)
- [ ] `ElementRenderLocation.UNKNOWN`: it creates a ElementRenderLocation with a constructor that isn't translated (xaero/map/element/render/ElementRenderLocation.java:8)
- [ ] `ElementRenderLocation.IN_MINIMAP`: it creates a ElementRenderLocation with a constructor that isn't translated (xaero/map/element/render/ElementRenderLocation.java:9)
- [ ] `ElementRenderLocation.OVER_MINIMAP`: it creates a ElementRenderLocation with a constructor that isn't translated (xaero/map/element/render/ElementRenderLocation.java:10)
- [ ] `ElementRenderLocation.IN_WORLD`: it creates a ElementRenderLocation with a constructor that isn't translated (xaero/map/element/render/ElementRenderLocation.java:11)
- [ ] `ElementRenderLocation.WORLD_MAP`: it creates a ElementRenderLocation with a constructor that isn't translated (xaero/map/element/render/ElementRenderLocation.java:12)
- [ ] `ElementRenderLocation.WORLD_MAP_MENU`: it creates a ElementRenderLocation with a constructor that isn't translated (xaero/map/element/render/ElementRenderLocation.java:13)
- [ ] `ElementRenderLocation.ElementRenderLocation(1)`: it uses ElementRenderLocation.ALL, which isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/element/render/ElementRenderLocation.java:16)
- [x] `ElementRenderLocation.getIndex(0)` (xaero/map/element/render/ElementRenderLocation.java:21)
- [ ] `ElementRenderLocation.fromIndex(1)`: it uses ElementRenderLocation.ALL, which isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/element/render/ElementRenderLocation.java:25)

### `ElementRenderProvider`: it's generic

- [ ] `ElementRenderProvider.begin(2)`: it has no body (xaero/map/element/render/ElementRenderProvider.java:4)
- [ ] `ElementRenderProvider.hasNext(2)`: it has no body (xaero/map/element/render/ElementRenderProvider.java:6)
- [ ] `ElementRenderProvider.getNext(2)`: it has no body (xaero/map/element/render/ElementRenderProvider.java:8)
- [ ] `ElementRenderProvider.setupContextAndGetNext(2)`: its class isn't translated (it's generic) (xaero/map/element/render/ElementRenderProvider.java:10)
- [ ] `ElementRenderProvider.end(2)`: it has no body (xaero/map/element/render/ElementRenderProvider.java:14)

### `EntityUtil`

- [ ] `EntityUtil.getEntityX(2)`: it uses Minecraft's `Entity` (xaero/map/entity/util/EntityUtil.java:7)
- [ ] `EntityUtil.getEntityY(2)`: it uses Minecraft's `Entity` (xaero/map/entity/util/EntityUtil.java:12)
- [ ] `EntityUtil.getEntityZ(2)`: it uses Minecraft's `Entity` (xaero/map/entity/util/EntityUtil.java:17)
- [ ] `EntityUtil.getEntityPos(2)`: it uses Minecraft's `Entity` (xaero/map/entity/util/EntityUtil.java:22)

### `ClientEvents`: it uses Minecraft's `RealmsServer`

- [ ] `ClientEvents.handleGuiOpen(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:38)
- [ ] `ClientEvents.onMinecraftRunTickStart(0)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:71)
- [ ] `ClientEvents.handleRenderTick(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:79)
- [ ] `ClientEvents.handleDrawScreen(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:116)
- [ ] `ClientEvents.handlePlayerSetSpawnEvent(2)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:122)
- [ ] `ClientEvents.handleWorldUnload(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:130)
- [ ] `ClientEvents.getLatestRealm(0)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:146)
- [ ] `ClientEvents.handleRenderCrosshairOverlay(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:150)
- [ ] `ClientEvents.handleClientPlayerChatReceivedEvent(3)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:169)
- [ ] `ClientEvents.handleClientSystemChatReceivedEvent(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:173)
- [ ] `ClientEvents.handleChatMessage(2)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:198)
- [ ] `ClientEvents.handlePlayerTickStart(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:202)
- [ ] `ClientEvents.handleClientTickStart(0)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:211)
- [ ] `ClientEvents.handleClientRunTickStart(0)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:229)
- [ ] `ClientEvents.handleTextureStitchEventPost(1)`: its class isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/events/ClientEvents.java:243)

### `ClientEventsFabric`: it extends `ClientEvents`


### `CommonEvents`

- [ ] `CommonEvents.onPlayerClone(3)`: it uses Minecraft's `Player` (xaero/map/events/CommonEvents.java:26)
- [ ] `CommonEvents.onServerStarting(1)`: it uses Minecraft's `MinecraftServer` (xaero/map/events/CommonEvents.java:32)
- [ ] `CommonEvents.onServerStopped(1)`: it uses Minecraft's `MinecraftServer` (xaero/map/events/CommonEvents.java:36)
- [ ] `CommonEvents.onPlayerLogIn(1)`: it uses Minecraft's `Player` (xaero/map/events/CommonEvents.java:39)
- [ ] `CommonEvents.onPlayerWorldJoin(1)`: it uses Minecraft's `ServerPlayer` (xaero/map/events/CommonEvents.java:45)
- [ ] `CommonEvents.handlePlayerTickStart(1)`: it uses Minecraft's `Player` (xaero/map/events/CommonEvents.java:70)

### `CommonEventsFabric`: it extends `CommonEvents`

- [ ] `CommonEventsFabric.register(0)`: its class isn't translated (it extends `CommonEvents`) (xaero/map/events/CommonEventsFabric.java:7)

### `ModClientEvents`


### `ModClientEventsFabric`: it extends `ModClientEvents`


### `ModCommonEvents`


### `ModCommonEventsFabric`: it extends `ModCommonEvents`


### `Executor`: it extends Minecraft's `BlockableEventLoop`

- [ ] `Executor.Executor(3)`: its class isn't translated (it extends Minecraft's `BlockableEventLoop`) (xaero/map/executor/Executor.java:8)
- [ ] `Executor.wrapRunnable(1)`: its class isn't translated (it extends Minecraft's `BlockableEventLoop`) (xaero/map/executor/Executor.java:13)
- [ ] `Executor.shouldRun(1)`: its class isn't translated (it extends Minecraft's `BlockableEventLoop`) (xaero/map/executor/Executor.java:17)
- [ ] `Executor.getRunningThread(0)`: its class isn't translated (it extends Minecraft's `BlockableEventLoop`) (xaero/map/executor/Executor.java:21)
- [ ] `Executor.runAllTasks(0)`: its class isn't translated (it extends Minecraft's `BlockableEventLoop`) (xaero/map/executor/Executor.java:25)

### `PNGExporter`: it uses `Calendar` (java.util)

- [ ] `PNGExporter.PNGExporter(1)`: its class isn't translated (it uses `Calendar` (java.util)) (xaero/map/file/export/PNGExporter.java:78)
- [ ] `PNGExporter.export(5)`: its class isn't translated (it uses `Calendar` (java.util)) (xaero/map/file/export/PNGExporter.java:84)
- [ ] `PNGExporter.saveImage(4)`: its class isn't translated (it uses `Calendar` (java.util)) (xaero/map/file/export/PNGExporter.java:483)
- [ ] `PNGExporter.getExportBaseName(0)`: its class isn't translated (it uses `Calendar` (java.util)) (xaero/map/file/export/PNGExporter.java:510)

### `PNGExportResult`: it uses PNGExportResultType, which isn't translated (its constants carry values)

- [ ] `PNGExportResult.PNGExportResult(2)`: its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)) (xaero/map/file/export/PNGExportResult.java:10)
- [ ] `PNGExportResult.getType(0)`: its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)) (xaero/map/file/export/PNGExportResult.java:15)
- [ ] `PNGExportResult.getFolderToOpen(0)`: its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)) (xaero/map/file/export/PNGExportResult.java:19)
- [ ] `PNGExportResult.getMessage(0)`: its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)) (xaero/map/file/export/PNGExportResult.java:23)

### `PNGExportResultType`: its constants carry values

- [ ] `PNGExportResultType.PNGExportResultType(2)`: its class isn't translated (its constants carry values) (xaero/map/file/export/PNGExportResultType.java:19)
- [ ] `PNGExportResultType.getMessage(0)`: its class isn't translated (its constants carry values) (xaero/map/file/export/PNGExportResultType.java:24)
- [ ] `PNGExportResultType.isSuccess(0)`: its class isn't translated (its constants carry values) (xaero/map/file/export/PNGExportResultType.java:28)

### `IOHelper`

- [ ] `IOHelper.readToBuffer(3)`: it uses `DataInputStream` (java.io) (xaero/map/file/IOHelper.java:8)

### `MapRegionInfo`: it's an interface

- [ ] `MapRegionInfo.shouldCache(0)`: it has no body (xaero/map/file/MapRegionInfo.java:6)
- [ ] `MapRegionInfo.getRegionFile(0)`: it has no body (xaero/map/file/MapRegionInfo.java:8)
- [ ] `MapRegionInfo.getCacheFile(0)`: it has no body (xaero/map/file/MapRegionInfo.java:10)
- [ ] `MapRegionInfo.getWorldId(0)`: it has no body (xaero/map/file/MapRegionInfo.java:12)
- [ ] `MapRegionInfo.getDimId(0)`: it has no body (xaero/map/file/MapRegionInfo.java:14)
- [ ] `MapRegionInfo.getMwId(0)`: it has no body (xaero/map/file/MapRegionInfo.java:16)
- [ ] `MapRegionInfo.getRegionX(0)`: it has no body (xaero/map/file/MapRegionInfo.java:18)
- [ ] `MapRegionInfo.getRegionZ(0)`: it has no body (xaero/map/file/MapRegionInfo.java:20)
- [ ] `MapRegionInfo.setShouldCache(2)`: it has no body (xaero/map/file/MapRegionInfo.java:22)
- [ ] `MapRegionInfo.setCacheFile(1)`: it has no body (xaero/map/file/MapRegionInfo.java:24)
- [ ] `MapRegionInfo.hasLookedForCache(0)`: it has no body (xaero/map/file/MapRegionInfo.java:26)

### `MapSaveLoad`: it uses MapRegion, which isn't translated (it extends `LeveledRegion`)

- [x] `MapSaveLoad.currentSaveMajorVersion` (xaero/map/file/MapSaveLoad.java:67)
- [x] `MapSaveLoad.currentSaveMinorVersion` (xaero/map/file/MapSaveLoad.java:68)
- [x] `MapSaveLoad.SAVE_TIME` (xaero/map/file/MapSaveLoad.java:69)
- [x] `MapSaveLoad.currentCacheSaveMajorVersion` (xaero/map/file/MapSaveLoad.java:70)
- [x] `MapSaveLoad.currentCacheSaveMinorVersion` (xaero/map/file/MapSaveLoad.java:71)
- [ ] `MapSaveLoad.MapSaveLoad(4)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:94)
- [ ] `MapSaveLoad.setMapProcessor(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:108)
- [ ] `MapSaveLoad.exportPNG(2)`: its body couldn't be read: an anonymous class (xaero/map/file/MapSaveLoad.java:112)
- [ ] `MapSaveLoad.getSecondaryFile(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:165)
- [ ] `MapSaveLoad.getTempFile(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:178)
- [ ] `MapSaveLoad.getCacheFolder(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:182)
- [ ] `MapSaveLoad.getCacheFile(4)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:186)
- [ ] `MapSaveLoad.getFile(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:211)
- [ ] `MapSaveLoad.getNormalFile(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:232)
- [ ] `MapSaveLoad.getRootFolder(1)`: it uses `File` (java.io) (xaero/map/file/MapSaveLoad.java:279)
- [ ] `MapSaveLoad.getMainFolder(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:283)
- [ ] `MapSaveLoad.getMWSubFolder(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:287)
- [ ] `MapSaveLoad.getCaveLayerFolder(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:295)
- [ ] `MapSaveLoad.getMWSubFolder(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:304)
- [ ] `MapSaveLoad.getOldFolder(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:308)
- [ ] `MapSaveLoad.xaeroToZip(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:312)
- [ ] `MapSaveLoad.detectRegions(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:337)
- [ ] `MapSaveLoad.detectRegionsFromFiles(11)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:427)
- [ ] `MapSaveLoad.saveRegion(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:485)
- [ ] `MapSaveLoad.getBackupFolder(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:646)
- [ ] `MapSaveLoad.backupFile(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:650)
- [ ] `MapSaveLoad.loadRegion(7)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:673)
- [ ] `MapSaveLoad.beingSaved(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:959)
- [ ] `MapSaveLoad.requestLoad(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:970)
- [ ] `MapSaveLoad.requestLoad(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:974)
- [ ] `MapSaveLoad.requestBranchCache(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:978)
- [ ] `MapSaveLoad.requestBranchCache(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:987)
- [ ] `MapSaveLoad.addToLoad(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:998)
- [ ] `MapSaveLoad.removeToLoad(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1021)
- [ ] `MapSaveLoad.clearToLoad(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1027)
- [ ] `MapSaveLoad.getSizeOfToLoad(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1037)
- [ ] `MapSaveLoad.saveExists(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1041)
- [ ] `MapSaveLoad.updateSave(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1056)
- [ ] `MapSaveLoad.run(5)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1082)
- [ ] `MapSaveLoad.removeTempCacheRequest(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1496)
- [ ] `MapSaveLoad.addTempCacheRequest(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1506)
- [ ] `MapSaveLoad.cleanupLoadedCache(6)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1510)
- [ ] `MapSaveLoad.savePixel(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1548)
- [ ] `MapSaveLoad.loadPixel(9)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1613)
- [ ] `MapSaveLoad.saveOverlay(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1736)
- [ ] `MapSaveLoad.loadOverlay(7)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1765)
- [ ] `MapSaveLoad.isRegionDetectionComplete(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1820)
- [ ] `MapSaveLoad.setRegionDetectionComplete(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1824)
- [ ] `MapSaveLoad.requestCache(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1828)
- [ ] `MapSaveLoad.removeToCache(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1840)
- [ ] `MapSaveLoad.removeToCache(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1846)
- [ ] `MapSaveLoad.toCacheContains(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1852)
- [ ] `MapSaveLoad.getToSave(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1858)
- [ ] `MapSaveLoad.getNextToLoadByViewing(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1862)
- [ ] `MapSaveLoad.setNextToLoadByViewing(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1866)
- [ ] `MapSaveLoad.getOldFormatSupport(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1870)
- [ ] `MapSaveLoad.safeDelete(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1874)
- [ ] `MapSaveLoad.safeMoveAndReplace(4)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1882)
- [ ] `MapSaveLoad.getSizeOfToLoadBranchCache(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/file/MapSaveLoad.java:1890)

### `OldFormatSupport`: it uses Minecraft's `BlockState`

- [ ] `OldFormatSupport.OldFormatSupport(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:98)
- [ ] `OldFormatSupport.loadVanillaStates(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:282)
- [ ] `OldFormatSupport.loadModdedStates(4)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:291)
- [ ] `OldFormatSupport.loadStates(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:306)
- [ ] `OldFormatSupport.loadStates(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:328)
- [ ] `OldFormatSupport.putState(4)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:334)
- [ ] `OldFormatSupport.getStateForId(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:343)
- [ ] `OldFormatSupport.getStateForId(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:354)
- [ ] `OldFormatSupport.fixBlock(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:359)
- [ ] `OldFormatSupport.fixBlockName1314(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:381)
- [ ] `OldFormatSupport.fixBlock1516(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:386)
- [ ] `OldFormatSupport.fixBlock1617(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:394)
- [ ] `OldFormatSupport.fixBlock1215(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:402)
- [ ] `OldFormatSupport.fixBlockState262_263(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:410)
- [ ] `OldFormatSupport.fixBiome(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:424)
- [ ] `OldFormatSupport.fixBiome(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:428)
- [ ] `OldFormatSupport.fixBiome(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:437)
- [ ] `OldFormatSupport.fixBiome1718(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/file/OldFormatSupport.java:441)

### `OldFormatSupport.Fixer`: it's an interface

- [ ] `OldFormatSupport.Fixer.fix(1)`: it has no body (xaero/map/file/OldFormatSupport.java:446)

### `RegionDetection`: it implements `MapRegionInfo`

- [ ] `RegionDetection.RegionDetection(8)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:32)
- [ ] `RegionDetection.getInitialVersion(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:43)
- [ ] `RegionDetection.shouldCache(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:47)
- [ ] `RegionDetection.getCacheFile(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:52)
- [ ] `RegionDetection.setShouldCache(2)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:57)
- [ ] `RegionDetection.setCacheFile(1)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:62)
- [ ] `RegionDetection.getWorldId(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:67)
- [ ] `RegionDetection.getDimId(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:72)
- [ ] `RegionDetection.getMwId(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:77)
- [ ] `RegionDetection.getRegionX(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:82)
- [ ] `RegionDetection.getRegionZ(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:87)
- [ ] `RegionDetection.transferInfoTo(1)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:92)
- [ ] `RegionDetection.transferInfoPostAddTo(2)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:103)
- [ ] `RegionDetection.transferInfoFrom(1)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:109)
- [ ] `RegionDetection.getRegionFile(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:134)
- [ ] `RegionDetection.isHasHadTerrain(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:139)
- [ ] `RegionDetection.hasLookedForCache(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:143)
- [ ] `RegionDetection.setRemoved(1)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:148)
- [ ] `RegionDetection.isRemoved(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:152)
- [ ] `RegionDetection.setNext(1)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:156)
- [ ] `RegionDetection.setPrevious(1)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:160)
- [ ] `RegionDetection.getNext(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:164)
- [ ] `RegionDetection.getPrevious(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:168)
- [ ] `RegionDetection.isDestroyed(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:172)
- [ ] `RegionDetection.onDestroyed(0)`: its class isn't translated (it implements `MapRegionInfo`) (xaero/map/file/RegionDetection.java:177)

### `WorldDataBiomeManager`: it implements Minecraft's `BiomeResolver`

- [ ] `WorldDataBiomeManager.WorldDataBiomeManager(0)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:23)
- [ ] `WorldDataBiomeManager.resetChunkBiomeData(4)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:32)
- [ ] `WorldDataBiomeManager.clear(0)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:39)
- [ ] `WorldDataBiomeManager.getChunkBiomeData(2)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:45)
- [ ] `WorldDataBiomeManager.addBiomeSectionForRegionChunk(4)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:49)
- [ ] `WorldDataBiomeManager.getBiome(4)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:53)
- [ ] `WorldDataBiomeManager.getNoiseBiome(3)`: its class isn't translated (it implements Minecraft's `BiomeResolver`) (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java:64)

### `WorldDataReaderChunkBiomeData`: it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)

- [ ] `WorldDataReaderChunkBiomeData.addSection(2)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/file/worldsave/biome/WorldDataReaderChunkBiomeData.java:13)
- [ ] `WorldDataReaderChunkBiomeData.getNoiseBiome(5)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/file/worldsave/biome/WorldDataReaderChunkBiomeData.java:21)
- [ ] `WorldDataReaderChunkBiomeData.clear(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/file/worldsave/biome/WorldDataReaderChunkBiomeData.java:37)

### `WorldDataReaderSectionBiomeData`: it uses Minecraft's `ListTag`

- [ ] `WorldDataReaderSectionBiomeData.WorldDataReaderSectionBiomeData(2)`: its class isn't translated (it uses Minecraft's `ListTag`) (xaero/map/file/worldsave/biome/WorldDataReaderSectionBiomeData.java:15)
- [ ] `WorldDataReaderSectionBiomeData.hasDifferentBiomes(0)`: its class isn't translated (it uses Minecraft's `ListTag`) (xaero/map/file/worldsave/biome/WorldDataReaderSectionBiomeData.java:20)
- [ ] `WorldDataReaderSectionBiomeData.get(3)`: its class isn't translated (it uses Minecraft's `ListTag`) (xaero/map/file/worldsave/biome/WorldDataReaderSectionBiomeData.java:24)

### `WorldDataChunkTileEntityLookup`: it uses Minecraft's `ListTag`

- [ ] `WorldDataChunkTileEntityLookup.WorldDataChunkTileEntityLookup(1)`: its class isn't translated (it uses Minecraft's `ListTag`) (xaero/map/file/worldsave/WorldDataChunkTileEntityLookup.java:12)
- [ ] `WorldDataChunkTileEntityLookup.loadIfNeeded(0)`: its class isn't translated (it uses Minecraft's `ListTag`) (xaero/map/file/worldsave/WorldDataChunkTileEntityLookup.java:16)
- [ ] `WorldDataChunkTileEntityLookup.getTileEntityNbt(3)`: its class isn't translated (it uses Minecraft's `ListTag`) (xaero/map/file/worldsave/WorldDataChunkTileEntityLookup.java:53)

### `WorldDataHandler`: it uses Minecraft's `ServerLevel`

- [ ] `WorldDataHandler.WorldDataHandler(2)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:29)
- [ ] `WorldDataHandler.handleRenderExecutor(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:34)
- [ ] `WorldDataHandler.prepareSingleplayer(2)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:38)
- [ ] `WorldDataHandler.buildRegion(6)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:55)
- [ ] `WorldDataHandler.onServerWorldUnload(1)`: it uses Minecraft's `ServerLevel` (xaero/map/file/worldsave/WorldDataHandler.java:90)
- [ ] `WorldDataHandler.getWorldServer(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:103)
- [ ] `WorldDataHandler.getWorldDataReader(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:107)
- [ ] `WorldDataHandler.getWorldDir(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:111)
- [ ] `WorldDataHandler.getWorldDataRenderExecutor(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:115)
- [ ] `WorldDataHandler.onResourceReload(0)`: its class isn't translated (it uses Minecraft's `ServerLevel`) (xaero/map/file/worldsave/WorldDataHandler.java:119)

### `WorldDataHandler.Result`


### `WorldDataReader`: it uses MapBlock, which isn't translated (it extends `MapPixel`)

- [ ] `WorldDataReader.WorldDataReader(5)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:88)
- [ ] `WorldDataReader.setMapProcessor(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:124)
- [ ] `WorldDataReader.updateHeightArray(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:128)
- [ ] `WorldDataReader.buildRegion(8)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:134)
- [ ] `WorldDataReader.readChunkNBTCompounds(2)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:261)
- [ ] `WorldDataReader.readChunk(2)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:270)
- [ ] `WorldDataReader.buildTileChunk(14)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:276)
- [ ] `WorldDataReader.buildTile(19)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:363)
- [ ] `WorldDataReader.buildPixel(20)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:744)
- [ ] `WorldDataReader.buildPixelHelp(18)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:839)
- [ ] `WorldDataReader.handleTileChunkOutsideRegion(9)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:904)
- [ ] `WorldDataReader.handleTileOutsideRegion(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:948)
- [ ] `WorldDataReader.handleChunkBiomes(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:956)
- [ ] `WorldDataReader.fillBiomes(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:974)
- [ ] `WorldDataReader.transferFilledBiomes(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:1005)
- [ ] `WorldDataReader.shouldOverlayCached(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:1025)
- [ ] `WorldDataReader.nibbleValue(2)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:1029)
- [ ] `WorldDataReader.onResourceReload(0)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/file/worldsave/WorldDataReader.java:1034)

### `CustomRenderTypes`

- [ ] `CustomRenderTypes.GUI_NEAREST`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:18)
- [ ] `CustomRenderTypes.GUI_BILINEAR`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:19)
- [ ] `CustomRenderTypes.GUI_BILINEAR_PRE`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:20)
- [ ] `CustomRenderTypes.MAP_BILINEAR`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:21)
- [ ] `CustomRenderTypes.MAP`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:22)
- [ ] `CustomRenderTypes.MAP_EXPORT`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:23)
- [ ] `CustomRenderTypes.MAP_BRANCH`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:24)
- [ ] `CustomRenderTypes.MAP_COLOR_OVERLAY`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:25)
- [ ] `CustomRenderTypes.MAP_FRAME`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:28)
- [ ] `CustomRenderTypes.MAP_COLOR_FILLER`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:29)
- [ ] `CustomRenderTypes.MAP_ELEMENT_TEXT_BG`: it uses Minecraft's `RenderType` (xaero/map/graphics/CustomRenderTypes.java:32)
- [ ] `CustomRenderTypes.applyFixedOrder(0)`: it uses `XaeroBufferProvider` (xaero.lib.client.graphics) (xaero/map/graphics/CustomRenderTypes.java:36)

### `GpuObjectDeleter`: it uses `GpuTextureAndView` (xaero.lib.client.graphics)

- [x] `GpuObjectDeleter.DELETE_PER_FRAME` (xaero/map/graphics/GpuObjectDeleter.java:9)
- [ ] `GpuObjectDeleter.work(0)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/map/graphics/GpuObjectDeleter.java:13)
- [ ] `GpuObjectDeleter.requestTextureDeletion(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/map/graphics/GpuObjectDeleter.java:31)
- [ ] `GpuObjectDeleter.requestBufferToDelete(1)`: its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/map/graphics/GpuObjectDeleter.java:37)
- [ ] `GpuObjectDeleter.deleteBuffers(2)`: it uses Minecraft's `GpuBuffer` (xaero/map/graphics/GpuObjectDeleter.java:43)
- [ ] `GpuObjectDeleter.deleteTextures(2)`: it uses `GpuTextureAndView` (xaero.lib.client.graphics) (xaero/map/graphics/GpuObjectDeleter.java:52)

### `ImprovedFramebuffer`: it extends Minecraft's `TextureTarget`

- [ ] `ImprovedFramebuffer.MAIN_RENDER_TARGET_FIELD`: it uses `Field` (java.lang.reflect) (xaero/map/graphics/ImprovedFramebuffer.java:20)
- [ ] `ImprovedFramebuffer.mainRenderTargetBackup`: it uses Minecraft's `RenderTarget` (xaero/map/graphics/ImprovedFramebuffer.java:23)
- [ ] `ImprovedFramebuffer.outputColorTextureOverrideBU`: it uses Minecraft's `GpuTextureView` (xaero/map/graphics/ImprovedFramebuffer.java:24)
- [ ] `ImprovedFramebuffer.outputDepthTextureOverrideBU`: it uses Minecraft's `GpuTextureView` (xaero/map/graphics/ImprovedFramebuffer.java:25)
- [ ] `ImprovedFramebuffer.ImprovedFramebuffer(4)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:29)
- [ ] `ImprovedFramebuffer.bindDefaultFramebuffer(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:37)
- [ ] `ImprovedFramebuffer.forceAsMainRenderTarget(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:41)
- [x] `ImprovedFramebuffer.restoreMainRenderTarget(0)` (xaero/map/graphics/ImprovedFramebuffer.java:53)
- [ ] `ImprovedFramebuffer.bindAsMainTarget(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:67)
- [ ] `ImprovedFramebuffer.setColorTexture(2)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:71)
- [ ] `ImprovedFramebuffer.setColorTexture(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:87)
- [ ] `ImprovedFramebuffer.setDepthTexture(2)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:91)
- [ ] `ImprovedFramebuffer.getSampler(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:96)
- [ ] `ImprovedFramebuffer.setSampler(1)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:100)
- [ ] `ImprovedFramebuffer.closeColorTexture(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:109)
- [ ] `ImprovedFramebuffer.closeDepthTexture(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:114)
- [ ] `ImprovedFramebuffer.getTas(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:119)
- [ ] `ImprovedFramebuffer.getFormat(0)`: its class isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/graphics/ImprovedFramebuffer.java:123)

### `MapRenderHelper`

- [ ] `MapRenderHelper.renderBranchUpdate(12)`: it uses Minecraft's `GpuTextureView` (xaero/map/graphics/MapRenderHelper.java:15)
- [ ] `MapRenderHelper.fillIntoExistingBuffer(10)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:40)
- [ ] `MapRenderHelper.blitIntoExistingBuffer(16)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:47)
- [ ] `MapRenderHelper.blitIntoExistingBuffer(12)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:77)
- [ ] `MapRenderHelper.blitIntoMultiTextureRenderer(17)`: it uses MultiTextureRenderTypeRenderer, which isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/MapRenderHelper.java:83)
- [ ] `MapRenderHelper.blitIntoMultiTextureRenderer(15)`: it uses MultiTextureRenderTypeRenderer, which isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/MapRenderHelper.java:106)
- [ ] `MapRenderHelper.blitIntoMultiTextureRenderer(13)`: it uses MultiTextureRenderTypeRenderer, which isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/MapRenderHelper.java:126)
- [ ] `MapRenderHelper.renderDynamicHighlight(16)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:144)
- [ ] `MapRenderHelper.renderDynamicChunkHighlight(16)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:224)
- [ ] `MapRenderHelper.drawCenteredStringWithBackground(10)`: it's overloaded with the same number of parameters (xaero/map/graphics/MapRenderHelper.java:266)
- [ ] `MapRenderHelper.drawStringWithBackground(10)`: it's overloaded with the same number of parameters (xaero/map/graphics/MapRenderHelper.java:273)
- [ ] `MapRenderHelper.drawCenteredStringWithBackground(10)`: it's overloaded with the same number of parameters (xaero/map/graphics/MapRenderHelper.java:282)
- [ ] `MapRenderHelper.drawStringWithBackground(10)`: it's overloaded with the same number of parameters (xaero/map/graphics/MapRenderHelper.java:289)
- [ ] `MapRenderHelper.blitIntoExistingBuffer(10)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:298)
- [ ] `MapRenderHelper.blitIntoExistingBuffer(8)`: it uses Minecraft's `VertexConsumer` (xaero/map/graphics/MapRenderHelper.java:313)
- [ ] `MapRenderHelper.blitIntoMultiTextureRenderer(11)`: it uses MultiTextureRenderTypeRenderer, which isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/MapRenderHelper.java:317)
- [ ] `MapRenderHelper.blitIntoMultiTextureRenderer(9)`: it uses MultiTextureRenderTypeRenderer, which isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/MapRenderHelper.java:334)
- [x] `MapRenderHelper.restoreDefaultShaderBlendState(0)` (xaero/map/graphics/MapRenderHelper.java:340)

### `OpenGlHelper`

- [ ] `OpenGlHelper.deviceCheckCache`: it uses Minecraft's `GpuDevice` (xaero/map/graphics/OpenGlHelper.java:7)
- [x] `OpenGlHelper.cachedDeviceIsOpenGL` (xaero/map/graphics/OpenGlHelper.java:8)
- [ ] `OpenGlHelper.isUsingOpenGL(0)`: it uses Minecraft's `GpuDevice` (xaero/map/graphics/OpenGlHelper.java:10)

### `PixelBuffers`

- [x] `PixelBuffers.buffersType` (xaero/map/graphics/PixelBuffers.java:6)
- [ ] `PixelBuffers.glBindBuffer(2)`: it calls `GL15` (org.lwjgl.opengl).glBindBuffer (xaero/map/graphics/PixelBuffers.java:8)

### `MultiTextureRenderTypeRenderer`: it uses Minecraft's `ByteBufferBuilder`

- [x] `MultiTextureRenderTypeRenderer.RENDER_PASS_NAME` (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:31)
- [ ] `MultiTextureRenderTypeRenderer.MultiTextureRenderTypeRenderer(0)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:44)
- [ ] `MultiTextureRenderTypeRenderer.init(3)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:47)
- [ ] `MultiTextureRenderTypeRenderer.draw(0)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:59)
- [ ] `MultiTextureRenderTypeRenderer.uploadImmediateVertexBuffer(1)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:176)
- [ ] `MultiTextureRenderTypeRenderer.endBuffer(1)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:187)
- [ ] `MultiTextureRenderTypeRenderer.begin(1)`: its class isn't translated (it uses Minecraft's `ByteBufferBuilder`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:191)

### `MultiTextureRenderTypeRenderer.DrawCall`: it uses Minecraft's `GpuBufferSlice`

- [ ] `MultiTextureRenderTypeRenderer.DrawCall.DrawCall(5)`: its class isn't translated (it uses Minecraft's `GpuBufferSlice`) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java:220)

### `MultiTextureRenderTypeRendererProvider`: it uses `Deque` (java.util)

- [ ] `MultiTextureRenderTypeRendererProvider.MultiTextureRenderTypeRendererProvider(1)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:14)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(1)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:22)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(2)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:26)
- [ ] `MultiTextureRenderTypeRendererProvider.getRenderer(3)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:30)
- [ ] `MultiTextureRenderTypeRendererProvider.draw(1)`: its class isn't translated (it uses `Deque` (java.util)) (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java:41)

### `TextureUpload`: it implements `PoolUnit` (xaero.map.pool)

- [ ] `TextureUpload.DOWNLOAD_CALLBACK`: a function kept in a variable or field (only parameters can be) (xaero/map/graphics/TextureUpload.java:12)
- [ ] `TextureUpload.create(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/graphics/TextureUpload.java:23)
- [ ] `TextureUpload.run(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/graphics/TextureUpload.java:35)
- [ ] `TextureUpload.upload(0)`: it has no body (xaero/map/graphics/TextureUpload.java:39)
- [ ] `TextureUpload.getUploadType(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/graphics/TextureUpload.java:41)

### `TextureUpload.BranchDownload`: it extends `TextureUpload`

- [ ] `TextureUpload.BranchDownload.BranchDownload(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:49)
- [ ] `TextureUpload.BranchDownload.BranchDownload(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:53)
- [ ] `TextureUpload.BranchDownload.upload(0)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:58)
- [ ] `TextureUpload.BranchDownload.create(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:67)

### `TextureUpload.BranchUpdate`: it extends `TextureUpload`

- [ ] `TextureUpload.BranchUpdate.BranchUpdate(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:85)
- [ ] `TextureUpload.BranchUpdate.BranchUpdate(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:89)
- [ ] `TextureUpload.BranchUpdate.upload(0)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:94)
- [ ] `TextureUpload.BranchUpdate.create(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:113)

### `TextureUpload.Normal`: it extends `TextureUpload`

- [ ] `TextureUpload.Normal.Normal(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:128)
- [ ] `TextureUpload.Normal.Normal(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:132)
- [ ] `TextureUpload.Normal.upload(0)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:137)
- [ ] `TextureUpload.Normal.create(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:144)

### `TextureUpload.SubsequentNormal`: it extends `TextureUpload`

- [ ] `TextureUpload.SubsequentNormal.SubsequentNormal(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:154)
- [ ] `TextureUpload.SubsequentNormal.SubsequentNormal(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:158)
- [ ] `TextureUpload.SubsequentNormal.upload(0)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:163)
- [ ] `TextureUpload.SubsequentNormal.create(1)`: its class isn't translated (it extends `TextureUpload`) (xaero/map/graphics/TextureUpload.java:183)

### `TextureUploadBenchmark`

- [x] `TextureUploadBenchmark.TextureUploadBenchmark(1)` (xaero/map/graphics/TextureUploadBenchmark.java:17)
- [x] `TextureUploadBenchmark.pre(0)` (xaero/map/graphics/TextureUploadBenchmark.java:26)
- [ ] `TextureUploadBenchmark.post(1)`: it uses Minecraft's `GpuFence` (xaero/map/graphics/TextureUploadBenchmark.java:30)
- [x] `TextureUploadBenchmark.finish(1)` (xaero/map/graphics/TextureUploadBenchmark.java:43)
- [x] `TextureUploadBenchmark.isFinished(0)` (xaero/map/graphics/TextureUploadBenchmark.java:52)
- [x] `TextureUploadBenchmark.isFinished(1)` (xaero/map/graphics/TextureUploadBenchmark.java:56)
- [x] `TextureUploadBenchmark.getAverage(1)` (xaero/map/graphics/TextureUploadBenchmark.java:60)
- [ ] `TextureUploadBenchmark.getTotalsString(0)`: it calls `Arrays` (java.util).toString (xaero/map/graphics/TextureUploadBenchmark.java:64)

### `TextureUploader`: it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))

- [x] `TextureUploader.NORMAL` (xaero/map/graphics/TextureUploader.java:14)
- [x] `TextureUploader.NORMALDOWNLOAD` (xaero/map/graphics/TextureUploader.java:15)
- [x] `TextureUploader.BRANCHUPDATE` (xaero/map/graphics/TextureUploader.java:16)
- [x] `TextureUploader.BRANCHUPDATE_ALLOCATE` (xaero/map/graphics/TextureUploader.java:17)
- [x] `TextureUploader.BRANCHDOWNLOAD` (xaero/map/graphics/TextureUploader.java:18)
- [x] `TextureUploader.SUBSEQUENT_NORMAL` (xaero/map/graphics/TextureUploader.java:19)
- [x] `TextureUploader.DEFAULT_NORMAL_TIME` (xaero/map/graphics/TextureUploader.java:20)
- [x] `TextureUploader.DEFAULT_COMPRESSED_TIME` (xaero/map/graphics/TextureUploader.java:21)
- [x] `TextureUploader.DEFAULT_BRANCHUPDATED_TIME` (xaero/map/graphics/TextureUploader.java:22)
- [x] `TextureUploader.DEFAULT_BRANCHUPDATE_ALLOCATE_TIME` (xaero/map/graphics/TextureUploader.java:23)
- [x] `TextureUploader.DEFAULT_BRANCHDOWNLOAD_TIME` (xaero/map/graphics/TextureUploader.java:24)
- [x] `TextureUploader.DEFAULT_SUBSEQUENT_NORMAL_TIME` (xaero/map/graphics/TextureUploader.java:25)
- [ ] `TextureUploader.TextureUploader(6)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:34)
- [ ] `TextureUploader.requestUpload(1)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:50)
- [ ] `TextureUploader.requestNormal(8)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:67)
- [ ] `TextureUploader.requestSubsequentNormal(9)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:74)
- [ ] `TextureUploader.requestBranchUpdate(16)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:81)
- [ ] `TextureUploader.requestBranchDownload(3)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:143)
- [ ] `TextureUploader.finishNewestRequestImmediately(0)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:148)
- [ ] `TextureUploader.uploadTextures(0)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:154)
- [ ] `TextureUploader.addToPool(1)`: its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))) (xaero/map/graphics/TextureUploader.java:186)

### `ConfirmScreenBase`: it extends Minecraft's `ConfirmScreen`

- [ ] `ConfirmScreenBase.ConfirmScreenBase(6)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:19)
- [ ] `ConfirmScreenBase.ConfirmScreenBase(8)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:29)
- [ ] `ConfirmScreenBase.onExit(1)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:46)
- [ ] `ConfirmScreenBase.goBack(0)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:50)
- [ ] `ConfirmScreenBase.onClose(0)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:54)
- [ ] `ConfirmScreenBase.renderEscapeScreen(4)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:58)
- [ ] `ConfirmScreenBase.extractBackground(4)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:66)
- [ ] `ConfirmScreenBase.init(0)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:74)
- [ ] `ConfirmScreenBase.shouldSkipWorldRender(0)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:81)
- [ ] `ConfirmScreenBase.onDropdownOpen(1)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:88)
- [ ] `ConfirmScreenBase.onDropdownClosed(1)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:91)
- [ ] `ConfirmScreenBase.getEscape(0)`: its class isn't translated (it extends Minecraft's `ConfirmScreen`) (xaero/map/gui/ConfirmScreenBase.java:94)

### `CursorBox`: it extends `Tooltip` (xaero.lib.client.gui.widget)

- [ ] `CursorBox.CursorBox(1)`: its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)) (xaero/map/gui/CursorBox.java:9)
- [ ] `CursorBox.CursorBox(2)`: its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)) (xaero/map/gui/CursorBox.java:13)
- [ ] `CursorBox.CursorBox(3)`: its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)) (xaero/map/gui/CursorBox.java:17)
- [ ] `CursorBox.CursorBox(1)`: its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)) (xaero/map/gui/CursorBox.java:21)
- [ ] `CursorBox.CursorBox(2)`: its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)) (xaero/map/gui/CursorBox.java:25)
- [ ] `CursorBox.CursorBox(1)`: its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)) (xaero/map/gui/CursorBox.java:29)

### `GuiRightClickMenu`: it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)

- [ ] `GuiRightClickMenu.GuiRightClickMenu(8)`: its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:17)
- [x] `GuiRightClickMenu.shouldOpenLeft(4)` (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:48)
- [x] `GuiRightClickMenu.shouldOpenUp(3)` (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:52)
- [ ] `GuiRightClickMenu.setClosed(1)`: its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:57)
- [ ] `GuiRightClickMenu.selectId(2)`: its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:65)
- [ ] `GuiRightClickMenu.getMenu(5)`: it uses GuiRightClickMenu, which isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:74)
- [ ] `GuiRightClickMenu.getTarget(0)`: its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java:80)

### `RightClickOption`: it uses Minecraft's `Style`

- [ ] `RightClickOption.RightClickOption(4)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:18)
- [ ] `RightClickOption.RightClickOption(3)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:27)
- [ ] `RightClickOption.onAction(1)`: it has no body (xaero/map/gui/dropdown/rightclick/RightClickOption.java:31)
- [ ] `RightClickOption.onSelected(1)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:33)
- [ ] `RightClickOption.getName(0)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:42)
- [ ] `RightClickOption.getDisplayName(0)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:46)
- [ ] `RightClickOption.isActive(0)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:57)
- [ ] `RightClickOption.setActive(1)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:61)
- [ ] `RightClickOption.setNameFormatArgs(1)`: its class isn't translated (it uses Minecraft's `Style`) (xaero/map/gui/dropdown/rightclick/RightClickOption.java:66)

### `EditDefaultTpCommandScreen`: it extends `EditStringConfigOptionScreen` (xaero.lib.client.gui.config)

- [ ] `EditDefaultTpCommandScreen.EditDefaultTpCommandScreen(8)`: its class isn't translated (it extends `EditStringConfigOptionScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/EditDefaultTpCommandScreen.java:17)
- [ ] `EditDefaultTpCommandScreen.extractBackground(4)`: its class isn't translated (it extends `EditStringConfigOptionScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/EditDefaultTpCommandScreen.java:32)

### `ExportScreen`: it extends `GuiSettings` (xaero.lib.client.gui)

- [x] `ExportScreen.EXPORTING_MESSAGE` in game_side.rs (xaero/map/gui/ExportScreen.java:22)
- [ ] `ExportScreen.ExportScreen(4)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:29)
- [ ] `ExportScreen.init(0)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:58)
- [ ] `ExportScreen.confirm(0)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:64)
- [ ] `ExportScreen.extractBackground(4)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:69)
- [ ] `ExportScreen.extractRenderState(4)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:74)
- [ ] `ExportScreen.primaryOptionEntry(1)`: it's generic (xaero/map/gui/ExportScreen.java:105)
- [ ] `ExportScreen.onExportDone(1)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:118)
- [ ] `ExportScreen.getSelection(0)`: its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)) (xaero/map/gui/ExportScreen.java:123)

### `GuiCaveModeOptions`: it uses Minecraft's `AbstractWidget`

- [ ] `GuiCaveModeOptions.onInit(2)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:27)
- [ ] `GuiCaveModeOptions.onCaveModeTypeButton(2)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:44)
- [ ] `GuiCaveModeOptions.createField(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:53)
- [ ] `GuiCaveModeOptions.getCaveStart(0)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:68)
- [ ] `GuiCaveModeOptions.setCaveStart(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:73)
- [ ] `GuiCaveModeOptions.createSlider(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:79)
- [ ] `GuiCaveModeOptions.updateField(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:99)
- [ ] `GuiCaveModeOptions.updateSlider(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:107)
- [ ] `GuiCaveModeOptions.toggle(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:115)
- [ ] `GuiCaveModeOptions.onCaveModeStartSet(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:120)
- [ ] `GuiCaveModeOptions.tick(1)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:126)
- [ ] `GuiCaveModeOptions.unfocusAll(0)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:138)
- [ ] `GuiCaveModeOptions.getCaveModeTypeButtonMessage(0)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:148)
- [ ] `GuiCaveModeOptions.isEnabled(0)`: its class isn't translated (it uses Minecraft's `AbstractWidget`) (xaero/map/gui/GuiCaveModeOptions.java:165)

### `GuiDimensionOptions`: it uses Minecraft's `ResourceKey`

- [ ] `GuiDimensionOptions.GuiDimensionOptions(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/gui/GuiDimensionOptions.java:10)

### `GuiMap`: it extends `ScreenBase` (xaero.lib.client.gui)

- [x] `GuiMap.FULL_RELOAD_IN_PROGRESS` in game_side.rs (xaero/map/gui/GuiMap.java:109)
- [x] `GuiMap.UNKNOWN_DIMENSION_TYPE1` in game_side.rs (xaero/map/gui/GuiMap.java:110)
- [x] `GuiMap.UNKNOWN_DIMENSION_TYPE2` in game_side.rs (xaero/map/gui/GuiMap.java:111)
- [x] `GuiMap.ZOOM_STEP` (xaero/map/gui/GuiMap.java:112)
- [x] `GuiMap.white` (xaero/map/gui/GuiMap.java:113)
- [x] `GuiMap.black` (xaero/map/gui/GuiMap.java:114)
- [ ] `GuiMap.black4f`: it uses `Vector4f` (org.joml) (xaero/map/gui/GuiMap.java:115)
- [x] `GuiMap.lastAmountOfRegionsViewed` (xaero/map/gui/GuiMap.java:116)
- [x] `GuiMap.destScale` (xaero/map/gui/GuiMap.java:138)
- [ ] `GuiMap.primaryScaleFBO`: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/gui/GuiMap.java:150)
- [ ] `GuiMap.immediateRenderFBO`: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`) (xaero/map/gui/GuiMap.java:151)
- [x] `GuiMap.attachedCamera` (xaero/map/gui/GuiMap.java:204)
- [x] `GuiMap.hiddenUI` (xaero/map/gui/GuiMap.java:206)
- [x] `GuiMap.identityMatrix` (xaero/map/gui/GuiMap.java:207)
- [ ] `GuiMap.GuiMap(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:209)
- [ ] `GuiMap.getScaleMultiplier(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:227)
- [ ] `GuiMap.addRenderableWidget(1)`: it's generic (xaero/map/gui/GuiMap.java:231)
- [ ] `GuiMap.addButton(1)`: it's generic (xaero/map/gui/GuiMap.java:235)
- [ ] `GuiMap.addWidget(1)`: it's generic (xaero/map/gui/GuiMap.java:239)
- [ ] `GuiMap.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:243)
- [ ] `GuiMap.setInitialFocus(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:507)
- [ ] `GuiMap.onRenderWaypointsButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:510)
- [ ] `GuiMap.onCaveModeButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:516)
- [ ] `GuiMap.onDimensionToggleButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:521)
- [ ] `GuiMap.onSettingsButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:534)
- [ ] `GuiMap.onKeybindingsButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:538)
- [ ] `GuiMap.onExportButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:542)
- [ ] `GuiMap.toggleWaypointMenu(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:546)
- [ ] `GuiMap.togglePlayerMenu(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:558)
- [ ] `GuiMap.onPlayersButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:570)
- [ ] `GuiMap.onClaimsButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:576)
- [ ] `GuiMap.onWaypointsButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:582)
- [ ] `GuiMap.onRadarButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:588)
- [ ] `GuiMap.onZoomInButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:594)
- [ ] `GuiMap.onZoomOutButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:598)
- [ ] `GuiMap.onHopButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:602)
- [ ] `GuiMap.activateHopToCoords(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:608)
- [ ] `GuiMap.onAttachedCameraButton(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:644)
- [ ] `GuiMap.mouseClicked(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:650)
- [ ] `GuiMap.mouseReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:695)
- [ ] `GuiMap.mouseScrolled(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:732)
- [ ] `GuiMap.changeZoom(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:745)
- [ ] `GuiMap.applyZoomLimits(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:787)
- [ ] `GuiMap.removed(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:808)
- [ ] `GuiMap.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:814)
- [ ] `GuiMap.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2246)
- [ ] `GuiMap.renderPreDropdown(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2249)
- [ ] `GuiMap.hoveredElementTooltipHelper(2)`: it's generic (xaero/map/gui/GuiMap.java:2262)
- [ ] `GuiMap.renderLoadingScreen(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2268)
- [ ] `GuiMap.renderMessageScreen(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2272)
- [ ] `GuiMap.renderMessageScreen(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2276)
- [ ] `GuiMap.drawDotOnMap(6)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2288)
- [ ] `GuiMap.drawArrowOnMap(6)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2292)
- [ ] `GuiMap.drawFarArrowOnMap(6)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2296)
- [ ] `GuiMap.drawObjectOnMap(12)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2300)
- [ ] `GuiMap.renderTexturedModalRectWithLighting3(8)`: it uses Minecraft's `GpuTextureView` (xaero/map/gui/GuiMap.java:2341)
- [ ] `GuiMap.renderTexturedModalSubRectWithLighting(12)`: it uses Minecraft's `GpuTextureView` (xaero/map/gui/GuiMap.java:2347)
- [ ] `GuiMap.buildTexturedModalRectWithLighting(6)`: it uses Minecraft's `BufferBuilder` (xaero/map/gui/GuiMap.java:2364)
- [ ] `GuiMap.buildTexturedModalSubRectWithLighting(10)`: it uses Minecraft's `BufferBuilder` (xaero/map/gui/GuiMap.java:2371)
- [ ] `GuiMap.renderTexturedModalRect(14)`: it uses Minecraft's `VertexConsumer` (xaero/map/gui/GuiMap.java:2389)
- [ ] `GuiMap.mapClicked(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2415)
- [ ] `GuiMap.handleRightClick(3)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2431)
- [ ] `GuiMap.charTyped(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2445)
- [ ] `GuiMap.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2454)
- [ ] `GuiMap.keyReleased(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2493)
- [x] `GuiMap.bytesToMb(1)` (xaero/map/gui/GuiMap.java:2502)
- [ ] `GuiMap.setColourBuffer(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2506)
- [ ] `GuiMap.isUsingTextField(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2513)
- [ ] `GuiMap.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2518)
- [ ] `GuiMap.getTrackedPlayerKeyBinding(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2531)
- [ ] `GuiMap.onInputPress(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2535)
- [ ] `GuiMap.getCurrentMapCoordinateScale(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2616)
- [ ] `GuiMap.onInputRelease(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2620)
- [ ] `GuiMap.getHoverTarget(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2711)
- [ ] `GuiMap.unfocusAll(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2715)
- [ ] `GuiMap.closeRightClick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2729)
- [ ] `GuiMap.onRightClickClosed(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2735)
- [ ] `GuiMap.closeDropdowns(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2740)
- [ ] `GuiMap.getRightClickOptions(0)`: its body couldn't be read: an anonymous class (xaero/map/gui/GuiMap.java:2746)
- [ ] `GuiMap.isRightClickValid(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2922)
- [ ] `GuiMap.getRightClickTitleBackgroundColor(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2927)
- [ ] `GuiMap.shouldSkipWorldRender(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2932)
- [ ] `GuiMap.getUserScale(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2936)
- [ ] `GuiMap.getRadarButton(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2940)
- [ ] `GuiMap.onDropdownOpen(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2944)
- [ ] `GuiMap.onDropdownClosed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2949)
- [ ] `GuiMap.onCaveModeStartSet(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2956)
- [ ] `GuiMap.getDimensionOnInit(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2960)
- [ ] `GuiMap.getMapProcessor(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2964)
- [ ] `GuiMap.enableCaveModeOptions(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2968)
- [ ] `GuiMap.removeWidget(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2974)
- [ ] `GuiMap.toggleHiddenUI(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2978)
- [ ] `GuiMap.isMapNavigationInput(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2986)
- [ ] `GuiMap.isExitInput(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:2999)
- [ ] `GuiMap.renderServerChunkRadius(7)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMap.java:3003)

### `GuiMapName`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiMapName.GuiMapName(5)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:28)
- [ ] `GuiMapName.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:37)
- [ ] `GuiMapName.setInitialFocus(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:102)
- [ ] `GuiMapName.canConfirm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:105)
- [ ] `GuiMapName.updateConfirmButton(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:109)
- [ ] `GuiMapName.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:113)
- [ ] `GuiMapName.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:123)
- [ ] `GuiMapName.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:127)
- [ ] `GuiMapName.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:134)
- [ ] `GuiMapName.isInGameUi(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapName.java:138)

### `GuiMapSwitching`: it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)

- [x] `GuiMapSwitching.CONNECT_MAP` in game_side.rs (xaero/map/gui/GuiMapSwitching.java:32)
- [x] `GuiMapSwitching.DISCONNECT_MAP` in game_side.rs (xaero/map/gui/GuiMapSwitching.java:33)
- [ ] `GuiMapSwitching.GuiMapSwitching(1)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:52)
- [ ] `GuiMapSwitching.init(4)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:58)
- [ ] `GuiMapSwitching.getSortedDimensionOptions(1)`: it uses GuiDimensionOptions, which isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/gui/GuiMapSwitching.java:264)
- [ ] `GuiMapSwitching.createDimensionDropdown(4)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:280)
- [ ] `GuiMapSwitching.createMapDropdown(4)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:325)
- [ ] `GuiMapSwitching.isUIPaused(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:390)
- [ ] `GuiMapSwitching.isMapSelectionOptionEnabled(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:394)
- [ ] `GuiMapSwitching.canToggleThisScreen(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:398)
- [ ] `GuiMapSwitching.canDeleteMap(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:402)
- [ ] `GuiMapSwitching.canRenameMap(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:410)
- [ ] `GuiMapSwitching.canConnectMap(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:414)
- [ ] `GuiMapSwitching.canConfirm(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:428)
- [ ] `GuiMapSwitching.getConnectButtonLabel(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:432)
- [ ] `GuiMapSwitching.updateButtons(3)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:453)
- [ ] `GuiMapSwitching.getMultiworldTypeButtonMessage(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:496)
- [ ] `GuiMapSwitching.confirm(4)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:508)
- [ ] `GuiMapSwitching.getDropdownSelectionIdFromValue(2)`: it's generic (xaero/map/gui/GuiMapSwitching.java:515)
- [ ] `GuiMapSwitching.preMapRender(4)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:525)
- [ ] `GuiMapSwitching.refreshScreen(3)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:550)
- [ ] `GuiMapSwitching.renderText(6)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:561)
- [ ] `GuiMapSwitching.postMapRender(6)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:573)
- [ ] `GuiMapSwitching.getCreatedDimensionDropdown(0)`: its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)) (xaero/map/gui/GuiMapSwitching.java:576)

### `GuiMapSwitchingButton`: it extends `GuiTexturedButton`

- [ ] `GuiMapSwitchingButton.TOOLTIP`: it uses `Tooltip` (xaero.lib.client.gui.widget) (xaero/map/gui/GuiMapSwitchingButton.java:8)
- [ ] `GuiMapSwitchingButton.GuiMapSwitchingButton(4)`: its class isn't translated (it extends `GuiTexturedButton`) (xaero/map/gui/GuiMapSwitchingButton.java:10)

### `GuiMapTpCommand`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiMapTpCommand.GuiMapTpCommand(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:30)
- [ ] `GuiMapTpCommand.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:40)
- [ ] `GuiMapTpCommand.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:118)
- [ ] `GuiMapTpCommand.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:129)
- [ ] `GuiMapTpCommand.canConfirm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:135)
- [ ] `GuiMapTpCommand.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:139)
- [ ] `GuiMapTpCommand.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiMapTpCommand.java:143)

### `GuiPlayerTpCommand`: it extends `ScreenBase` (xaero.lib.client.gui)

- [ ] `GuiPlayerTpCommand.GuiPlayerTpCommand(2)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:26)
- [ ] `GuiPlayerTpCommand.init(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:35)
- [ ] `GuiPlayerTpCommand.extractBackground(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:87)
- [ ] `GuiPlayerTpCommand.extractRenderState(4)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:94)
- [ ] `GuiPlayerTpCommand.canConfirm(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:99)
- [ ] `GuiPlayerTpCommand.tick(0)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:103)
- [ ] `GuiPlayerTpCommand.keyPressed(1)`: its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)) (xaero/map/gui/GuiPlayerTpCommand.java:108)

### `GuiTexturedButton`: it extends `TooltipButton`

- [ ] `GuiTexturedButton.GuiTexturedButton(13)`: its class isn't translated (it extends `TooltipButton`) (xaero/map/gui/GuiTexturedButton.java:20)
- [ ] `GuiTexturedButton.getMessage(0)`: its class isn't translated (it extends `TooltipButton`) (xaero/map/gui/GuiTexturedButton.java:45)
- [ ] `GuiTexturedButton.extractContents(4)`: its class isn't translated (it extends `TooltipButton`) (xaero/map/gui/GuiTexturedButton.java:49)

### `GuiWorldMapSettings`: it extends `EditConfigScreen` (xaero.lib.client.gui.config)

- [x] `GuiWorldMapSettings.PLAYER_TELEPORT_COMMAND_TOOLTIP` in game_side.rs (xaero/map/gui/GuiWorldMapSettings.java:33)
- [x] `GuiWorldMapSettings.MAP_TELEPORT_COMMAND_TOOLTIP` in game_side.rs (xaero/map/gui/GuiWorldMapSettings.java:34)
- [ ] `GuiWorldMapSettings.GuiWorldMapSettings(1)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:36)
- [ ] `GuiWorldMapSettings.GuiWorldMapSettings(2)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:40)
- [ ] `GuiWorldMapSettings.GuiWorldMapSettings(3)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:44)
- [ ] `GuiWorldMapSettings.getEntryForFullReloadOption(1)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:212)
- [ ] `GuiWorldMapSettings.resetConfirmResult(3)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:254)
- [ ] `GuiWorldMapSettings.init(0)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:262)
- [ ] `GuiWorldMapSettings.onSettingsButton(1)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:274)
- [ ] `GuiWorldMapSettings.keyPressed(1)`: its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)) (xaero/map/gui/GuiWorldMapSettings.java:278)

### `IRightClickableElement`: it's an interface

- [ ] `IRightClickableElement.getRightClickOptions(0)`: it has no body (xaero/map/gui/IRightClickableElement.java:7)
- [ ] `IRightClickableElement.isRightClickValid(0)`: it has no body (xaero/map/gui/IRightClickableElement.java:9)
- [ ] `IRightClickableElement.getRightClickTitleBackgroundColor(0)`: its class isn't translated (it's an interface) (xaero/map/gui/IRightClickableElement.java:11)

### `MapMouseButtonPress`


### `MapTileSelection`

- [x] `MapTileSelection.MapTileSelection(2)` (xaero/map/gui/MapTileSelection.java:9)
- [x] `MapTileSelection.setEnd(2)` (xaero/map/gui/MapTileSelection.java:14)
- [x] `MapTileSelection.getStartX(0)` (xaero/map/gui/MapTileSelection.java:19)
- [x] `MapTileSelection.getStartZ(0)` (xaero/map/gui/MapTileSelection.java:23)
- [x] `MapTileSelection.getEndX(0)` (xaero/map/gui/MapTileSelection.java:27)
- [x] `MapTileSelection.getEndZ(0)` (xaero/map/gui/MapTileSelection.java:31)
- [x] `MapTileSelection.getLeft(0)` (xaero/map/gui/MapTileSelection.java:35)
- [x] `MapTileSelection.getRight(0)` (xaero/map/gui/MapTileSelection.java:39)
- [x] `MapTileSelection.getTop(0)` (xaero/map/gui/MapTileSelection.java:43)
- [x] `MapTileSelection.getBottom(0)` (xaero/map/gui/MapTileSelection.java:47)

### `Message`: it uses Minecraft's `Component`

- [ ] `Message.Message(2)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/map/gui/message/Message.java:9)
- [ ] `Message.getText(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/map/gui/message/Message.java:14)
- [ ] `Message.getAdditionTime(0)`: its class isn't translated (it uses Minecraft's `Component`) (xaero/map/gui/message/Message.java:18)

### `MessageBox`: it uses Message, which isn't translated (it uses Minecraft's `Component`)

- [ ] `MessageBox.MessageBox(3)`: its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:14)
- [ ] `MessageBox.addMessageLine(1)`: its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:20)
- [ ] `MessageBox.addMessage(1)`: its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:28)
- [ ] `MessageBox.addMessageWithSource(2)`: its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:37)
- [ ] `MessageBox.getCapacity(0)`: its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:45)
- [ ] `MessageBox.getIterator(0)`: its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:49)

### `MessageBox.Builder`

- [x] `MessageBox.Builder.Builder(0)` (xaero/map/gui/message/MessageBox.java:57)
- [ ] `MessageBox.Builder.setDefault(0)`: `this` used as a value (xaero/map/gui/message/MessageBox.java:60)
- [ ] `MessageBox.Builder.setWidth(1)`: `this` used as a value (xaero/map/gui/message/MessageBox.java:66)
- [ ] `MessageBox.Builder.setCapacity(1)`: `this` used as a value (xaero/map/gui/message/MessageBox.java:71)
- [ ] `MessageBox.Builder.build(0)`: it uses MessageBox, which isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)) (xaero/map/gui/message/MessageBox.java:76)
- [ ] `MessageBox.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/map/gui/message/MessageBox.java:80)

### `MessageBoxRenderer`

- [ ] `MessageBoxRenderer.render(6)`: it uses Minecraft's `GuiGraphicsExtractor` (xaero/map/gui/message/render/MessageBoxRenderer.java:14)

### `ScreenSwitchSettingEntry`: it implements `ISettingEntry` (xaero.lib.client.gui)

- [ ] `ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(4)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/map/gui/ScreenSwitchSettingEntry.java:21)
- [ ] `ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(5)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/map/gui/ScreenSwitchSettingEntry.java:25)
- [ ] `ScreenSwitchSettingEntry.getStringForSearch(0)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/map/gui/ScreenSwitchSettingEntry.java:35)
- [ ] `ScreenSwitchSettingEntry.createWidget(3)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/map/gui/ScreenSwitchSettingEntry.java:45)
- [ ] `ScreenSwitchSettingEntry.getScreenFactory(0)`: its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)) (xaero/map/gui/ScreenSwitchSettingEntry.java:61)

### `TooltipButton`: it extends Minecraft's `Plain`

- [ ] `TooltipButton.TooltipButton(7)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/map/gui/TooltipButton.java:13)
- [ ] `TooltipButton.getXaero_tooltip(0)`: its class isn't translated (it extends Minecraft's `Plain`) (xaero/map/gui/TooltipButton.java:18)

### `TooltipSlider`: it extends Minecraft's `AbstractSliderButton`

- [ ] `TooltipSlider.TooltipSlider(9)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/map/gui/TooltipSlider.java:16)
- [ ] `TooltipSlider.getXaero_tooltip(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/map/gui/TooltipSlider.java:33)
- [ ] `TooltipSlider.updateMessage(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/map/gui/TooltipSlider.java:37)
- [ ] `TooltipSlider.applyValue(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (xaero/map/gui/TooltipSlider.java:41)

### `GuiUtils`

- [ ] `GuiUtils.getBoundKeyComponent(1)`: it's overloaded with the same number of parameters (xaero/map/gui/util/GuiUtils.java:9)
- [ ] `GuiUtils.getBoundKeyComponent(1)`: it's overloaded with the same number of parameters (xaero/map/gui/util/GuiUtils.java:13)

### `AbstractHighlighter`

- [x] `AbstractHighlighter.AbstractHighlighter(1)` (xaero/map/highlight/AbstractHighlighter.java:12)
- [ ] `AbstractHighlighter.calculateRegionHash(3)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:16)
- [ ] `AbstractHighlighter.regionHasHighlights(3)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:18)
- [ ] `AbstractHighlighter.chunkIsHighlit(3)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:20)
- [ ] `AbstractHighlighter.getChunkHighlitColor(3)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:22)
- [ ] `AbstractHighlighter.getBlockHighlightSubtleTooltip(3)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:24)
- [ ] `AbstractHighlighter.getBlockHighlightBluntTooltip(3)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:26)
- [ ] `AbstractHighlighter.addMinimapBlockHighlightTooltips(5)`: it has no body (xaero/map/highlight/AbstractHighlighter.java:28)
- [x] `AbstractHighlighter.setResult(3)` (xaero/map/highlight/AbstractHighlighter.java:30)
- [x] `AbstractHighlighter.getBlend(2)` (xaero/map/highlight/AbstractHighlighter.java:34)
- [x] `AbstractHighlighter.isCoveringOutsideDiscovered(0)` (xaero/map/highlight/AbstractHighlighter.java:54)

### `ChunkHighlighter`: it extends `AbstractHighlighter`

- [ ] `ChunkHighlighter.ChunkHighlighter(1)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/map/highlight/ChunkHighlighter.java:8)
- [ ] `ChunkHighlighter.getColors(3)`: it has no body (xaero/map/highlight/ChunkHighlighter.java:12)
- [ ] `ChunkHighlighter.getChunkHighlitColor(3)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/map/highlight/ChunkHighlighter.java:14)
- [ ] `ChunkHighlighter.getSideBlend(3)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/map/highlight/ChunkHighlighter.java:49)
- [ ] `ChunkHighlighter.getBlockHighlightBluntTooltip(3)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/map/highlight/ChunkHighlighter.java:53)
- [ ] `ChunkHighlighter.getBlockHighlightSubtleTooltip(3)`: its class isn't translated (it extends `AbstractHighlighter`) (xaero/map/highlight/ChunkHighlighter.java:58)
- [ ] `ChunkHighlighter.getChunkHighlightSubtleTooltip(3)`: it has no body (xaero/map/highlight/ChunkHighlighter.java:63)
- [ ] `ChunkHighlighter.getChunkHighlightBluntTooltip(3)`: it has no body (xaero/map/highlight/ChunkHighlighter.java:65)

### `DimensionHighlighterHandler`: it uses Minecraft's `ResourceKey`

- [ ] `DimensionHighlighterHandler.DimensionHighlighterHandler(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:23)
- [ ] `DimensionHighlighterHandler.getRegionHash(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:30)
- [ ] `DimensionHighlighterHandler.shouldApplyRegionHighlights(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:42)
- [ ] `DimensionHighlighterHandler.shouldApplyTileChunkHighlights(5)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:54)
- [ ] `DimensionHighlighterHandler.shouldApplyTileChunkHighlights(6)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:67)
- [ ] `DimensionHighlighterHandler.shouldApplyTileChunkHighlightsHelp(6)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:75)
- [ ] `DimensionHighlighterHandler.applyChunkHighlightColors(9)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:96)
- [ ] `DimensionHighlighterHandler.recalculateHash(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:176)
- [ ] `DimensionHighlighterHandler.clearCachedHash(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:190)
- [ ] `DimensionHighlighterHandler.clearCachedHashes(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:199)
- [ ] `DimensionHighlighterHandler.getBlockHighlightSubtleTooltip(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:207)
- [ ] `DimensionHighlighterHandler.getBlockHighlightBluntTooltip(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:211)
- [ ] `DimensionHighlighterHandler.getBlockHighlightTooltip(4)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/highlight/DimensionHighlighterHandler.java:215)
- [x] `DimensionHighlighterHandler.getKey(2)` (xaero/map/highlight/DimensionHighlighterHandler.java:249)
- [x] `DimensionHighlighterHandler.getXFromKey(1)` (xaero/map/highlight/DimensionHighlighterHandler.java:253)
- [x] `DimensionHighlighterHandler.getZFromKey(1)` (xaero/map/highlight/DimensionHighlighterHandler.java:257)

### `HighlighterRegistry`

- [ ] `HighlighterRegistry.register(1)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/map/highlight/HighlighterRegistry.java:10)
- [x] `HighlighterRegistry.end(0)` (xaero/map/highlight/HighlighterRegistry.java:14)
- [x] `HighlighterRegistry.getHighlighters(0)` (xaero/map/highlight/HighlighterRegistry.java:18)

### `MapRegionHighlightsPreparer`

- [ ] `MapRegionHighlightsPreparer.prepare(2)`: it uses MapRegion, which isn't translated (it extends `LeveledRegion`) (xaero/map/highlight/MapRegionHighlightsPreparer.java:8)
- [ ] `MapRegionHighlightsPreparer.prepare(5)`: it uses MapRegion, which isn't translated (it extends `LeveledRegion`) (xaero/map/highlight/MapRegionHighlightsPreparer.java:24)
- [ ] `MapRegionHighlightsPreparer.prepare(4)`: it uses MapRegion, which isn't translated (it extends `LeveledRegion`) (xaero/map/highlight/MapRegionHighlightsPreparer.java:47)

### `RegionHighlightExistenceTracker`: it uses `LongSet` (it.unimi.dsi.fastutil.longs)

- [ ] `RegionHighlightExistenceTracker.RegionHighlightExistenceTracker(2)`: its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)) (xaero/map/highlight/RegionHighlightExistenceTracker.java:14)
- [ ] `RegionHighlightExistenceTracker.requestBranchUpdates(2)`: its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)) (xaero/map/highlight/RegionHighlightExistenceTracker.java:20)
- [ ] `RegionHighlightExistenceTracker.onClearCachedHash(2)`: its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)) (xaero/map/highlight/RegionHighlightExistenceTracker.java:32)
- [ ] `RegionHighlightExistenceTracker.onClearCachedHashes(0)`: its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)) (xaero/map/highlight/RegionHighlightExistenceTracker.java:39)
- [ ] `RegionHighlightExistenceTracker.track(2)`: its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)) (xaero/map/highlight/RegionHighlightExistenceTracker.java:45)
- [ ] `RegionHighlightExistenceTracker.stopTracking(2)`: its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)) (xaero/map/highlight/RegionHighlightExistenceTracker.java:49)

### `TestHighlighter`: it extends `ChunkHighlighter`

- [ ] `TestHighlighter.TestHighlighter(0)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:9)
- [ ] `TestHighlighter.regionHasHighlights(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:13)
- [ ] `TestHighlighter.getColors(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:18)
- [ ] `TestHighlighter.calculateRegionHash(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:34)
- [ ] `TestHighlighter.chunkIsHighlit(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:39)
- [ ] `TestHighlighter.getChunkHighlightSubtleTooltip(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:44)
- [ ] `TestHighlighter.getChunkHighlightBluntTooltip(3)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:49)
- [ ] `TestHighlighter.addMinimapBlockHighlightTooltips(5)`: its class isn't translated (it extends `ChunkHighlighter`) (xaero/map/highlight/TestHighlighter.java:54)

### `XaeroIcon`: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)

- [ ] `XaeroIcon.XaeroIcon(3)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/map/icon/XaeroIcon.java:8)
- [ ] `XaeroIcon.getTextureAtlas(0)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/map/icon/XaeroIcon.java:14)
- [ ] `XaeroIcon.getOffsetX(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/map/icon/XaeroIcon.java:18)
- [ ] `XaeroIcon.getOffsetY(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/map/icon/XaeroIcon.java:22)

### `XaeroIconAtlas`: it uses Minecraft's `GpuTexture`

- [ ] `XaeroIconAtlas.XaeroIconAtlas(4)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:19)
- [ ] `XaeroIconAtlas.getTextureId(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:28)
- [ ] `XaeroIconAtlas.getTextureView(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:32)
- [ ] `XaeroIconAtlas.close(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:36)
- [ ] `XaeroIconAtlas.getWidth(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:41)
- [ ] `XaeroIconAtlas.getCurrentIndex(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:45)
- [ ] `XaeroIconAtlas.isFull(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:49)
- [ ] `XaeroIconAtlas.createIcon(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:53)

### `XaeroIconAtlas.Builder`: it uses Minecraft's `GpuTexture`

- [ ] `XaeroIconAtlas.Builder.Builder(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:69)
- [ ] `XaeroIconAtlas.Builder.setDefault(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:72)
- [ ] `XaeroIconAtlas.Builder.setPreparedTexture(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:77)
- [ ] `XaeroIconAtlas.Builder.setWidth(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:82)
- [ ] `XaeroIconAtlas.Builder.setIconWidth(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:87)
- [ ] `XaeroIconAtlas.Builder.createGlTexture(1)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:92)
- [ ] `XaeroIconAtlas.Builder.build(0)`: its class isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:102)
- [ ] `XaeroIconAtlas.Builder.begin(0)`: it uses Builder, which isn't translated (it uses Minecraft's `GpuTexture`) (xaero/map/icon/XaeroIconAtlas.java:121)

### `XaeroIconAtlasManager`: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)

- [ ] `XaeroIconAtlasManager.XaeroIconAtlasManager(3)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/map/icon/XaeroIconAtlasManager.java:11)
- [ ] `XaeroIconAtlasManager.clearAtlases(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/map/icon/XaeroIconAtlasManager.java:18)
- [ ] `XaeroIconAtlasManager.getCurrentAtlas(0)`: its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)) (xaero/map/icon/XaeroIconAtlasManager.java:27)

### `MapFullReloader`: it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)

- [x] `MapFullReloader.CONVERTED_WORLD_SAVE_MW` (xaero/map/MapFullReloader.java:15)
- [ ] `MapFullReloader.MapFullReloader(5)`: its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)) (xaero/map/MapFullReloader.java:24)
- [ ] `MapFullReloader.onRenderProcess(0)`: its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)) (xaero/map/MapFullReloader.java:35)
- [ ] `MapFullReloader.isPartOfReload(1)`: its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)) (xaero/map/MapFullReloader.java:93)
- [ ] `MapFullReloader.isResave(0)`: its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)) (xaero/map/MapFullReloader.java:97)

### `MapLimiter`: it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)

- [x] `MapLimiter.MIN_LIMIT` (xaero/map/MapLimiter.java:12)
- [ ] `MapLimiter.getRegionsFitOnScreen(0)`: its class isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)) (xaero/map/MapLimiter.java:16)
- [ ] `MapLimiter.regionsFitOnScreen(1)`: its class isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)) (xaero/map/MapLimiter.java:20)
- [ ] `MapLimiter.applyLimit(2)`: it uses WorldMapPrimaryClientConfigOptions.MAX_LOADED_REGIONS, which isn't translated (it uses `RangeConfigOption` (xaero.lib.common.config.option)) (xaero/map/MapLimiter.java:24)
- [ ] `MapLimiter.onSessionFinalized(0)`: its class isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)) (xaero/map/MapLimiter.java:81)

### `MapProcessor`: it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))

- [x] `MapProcessor.ROOT_FOLDER_FORMAT` (xaero/map/MapProcessor.java:92)
- [x] `MapProcessor.DEFAULT_LIGHT_LEVELS` (xaero/map/MapProcessor.java:93)
- [x] `MapProcessor.SPAWNPOINT_TIMEOUT` (xaero/map/MapProcessor.java:159)
- [ ] `MapProcessor.MapProcessor(20)`: its body couldn't be read: an anonymous class (xaero/map/MapProcessor.java:202)
- [ ] `MapProcessor.onInit(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:265)
- [ ] `MapProcessor.run(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:272)
- [ ] `MapProcessor.shouldSkipWorldRender(0)`: `instanceof` (xaero/map/MapProcessor.java:331)
- [ ] `MapProcessor.actuallySkippingWorldRender(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:335)
- [ ] `MapProcessor.updateConfigValuesForMultipleThreads(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:340)
- [ ] `MapProcessor.getMaxLoadedRegionsConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:356)
- [ ] `MapProcessor.isReloadViewedConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:360)
- [ ] `MapProcessor.getReloadViewedVersionConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:364)
- [ ] `MapProcessor.isFlowersConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:368)
- [ ] `MapProcessor.isAdjustHeightForCarpetLikeBlocksConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:372)
- [ ] `MapProcessor.onRenderProcess(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:376)
- [ ] `MapProcessor.updateCaveStart(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:566)
- [ ] `MapProcessor.ignoreWorld(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:622)
- [ ] `MapProcessor.getDimensionName(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:632)
- [ ] `MapProcessor.getDimensionIdForFolder(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:651)
- [ ] `MapProcessor.waitForLoadingToFinish(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:681)
- [ ] `MapProcessor.changeWorld(6)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:695)
- [ ] `MapProcessor.updateVisitedDimension(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:727)
- [ ] `MapProcessor.updateDimension(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:731)
- [ ] `MapProcessor.getMainId(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:743)
- [ ] `MapProcessor.getMainId(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:748)
- [ ] `MapProcessor.toggleMultiworldType(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:802)
- [ ] `MapProcessor.quickConfirmMultiworld(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:808)
- [ ] `MapProcessor.confirmMultiworld(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:814)
- [ ] `MapProcessor.setMultiworld(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:827)
- [ ] `MapProcessor.canQuickConfirmUnsynced(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:833)
- [ ] `MapProcessor.getCrosshairMessage(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:839)
- [ ] `MapProcessor.checkForWorldUpdate(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:867)
- [ ] `MapProcessor.updateWorld(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:909)
- [ ] `MapProcessor.updateWorldSynced(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:919)
- [ ] `MapProcessor.updateFootprints(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1170)
- [ ] `MapProcessor.addToRefresh(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1188)
- [ ] `MapProcessor.removeToRefresh(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1200)
- [ ] `MapProcessor.clearToRefresh(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1206)
- [ ] `MapProcessor.handleRefresh(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1212)
- [ ] `MapProcessor.regionExists(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1324)
- [ ] `MapProcessor.regionDetectionExists(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1328)
- [ ] `MapProcessor.removeMapRegion(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1334)
- [ ] `MapProcessor.getLeveledRegion(4)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1346)
- [ ] `MapProcessor.initMinimapRender(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1352)
- [ ] `MapProcessor.beforeMinimapRegionRender(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1356)
- [ ] `MapProcessor.finalizeMinimapRender(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1360)
- [ ] `MapProcessor.getLeafMapRegion(4)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1364)
- [ ] `MapProcessor.getMinimapMapRegion(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1427)
- [ ] `MapProcessor.getMapChunk(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1432)
- [ ] `MapProcessor.getMapTile(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1445)
- [ ] `MapProcessor.updateWorldSpawn(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1456)
- [ ] `MapProcessor.onServerLevelId(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1476)
- [ ] `MapProcessor.onWorldUnload(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1486)
- [ ] `MapProcessor.onClientTickStart(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1499)
- [ ] `MapProcessor.updateRenderStartTime(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1513)
- [ ] `MapProcessor.pushWriterPause(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1519)
- [ ] `MapProcessor.popWriterPause(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1525)
- [ ] `MapProcessor.pushRenderPause(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1531)
- [ ] `MapProcessor.popRenderPause(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1543)
- [ ] `MapProcessor.pushIsLoading(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1555)
- [ ] `MapProcessor.popIsLoading(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1561)
- [ ] `MapProcessor.pushUIPause(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1567)
- [ ] `MapProcessor.popUIPause(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1573)
- [ ] `MapProcessor.isUIPaused(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1579)
- [ ] `MapProcessor.isWritingPaused(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1583)
- [ ] `MapProcessor.isRenderingPaused(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1587)
- [ ] `MapProcessor.isUploadingPaused(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1591)
- [ ] `MapProcessor.isProcessingPaused(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1595)
- [ ] `MapProcessor.isProcessed(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1599)
- [ ] `MapProcessor.addToProcess(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1606)
- [ ] `MapProcessor.removeToProcess(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1613)
- [ ] `MapProcessor.getProcessedCount(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1620)
- [ ] `MapProcessor.getAffectingLoadingFrequencyCount(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1630)
- [ ] `MapProcessor.getMapSaveLoad(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1652)
- [ ] `MapProcessor.getWorld(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1656)
- [ ] `MapProcessor.getNewWorld(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1660)
- [ ] `MapProcessor.getCurrentWorldId(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1664)
- [ ] `MapProcessor.getCurrentDimId(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1668)
- [ ] `MapProcessor.getCurrentMWId(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1672)
- [ ] `MapProcessor.getMapWriter(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1676)
- [ ] `MapProcessor.getMapLimiter(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1680)
- [ ] `MapProcessor.getFootprints(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1684)
- [ ] `MapProcessor.getBufferDeallocator(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1688)
- [ ] `MapProcessor.getTilePool(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1692)
- [ ] `MapProcessor.getOverlayManager(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1696)
- [ ] `MapProcessor.getGlobalVersion(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1700)
- [ ] `MapProcessor.getRenderStartTime(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1704)
- [ ] `MapProcessor.resetRenderStartTime(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1708)
- [ ] `MapProcessor.getRenderStartTimeUpdater(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1719)
- [ ] `MapProcessor.isWaitingForWorldUpdate(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1723)
- [ ] `MapProcessor.getWorldDataHandler(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1727)
- [ ] `MapProcessor.setMainValues(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1731)
- [ ] `MapProcessor.getBrightness(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1777)
- [ ] `MapProcessor.getBrightness(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1782)
- [ ] `MapProcessor.getBrightness(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1786)
- [ ] `MapProcessor.getAmbientBrightness(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1817)
- [x] `MapProcessor.isWorldRealms(1)` (xaero/map/MapProcessor.java:1826)
- [x] `MapProcessor.isWorldMultiplayer(2)` (xaero/map/MapProcessor.java:1830)
- [ ] `MapProcessor.getMapWorld(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1834)
- [ ] `MapProcessor.isCurrentMultiworldWritable(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1838)
- [ ] `MapProcessor.getCurrentDimension(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1842)
- [ ] `MapProcessor.requestCurrentMapDeletion(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1846)
- [ ] `MapProcessor.isFinalizing(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1854)
- [ ] `MapProcessor.stop(0)`: its body couldn't be read: an anonymous class (xaero/map/MapProcessor.java:1858)
- [ ] `MapProcessor.forceClean(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1875)
- [ ] `MapProcessor.isMapWorldUsable(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1902)
- [ ] `MapProcessor.getAutoIdBase(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1906)
- [ ] `MapProcessor.getUsedAutoIdBase(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1912)
- [ ] `MapProcessor.setUsedAutoIdBase(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1917)
- [ ] `MapProcessor.removeUsedAutoIdBase(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1926)
- [ ] `MapProcessor.hasServerLevelId(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1935)
- [ ] `MapProcessor.isEqual(3)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1940)
- [ ] `MapProcessor.isFinished(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1946)
- [ ] `MapProcessor.getMultiTextureRenderTypeRenderers(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1950)
- [ ] `MapProcessor.isCurrentMapLocked(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1954)
- [ ] `MapProcessor.releaseLocksIfNeeded(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1958)
- [ ] `MapProcessor.getCaveLayer(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1985)
- [ ] `MapProcessor.getCurrentCaveLayer(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1989)
- [ ] `MapProcessor.getBlockStateShortShapeCache(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1993)
- [ ] `MapProcessor.getWorldBlockTintProvider(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:1997)
- [ ] `MapProcessor.getHighlighterRegistry(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2001)
- [ ] `MapProcessor.getMapRegionHighlightsPreparer(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2005)
- [ ] `MapProcessor.getMessageBox(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2009)
- [ ] `MapProcessor.getMessageBoxRenderer(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2013)
- [ ] `MapProcessor.getWorldBlockRegistry(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2017)
- [ ] `MapProcessor.getWorldBlockLookup(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2021)
- [ ] `MapProcessor.isConsideringNetherFairPlay(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2025)
- [ ] `MapProcessor.setConsideringNetherFairPlayMessage(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2029)
- [ ] `MapProcessor.getBiomeColorCalculator(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2033)
- [ ] `MapProcessor.getClientSyncedTrackedPlayerManager(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2037)
- [ ] `MapProcessor.serverHasMod(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2041)
- [ ] `MapProcessor.setServerModNetworkVersion(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2046)
- [ ] `MapProcessor.getServerModNetworkVersion(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2053)
- [ ] `MapProcessor.getWorldDimensionTypeRegistry(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2058)
- [ ] `MapProcessor.checkFootstepsReset(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2062)
- [ ] `MapProcessor.fixRootFolder(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2070)
- [ ] `MapProcessor.fixRootFolder(2)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2076)
- [ ] `MapProcessor.fairplayMessageWasReceived(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2098)
- [ ] `MapProcessor.setFairplayMessageReceived(1)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2102)
- [ ] `MapProcessor.updateMapItem(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2106)
- [ ] `MapProcessor.getMapItem(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2132)
- [ ] `MapProcessor.isDebugConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2136)
- [ ] `MapProcessor.isFootstepsConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2140)
- [ ] `MapProcessor.getCaveModeDepthConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2144)
- [ ] `MapProcessor.isIgnoreMapFileLocksConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2148)
- [ ] `MapProcessor.isForceFastWritingConfig(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2152)
- [ ] `MapProcessor.getSaveTime(0)`: its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/MapProcessor.java:2156)

### `MapRunner`: it implements `Runnable`

- [ ] `MapRunner.run(0)`: its class isn't translated (it implements `Runnable`) (xaero/map/MapRunner.java:11)
- [ ] `MapRunner.doTasks(1)`: its body couldn't be read: a labelled statement (xaero/map/MapRunner.java:29)
- [ ] `MapRunner.addTask(1)`: its class isn't translated (it implements `Runnable`) (xaero/map/MapRunner.java:49)
- [ ] `MapRunner.stop(0)`: its class isn't translated (it implements `Runnable`) (xaero/map/MapRunner.java:55)

### `MapWriter`: it uses MapBlock, which isn't translated (it extends `MapPixel`)

- [x] `MapWriter.NO_Y_VALUE` (xaero/map/MapWriter.java:71)
- [x] `MapWriter.MAX_TRANSPARENCY_BLEND_DEPTH` (xaero/map/MapWriter.java:72)
- [x] `MapWriter.DEFAULT_RESOURCE` (xaero/map/MapWriter.java:73)
- [ ] `MapWriter.MapWriter(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:117)
- [ ] `MapWriter.onRender(2)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:131)
- [ ] `MapWriter.getWriteDistance(0)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:361)
- [ ] `MapWriter.writeMap(17)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:373)
- [ ] `MapWriter.writeChunk(23)`: its body couldn't be read: a labelled statement (xaero/map/MapWriter.java:459)
- [ ] `MapWriter.updateBottomRightTile(5)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:801)
- [ ] `MapWriter.getSectionBasedHeight(2)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:837)
- [ ] `MapWriter.isGlowing(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:871)
- [ ] `MapWriter.shouldOverlayCached(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:875)
- [ ] `MapWriter.shouldOverlay(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:879)
- [ ] `MapWriter.isInvisible(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:894)
- [ ] `MapWriter.hasVanillaColor(4)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:928)
- [ ] `MapWriter.unpackFramedBlocks(3)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:944)
- [ ] `MapWriter.loadPixel(18)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:963)
- [ ] `MapWriter.loadPixelHelp(19)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1143)
- [ ] `MapWriter.getQuads(5)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1224)
- [ ] `MapWriter.getParticleIcon(5)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1229)
- [ ] `MapWriter.loadBlockColourFromTexture(5)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1234)
- [ ] `MapWriter.getUpdateCounter(0)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1258)
- [ ] `MapWriter.resetPosition(0)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1262)
- [ ] `MapWriter.requestCachedColoursClear(0)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1269)
- [ ] `MapWriter.setMapProcessor(1)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1274)
- [ ] `MapWriter.getMapProcessor(0)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1278)
- [ ] `MapWriter.setDirtyInWriteDistance(2)`: its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)) (xaero/map/MapWriter.java:1282)

### `MapWriterFabric`: it extends `MapWriter`

- [ ] `MapWriterFabric.MapWriterFabric(3)`: its class isn't translated (it extends `MapWriter`) (xaero/map/MapWriterFabric.java:8)

### `IWorldMapClientWorld`: it's an interface

- [ ] `IWorldMapClientWorld.getXaero_worldmapData(0)`: it has no body (xaero/map/mcworld/IWorldMapClientWorld.java:4)
- [ ] `IWorldMapClientWorld.setXaero_worldmapData(1)`: it has no body (xaero/map/mcworld/IWorldMapClientWorld.java:6)

### `WorldMapClientWorldData`: it uses Minecraft's `BlockPos`

- [ ] `WorldMapClientWorldData.WorldMapClientWorldData(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldData.java:15)
- [ ] `WorldMapClientWorldData.setServerModNetworkVersion(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldData.java:18)
- [ ] `WorldMapClientWorldData.getServerModNetworkVersion(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldData.java:22)
- [ ] `WorldMapClientWorldData.setSyncedRules(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldData.java:26)
- [ ] `WorldMapClientWorldData.getSyncedRules(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldData.java:30)

### `WorldMapClientWorldDataHelper`

- [ ] `WorldMapClientWorldDataHelper.getCurrentWorldData(0)`: it uses WorldMapClientWorldData, which isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldDataHelper.java:7)
- [ ] `WorldMapClientWorldDataHelper.getWorldData(1)`: it uses WorldMapClientWorldData, which isn't translated (it uses Minecraft's `BlockPos`) (xaero/map/mcworld/WorldMapClientWorldDataHelper.java:11)

### `ClientboundRulesPacket`

- [x] `ClientboundRulesPacket.ClientboundRulesPacket(2)` (xaero/map/message/basic/ClientboundRulesPacket.java:12)
- [ ] `ClientboundRulesPacket.write(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/map/message/basic/ClientboundRulesPacket.java:17)
- [ ] `ClientboundRulesPacket.read(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/map/message/basic/ClientboundRulesPacket.java:24)

### `ClientboundRulesPacket.ClientHandler`: it implements `Consumer` (java.util.function)

- [ ] `ClientboundRulesPacket.ClientHandler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/map/message/basic/ClientboundRulesPacket.java:30)

### `HandshakePacket`

- [x] `HandshakePacket.NETWORK_COMPATIBILITY` (xaero/map/message/basic/HandshakePacket.java:12)
- [x] `HandshakePacket.HandshakePacket(1)` (xaero/map/message/basic/HandshakePacket.java:15)
- [x] `HandshakePacket.HandshakePacket(0)` (xaero/map/message/basic/HandshakePacket.java:19)
- [ ] `HandshakePacket.write(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/map/message/basic/HandshakePacket.java:23)
- [ ] `HandshakePacket.read(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/map/message/basic/HandshakePacket.java:27)

### `HandshakePacket.ClientHandler`: it implements `Consumer` (java.util.function)

- [ ] `HandshakePacket.ClientHandler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/map/message/basic/HandshakePacket.java:32)

### `HandshakePacket.ServerHandler`: it implements `BiConsumer` (java.util.function)

- [ ] `HandshakePacket.ServerHandler.accept(2)`: its class isn't translated (it implements `BiConsumer` (java.util.function)) (xaero/map/message/basic/HandshakePacket.java:42)

### `LevelMapPropertiesConsumer`: it implements `Consumer` (java.util.function)

- [ ] `LevelMapPropertiesConsumer.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/map/message/LevelMapPropertiesConsumer.java:8)

### `ClientboundPlayerTrackerResetPacket`

- [ ] `ClientboundPlayerTrackerResetPacket.write(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/map/message/tracker/ClientboundPlayerTrackerResetPacket.java:8)
- [ ] `ClientboundPlayerTrackerResetPacket.read(1)`: it uses Minecraft's `FriendlyByteBuf` (xaero/map/message/tracker/ClientboundPlayerTrackerResetPacket.java:11)

### `ClientboundPlayerTrackerResetPacket.Handler`: it implements `Consumer` (java.util.function)

- [ ] `ClientboundPlayerTrackerResetPacket.Handler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/map/message/tracker/ClientboundPlayerTrackerResetPacket.java:16)

### `ClientboundTrackedPlayerPacket`: it uses `UUID` (java.util)

- [ ] `ClientboundTrackedPlayerPacket.ClientboundTrackedPlayerPacket(7)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/message/tracker/ClientboundTrackedPlayerPacket.java:23)
- [ ] `ClientboundTrackedPlayerPacket.write(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/message/tracker/ClientboundTrackedPlayerPacket.java:33)
- [ ] `ClientboundTrackedPlayerPacket.read(1)`: it uses ClientboundTrackedPlayerPacket, which isn't translated (it uses `UUID` (java.util)) (xaero/map/message/tracker/ClientboundTrackedPlayerPacket.java:52)

### `ClientboundTrackedPlayerPacket.Handler`: it implements `Consumer` (java.util.function)

- [ ] `ClientboundTrackedPlayerPacket.Handler.accept(1)`: its class isn't translated (it implements `Consumer` (java.util.function)) (xaero/map/message/tracker/ClientboundTrackedPlayerPacket.java:65)

### `WorldMapMessageRegister`

- [ ] `WorldMapMessageRegister.register(1)`: it uses `IPacketHandler` (xaero.lib.common.packet) (xaero/map/message/WorldMapMessageRegister.java:11)

### `MinimapRenderListener`: it uses MapRegion, which isn't translated (it extends `LeveledRegion`)

- [ ] `MinimapRenderListener.init(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:26)
- [ ] `MinimapRenderListener.beforeMinimapRender(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:47)
- [ ] `MinimapRenderListener.finalize(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:81)
- [ ] `MinimapRenderListener.getRenderedCaveLayer(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:113)
- [ ] `MinimapRenderListener.shouldRequestLoading(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:117)
- [ ] `MinimapRenderListener.isPlayerMoving(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:121)
- [ ] `MinimapRenderListener.isCacheOnlyMode(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:125)
- [ ] `MinimapRenderListener.getGlobalRegionCacheHashCode(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:129)
- [ ] `MinimapRenderListener.isReloadEverything(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:133)
- [ ] `MinimapRenderListener.getGlobalVersion(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:137)
- [ ] `MinimapRenderListener.getGlobalReloadVersion(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:141)
- [ ] `MinimapRenderListener.getGlobalCaveStart(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:145)
- [ ] `MinimapRenderListener.getGlobalCaveDepth(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/minimap/MinimapRenderListener.java:149)

### `Area`

- [x] `Area.Area(4)` (xaero/map/misc/Area.java:9)
- [x] `Area.getLeft(0)` (xaero/map/misc/Area.java:16)
- [x] `Area.getTop(0)` (xaero/map/misc/Area.java:20)
- [x] `Area.getRight(0)` (xaero/map/misc/Area.java:24)
- [x] `Area.getBottom(0)` (xaero/map/misc/Area.java:28)
- [x] `Area.hashCode(0)` (xaero/map/misc/Area.java:32)
- [ ] `Area.equals(1)`: it uses `Object` (xaero/map/misc/Area.java:43)

### `CachedFunction`: it's generic

- [ ] `CachedFunction.CachedFunction(1)`: its class isn't translated (it's generic) (xaero/map/misc/CachedFunction.java:12)
- [ ] `CachedFunction.apply(1)`: its class isn't translated (it's generic) (xaero/map/misc/CachedFunction.java:16)
- [ ] `CachedFunction.clear(0)`: its class isn't translated (it's generic) (xaero/map/misc/CachedFunction.java:32)

### `CaveStartCalculator`: it uses Minecraft's `MutableBlockPos`

- [ ] `CaveStartCalculator.CaveStartCalculator(1)`: its class isn't translated (it uses Minecraft's `MutableBlockPos`) (xaero/map/misc/CaveStartCalculator.java:26)
- [ ] `CaveStartCalculator.getCaving(4)`: its body couldn't be read: a labelled statement (xaero/map/misc/CaveStartCalculator.java:31)

### `ConsistentBitArray`

- [x] `ConsistentBitArray.ConsistentBitArray(2)` (xaero/map/misc/ConsistentBitArray.java:13)
- [x] `ConsistentBitArray.ConsistentBitArray(3)` (xaero/map/misc/ConsistentBitArray.java:17)
- [x] `ConsistentBitArray.get(1)` (xaero/map/misc/ConsistentBitArray.java:39)
- [x] `ConsistentBitArray.set(2)` (xaero/map/misc/ConsistentBitArray.java:49)
- [ ] `ConsistentBitArray.write(1)`: it uses `DataOutputStream` (java.io) (xaero/map/misc/ConsistentBitArray.java:63)
- [x] `ConsistentBitArray.getData(0)` (xaero/map/misc/ConsistentBitArray.java:69)
- [x] `ConsistentBitArray.setData(1)` (xaero/map/misc/ConsistentBitArray.java:73)
- [x] `ConsistentBitArray.getBitsPerEntry(0)` (xaero/map/misc/ConsistentBitArray.java:77)

### `Internet`

- [ ] `Internet.cipher`: it uses `Cipher` (javax.crypto) (xaero/map/misc/Internet.java:21)
- [ ] `Internet.checkModVersion(0)`: it uses `XaeroLib` (xaero.lib).INSTANCE (xaero/map/misc/Internet.java:23)

### `Misc`

- [x] `Misc.ZERO_LONG_1024` (xaero/map/misc/Misc.java:33)
- [x] `Misc.cpuTimerPreTime` (xaero/map/misc/Misc.java:34)
- [x] `Misc.glTimerPreTime` (xaero/map/misc/Misc.java:35)
- [x] `Misc.OUTDATED_FILE_EXT` (xaero/map/misc/Misc.java:36)
- [ ] `Misc.getStateById(1)`: it uses Minecraft's `BlockState` (xaero/map/misc/Misc.java:38)
- [ ] `Misc.getDefaultBlockStateForStateId(1)`: it uses Minecraft's `BlockState` (xaero/map/misc/Misc.java:46)
- [x] `Misc.timerPre(0)` (xaero/map/misc/Misc.java:54)
- [x] `Misc.timerResult(0)` (xaero/map/misc/Misc.java:58)
- [ ] `Misc.getMouseX(2)`: it uses Minecraft's Minecraft.mouseHandler (no Horizon helper for it yet) (xaero/map/misc/Misc.java:62)
- [ ] `Misc.getMouseY(2)`: it uses Minecraft's Minecraft.mouseHandler (no Horizon helper for it yet) (xaero/map/misc/Misc.java:66)
- [ ] `Misc.minecraftOrtho(2)`: it uses Minecraft's `Window` (xaero/map/misc/Misc.java:70)
- [x] `Misc.clearHeightsData1024(1)` (xaero/map/misc/Misc.java:84)
- [ ] `Misc.addToListOfSmallest(3)`: it's generic (xaero/map/misc/Misc.java:88)
- [ ] `Misc.convertToOutdated(2)`: it calls Misc.convertToOutdated, which isn't translated (xaero/map/misc/Misc.java:105)
- [ ] `Misc.convertToOutdated(3)`: a catch block that uses its exception (xaero/map/misc/Misc.java:118)
- [ ] `Misc.hasItem(2)`: it calls Minecraft's Inventory.getContainerSize (no Horizon helper for it yet) (xaero/map/misc/Misc.java:139)
- [ ] `Misc.hasItem(3)`: it uses Minecraft's `NonNullList` (xaero/map/misc/Misc.java:157)
- [ ] `Misc.getTeamColour(1)`: it uses Minecraft's `Entity` (xaero/map/misc/Misc.java:167)
- [ ] `Misc.hasEffect(2)`: it uses Minecraft's `Holder` (xaero/map/misc/Misc.java:177)
- [ ] `Misc.hasEffect(1)`: it uses Minecraft's `Holder` (xaero/map/misc/Misc.java:181)

### `OptimizedMath`

- [x] `OptimizedMath.XP` (xaero/map/misc/OptimizedMath.java:9)
- [x] `OptimizedMath.YP` (xaero/map/misc/OptimizedMath.java:10)
- [x] `OptimizedMath.ZP` (xaero/map/misc/OptimizedMath.java:11)
- [x] `OptimizedMath.myFloor(1)` (xaero/map/misc/OptimizedMath.java:13)
- [ ] `OptimizedMath.rotatePose(3)`: it uses Minecraft's `Pose` (xaero/map/misc/OptimizedMath.java:22)
- [ ] `MixinChatListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:13)
- [ ] `MixinChunkDeltaUpdateS2CPacketAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:10)
- [ ] `MixinClientPlayNetworkHandler`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:21)
- [ ] `MixinClientWorld`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:9)
- [ ] `MixinFabricAbstractClientPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:7)
- [ ] `MixinFabricChatListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:16)
- [ ] `MixinFabricClientPacketListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:13)
- [ ] `MixinFabricGameRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:12)
- [ ] `MixinFabricGui`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:15)
- [ ] `MixinFabricMinecraftClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:18)
- [ ] `MixinFabricMinecraftServer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:14)
- [ ] `MixinFabricPlayerClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:7)
- [ ] `MixinFabricPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:11)
- [ ] `MixinFabricPlayerList`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:14)
- [ ] `MixinFabricSpriteAtlasTexture`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:12)
- [ ] `MixinFabricWorld`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:12)
- [ ] `MixinForgeClientPacketListener`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:13)
- [ ] `MixinForgeMinecraftClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:12)
- [ ] `MixinGlBuffer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:9)
- [ ] `MixinGuiRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:11)
- [ ] `MixinLevelStorageAccess`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:11)
- [ ] `MixinMinecraftClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:12)
- [ ] `MixinMinecraftServer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:9)
- [ ] `MixinOptionalGui`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:13)
- [ ] `MixinPlayerList`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:13)
- [ ] `MixinServerPlayer`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:9)
- [ ] `MixinServerWorld`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:9)
- [ ] `MixinWorldChunk`: a Mixin class: its handlers are the injections, in the bundle's helpers (xaero/map/misc/OptimizedMath.java:7)

### `Waypoint`: it implements `Comparable`

- [x] `Waypoint.minimapTextures` (xaero/map/mods/gui/Waypoint.java:10)
- [x] `Waypoint.white` (xaero/map/mods/gui/Waypoint.java:11)
- [ ] `Waypoint.Waypoint(4)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:20)
- [ ] `Waypoint.getName(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:28)
- [ ] `Waypoint.compareTo(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:32)
- [ ] `Waypoint.toString(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:38)
- [ ] `Waypoint.getX(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:43)
- [ ] `Waypoint.getY(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:47)
- [ ] `Waypoint.getZ(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:51)
- [ ] `Waypoint.isDisabled(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:55)
- [ ] `Waypoint.setDisabled(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:59)
- [ ] `Waypoint.getType(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:63)
- [ ] `Waypoint.getPurpose(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:68)
- [ ] `Waypoint.getYaw(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:72)
- [ ] `Waypoint.setYaw(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:76)
- [ ] `Waypoint.isRotation(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:80)
- [ ] `Waypoint.setRotation(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:84)
- [ ] `Waypoint.isEditable(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:88)
- [ ] `Waypoint.getOriginal(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:92)
- [ ] `Waypoint.getSymbol(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:96)
- [ ] `Waypoint.setTemporary(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:100)
- [ ] `Waypoint.setGlobal(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:104)
- [ ] `Waypoint.getSetName(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:108)
- [ ] `Waypoint.getComparisonName(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:112)
- [ ] `Waypoint.getColor(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:125)
- [ ] `Waypoint.isGlobal(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:129)
- [ ] `Waypoint.getRenderX(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:133)
- [ ] `Waypoint.getRenderZ(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:138)
- [ ] `Waypoint.isTemporary(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:143)
- [ ] `Waypoint.getDestAlpha(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:147)
- [ ] `Waypoint.setDestAlpha(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:151)
- [ ] `Waypoint.getAlphaAnim(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:155)
- [ ] `Waypoint.setAlphaAnim(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:159)
- [ ] `Waypoint.getAlpha(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:163)
- [ ] `Waypoint.setAlpha(1)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:167)
- [ ] `Waypoint.isyIncluded(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:171)
- [ ] `Waypoint.getCachedNameLength(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:175)
- [ ] `Waypoint.getThirdPartyOrigin(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:179)
- [ ] `Waypoint.isThirdParty(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:183)
- [ ] `Waypoint.isThirdPartyDeleted(0)`: its class isn't translated (it implements `Comparable`) (xaero/map/mods/gui/Waypoint.java:187)

### `WaypointMenuRenderContext`


### `WaypointMenuRenderer`: it extends `MapElementMenuRenderer` (xaero.map.element)

- [ ] `WaypointMenuRenderer.WaypointMenuRenderer(3)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:47)
- [ ] `WaypointMenuRenderer.onMapInit(7)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:52)
- [ ] `WaypointMenuRenderer.onFullMenuButton(2)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:209)
- [ ] `WaypointMenuRenderer.onRenderAllSetsButton(4)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:213)
- [ ] `WaypointMenuRenderer.onShowDisabledButton(4)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:218)
- [ ] `WaypointMenuRenderer.onCloseMenuWhenHoppingButton(4)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:224)
- [ ] `WaypointMenuRenderer.onCurrentMapWaypointsButton(4)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:230)
- [ ] `WaypointMenuRenderer.renderInMenu(11)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:236)
- [ ] `WaypointMenuRenderer.menuStartPos(1)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:281)
- [ ] `WaypointMenuRenderer.menuSearchPadding(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:286)
- [ ] `WaypointMenuRenderer.getFilterPlaceholder(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:291)
- [ ] `WaypointMenuRenderer.getRenderer(1)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:296)
- [ ] `WaypointMenuRenderer.beforeFiltering(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:300)
- [ ] `WaypointMenuRenderer.beforeMenuRender(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:304)
- [ ] `WaypointMenuRenderer.afterMenuRender(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/mods/gui/WaypointMenuRenderer.java:308)

### `WaypointMenuRenderProvider`: it extends `ElementRenderProvider` (xaero.map.element.render)

- [ ] `WaypointMenuRenderProvider.WaypointMenuRenderProvider(1)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointMenuRenderProvider.java:13)
- [ ] `WaypointMenuRenderProvider.begin(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointMenuRenderProvider.java:17)
- [ ] `WaypointMenuRenderProvider.hasNext(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointMenuRenderProvider.java:26)
- [ ] `WaypointMenuRenderProvider.getNext(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointMenuRenderProvider.java:30)
- [ ] `WaypointMenuRenderProvider.end(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointMenuRenderProvider.java:34)

### `WaypointReader`: it extends `ElementReader` (xaero.map.element.render)

- [ ] `WaypointReader.waypointIsGood(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:20)
- [ ] `WaypointReader.isHidden(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:24)
- [ ] `WaypointReader.isInteractable(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:28)
- [ ] `WaypointReader.getBoxScale(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:32)
- [ ] `WaypointReader.getRenderX(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:36)
- [ ] `WaypointReader.getRenderY(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:40)
- [ ] `WaypointReader.hasYCoordinate(0)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:44)
- [ ] `WaypointReader.getRenderZ(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:49)
- [ ] `WaypointReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:53)
- [ ] `WaypointReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:57)
- [ ] `WaypointReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:61)
- [ ] `WaypointReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:65)
- [ ] `WaypointReader.getLeftSideLength(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:69)
- [ ] `WaypointReader.getMenuName(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:73)
- [ ] `WaypointReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:82)
- [ ] `WaypointReader.getFilterName(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:86)
- [ ] `WaypointReader.getRightClickOptions(2)`: its body couldn't be read: an anonymous class (xaero/map/mods/gui/WaypointReader.java:90)
- [ ] `WaypointReader.isRightClickValid(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:176)
- [ ] `WaypointReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:180)
- [ ] `WaypointReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:184)
- [ ] `WaypointReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:189)
- [ ] `WaypointReader.getRenderBoxRight(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:194)
- [ ] `WaypointReader.getRenderBoxTop(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:199)
- [ ] `WaypointReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointReader.java:203)

### `WaypointRenderContext`: it uses Minecraft's `VertexConsumer`


### `WaypointRenderer`: it extends `ElementRenderer` (xaero.map.element.render)

- [ ] `WaypointRenderer.WaypointRenderer(5)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:34)
- [ ] `WaypointRenderer.getSymbolCreator(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:42)
- [ ] `WaypointRenderer.renderElementShadow(9)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:46)
- [ ] `WaypointRenderer.shouldRender(2)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:78)
- [ ] `WaypointRenderer.renderElement(10)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:86)
- [ ] `WaypointRenderer.preRender(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:222)
- [ ] `WaypointRenderer.postRender(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:239)
- [ ] `WaypointRenderer.getOrder(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:248)
- [ ] `WaypointRenderer.shouldBeDimScaled(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:253)

### `WaypointRenderer.Builder`: it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common))

- [ ] `WaypointRenderer.Builder.Builder(0)`: its class isn't translated (it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common))) (xaero/map/mods/gui/WaypointRenderer.java:262)
- [ ] `WaypointRenderer.Builder.setDefault(0)`: `this` used as a value (xaero/map/mods/gui/WaypointRenderer.java:265)
- [ ] `WaypointRenderer.Builder.setMinimap(1)`: `this` used as a value (xaero/map/mods/gui/WaypointRenderer.java:271)
- [ ] `WaypointRenderer.Builder.setSymbolCreator(1)`: `this` used as a value (xaero/map/mods/gui/WaypointRenderer.java:276)
- [ ] `WaypointRenderer.Builder.build(0)`: it uses WaypointRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderer.java:281)
- [ ] `WaypointRenderer.Builder.begin(0)`: it uses Builder, which isn't translated (it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common))) (xaero/map/mods/gui/WaypointRenderer.java:291)

### `WaypointRenderProvider`: it extends `ElementRenderProvider` (xaero.map.element.render)

- [ ] `WaypointRenderProvider.WaypointRenderProvider(1)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderProvider.java:19)
- [ ] `WaypointRenderProvider.begin(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderProvider.java:23)
- [ ] `WaypointRenderProvider.hasNext(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderProvider.java:37)
- [ ] `WaypointRenderProvider.getNext(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderProvider.java:41)
- [ ] `WaypointRenderProvider.end(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/gui/WaypointRenderProvider.java:53)

### `WaypointSymbolCreator`: it uses Minecraft's `Minecraft`

- [x] `WaypointSymbolCreator.PREFERRED_ATLAS_WIDTH` (xaero/map/mods/gui/WaypointSymbolCreator.java:28)
- [x] `WaypointSymbolCreator.ICON_WIDTH` (xaero/map/mods/gui/WaypointSymbolCreator.java:29)
- [x] `WaypointSymbolCreator.minimapTextures` (xaero/map/mods/gui/WaypointSymbolCreator.java:30)
- [x] `WaypointSymbolCreator.white` (xaero/map/mods/gui/WaypointSymbolCreator.java:31)
- [ ] `WaypointSymbolCreator.getDeathSymbolTexture(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (xaero/map/mods/gui/WaypointSymbolCreator.java:41)
- [ ] `WaypointSymbolCreator.createDeathSymbolTexture(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (xaero/map/mods/gui/WaypointSymbolCreator.java:49)
- [ ] `WaypointSymbolCreator.getSymbolTexture(2)`: its class isn't translated (it uses Minecraft's `Minecraft`) (xaero/map/mods/gui/WaypointSymbolCreator.java:53)
- [ ] `WaypointSymbolCreator.createCharSymbol(3)`: its class isn't translated (it uses Minecraft's `Minecraft`) (xaero/map/mods/gui/WaypointSymbolCreator.java:66)
- [ ] `WaypointSymbolCreator.resetChars(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (xaero/map/mods/gui/WaypointSymbolCreator.java:127)

### `MinimapElementGraphicsWrapper`: it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)

- [ ] `MinimapElementGraphicsWrapper.MinimapElementGraphicsWrapper(0)`: its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)) (xaero/map/mods/minimap/element/MinimapElementGraphicsWrapper.java:11)
- [ ] `MinimapElementGraphicsWrapper.setGraphics(1)`: its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)) (xaero/map/mods/minimap/element/MinimapElementGraphicsWrapper.java:15)
- [ ] `MinimapElementGraphicsWrapper.pose(0)`: its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)) (xaero/map/mods/minimap/element/MinimapElementGraphicsWrapper.java:20)
- [ ] `MinimapElementGraphicsWrapper.getBufferSource(0)`: its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)) (xaero/map/mods/minimap/element/MinimapElementGraphicsWrapper.java:24)

### `MinimapElementReaderWrapper`: it's generic

- [ ] `MinimapElementReaderWrapper.MinimapElementReaderWrapper(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:11)
- [ ] `MinimapElementReaderWrapper.isHidden(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:15)
- [ ] `MinimapElementReaderWrapper.getBoxScale(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:20)
- [ ] `MinimapElementReaderWrapper.getRenderX(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:25)
- [ ] `MinimapElementReaderWrapper.getRenderZ(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:30)
- [ ] `MinimapElementReaderWrapper.getInteractionBoxLeft(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:35)
- [ ] `MinimapElementReaderWrapper.getInteractionBoxRight(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:40)
- [ ] `MinimapElementReaderWrapper.getInteractionBoxTop(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:45)
- [ ] `MinimapElementReaderWrapper.getInteractionBoxBottom(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:50)
- [ ] `MinimapElementReaderWrapper.getLeftSideLength(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:55)
- [ ] `MinimapElementReaderWrapper.getMenuName(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:60)
- [ ] `MinimapElementReaderWrapper.getFilterName(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:65)
- [ ] `MinimapElementReaderWrapper.getMenuTextFillLeftPadding(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:70)
- [ ] `MinimapElementReaderWrapper.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:75)
- [ ] `MinimapElementReaderWrapper.getRenderBoxLeft(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:80)
- [ ] `MinimapElementReaderWrapper.getRenderBoxRight(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:85)
- [ ] `MinimapElementReaderWrapper.getRenderBoxTop(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:90)
- [ ] `MinimapElementReaderWrapper.getRenderBoxBottom(3)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:95)
- [ ] `MinimapElementReaderWrapper.isInteractable(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:100)
- [ ] `MinimapElementReaderWrapper.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java:105)

### `MinimapElementRendererWrapper`: it's generic

- [ ] `MinimapElementRendererWrapper.MinimapElementRendererWrapper(7)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:25)
- [ ] `MinimapElementRendererWrapper.getRenderLocation(1)`: it uses `MinimapElementRenderLocation` (xaero.hud.minimap.element.render) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:41)
- [ ] `MinimapElementRendererWrapper.convertRenderInfo(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:55)
- [ ] `MinimapElementRendererWrapper.preRender(4)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:69)
- [ ] `MinimapElementRendererWrapper.postRender(4)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:79)
- [ ] `MinimapElementRendererWrapper.renderElement(10)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:89)
- [ ] `MinimapElementRendererWrapper.renderElementShadow(9)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:117)
- [ ] `MinimapElementRendererWrapper.shouldRender(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:131)
- [ ] `MinimapElementRendererWrapper.getOrder(0)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:136)

### `MinimapElementRendererWrapper.Builder`: it's generic

- [ ] `MinimapElementRendererWrapper.Builder.Builder(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:147)
- [ ] `MinimapElementRendererWrapper.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:151)
- [ ] `MinimapElementRendererWrapper.Builder.setModMain(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:158)
- [ ] `MinimapElementRendererWrapper.Builder.setShouldRenderSupplier(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:163)
- [ ] `MinimapElementRendererWrapper.Builder.setOrder(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:168)
- [ ] `MinimapElementRendererWrapper.Builder.build(0)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:173)
- [ ] `MinimapElementRendererWrapper.Builder.begin(1)`: it's generic (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java:186)

### `MinimapElementRenderProviderWrapper`: it's generic

- [ ] `MinimapElementRenderProviderWrapper.MinimapElementRenderProviderWrapper(1)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java:10)
- [ ] `MinimapElementRenderProviderWrapper.begin(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java:14)
- [ ] `MinimapElementRenderProviderWrapper.hasNext(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java:19)
- [ ] `MinimapElementRenderProviderWrapper.setupContextAndGetNext(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java:24)
- [ ] `MinimapElementRenderProviderWrapper.getNext(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java:29)
- [ ] `MinimapElementRenderProviderWrapper.end(2)`: its class isn't translated (it's generic) (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java:34)

### `RadarRendererWrapperHelper`

- [ ] `RadarRendererWrapperHelper.createWrapper(2)`: it uses `IXaeroMinimap` (xaero.common) (xaero/map/mods/minimap/element/RadarRendererWrapperHelper.java:10)

### `MinimapSyncedPlayerTrackerSystem`: it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)

- [ ] `MinimapSyncedPlayerTrackerSystem.MinimapSyncedPlayerTrackerSystem(1)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedPlayerTrackerSystem.java:17)
- [ ] `MinimapSyncedPlayerTrackerSystem.getReader(0)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedPlayerTrackerSystem.java:21)
- [ ] `MinimapSyncedPlayerTrackerSystem.getTrackedPlayerIterator(0)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedPlayerTrackerSystem.java:26)

### `MinimapSyncedTrackedPlayerReader`: it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)

- [ ] `MinimapSyncedTrackedPlayerReader.MinimapSyncedTrackedPlayerReader(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java:13)
- [ ] `MinimapSyncedTrackedPlayerReader.getId(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java:17)
- [ ] `MinimapSyncedTrackedPlayerReader.getX(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java:21)
- [ ] `MinimapSyncedTrackedPlayerReader.getY(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java:25)
- [ ] `MinimapSyncedTrackedPlayerReader.getZ(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java:29)
- [ ] `MinimapSyncedTrackedPlayerReader.getDimension(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java:33)

### `ClientClaimChangeListener`: it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)

- [ ] `ClientClaimChangeListener.onWholeRegionChange(3)`: its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)) (xaero/map/mods/pac/ClientClaimChangeListener.java:12)
- [ ] `ClientClaimChangeListener.onChunkChange(4)`: its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)) (xaero/map/mods/pac/ClientClaimChangeListener.java:26)
- [ ] `ClientClaimChangeListener.onDimensionChange(1)`: its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)) (xaero/map/mods/pac/ClientClaimChangeListener.java:40)

### `ClaimResultElement`: it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)

- [ ] `ClaimResultElement.ClaimResultElement(8)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:25)
- [ ] `ClaimResultElement.getKey(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:45)
- [ ] `ClaimResultElement.getFadeOutStartTime(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:49)
- [ ] `ClaimResultElement.getCreationTime(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:53)
- [ ] `ClaimResultElement.setFadeOutStartTime(1)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:57)
- [ ] `ClaimResultElement.getDimension(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:61)
- [ ] `ClaimResultElement.getLeft(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:65)
- [ ] `ClaimResultElement.getTop(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:69)
- [ ] `ClaimResultElement.getRight(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:73)
- [ ] `ClaimResultElement.getBottom(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:77)
- [ ] `ClaimResultElement.getTooltip(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:81)
- [ ] `ClaimResultElement.hasNegative(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:85)
- [ ] `ClaimResultElement.hasPositive(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:89)
- [ ] `ClaimResultElement.getFilteredResultTypeIterator(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:93)

### `ClaimResultElement.Builder`: it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)

- [ ] `ClaimResultElement.Builder.Builder(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:101)
- [ ] `ClaimResultElement.Builder.setDefault(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:104)
- [ ] `ClaimResultElement.Builder.setKey(1)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:110)
- [ ] `ClaimResultElement.Builder.setResult(1)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:115)
- [ ] `ClaimResultElement.Builder.build(0)`: its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:120)
- [ ] `ClaimResultElement.Builder.begin(0)`: it uses Builder, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)) (xaero/map/mods/pac/gui/claim/ClaimResultElement.java:174)

### `ClaimResultElementManager`: it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))

- [ ] `ClaimResultElementManager.ClaimResultElementManager(1)`: its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:13)
- [x] `ClaimResultElementManager.getLongCoordinatesFor(2)` (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:17)
- [x] `ClaimResultElementManager.getXFromLongCoordinates(1)` (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:21)
- [x] `ClaimResultElementManager.getZFromLongCoordinates(1)` (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:25)
- [ ] `ClaimResultElementManager.add(1)`: its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:29)
- [ ] `ClaimResultElementManager.remove(1)`: its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:36)
- [ ] `ClaimResultElementManager.getIterator(0)`: its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:40)
- [ ] `ClaimResultElementManager.clear(0)`: its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:44)

### `ClaimResultElementManager.Builder`: it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api))

- [ ] `ClaimResultElementManager.Builder.Builder(0)`: its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:51)
- [ ] `ClaimResultElementManager.Builder.setDefault(0)`: `this` used as a value (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:54)
- [ ] `ClaimResultElementManager.Builder.setPac(1)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:58)
- [ ] `ClaimResultElementManager.Builder.build(0)`: it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:63)
- [ ] `ClaimResultElementManager.Builder.begin(0)`: it uses Builder, which isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api))) (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java:71)

### `ClaimResultElementRenderContext`: it uses Minecraft's `VertexConsumer`


### `ClaimResultElementRenderer`: it extends `ElementRenderer` (xaero.map.element.render)

- [ ] `ClaimResultElementRenderer.ClaimResultElementRenderer(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:19)
- [ ] `ClaimResultElementRenderer.preRender(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:29)
- [ ] `ClaimResultElementRenderer.postRender(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:39)
- [ ] `ClaimResultElementRenderer.renderElementShadow(9)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:51)
- [ ] `ClaimResultElementRenderer.renderElement(10)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:64)
- [ ] `ClaimResultElementRenderer.shouldRender(2)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:114)
- [ ] `ClaimResultElementRenderer.getOrder(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:119)
- [ ] `ClaimResultElementRenderer.shouldBeDimScaled(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:124)

### `ClaimResultElementRenderer.Builder`: it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))

- [ ] `ClaimResultElementRenderer.Builder.Builder(0)`: its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:132)
- [ ] `ClaimResultElementRenderer.Builder.setDefault(0)`: its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:135)
- [ ] `ClaimResultElementRenderer.Builder.setManager(1)`: its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:140)
- [ ] `ClaimResultElementRenderer.Builder.build(0)`: its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:145)
- [ ] `ClaimResultElementRenderer.Builder.begin(0)`: it uses Builder, which isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java:155)

### `ClaimResultElementRenderProvider`: it extends `ElementRenderProvider` (xaero.map.element.render)

- [ ] `ClaimResultElementRenderProvider.ClaimResultElementRenderProvider(1)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderProvider.java:13)
- [ ] `ClaimResultElementRenderProvider.begin(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderProvider.java:17)
- [ ] `ClaimResultElementRenderProvider.hasNext(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderProvider.java:21)
- [ ] `ClaimResultElementRenderProvider.getNext(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderProvider.java:25)
- [ ] `ClaimResultElementRenderProvider.end(2)`: its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderProvider.java:29)

### `ClaimResultElementRenderReader`: it extends `ElementReader` (xaero.map.element.render)

- [ ] `ClaimResultElementRenderReader.isHidden(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:10)
- [ ] `ClaimResultElementRenderReader.getRenderX(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:14)
- [ ] `ClaimResultElementRenderReader.getRenderZ(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:18)
- [ ] `ClaimResultElementRenderReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:22)
- [ ] `ClaimResultElementRenderReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:26)
- [ ] `ClaimResultElementRenderReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:30)
- [ ] `ClaimResultElementRenderReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:34)
- [ ] `ClaimResultElementRenderReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:38)
- [ ] `ClaimResultElementRenderReader.getRenderBoxRight(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:42)
- [ ] `ClaimResultElementRenderReader.getRenderBoxTop(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:46)
- [ ] `ClaimResultElementRenderReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:50)
- [ ] `ClaimResultElementRenderReader.getLeftSideLength(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:54)
- [ ] `ClaimResultElementRenderReader.getMenuName(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:58)
- [ ] `ClaimResultElementRenderReader.getFilterName(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:62)
- [ ] `ClaimResultElementRenderReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:66)
- [ ] `ClaimResultElementRenderReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:70)
- [ ] `ClaimResultElementRenderReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:74)
- [ ] `ClaimResultElementRenderReader.isInteractable(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:79)
- [ ] `ClaimResultElementRenderReader.getTooltip(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java:83)

### `ClientClaimResultListener`: it implements `IClaimsManagerClaimResultListenerAPI` (xaero.pac.client.claims.tracker.result.api)

- [ ] `ClientClaimResultListener.onClaimResult(1)`: its class isn't translated (it implements `IClaimsManagerClaimResultListenerAPI` (xaero.pac.client.claims.tracker.result.api)) (xaero/map/mods/pac/gui/claim/result/ClientClaimResultListener.java:12)

### `ClaimsHighlighter`: it extends `ChunkHighlighter` (xaero.map.highlight)

- [ ] `ClaimsHighlighter.ClaimsHighlighter(1)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:36)
- [ ] `ClaimsHighlighter.regionHasHighlights(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:41)
- [ ] `ClaimsHighlighter.getColors(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:52)
- [ ] `ClaimsHighlighter.getChunkHighlitColor(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:85)
- [ ] `ClaimsHighlighter.applyAnchorPixel(4)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:114)
- [ ] `ClaimsHighlighter.calculateRegionHash(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:119)
- [ ] `ClaimsHighlighter.accountClaim(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:170)
- [ ] `ClaimsHighlighter.chunkIsHighlit(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:187)
- [ ] `ClaimsHighlighter.getChunkHighlightSubtleTooltip(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:192)
- [ ] `ClaimsHighlighter.getChunkHighlightBluntTooltip(3)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:232)
- [ ] `ClaimsHighlighter.addMinimapBlockHighlightTooltips(5)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:237)
- [ ] `ClaimsHighlighter.getClaimsCustomName(2)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:241)
- [ ] `ClaimsHighlighter.getClaimsColor(2)`: its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)) (xaero/map/mods/pac/highlight/ClaimsHighlighter.java:245)

### `OPACPlayerTrackerSystem`: it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)

- [ ] `OPACPlayerTrackerSystem.OPACPlayerTrackerSystem(1)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACPlayerTrackerSystem.java:13)
- [ ] `OPACPlayerTrackerSystem.getReader(0)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACPlayerTrackerSystem.java:18)
- [ ] `OPACPlayerTrackerSystem.getTrackedPlayerIterator(0)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACPlayerTrackerSystem.java:23)

### `OPACTrackedPlayerReader`: it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)

- [ ] `OPACTrackedPlayerReader.getId(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACTrackedPlayerReader.java:16)
- [ ] `OPACTrackedPlayerReader.getX(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACTrackedPlayerReader.java:20)
- [ ] `OPACTrackedPlayerReader.getY(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACTrackedPlayerReader.java:24)
- [ ] `OPACTrackedPlayerReader.getZ(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACTrackedPlayerReader.java:28)
- [ ] `OPACTrackedPlayerReader.getDimension(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/mods/pac/party/OPACTrackedPlayerReader.java:32)

### `SupportOpenPartiesAndClaims`: it uses `OpenPACClientAPI` (xaero.pac.client.api)

- [x] `SupportOpenPartiesAndClaims.CLAIMS_MESSAGE_SOURCE` in game_side.rs (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:50)
- [ ] `SupportOpenPartiesAndClaims.register(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:61)
- [ ] `SupportOpenPartiesAndClaims.claimAt(3)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:67)
- [ ] `SupportOpenPartiesAndClaims.onMapRender(8)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:71)
- [ ] `SupportOpenPartiesAndClaims.isFromParty(1)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:83)
- [ ] `SupportOpenPartiesAndClaims.openPlayerConfigScreen(3)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:88)
- [ ] `SupportOpenPartiesAndClaims.onMapKeyPressed(3)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:99)
- [ ] `SupportOpenPartiesAndClaims.getControlsTooltip(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:111)
- [ ] `SupportOpenPartiesAndClaims.registerHighlighters(1)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:115)
- [ ] `SupportOpenPartiesAndClaims.getClaimDistance(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:119)
- [ ] `SupportOpenPartiesAndClaims.getClaimResultElementManager(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:123)
- [ ] `SupportOpenPartiesAndClaims.getCaimResultElementRenderer(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:127)
- [ ] `SupportOpenPartiesAndClaims.onMapChange(1)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:131)
- [ ] `SupportOpenPartiesAndClaims.addRightClickOptions(5)`: its body couldn't be read: an anonymous class (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:137)
- [ ] `SupportOpenPartiesAndClaims.getPacClaimsKeyBinding(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:374)
- [ ] `SupportOpenPartiesAndClaims.resetDetection(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:378)
- [ ] `SupportOpenPartiesAndClaims.detectStuff(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:382)
- [ ] `SupportOpenPartiesAndClaims.serverHasMod(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:394)
- [ ] `SupportOpenPartiesAndClaims.getAllyIterator(0)`: its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java:399)

### `SupportAmecs`

- [ ] `SupportAmecs.SupportAmecs(1)`: it uses `Logger` (org.apache.logging.log4j) (xaero/map/mods/SupportAmecs.java:9)
- [ ] `SupportAmecs.modifiersArePressed(1)`: it uses Minecraft's `KeyMapping` (xaero/map/mods/SupportAmecs.java:12)

### `SupportFramedBlocks`: it uses `Class`

- [ ] `SupportFramedBlocks.SupportFramedBlocks(0)`: its class isn't translated (it uses `Class`) (xaero/map/mods/SupportFramedBlocks.java:26)
- [ ] `SupportFramedBlocks.onWorldChange(0)`: its class isn't translated (it uses `Class`) (xaero/map/mods/SupportFramedBlocks.java:76)
- [ ] `SupportFramedBlocks.findFramedBlocks(2)`: its class isn't translated (it uses `Class`) (xaero/map/mods/SupportFramedBlocks.java:80)
- [ ] `SupportFramedBlocks.isFrameBlock(3)`: its class isn't translated (it uses `Class`) (xaero/map/mods/SupportFramedBlocks.java:96)
- [ ] `SupportFramedBlocks.unpackFramedBlock(4)`: its class isn't translated (it uses `Class`) (xaero/map/mods/SupportFramedBlocks.java:105)

### `SupportIris`

- [x] `SupportIris.SupportIris(0)` (xaero/map/mods/SupportIris.java:4)

### `SupportMods`

- [ ] `SupportMods.xaeroMinimap`: it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportMods.java:9)
- [ ] `SupportMods.xaeroPac`: it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)) (xaero/map/mods/SupportMods.java:10)
- [x] `SupportMods.optifine` (xaero/map/mods/SupportMods.java:11)
- [x] `SupportMods.vivecraft` (xaero/map/mods/SupportMods.java:12)
- [x] `SupportMods.iris` (xaero/map/mods/SupportMods.java:13)
- [x] `SupportMods.supportIris` (xaero/map/mods/SupportMods.java:14)
- [ ] `SupportMods.supportFramedBlocks`: it uses SupportFramedBlocks, which isn't translated (it uses `Class`) (xaero/map/mods/SupportMods.java:15)
- [ ] `SupportMods.minimap(0)`: a nullable object kept in a changing static (xaero/map/mods/SupportMods.java:17)
- [ ] `SupportMods.framedBlocks(0)`: it uses SupportMods.supportFramedBlocks, which isn't translated (it uses SupportFramedBlocks, which isn't translated (it uses `Class`)) (xaero/map/mods/SupportMods.java:21)
- [ ] `SupportMods.pac(0)`: it uses SupportMods.xaeroPac, which isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api))) (xaero/map/mods/SupportMods.java:25)
- [ ] `SupportMods.load(0)`: a try/catch (ClassNotFoundException) (xaero/map/mods/SupportMods.java:29)

### `SupportModsFabric`: it extends `SupportMods`

- [x] `SupportModsFabric.amecs` (xaero/map/mods/SupportModsFabric.java:6)
- [ ] `SupportModsFabric.load(0)`: its class isn't translated (it extends `SupportMods`) (xaero/map/mods/SupportModsFabric.java:8)
- [x] `SupportModsFabric.amecs(0)` (xaero/map/mods/SupportModsFabric.java:19)

### `SupportXaeroMinimap`: it uses `HudMod` (xaero.common)

- [ ] `SupportXaeroMinimap.SupportXaeroMinimap(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:87)
- [ ] `SupportXaeroMinimap.register(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:117)
- [ ] `SupportXaeroMinimap.convertWaypoints(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:121)
- [ ] `SupportXaeroMinimap.convertSet(3)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:153)
- [ ] `SupportXaeroMinimap.convertWaypoint(4)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:166)
- [ ] `SupportXaeroMinimap.openWaypoint(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:170)
- [ ] `SupportXaeroMinimap.createWaypoint(6)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:190)
- [ ] `SupportXaeroMinimap.createTempWaypoint(5)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:215)
- [ ] `SupportXaeroMinimap.canTeleport(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:226)
- [ ] `SupportXaeroMinimap.teleportToWaypoint(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:233)
- [ ] `SupportXaeroMinimap.teleportToWaypoint(3)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:237)
- [ ] `SupportXaeroMinimap.disableWaypoint(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:246)
- [ ] `SupportXaeroMinimap.deleteWaypoint(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:263)
- [ ] `SupportXaeroMinimap.checkWaypoints(8)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:291)
- [ ] `SupportXaeroMinimap.getDimensionDivision(5)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:381)
- [ ] `SupportXaeroMinimap.renderWaypointsMenu(11)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:403)
- [ ] `SupportXaeroMinimap.requestWaypointsRefresh(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:419)
- [ ] `SupportXaeroMinimap.getWaypointKeyBinding(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:423)
- [ ] `SupportXaeroMinimap.getTempWaypointKeyBinding(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:427)
- [ ] `SupportXaeroMinimap.getTempWaypointsMenuKeyBinding(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:431)
- [ ] `SupportXaeroMinimap.onMapKeyPressed(3)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:435)
- [ ] `SupportXaeroMinimap.onMapKeyReleased(3)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:487)
- [ ] `SupportXaeroMinimap.handleMinimapKeyBinding(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:503)
- [ ] `SupportXaeroMinimap.drawSetChange(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:523)
- [ ] `SupportXaeroMinimap.getSettingsScreen(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:527)
- [ ] `SupportXaeroMinimap.getControlsTooltip(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:531)
- [ ] `SupportXaeroMinimap.onMapMouseRelease(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:544)
- [ ] `SupportXaeroMinimap.onMapConstruct(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:548)
- [ ] `SupportXaeroMinimap.onMapInit(4)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:552)
- [ ] `SupportXaeroMinimap.getWaypointsSorted(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:557)
- [ ] `SupportXaeroMinimap.waypointExists(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:561)
- [ ] `SupportXaeroMinimap.toggleTemporaryWaypoint(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:565)
- [ ] `SupportXaeroMinimap.openWaypointsMenu(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:578)
- [ ] `SupportXaeroMinimap.hidingWaypointCoordinates(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:585)
- [ ] `SupportXaeroMinimap.shareWaypoint(3)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:590)
- [ ] `SupportXaeroMinimap.shareLocation(4)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:595)
- [ ] `SupportXaeroMinimap.getMapWaypointWorld(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:604)
- [ ] `SupportXaeroMinimap.getWaypointWorld(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:608)
- [ ] `SupportXaeroMinimap.getDimDiv(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:612)
- [ ] `SupportXaeroMinimap.getArrowColorIndex(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:616)
- [ ] `SupportXaeroMinimap.getArrowColor(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:621)
- [ ] `SupportXaeroMinimap.getSubWorldNameToRender(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:626)
- [ ] `SupportXaeroMinimap.registerMinimapHighlighters(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:639)
- [ ] `SupportXaeroMinimap.getWaypoints(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:642)
- [ ] `SupportXaeroMinimap.getDeathpoints(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:646)
- [ ] `SupportXaeroMinimap.getWaypointRenderer(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:650)
- [ ] `SupportXaeroMinimap.getWaypointMenuRenderer(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:654)
- [ ] `SupportXaeroMinimap.onClearHighlightHash(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:658)
- [ ] `SupportXaeroMinimap.createRadarRendererWrapper(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:668)
- [ ] `SupportXaeroMinimap.getToggleRadarKey(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:672)
- [ ] `SupportXaeroMinimap.onClearHighlightHashes(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:676)
- [ ] `SupportXaeroMinimap.getToggleAllyPlayersKey(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:686)
- [ ] `SupportXaeroMinimap.getToggleClaimsKey(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:690)
- [ ] `SupportXaeroMinimap.getToggleWaypointsKeybinding(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:694)
- [ ] `SupportXaeroMinimap.onSessionFinalized(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:698)
- [ ] `SupportXaeroMinimap.openWaypointWorldTeleportCommandScreen(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:703)
- [ ] `SupportXaeroMinimap.getMinimapListPlayersAlternative(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:714)
- [ ] `SupportXaeroMinimap.getCaveStart(2)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:718)
- [ ] `SupportXaeroMinimap.getUsedCaving(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:735)
- [ ] `SupportXaeroMinimap.isFairPlay(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:740)
- [ ] `SupportXaeroMinimap.getMinimapSyncedPlayerTrackerSystem(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:744)
- [ ] `SupportXaeroMinimap.onBlockHover(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:752)
- [ ] `SupportXaeroMinimap.onRightClick(0)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:756)
- [ ] `SupportXaeroMinimap.wrapElementGraphics(1)`: its class isn't translated (it uses `HudMod` (xaero.common)) (xaero/map/mods/SupportXaeroMinimap.java:760)

### `XaeroWorldMapModMenu`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `XaeroWorldMapModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (xaero/map/mods/XaeroWorldMapModMenu.java:10)

### `XaeroNbtUtils`

- [ ] `XaeroNbtUtils.getUUID(2)`: it uses `UUID` (java.util) (xaero/map/nbt/util/XaeroNbtUtils.java:10)
- [ ] `XaeroNbtUtils.putUUID(3)`: it uses Minecraft's `CompoundTag` (xaero/map/nbt/util/XaeroNbtUtils.java:20)
- [ ] `XaeroNbtUtils.putUUIDAsLongArray(3)`: it uses Minecraft's `CompoundTag` (xaero/map/nbt/util/XaeroNbtUtils.java:24)
- [ ] `XaeroNbtUtils.createUUIDTag(1)`: it uses Minecraft's `IntArrayTag` (xaero/map/nbt/util/XaeroNbtUtils.java:28)

### `FastIntPalette`: it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)

- [ ] `FastIntPalette.FastIntPalette(3)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:13)
- [ ] `FastIntPalette.get(2)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:19)
- [ ] `FastIntPalette.add(1)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:28)
- [ ] `FastIntPalette.add(2)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:56)
- [ ] `FastIntPalette.append(2)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:66)
- [ ] `FastIntPalette.getIndex(1)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:84)
- [ ] `FastIntPalette.count(2)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:88)
- [ ] `FastIntPalette.getCount(1)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:94)
- [ ] `FastIntPalette.remove(1)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:99)
- [ ] `FastIntPalette.replace(2)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:112)
- [ ] `FastIntPalette.replaceAtIndex(2)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:117)
- [ ] `FastIntPalette.addNull(0)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:126)
- [ ] `FastIntPalette.getSize(0)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:130)
- [ ] `FastIntPalette.getNonNullCount(0)`: its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:134)

### `FastIntPalette.Builder`

- [x] `FastIntPalette.Builder.Builder(0)` (xaero/map/palette/FastIntPalette.java:141)
- [ ] `FastIntPalette.Builder.setDefault(0)`: `this` used as a value (xaero/map/palette/FastIntPalette.java:144)
- [ ] `FastIntPalette.Builder.setMaxCountPerElement(1)`: `this` used as a value (xaero/map/palette/FastIntPalette.java:149)
- [ ] `FastIntPalette.Builder.build(0)`: it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/palette/FastIntPalette.java:154)
- [ ] `FastIntPalette.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/map/palette/FastIntPalette.java:168)

### `FastIntPalette.Element`

- [x] `FastIntPalette.Element.Element(1)` (xaero/map/palette/FastIntPalette.java:177)
- [x] `FastIntPalette.Element.setValue(1)` (xaero/map/palette/FastIntPalette.java:181)
- [x] `FastIntPalette.Element.getValue(0)` (xaero/map/palette/FastIntPalette.java:185)
- [x] `FastIntPalette.Element.getCount(0)` (xaero/map/palette/FastIntPalette.java:189)
- [x] `FastIntPalette.Element.count(2)` (xaero/map/palette/FastIntPalette.java:193)

### `FastPalette`: it's generic

- [ ] `FastPalette.FastPalette(3)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:13)
- [ ] `FastPalette.get(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:19)
- [ ] `FastPalette.add(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:28)
- [ ] `FastPalette.add(2)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:56)
- [ ] `FastPalette.append(2)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:66)
- [ ] `FastPalette.getIndex(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:84)
- [ ] `FastPalette.count(2)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:88)
- [ ] `FastPalette.getCount(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:94)
- [ ] `FastPalette.remove(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:99)
- [ ] `FastPalette.replace(2)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:112)
- [ ] `FastPalette.replace(2)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:117)
- [ ] `FastPalette.addNull(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:126)
- [ ] `FastPalette.getSize(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:130)
- [ ] `FastPalette.getNonNullCount(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:134)

### `FastPalette.Builder`: it's generic

- [ ] `FastPalette.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:141)
- [ ] `FastPalette.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:144)
- [ ] `FastPalette.Builder.setMaxCountPerElement(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:149)
- [ ] `FastPalette.Builder.build(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:154)
- [ ] `FastPalette.Builder.begin(0)`: it's generic (xaero/map/palette/FastPalette.java:168)

### `FastPalette.Element`: it's generic

- [ ] `FastPalette.Element.Element(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:177)
- [ ] `FastPalette.Element.setObject(1)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:181)
- [ ] `FastPalette.Element.getObject(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:185)
- [ ] `FastPalette.Element.getCount(0)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:189)
- [ ] `FastPalette.Element.count(2)`: its class isn't translated (it's generic) (xaero/map/palette/FastPalette.java:193)

### `Paletted2DFastBitArrayIntStorage`: it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))

- [ ] `Paletted2DFastBitArrayIntStorage.Paletted2DFastBitArrayIntStorage(6)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:15)
- [ ] `Paletted2DFastBitArrayIntStorage.checkRange(2)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:24)
- [ ] `Paletted2DFastBitArrayIntStorage.getIndex(2)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:30)
- [ ] `Paletted2DFastBitArrayIntStorage.get(2)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:34)
- [ ] `Paletted2DFastBitArrayIntStorage.getRaw(2)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:41)
- [ ] `Paletted2DFastBitArrayIntStorage.set(3)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:47)
- [ ] `Paletted2DFastBitArrayIntStorage.writeData(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:79)
- [ ] `Paletted2DFastBitArrayIntStorage.contains(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:83)
- [ ] `Paletted2DFastBitArrayIntStorage.getPaletteSize(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:87)
- [ ] `Paletted2DFastBitArrayIntStorage.getPaletteNonNullCount(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:91)
- [ ] `Paletted2DFastBitArrayIntStorage.getPaletteElement(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:95)
- [ ] `Paletted2DFastBitArrayIntStorage.getPaletteElementCount(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:99)
- [ ] `Paletted2DFastBitArrayIntStorage.getDefaultValueCount(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:103)
- [ ] `Paletted2DFastBitArrayIntStorage.getBiomePaletteDebug(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:107)

### `Paletted2DFastBitArrayIntStorage.Builder`: it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))

- [ ] `Paletted2DFastBitArrayIntStorage.Builder.Builder(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:131)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setDefault(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:134)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setWidth(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:145)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setHeight(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:150)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setMaxPaletteElements(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:155)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setDefaultValue(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:160)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setPalette(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:165)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setData(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:170)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.setDefaultValueCount(1)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:175)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.build(0)`: its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))) (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:180)
- [ ] `Paletted2DFastBitArrayIntStorage.Builder.begin(0)`: it's generic (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java:209)

### `Paletted2DFastBitArrayStorage`: it's generic

- [ ] `Paletted2DFastBitArrayStorage.Paletted2DFastBitArrayStorage(6)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:13)
- [ ] `Paletted2DFastBitArrayStorage.checkRange(2)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:22)
- [ ] `Paletted2DFastBitArrayStorage.getIndex(2)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:28)
- [ ] `Paletted2DFastBitArrayStorage.get(2)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:32)
- [ ] `Paletted2DFastBitArrayStorage.set(3)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:39)
- [ ] `Paletted2DFastBitArrayStorage.contains(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:71)
- [ ] `Paletted2DFastBitArrayStorage.getPaletteSize(0)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:75)
- [ ] `Paletted2DFastBitArrayStorage.getPaletteNonNullCount(0)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:79)
- [ ] `Paletted2DFastBitArrayStorage.getPaletteElement(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:83)
- [ ] `Paletted2DFastBitArrayStorage.getPaletteElementCount(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:87)
- [ ] `Paletted2DFastBitArrayStorage.getDefaultValueCount(0)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:91)

### `Paletted2DFastBitArrayStorage.Builder`: it's generic

- [ ] `Paletted2DFastBitArrayStorage.Builder.Builder(0)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:104)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setDefault(0)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:107)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setWidth(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:118)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setHeight(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:123)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setMaxPaletteElements(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:128)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setDefaultValue(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:133)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setPalette(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:138)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setData(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:143)
- [ ] `Paletted2DFastBitArrayStorage.Builder.setDefaultValueCount(1)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:148)
- [ ] `Paletted2DFastBitArrayStorage.Builder.build(0)`: its class isn't translated (it's generic) (xaero/map/palette/Paletted2DFastBitArrayStorage.java:153)
- [ ] `Paletted2DFastBitArrayStorage.Builder.begin(0)`: it's generic (xaero/map/palette/Paletted2DFastBitArrayStorage.java:182)

### `Services`

- [ ] `Services.PLATFORM`: it uses IPlatformHelper, which isn't translated (it's an interface) (xaero/map/platform/Services.java:8)
- [ ] `Services.load(1)`: it's generic (xaero/map/platform/Services.java:10)

### `FabricPlatformHelper`: it implements `IPlatformHelper`

- [ ] `FabricPlatformHelper.getPlatformName(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/map/platform/services/FabricPlatformHelper.java:8)
- [ ] `FabricPlatformHelper.isModLoaded(1)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/map/platform/services/FabricPlatformHelper.java:13)
- [ ] `FabricPlatformHelper.isDevelopmentEnvironment(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/map/platform/services/FabricPlatformHelper.java:18)
- [ ] `FabricPlatformHelper.isDedicatedServer(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/map/platform/services/FabricPlatformHelper.java:23)
- [ ] `FabricPlatformHelper.getGameDir(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/map/platform/services/FabricPlatformHelper.java:28)
- [ ] `FabricPlatformHelper.getConfigDir(0)`: its class isn't translated (it implements `IPlatformHelper`) (xaero/map/platform/services/FabricPlatformHelper.java:33)

### `IPlatformHelper`: it's an interface

- [ ] `IPlatformHelper.getPlatformName(0)`: it has no body (xaero/map/platform/services/IPlatformHelper.java:6)
- [ ] `IPlatformHelper.isModLoaded(1)`: it has no body (xaero/map/platform/services/IPlatformHelper.java:8)
- [ ] `IPlatformHelper.checkModForMixin(1)`: its class isn't translated (it's an interface) (xaero/map/platform/services/IPlatformHelper.java:10)
- [ ] `IPlatformHelper.isDevelopmentEnvironment(0)`: it has no body (xaero/map/platform/services/IPlatformHelper.java:14)
- [ ] `IPlatformHelper.getEnvironmentName(0)`: its class isn't translated (it's an interface) (xaero/map/platform/services/IPlatformHelper.java:16)
- [ ] `IPlatformHelper.isDedicatedServer(0)`: it has no body (xaero/map/platform/services/IPlatformHelper.java:20)
- [ ] `IPlatformHelper.getGameDir(0)`: it has no body (xaero/map/platform/services/IPlatformHelper.java:22)
- [ ] `IPlatformHelper.getConfigDir(0)`: it has no body (xaero/map/platform/services/IPlatformHelper.java:24)

### `PoolTextureDirectBufferUnit`: it implements `PoolUnit` (xaero.map.pool)

- [ ] `PoolTextureDirectBufferUnit.PoolTextureDirectBufferUnit(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/pool/buffer/PoolTextureDirectBufferUnit.java:10)
- [ ] `PoolTextureDirectBufferUnit.getDirectBuffer(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/pool/buffer/PoolTextureDirectBufferUnit.java:14)
- [ ] `PoolTextureDirectBufferUnit.reset(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/pool/buffer/PoolTextureDirectBufferUnit.java:18)
- [ ] `PoolTextureDirectBufferUnit.create(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/pool/buffer/PoolTextureDirectBufferUnit.java:23)
- [ ] `PoolTextureDirectBufferUnit.createBuffer(0)`: it uses `ByteBuffer` (java.nio) (xaero/map/pool/buffer/PoolTextureDirectBufferUnit.java:35)

### `TextureDirectBufferPool`: it extends `MapPool` (xaero.map.pool)

- [ ] `TextureDirectBufferPool.TextureDirectBufferPool(0)`: its class isn't translated (it extends `MapPool` (xaero.map.pool)) (xaero/map/pool/buffer/TextureDirectBufferPool.java:6)
- [ ] `TextureDirectBufferPool.construct(1)`: its class isn't translated (it extends `MapPool` (xaero.map.pool)) (xaero/map/pool/buffer/TextureDirectBufferPool.java:10)
- [ ] `TextureDirectBufferPool.get(1)`: its class isn't translated (it extends `MapPool` (xaero.map.pool)) (xaero/map/pool/buffer/TextureDirectBufferPool.java:14)

### `MapPool`: it's generic

- [ ] `MapPool.MapPool(1)`: its class isn't translated (it's generic) (xaero/map/pool/MapPool.java:10)
- [ ] `MapPool.get(1)`: its class isn't translated (it's generic) (xaero/map/pool/MapPool.java:15)
- [ ] `MapPool.addToPool(1)`: its class isn't translated (it's generic) (xaero/map/pool/MapPool.java:31)
- [ ] `MapPool.takeFromPool(0)`: its class isn't translated (it's generic) (xaero/map/pool/MapPool.java:42)
- [ ] `MapPool.size(0)`: its class isn't translated (it's generic) (xaero/map/pool/MapPool.java:46)
- [ ] `MapPool.construct(1)`: it has no body (xaero/map/pool/MapPool.java:50)

### `MapTilePool`: it extends `MapPool`

- [ ] `MapTilePool.MapTilePool(0)`: its class isn't translated (it extends `MapPool`) (xaero/map/pool/MapTilePool.java:6)
- [ ] `MapTilePool.construct(1)`: its class isn't translated (it extends `MapPool`) (xaero/map/pool/MapTilePool.java:10)
- [ ] `MapTilePool.get(3)`: its class isn't translated (it extends `MapPool`) (xaero/map/pool/MapTilePool.java:14)

### `PoolUnit`: it's an interface

- [ ] `PoolUnit.create(1)`: it has no body (xaero/map/pool/PoolUnit.java:4)

### `TextureUploadPool`: it's generic

- [ ] `TextureUploadPool.TextureUploadPool(1)`: its class isn't translated (it's generic) (xaero/map/pool/TextureUploadPool.java:10)

### `TextureUploadPool.BranchDownload`: it extends `TextureUploadPool`

- [ ] `TextureUploadPool.BranchDownload.BranchDownload(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:15)
- [ ] `TextureUploadPool.BranchDownload.construct(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:19)
- [ ] `TextureUploadPool.BranchDownload.get(3)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:23)

### `TextureUploadPool.BranchUpdate`: it extends `TextureUploadPool`

- [ ] `TextureUploadPool.BranchUpdate.BranchUpdate(2)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:31)
- [ ] `TextureUploadPool.BranchUpdate.construct(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:36)
- [ ] `TextureUploadPool.BranchUpdate.get(15)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:40)

### `TextureUploadPool.Normal`: it extends `TextureUploadPool`

- [ ] `TextureUploadPool.Normal.Normal(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:79)
- [ ] `TextureUploadPool.Normal.construct(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:83)
- [ ] `TextureUploadPool.Normal.get(8)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:87)

### `TextureUploadPool.SubsequentNormal`: it extends `TextureUploadPool`

- [ ] `TextureUploadPool.SubsequentNormal.SubsequentNormal(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:95)
- [ ] `TextureUploadPool.SubsequentNormal.construct(1)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:99)
- [ ] `TextureUploadPool.SubsequentNormal.get(9)`: its class isn't translated (it extends `TextureUploadPool`) (xaero/map/pool/TextureUploadPool.java:103)

### `SimpleProfiler`: it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)

- [ ] `SimpleProfiler.reset(0)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (xaero/map/profile/SimpleProfiler.java:12)
- [ ] `SimpleProfiler.addTime(2)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (xaero/map/profile/SimpleProfiler.java:17)
- [ ] `SimpleProfiler.section(1)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (xaero/map/profile/SimpleProfiler.java:22)
- [ ] `SimpleProfiler.end(0)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (xaero/map/profile/SimpleProfiler.java:33)
- [ ] `SimpleProfiler.debug(0)`: its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)) (xaero/map/profile/SimpleProfiler.java:37)

### `PlayerTeleporter`

- [ ] `PlayerTeleporter.teleport(6)`: it uses Minecraft's `Screen` (xaero/map/radar/tracker/PlayerTeleporter.java:9)
- [ ] `PlayerTeleporter.teleportToPlayer(3)`: it uses Minecraft's `Screen` (xaero/map/radar/tracker/PlayerTeleporter.java:22)

### `PlayerTrackerIconRenderer`

- [ ] `PlayerTrackerIconRenderer.isPlayerUpsideDown(1)`: it uses Minecraft's `Player` (xaero/map/radar/tracker/PlayerTrackerIconRenderer.java:21)
- [x] `PlayerTrackerIconRenderer.isUpsideDownName(1)` (xaero/map/radar/tracker/PlayerTrackerIconRenderer.java:26)
- [ ] `PlayerTrackerIconRenderer.renderIcon(3)`: it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)) (xaero/map/radar/tracker/PlayerTrackerIconRenderer.java:30)
- [ ] `PlayerTrackerIconRenderer.renderIconGUI(3)`: it uses Minecraft's `GuiGraphicsExtractor` (xaero/map/radar/tracker/PlayerTrackerIconRenderer.java:57)

### `PlayerTrackerMapElement`: it's generic

- [ ] `PlayerTrackerMapElement.PlayerTrackerMapElement(2)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:15)
- [ ] `PlayerTrackerMapElement.getPlayerId(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:20)
- [ ] `PlayerTrackerMapElement.getX(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:24)
- [ ] `PlayerTrackerMapElement.getY(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:28)
- [ ] `PlayerTrackerMapElement.getZ(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:32)
- [ ] `PlayerTrackerMapElement.getDimension(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:36)
- [ ] `PlayerTrackerMapElement.getPlayer(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:40)
- [ ] `PlayerTrackerMapElement.setRenderedOnRadar(1)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:44)
- [ ] `PlayerTrackerMapElement.wasRenderedOnRadar(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:48)
- [ ] `PlayerTrackerMapElement.getFadeAnim(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:52)
- [ ] `PlayerTrackerMapElement.setFadeAnim(1)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:56)
- [ ] `PlayerTrackerMapElement.getSystem(0)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElement.java:60)

### `PlayerTrackerMapElementCollector`: it uses `UUID` (java.util)

- [ ] `PlayerTrackerMapElementCollector.PlayerTrackerMapElementCollector(2)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:18)
- [ ] `PlayerTrackerMapElementCollector.update(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:23)
- [ ] `PlayerTrackerMapElementCollector.updateForSystem(3)`: it's generic (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:41)
- [ ] `PlayerTrackerMapElementCollector.playerExists(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:69)
- [ ] `PlayerTrackerMapElementCollector.getElements(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:73)
- [ ] `PlayerTrackerMapElementCollector.resetRenderedOnRadarFlags(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:77)
- [ ] `PlayerTrackerMapElementCollector.confirmPlayerRadarRender(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java:83)

### `PlayerTrackerMapElementReader`: it extends `ElementReader` (xaero.map.element.render)

- [ ] `PlayerTrackerMapElementReader.isHidden(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:22)
- [ ] `PlayerTrackerMapElementReader.getRenderX(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:26)
- [ ] `PlayerTrackerMapElementReader.getRenderY(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:30)
- [ ] `PlayerTrackerMapElementReader.hasYCoordinate(0)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:34)
- [ ] `PlayerTrackerMapElementReader.getRenderZ(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:39)
- [ ] `PlayerTrackerMapElementReader.getInteractionBoxLeft(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:43)
- [ ] `PlayerTrackerMapElementReader.getInteractionBoxRight(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:47)
- [ ] `PlayerTrackerMapElementReader.getInteractionBoxTop(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:51)
- [ ] `PlayerTrackerMapElementReader.getInteractionBoxBottom(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:55)
- [ ] `PlayerTrackerMapElementReader.getRenderBoxLeft(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:59)
- [ ] `PlayerTrackerMapElementReader.getRenderBoxRight(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:63)
- [ ] `PlayerTrackerMapElementReader.getRenderBoxTop(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:67)
- [ ] `PlayerTrackerMapElementReader.getRenderBoxBottom(3)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:71)
- [ ] `PlayerTrackerMapElementReader.getLeftSideLength(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:75)
- [ ] `PlayerTrackerMapElementReader.getMenuName(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:80)
- [ ] `PlayerTrackerMapElementReader.getFilterName(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:85)
- [ ] `PlayerTrackerMapElementReader.getMenuTextFillLeftPadding(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:89)
- [ ] `PlayerTrackerMapElementReader.getRightClickTitleBackgroundColor(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:93)
- [ ] `PlayerTrackerMapElementReader.shouldScaleBoxWithOptionalScale(0)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:97)
- [ ] `PlayerTrackerMapElementReader.isRightClickValid(1)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:102)
- [ ] `PlayerTrackerMapElementReader.getRightClickOptions(2)`: its body couldn't be read: an anonymous class (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:106)
- [ ] `PlayerTrackerMapElementReader.isInteractable(2)`: its class isn't translated (it extends `ElementReader` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java:157)

### `PlayerTrackerMapElementRenderContext`: it uses Minecraft's `VertexConsumer`


### `PlayerTrackerMapElementRenderer`: it extends `ElementRenderer` (xaero.map.element.render)

- [ ] `PlayerTrackerMapElementRenderer.PlayerTrackerMapElementRenderer(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:31)
- [ ] `PlayerTrackerMapElementRenderer.getTrackedPlayerIconManager(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:41)
- [ ] `PlayerTrackerMapElementRenderer.preRender(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:45)
- [ ] `PlayerTrackerMapElementRenderer.postRender(4)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:61)
- [ ] `PlayerTrackerMapElementRenderer.renderElementShadow(9)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:73)
- [ ] `PlayerTrackerMapElementRenderer.renderElement(10)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:86)
- [ ] `PlayerTrackerMapElementRenderer.shouldRender(2)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:161)
- [ ] `PlayerTrackerMapElementRenderer.getOrder(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:167)
- [ ] `PlayerTrackerMapElementRenderer.getCollector(0)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:172)
- [ ] `PlayerTrackerMapElementRenderer.update(1)`: its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:176)

### `PlayerTrackerMapElementRenderer.Builder`

- [x] `PlayerTrackerMapElementRenderer.Builder.Builder(0)` (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:185)
- [ ] `PlayerTrackerMapElementRenderer.Builder.setDefault(0)`: `this` used as a value (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:188)
- [ ] `PlayerTrackerMapElementRenderer.Builder.build(0)`: it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:192)
- [ ] `PlayerTrackerMapElementRenderer.Builder.begin(0)`: it calls Builder.setDefault, which isn't translated (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java:201)

### `PlayerTrackerMapElementRenderProvider`: it's generic

- [ ] `PlayerTrackerMapElementRenderProvider.PlayerTrackerMapElementRenderProvider(1)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderProvider.java:11)
- [ ] `PlayerTrackerMapElementRenderProvider.begin(2)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderProvider.java:15)
- [ ] `PlayerTrackerMapElementRenderProvider.hasNext(2)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderProvider.java:20)
- [ ] `PlayerTrackerMapElementRenderProvider.getNext(2)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderProvider.java:25)
- [ ] `PlayerTrackerMapElementRenderProvider.end(2)`: its class isn't translated (it's generic) (xaero/map/radar/tracker/PlayerTrackerMapElementRenderProvider.java:29)

### `PlayerTrackerMenuRenderContext`


### `PlayerTrackerMenuRenderer`: it extends `MapElementMenuRenderer` (xaero.map.element)

- [ ] `PlayerTrackerMenuRenderer.PlayerTrackerMenuRenderer(4)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:27)
- [ ] `PlayerTrackerMenuRenderer.onMapInit(4)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:38)
- [ ] `PlayerTrackerMenuRenderer.onShowPlayersButton(3)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:72)
- [ ] `PlayerTrackerMenuRenderer.beforeMenuRender(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:78)
- [ ] `PlayerTrackerMenuRenderer.afterMenuRender(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:82)
- [ ] `PlayerTrackerMenuRenderer.renderInMenu(11)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:86)
- [ ] `PlayerTrackerMenuRenderer.beforeFiltering(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:107)
- [ ] `PlayerTrackerMenuRenderer.menuStartPos(1)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:111)
- [ ] `PlayerTrackerMenuRenderer.menuSearchPadding(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:116)
- [ ] `PlayerTrackerMenuRenderer.getFilterPlaceholder(0)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:121)
- [ ] `PlayerTrackerMenuRenderer.getRenderer(1)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:126)
- [ ] `PlayerTrackerMenuRenderer.canJumpTo(1)`: its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:130)

### `PlayerTrackerMenuRenderer.Builder`: it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))

- [ ] `PlayerTrackerMenuRenderer.Builder.Builder(0)`: its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:137)
- [ ] `PlayerTrackerMenuRenderer.Builder.setDefault(0)`: its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:140)
- [ ] `PlayerTrackerMenuRenderer.Builder.setRenderer(1)`: its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:145)
- [ ] `PlayerTrackerMenuRenderer.Builder.build(0)`: its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:150)
- [ ] `PlayerTrackerMenuRenderer.Builder.begin(0)`: it uses Builder, which isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))) (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java:163)

### `ClientSyncedTrackedPlayerManager`: it uses `UUID` (java.util)

- [ ] `ClientSyncedTrackedPlayerManager.remove(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/synced/ClientSyncedTrackedPlayerManager.java:13)
- [ ] `ClientSyncedTrackedPlayerManager.update(5)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/synced/ClientSyncedTrackedPlayerManager.java:17)
- [ ] `ClientSyncedTrackedPlayerManager.getPlayers(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/synced/ClientSyncedTrackedPlayerManager.java:26)
- [ ] `ClientSyncedTrackedPlayerManager.reset(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/radar/tracker/synced/ClientSyncedTrackedPlayerManager.java:30)

### `SyncedPlayerTrackerSystem`: it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)

- [ ] `SyncedPlayerTrackerSystem.getReader(0)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedPlayerTrackerSystem.java:16)
- [ ] `SyncedPlayerTrackerSystem.getTrackedPlayerIterator(0)`: its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedPlayerTrackerSystem.java:21)

### `SyncedTrackedPlayerReader`: it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)

- [ ] `SyncedTrackedPlayerReader.getId(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedTrackedPlayerReader.java:10)
- [ ] `SyncedTrackedPlayerReader.getX(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedTrackedPlayerReader.java:14)
- [ ] `SyncedTrackedPlayerReader.getY(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedTrackedPlayerReader.java:18)
- [ ] `SyncedTrackedPlayerReader.getZ(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedTrackedPlayerReader.java:22)
- [ ] `SyncedTrackedPlayerReader.getDimension(1)`: its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)) (xaero/map/radar/tracker/system/impl/SyncedTrackedPlayerReader.java:26)

### `IPlayerTrackerSystem`: it's an interface

- [ ] `IPlayerTrackerSystem.getReader(0)`: it has no body (xaero/map/radar/tracker/system/IPlayerTrackerSystem.java:6)
- [ ] `IPlayerTrackerSystem.getTrackedPlayerIterator(0)`: it has no body (xaero/map/radar/tracker/system/IPlayerTrackerSystem.java:8)

### `ITrackedPlayerReader`: it's an interface

- [ ] `ITrackedPlayerReader.getId(1)`: it has no body (xaero/map/radar/tracker/system/ITrackedPlayerReader.java:8)
- [ ] `ITrackedPlayerReader.getX(1)`: it has no body (xaero/map/radar/tracker/system/ITrackedPlayerReader.java:10)
- [ ] `ITrackedPlayerReader.getY(1)`: it has no body (xaero/map/radar/tracker/system/ITrackedPlayerReader.java:12)
- [ ] `ITrackedPlayerReader.getZ(1)`: it has no body (xaero/map/radar/tracker/system/ITrackedPlayerReader.java:14)
- [ ] `ITrackedPlayerReader.getDimension(1)`: it has no body (xaero/map/radar/tracker/system/ITrackedPlayerReader.java:16)

### `PlayerTrackerSystemManager`: it uses IPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `PlayerTrackerSystemManager.register(2)`: its class isn't translated (it uses IPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/radar/tracker/system/PlayerTrackerSystemManager.java:10)
- [ ] `PlayerTrackerSystemManager.getSystems(0)`: its class isn't translated (it uses IPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/radar/tracker/system/PlayerTrackerSystemManager.java:19)

### `TrackedPlayerIconManager`: it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))

- [x] `TrackedPlayerIconManager.ICON_WIDTH` (xaero/map/radar/tracker/TrackedPlayerIconManager.java:17)
- [x] `TrackedPlayerIconManager.PREFERRED_ATLAS_WIDTH` (xaero/map/radar/tracker/TrackedPlayerIconManager.java:18)
- [ ] `TrackedPlayerIconManager.TrackedPlayerIconManager(4)`: its class isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))) (xaero/map/radar/tracker/TrackedPlayerIconManager.java:24)
- [ ] `TrackedPlayerIconManager.getPlayerSkin(2)`: its class isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))) (xaero/map/radar/tracker/TrackedPlayerIconManager.java:33)
- [ ] `TrackedPlayerIconManager.getIcon(4)`: its class isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))) (xaero/map/radar/tracker/TrackedPlayerIconManager.java:44)

### `TrackedPlayerIconManager.Builder`

- [ ] `TrackedPlayerIconManager.Builder.build(0)`: it uses TrackedPlayerIconManager, which isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))) (xaero/map/radar/tracker/TrackedPlayerIconManager.java:56)
- [x] `TrackedPlayerIconManager.Builder.begin(0)` (xaero/map/radar/tracker/TrackedPlayerIconManager.java:64)

### `TrackedPlayerIconPrerenderer`: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)

- [ ] `TrackedPlayerIconPrerenderer.prerender(6)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/radar/tracker/TrackedPlayerIconPrerenderer.java:29)

### `BranchLeveledRegion`: it extends `LeveledRegion`

- [x] `BranchLeveledRegion.CHILD_LENGTH_IN_TEXTURES` (xaero/map/region/BranchLeveledRegion.java:17)
- [x] `BranchLeveledRegion.MAX_COORD_WITHIN_CHILD` (xaero/map/region/BranchLeveledRegion.java:18)
- [ ] `BranchLeveledRegion.BranchLeveledRegion(9)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:29)
- [ ] `BranchLeveledRegion.reset(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:36)
- [ ] `BranchLeveledRegion.checkAndTrackRegionExistence(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:51)
- [ ] `BranchLeveledRegion.checkForUpdates(9)`: its body couldn't be read: a labelled statement (xaero/map/region/BranchLeveledRegion.java:64)
- [ ] `BranchLeveledRegion.putTexture(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:277)
- [ ] `BranchLeveledRegion.getTexture(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:281)
- [ ] `BranchLeveledRegion.hasTextures(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:285)
- [ ] `BranchLeveledRegion.isEmpty(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:290)
- [ ] `BranchLeveledRegion.preCacheLoad(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:302)
- [ ] `BranchLeveledRegion.putLeaf(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:308)
- [ ] `BranchLeveledRegion.get(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:333)
- [ ] `BranchLeveledRegion.remove(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:352)
- [ ] `BranchLeveledRegion.loadingAnimation(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:376)
- [ ] `BranchLeveledRegion.addDebugLines(4)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:381)
- [ ] `BranchLeveledRegion.shouldEndProcessingAfterUpload(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:399)
- [ ] `BranchLeveledRegion.setLoaded(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:404)
- [ ] `BranchLeveledRegion.isLoaded(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:408)
- [ ] `BranchLeveledRegion.cleanAndCacheRequestsBlocked(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:413)
- [ ] `BranchLeveledRegion.onProcessingEnd(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:418)
- [ ] `BranchLeveledRegion.shouldBeProcessed(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:425)
- [ ] `BranchLeveledRegion.preCache(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:430)
- [ ] `BranchLeveledRegion.postCache(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:434)
- [ ] `BranchLeveledRegion.skipCaching(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:440)
- [ ] `BranchLeveledRegion.findCacheFile(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:445)
- [ ] `BranchLeveledRegion.onCurrentDimFinish(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:455)
- [ ] `BranchLeveledRegion.onLimiterRemoval(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:459)
- [ ] `BranchLeveledRegion.afterLimiterRemoval(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:464)
- [ ] `BranchLeveledRegion.createTexture(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:469)
- [ ] `BranchLeveledRegion.setShouldCheckForUpdatesRecursive(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:473)
- [ ] `BranchLeveledRegion.setShouldCheckForUpdatesSingle(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:480)
- [ ] `BranchLeveledRegion.startDownloadingTexturesForCache(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:484)
- [ ] `BranchLeveledRegion.postTextureUpdate(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:520)
- [ ] `BranchLeveledRegion.countTextureUpdate(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:528)
- [ ] `BranchLeveledRegion.eligibleForSaving(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:533)
- [ ] `BranchLeveledRegion.onCacheLoadFailed(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/BranchLeveledRegion.java:537)

### `ExportMapRegion`: it extends `MapRegion`

- [ ] `ExportMapRegion.ExportMapRegion(5)`: its class isn't translated (it extends `MapRegion`) (xaero/map/region/ExportMapRegion.java:9)
- [ ] `ExportMapRegion.createTileChunk(2)`: its class isn't translated (it extends `MapRegion`) (xaero/map/region/ExportMapRegion.java:13)
- [ ] `ExportMapRegion.getTexture(2)`: its class isn't translated (it extends `MapRegion`) (xaero/map/region/ExportMapRegion.java:18)
- [ ] `ExportMapRegion.getChunk(2)`: its class isn't translated (it extends `MapRegion`) (xaero/map/region/ExportMapRegion.java:22)

### `ExportMapTileChunk`: it extends `MapTileChunk`

- [ ] `ExportMapTileChunk.ExportMapTileChunk(3)`: its class isn't translated (it extends `MapTileChunk`) (xaero/map/region/ExportMapTileChunk.java:7)
- [ ] `ExportMapTileChunk.createLeafTexture(0)`: its class isn't translated (it extends `MapTileChunk`) (xaero/map/region/ExportMapTileChunk.java:11)
- [ ] `ExportMapTileChunk.getLeafTexture(0)`: its class isn't translated (it extends `MapTileChunk`) (xaero/map/region/ExportMapTileChunk.java:16)

### `LayeredRegionManager`: it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)

- [ ] `LayeredRegionManager.LayeredRegionManager(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:19)
- [ ] `LayeredRegionManager.putLeaf(3)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:26)
- [ ] `LayeredRegionManager.getLeaf(3)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:30)
- [ ] `LayeredRegionManager.get(4)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:34)
- [ ] `LayeredRegionManager.remove(4)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:38)
- [ ] `LayeredRegionManager.getLayer(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:42)
- [ ] `LayeredRegionManager.clear(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:53)
- [ ] `LayeredRegionManager.loadedCount(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:67)
- [ ] `LayeredRegionManager.removeListRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:71)
- [ ] `LayeredRegionManager.addListRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:77)
- [ ] `LayeredRegionManager.bumpLoadedRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:83)
- [ ] `LayeredRegionManager.bumpLoadedRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:87)
- [ ] `LayeredRegionManager.getLoadedListUnsynced(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:95)
- [ ] `LayeredRegionManager.getLoadedRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:99)
- [ ] `LayeredRegionManager.addLoadedRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:105)
- [ ] `LayeredRegionManager.removeLoadedRegion(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:111)
- [ ] `LayeredRegionManager.size(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:117)
- [ ] `LayeredRegionManager.getUnsyncedSet(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:121)
- [ ] `LayeredRegionManager.onClearCachedHighlightHash(2)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:125)
- [ ] `LayeredRegionManager.onClearCachedHighlightHashes(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:131)
- [ ] `LayeredRegionManager.applyToEachLoadedLayer(1)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:137)
- [ ] `LayeredRegionManager.preDetection(0)`: its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)) (xaero/map/region/LayeredRegionManager.java:143)

### `LeveledRegion`: it's generic

- [x] `LeveledRegion.SIDE_LENGTH` (xaero/map/region/LeveledRegion.java:31)
- [x] `LeveledRegion.comparisonX` (xaero/map/region/LeveledRegion.java:32)
- [x] `LeveledRegion.comparisonZ` (xaero/map/region/LeveledRegion.java:33)
- [x] `LeveledRegion.comparisonLevel` (xaero/map/region/LeveledRegion.java:34)
- [x] `LeveledRegion.comparisonLeafX` (xaero/map/region/LeveledRegion.java:35)
- [x] `LeveledRegion.comparisonLeafZ` (xaero/map/region/LeveledRegion.java:36)
- [ ] `LeveledRegion.LeveledRegion(9)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:60)
- [ ] `LeveledRegion.onDimensionClear(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:74)
- [ ] `LeveledRegion.deleteTexturesAndBuffers(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:78)
- [ ] `LeveledRegion.hasTextures(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:103)
- [ ] `LeveledRegion.deleteBuffers(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:107)
- [ ] `LeveledRegion.deleteGLBuffers(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:130)
- [ ] `LeveledRegion.isAllCachePrepared(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:143)
- [ ] `LeveledRegion.setAllCachePrepared(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:147)
- [ ] `LeveledRegion.getRegionX(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:155)
- [ ] `LeveledRegion.getRegionZ(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:159)
- [ ] `LeveledRegion.shouldCache(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:163)
- [ ] `LeveledRegion.getLevel(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:167)
- [ ] `LeveledRegion.setShouldCache(2)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:171)
- [ ] `LeveledRegion.recacheHasBeenRequested(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:178)
- [ ] `LeveledRegion.setRecacheHasBeenRequested(2)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:182)
- [ ] `LeveledRegion.getCacheFile(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:190)
- [ ] `LeveledRegion.setCacheFile(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:194)
- [ ] `LeveledRegion.getDim(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:198)
- [ ] `LeveledRegion.toString(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:202)
- [ ] `LeveledRegion.reloadHasBeenRequested(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:207)
- [ ] `LeveledRegion.setReloadHasBeenRequested(2)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:211)
- [x] `LeveledRegion.setComparison(5)` (xaero/map/region/LeveledRegion.java:219)
- [ ] `LeveledRegion.distanceFromPlayer(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:227)
- [ ] `LeveledRegion.leafDistanceFromPlayer(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:233)
- [ ] `LeveledRegion.calculateSortingDistance(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:239)
- [ ] `LeveledRegion.chunkDistanceFromPlayer(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:244)
- [ ] `LeveledRegion.calculateSortingChunkDistance(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:250)
- [ ] `LeveledRegion.compareTo(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:255)
- [ ] `LeveledRegion.onProcessingEnd(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:291)
- [ ] `LeveledRegion.addDebugLines(4)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:294)
- [ ] `LeveledRegion.writeCacheMetaData(3)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:328)
- [ ] `LeveledRegion.saveCacheTextures(3)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:347)
- [ ] `LeveledRegion.readCacheMetaData(7)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:418)
- [ ] `LeveledRegion.loadCacheTextures(9)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:432)
- [ ] `LeveledRegion.onCacheLoadFailed(1)`: it has no body (xaero/map/region/LeveledRegion.java:564)
- [ ] `LeveledRegion.saveBiomePalette(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:566)
- [ ] `LeveledRegion.loadBiomePalette(6)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:586)
- [ ] `LeveledRegion.shouldLeafAffectCache(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:631)
- [ ] `LeveledRegion.readCacheInput(9)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:635)
- [ ] `LeveledRegion.getAndResetCachedTextureVersion(2)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:675)
- [ ] `LeveledRegion.getParent(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:681)
- [ ] `LeveledRegion.shouldAffectLoadingRequestFrequency(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:685)
- [ ] `LeveledRegion.preCacheLoad(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:689)
- [ ] `LeveledRegion.processWhenLoadedChunksExist(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:692)
- [ ] `LeveledRegion.isMetaLoaded(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:695)
- [ ] `LeveledRegion.confirmMetaLoaded(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:699)
- [ ] `LeveledRegion.getRootRegion(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:703)
- [ ] `LeveledRegion.checkForUpdates(9)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:712)
- [ ] `LeveledRegion.ensureBiomePalette(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:725)
- [ ] `LeveledRegion.getBiomeKey(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:731)
- [ ] `LeveledRegion.getBiomePaletteIndex(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:735)
- [ ] `LeveledRegion.onBiomeAddedToTexture(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:739)
- [ ] `LeveledRegion.onBiomeRemovedFromTexture(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:746)
- [ ] `LeveledRegion.uncountTextureBiomes(1)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:755)
- [ ] `LeveledRegion.getBiomePaletteSize(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:769)
- [ ] `LeveledRegion.getBiomePalette(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:773)
- [ ] `LeveledRegion.resetBiomePalette(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:777)
- [ ] `LeveledRegion.isRefreshing(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:781)
- [ ] `LeveledRegion.shouldAllowAnotherRegionToLoad(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:785)
- [ ] `LeveledRegion.shouldEndProcessingAfterUpload(0)`: it has no body (xaero/map/region/LeveledRegion.java:791)
- [ ] `LeveledRegion.createTexture(2)`: it has no body (xaero/map/region/LeveledRegion.java:793)
- [ ] `LeveledRegion.putTexture(3)`: it has no body (xaero/map/region/LeveledRegion.java:795)
- [ ] `LeveledRegion.getTexture(2)`: it has no body (xaero/map/region/LeveledRegion.java:797)
- [ ] `LeveledRegion.putLeaf(3)`: it has no body (xaero/map/region/LeveledRegion.java:799)
- [ ] `LeveledRegion.remove(3)`: it has no body (xaero/map/region/LeveledRegion.java:801)
- [ ] `LeveledRegion.get(3)`: it has no body (xaero/map/region/LeveledRegion.java:803)
- [ ] `LeveledRegion.loadingAnimation(0)`: it has no body (xaero/map/region/LeveledRegion.java:805)
- [ ] `LeveledRegion.cleanAndCacheRequestsBlocked(0)`: it has no body (xaero/map/region/LeveledRegion.java:807)
- [ ] `LeveledRegion.shouldBeProcessed(0)`: it has no body (xaero/map/region/LeveledRegion.java:809)
- [ ] `LeveledRegion.isLoaded(0)`: it has no body (xaero/map/region/LeveledRegion.java:811)
- [ ] `LeveledRegion.preCache(0)`: it has no body (xaero/map/region/LeveledRegion.java:813)
- [ ] `LeveledRegion.postCache(3)`: it has no body (xaero/map/region/LeveledRegion.java:815)
- [ ] `LeveledRegion.skipCaching(1)`: it has no body (xaero/map/region/LeveledRegion.java:817)
- [ ] `LeveledRegion.findCacheFile(1)`: it has no body (xaero/map/region/LeveledRegion.java:819)
- [ ] `LeveledRegion.onCurrentDimFinish(2)`: it has no body (xaero/map/region/LeveledRegion.java:821)
- [ ] `LeveledRegion.onLimiterRemoval(1)`: it has no body (xaero/map/region/LeveledRegion.java:823)
- [ ] `LeveledRegion.afterLimiterRemoval(1)`: it has no body (xaero/map/region/LeveledRegion.java:825)
- [ ] `LeveledRegion.getExtraInfo(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:827)
- [ ] `LeveledRegion.updateLeafTextureVersion(3)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:831)
- [ ] `LeveledRegion.hasRemovableSourceData(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:834)
- [ ] `LeveledRegion.getCaveLayer(0)`: its class isn't translated (it's generic) (xaero/map/region/LeveledRegion.java:838)

### `LeveledRegionManager`: it uses LeveledRegion, which isn't translated (it's generic)

- [x] `LeveledRegionManager.MAX_LEVEL` (xaero/map/region/LeveledRegionManager.java:6)
- [ ] `LeveledRegionManager.putLeaf(3)`: its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)) (xaero/map/region/LeveledRegionManager.java:9)
- [ ] `LeveledRegionManager.getLeaf(2)`: its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)) (xaero/map/region/LeveledRegionManager.java:38)
- [ ] `LeveledRegionManager.get(3)`: its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)) (xaero/map/region/LeveledRegionManager.java:42)
- [ ] `LeveledRegionManager.remove(3)`: its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)) (xaero/map/region/LeveledRegionManager.java:70)

### `MapBlock`: it extends `MapPixel`

- [ ] `MapBlock.isGrass(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:28)
- [ ] `MapBlock.getParametres(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:32)
- [ ] `MapBlock.getPixelColour(27)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:43)
- [ ] `MapBlock.toRenderString(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:105)
- [ ] `MapBlock.equalsSlopesExcluded(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:127)
- [ ] `MapBlock.equals(2)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:146)
- [ ] `MapBlock.fixHeightType(11)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:154)
- [ ] `MapBlock.prepareForWriting(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:246)
- [ ] `MapBlock.write(7)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:260)
- [ ] `MapBlock.addOverlay(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:275)
- [ ] `MapBlock.getHeight(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:283)
- [ ] `MapBlock.setHeight(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:287)
- [ ] `MapBlock.getTopHeight(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:291)
- [ ] `MapBlock.setTopHeight(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:295)
- [ ] `MapBlock.getEffectiveHeight(2)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:299)
- [ ] `MapBlock.getEffectiveHeight(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:303)
- [ ] `MapBlock.getEffectiveTopHeight(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:312)
- [ ] `MapBlock.getBiome(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:321)
- [ ] `MapBlock.setBiome(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:325)
- [ ] `MapBlock.getOverlays(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:329)
- [ ] `MapBlock.getVerticalSlope(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:333)
- [ ] `MapBlock.setVerticalSlope(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:337)
- [ ] `MapBlock.getDiagonalSlope(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:341)
- [ ] `MapBlock.setDiagonalSlope(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:345)
- [ ] `MapBlock.setSlopeUnknown(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:349)
- [ ] `MapBlock.getNumberOfOverlays(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/MapBlock.java:353)

### `MapLayer`: it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))

- [ ] `MapLayer.MapLayer(2)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:18)
- [ ] `MapLayer.regionDetectionExists(2)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:27)
- [ ] `MapLayer.addRegionDetection(1)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:31)
- [ ] `MapLayer.getCompleteRegionDetection(2)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:43)
- [ ] `MapLayer.completeRegionDetectionContains(1)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:52)
- [ ] `MapLayer.tryAddingToCompleteRegionDetection(1)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:56)
- [ ] `MapLayer.getRegionDetection(2)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:74)
- [ ] `MapLayer.removeRegionDetection(2)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:104)
- [ ] `MapLayer.getRegionHighlightExistenceTracker(0)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:123)
- [ ] `MapLayer.getMapRegions(0)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:127)
- [ ] `MapLayer.getDetectedRegions(0)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:131)
- [ ] `MapLayer.getLinkedCompleteWorldSaveDetectedRegions(0)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:135)
- [ ] `MapLayer.preDetection(0)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:139)
- [ ] `MapLayer.getCaveStart(0)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:145)
- [ ] `MapLayer.setCaveStart(1)`: its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))) (xaero/map/region/MapLayer.java:149)

### `MapPixel`: it uses Minecraft's `BlockState`

- [x] `MapPixel.VOID_COLOR` (xaero/map/region/MapPixel.java:23)
- [x] `MapPixel.DEFAULT_AMBIENT_LIGHT` (xaero/map/region/MapPixel.java:24)
- [x] `MapPixel.DEFAULT_AMBIENT_LIGHT_COLORED` (xaero/map/region/MapPixel.java:25)
- [x] `MapPixel.DEFAULT_AMBIENT_LIGHT_WHITE` (xaero/map/region/MapPixel.java:26)
- [x] `MapPixel.DEFAULT_MAX_DIRECT_LIGHT` (xaero/map/region/MapPixel.java:27)
- [x] `MapPixel.GLOWING_MAX_DIRECT_LIGHT` (xaero/map/region/MapPixel.java:28)
- [ ] `MapPixel.getVanillaTransparency(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:33)
- [ ] `MapPixel.getPixelColours(29)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:37)
- [ ] `MapPixel.getBlockBrightness(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:348)
- [ ] `MapPixel.getPixelLight(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:352)
- [ ] `MapPixel.getState(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:356)
- [ ] `MapPixel.setState(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:360)
- [ ] `MapPixel.setLight(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:364)
- [ ] `MapPixel.setGlowing(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/MapPixel.java:368)

### `MapRegion`: it extends `LeveledRegion`

- [x] `MapRegion.SIDE_LENGTH` (xaero/map/region/MapRegion.java:26)
- [ ] `MapRegion.MapRegion(10)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:55)
- [ ] `MapRegion.setParent(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:74)
- [ ] `MapRegion.destroyBufferUpdateObjects(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:78)
- [ ] `MapRegion.restoreBufferUpdateObjects(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:83)
- [ ] `MapRegion.requestRefresh(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:88)
- [ ] `MapRegion.requestRefresh(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:92)
- [ ] `MapRegion.cancelRefresh(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:102)
- [ ] `MapRegion.distanceFromPlayer(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:112)
- [ ] `MapRegion.leafDistanceFromPlayer(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:117)
- [ ] `MapRegion.clean(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:122)
- [ ] `MapRegion.writeCacheMetaData(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:136)
- [ ] `MapRegion.readCacheMetaData(7)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:146)
- [ ] `MapRegion.clearRegion(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:180)
- [ ] `MapRegion.isResting(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:206)
- [ ] `MapRegion.getWorldId(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:210)
- [ ] `MapRegion.getDimId(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:215)
- [ ] `MapRegion.getMwId(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:220)
- [ ] `MapRegion.getVersion(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:225)
- [ ] `MapRegion.setVersion(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:229)
- [ ] `MapRegion.isBeingWritten(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:236)
- [ ] `MapRegion.setBeingWritten(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:240)
- [ ] `MapRegion.getLoadState(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:244)
- [ ] `MapRegion.setLoadState(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:248)
- [ ] `MapRegion.getChunk(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:252)
- [ ] `MapRegion.setChunk(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:256)
- [ ] `MapRegion.getInitialVersion(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:260)
- [ ] `MapRegion.setInitialVersion(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:264)
- [ ] `MapRegion.getPixelResultBuffer(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:268)
- [ ] `MapRegion.getMutableGlobalPos(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:272)
- [ ] `MapRegion.getRegionFile(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:276)
- [ ] `MapRegion.setRegionFile(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:281)
- [ ] `MapRegion.getSaveExists(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:285)
- [ ] `MapRegion.setSaveExists(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:289)
- [ ] `MapRegion.getLastSaveTime(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:293)
- [ ] `MapRegion.setLastSaveTime(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:297)
- [ ] `MapRegion.isRefreshing(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:301)
- [ ] `MapRegion.setRefreshing(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:306)
- [ ] `MapRegion.isWritingPaused(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:310)
- [ ] `MapRegion.pushWriterPause(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:314)
- [ ] `MapRegion.popWriterPause(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:320)
- [ ] `MapRegion.hasVersion(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:326)
- [ ] `MapRegion.isNormalMapData(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:330)
- [ ] `MapRegion.getLastVisited(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:334)
- [ ] `MapRegion.getTimeSinceVisit(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:338)
- [ ] `MapRegion.registerVisit(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:342)
- [ ] `MapRegion.countChunks(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:346)
- [ ] `MapRegion.getBiomeRegistry(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:361)
- [ ] `MapRegion.putLeaf(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:365)
- [ ] `MapRegion.putTexture(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:369)
- [ ] `MapRegion.getTexture(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:373)
- [ ] `MapRegion.get(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:378)
- [ ] `MapRegion.remove(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:387)
- [ ] `MapRegion.loadingAnimation(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:392)
- [ ] `MapRegion.cleanAndCacheRequestsBlocked(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:397)
- [ ] `MapRegion.shouldBeProcessed(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:402)
- [ ] `MapRegion.isLoaded(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:407)
- [ ] `MapRegion.shouldEndProcessingAfterUpload(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:412)
- [ ] `MapRegion.onProcessingEnd(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:417)
- [ ] `MapRegion.preCache(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:423)
- [ ] `MapRegion.postCache(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:428)
- [ ] `MapRegion.skipCaching(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:439)
- [ ] `MapRegion.findCacheFile(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:444)
- [ ] `MapRegion.onCurrentDimFinish(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:449)
- [ ] `MapRegion.onLimiterRemoval(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:465)
- [ ] `MapRegion.afterLimiterRemoval(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:483)
- [ ] `MapRegion.addDebugLines(4)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:489)
- [ ] `MapRegion.getExtraInfo(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:514)
- [ ] `MapRegion.createTexture(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:519)
- [ ] `MapRegion.createTileChunk(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:523)
- [ ] `MapRegion.checkForUpdates(9)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:527)
- [ ] `MapRegion.getReloadVersion(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:557)
- [ ] `MapRegion.setReloadVersion(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:561)
- [ ] `MapRegion.setCacheHashCode(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:565)
- [ ] `MapRegion.getCacheHashCode(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:569)
- [ ] `MapRegion.setCaveStart(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:573)
- [ ] `MapRegion.getCaveStart(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:577)
- [ ] `MapRegion.processWhenLoadedChunksExist(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:581)
- [ ] `MapRegion.updateLeafTextureVersion(3)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:593)
- [ ] `MapRegion.onDimensionClear(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:622)
- [ ] `MapRegion.restoreMetaData(7)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:630)
- [ ] `MapRegion.loadCacheTextures(9)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:659)
- [ ] `MapRegion.onCacheLoadFailed(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:690)
- [ ] `MapRegion.convertCacheToOutdated(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:694)
- [ ] `MapRegion.shouldAffectLoadingRequestFrequency(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:710)
- [ ] `MapRegion.hasRemovableSourceData(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:715)
- [ ] `MapRegion.getHighlightsHash(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:720)
- [ ] `MapRegion.setHighlightsHash(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:724)
- [ ] `MapRegion.shouldLeafAffectCache(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:728)
- [ ] `MapRegion.updateTargetHighlightsHash(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:733)
- [ ] `MapRegion.getTargetHighlightsHash(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:737)
- [ ] `MapRegion.setHasHadTerrain(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:741)
- [ ] `MapRegion.unsetHasHadTerrain(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:745)
- [ ] `MapRegion.hasHadTerrain(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:749)
- [ ] `MapRegion.hasLookedForCache(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:753)
- [ ] `MapRegion.setLookedForCache(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:758)
- [ ] `MapRegion.caveStartOutdated(2)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:762)
- [ ] `MapRegion.updateCaveMode(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:771)
- [ ] `MapRegion.getUpToDateCaveStart(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:777)
- [ ] `MapRegion.shouldConvertCacheToOutdatedOnFinishDim(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:781)
- [ ] `MapRegion.setOutdatedWithOtherLayers(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:785)
- [ ] `MapRegion.isOutdatedWithOtherLayers(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:789)
- [ ] `MapRegion.getCaveDepth(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:793)
- [ ] `MapRegion.canRequestReload_unsynced(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:797)
- [ ] `MapRegion.isResaving(0)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:804)
- [ ] `MapRegion.setResaving(1)`: its class isn't translated (it extends `LeveledRegion`) (xaero/map/region/MapRegion.java:808)

### `MapTile`: it implements `PoolUnit` (xaero.map.pool)

- [x] `MapTile.CURRENT_WORLD_INTERPRETATION_VERSION` (xaero/map/region/MapTile.java:6)
- [ ] `MapTile.MapTile(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:16)
- [ ] `MapTile.create(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:20)
- [ ] `MapTile.isLoaded(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:31)
- [ ] `MapTile.setLoaded(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:35)
- [ ] `MapTile.getBlock(2)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:39)
- [ ] `MapTile.getBlockColumn(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:43)
- [ ] `MapTile.setBlock(3)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:47)
- [ ] `MapTile.getChunkX(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:51)
- [ ] `MapTile.getChunkZ(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:55)
- [ ] `MapTile.wasWrittenOnce(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:59)
- [ ] `MapTile.setWrittenOnce(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:63)
- [ ] `MapTile.getWorldInterpretationVersion(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:67)
- [ ] `MapTile.setWorldInterpretationVersion(1)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:71)
- [ ] `MapTile.getWrittenCaveStart(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:75)
- [ ] `MapTile.setWrittenCave(2)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:79)
- [ ] `MapTile.getWrittenCaveDepth(0)`: its class isn't translated (it implements `PoolUnit` (xaero.map.pool)) (xaero/map/region/MapTile.java:84)

### `MapTileChunk`: it uses MapRegion, which isn't translated (it extends `LeveledRegion`)

- [x] `MapTileChunk.SIDE_LENGTH` (xaero/map/region/MapTileChunk.java:25)
- [ ] `MapTileChunk.MapTileChunk(3)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:40)
- [ ] `MapTileChunk.createLeafTexture(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:47)
- [ ] `MapTileChunk.updateBuffers(6)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:51)
- [ ] `MapTileChunk.putColour(8)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:156)
- [ ] `MapTileChunk.getNeighbourTileChunk(4)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:161)
- [ ] `MapTileChunk.clean(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:198)
- [ ] `MapTileChunk.getX(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:214)
- [ ] `MapTileChunk.getZ(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:218)
- [ ] `MapTileChunk.getTileGridsCache(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:222)
- [ ] `MapTileChunk.getLoadState(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:226)
- [ ] `MapTileChunk.setLoadState(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:230)
- [ ] `MapTileChunk.getTile(2)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:234)
- [ ] `MapTileChunk.setTile(5)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:238)
- [ ] `MapTileChunk.getInRegion(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:278)
- [ ] `MapTileChunk.wasChanged(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:282)
- [ ] `MapTileChunk.setChanged(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:286)
- [ ] `MapTileChunk.getTimer(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:290)
- [ ] `MapTileChunk.decTimer(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:294)
- [ ] `MapTileChunk.includeInSave(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:298)
- [ ] `MapTileChunk.unincludeInSave(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:302)
- [ ] `MapTileChunk.resetHeights(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:306)
- [ ] `MapTileChunk.getToUpdateBuffers(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:310)
- [ ] `MapTileChunk.setToUpdateBuffers(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:314)
- [ ] `MapTileChunk.getLeafTexture(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:318)
- [ ] `MapTileChunk.writeCacheData(4)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:322)
- [ ] `MapTileChunk.readCacheData(8)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:325)
- [ ] `MapTileChunk.toString(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:361)
- [ ] `MapTileChunk.hasHadTerrain(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:366)
- [ ] `MapTileChunk.setHasHadTerrain(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:370)
- [ ] `MapTileChunk.unsetHasHadTerrain(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:375)
- [ ] `MapTileChunk.hasHighlights(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:379)
- [ ] `MapTileChunk.setHasHighlights(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:383)
- [ ] `MapTileChunk.hasHighlightsIfUndiscovered(0)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:387)
- [ ] `MapTileChunk.setHasHighlightsIfUndiscovered(1)`: its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)) (xaero/map/region/MapTileChunk.java:391)

### `MapUpdateFastConfig`

- [ ] `MapUpdateFastConfig.MapUpdateFastConfig(1)`: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))) (xaero/map/region/MapUpdateFastConfig.java:18)

### `Overlay`: it extends `MapPixel`

- [ ] `Overlay.Overlay(3)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:19)
- [ ] `Overlay.write(3)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:23)
- [ ] `Overlay.isWater(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:30)
- [ ] `Overlay.getParametres(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:34)
- [ ] `Overlay.getPixelColour(25)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:41)
- [ ] `Overlay.toRenderString(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:101)
- [ ] `Overlay.equals(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:105)
- [ ] `Overlay.fillManagerKeyHolder(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:109)
- [ ] `Overlay.increaseOpacity(1)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:115)
- [ ] `Overlay.getOpacity(0)`: its class isn't translated (it extends `MapPixel`) (xaero/map/region/Overlay.java:126)

### `OverlayBuilder`: it uses Overlay, which isn't translated (it extends `MapPixel`)

- [x] `OverlayBuilder.MAX_OVERLAYS` (xaero/map/region/OverlayBuilder.java:12)
- [ ] `OverlayBuilder.OverlayBuilder(1)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:19)
- [ ] `OverlayBuilder.startBuilding(0)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:30)
- [ ] `OverlayBuilder.build(5)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:35)
- [ ] `OverlayBuilder.isEmpty(0)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:72)
- [ ] `OverlayBuilder.getCurrentOverlay(0)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:76)
- [ ] `OverlayBuilder.finishBuilding(1)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:85)
- [ ] `OverlayBuilder.getOverlayBiome(0)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:97)
- [ ] `OverlayBuilder.setOverlayBiome(1)`: its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)) (xaero/map/region/OverlayBuilder.java:101)

### `OverlayManager`: it uses Minecraft's `BlockState`

- [ ] `OverlayManager.OverlayManager(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/OverlayManager.java:11)
- [ ] `OverlayManager.getOriginal(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/OverlayManager.java:16)
- [ ] `OverlayManager.getOriginal(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/OverlayManager.java:21)
- [ ] `OverlayManager.getNumberOfUniqueOverlays(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (xaero/map/region/OverlayManager.java:41)

### `UnknownBlockState`: it extends Minecraft's `BlockState`

- [ ] `UnknownBlockState.UnknownBlockState(1)`: its class isn't translated (it extends Minecraft's `BlockState`) (xaero/map/region/state/UnknownBlockState.java:15)
- [ ] `UnknownBlockState.write(1)`: its class isn't translated (it extends Minecraft's `BlockState`) (xaero/map/region/state/UnknownBlockState.java:21)
- [ ] `UnknownBlockState.toString(0)`: its class isn't translated (it extends Minecraft's `BlockState`) (xaero/map/region/state/UnknownBlockState.java:25)

### `BranchRegionTexture`: it extends `RegionTexture`

- [ ] `BranchRegionTexture.BranchRegionTexture(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:32)
- [ ] `BranchRegionTexture.reset(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:37)
- [ ] `BranchRegionTexture.checkForUpdates(5)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:47)
- [ ] `BranchRegionTexture.isChildUpdated(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:94)
- [ ] `BranchRegionTexture.preUpload(7)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:108)
- [ ] `BranchRegionTexture.postUpload(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:120)
- [ ] `BranchRegionTexture.uploadBuffer(6)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:124)
- [ ] `BranchRegionTexture.copyNonColorData(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:136)
- [ ] `BranchRegionTexture.uploadNonCache(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:163)
- [ ] `BranchRegionTexture.onCacheUploadRequested(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:241)
- [ ] `BranchRegionTexture.onDownloadedBuffer(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:247)
- [ ] `BranchRegionTexture.endPBODownload(2)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:256)
- [ ] `BranchRegionTexture.hasSourceData(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:274)
- [ ] `BranchRegionTexture.addDebugLines(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:279)
- [ ] `BranchRegionTexture.onTextureDeletion(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:290)
- [ ] `BranchRegionTexture.requestDownload(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:304)
- [ ] `BranchRegionTexture.writeCacheMapData(4)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:309)
- [ ] `BranchRegionTexture.readCacheData(10)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/BranchRegionTexture.java:318)

### `BranchRegionTexture.ChildTextureInfo`: it uses RegionTexture, which isn't translated (it's generic)

- [ ] `BranchRegionTexture.ChildTextureInfo.getColorTextureForUpdate(1)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:344)
- [ ] `BranchRegionTexture.ChildTextureInfo.getTextureVersion(0)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:355)
- [ ] `BranchRegionTexture.ChildTextureInfo.hasLight(0)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:359)
- [ ] `BranchRegionTexture.ChildTextureInfo.onUpdate(1)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:363)
- [ ] `BranchRegionTexture.ChildTextureInfo.onParentDeletion(0)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:370)
- [ ] `BranchRegionTexture.ChildTextureInfo.getReferenceColorTexture(0)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:376)
- [ ] `BranchRegionTexture.ChildTextureInfo.toString(0)`: its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)) (xaero/map/region/texture/BranchRegionTexture.java:380)

### `BranchTextureRenderer`: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)

- [ ] `BranchTextureRenderer.BranchTextureRenderer(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/region/texture/BranchTextureRenderer.java:32)
- [ ] `BranchTextureRenderer.getEmptyTexture(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/region/texture/BranchTextureRenderer.java:39)
- [ ] `BranchTextureRenderer.ensureRenderTarget(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/region/texture/BranchTextureRenderer.java:44)
- [ ] `BranchTextureRenderer.render(7)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/region/texture/BranchTextureRenderer.java:52)
- [ ] `BranchTextureRenderer.renderCorner(4)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/region/texture/BranchTextureRenderer.java:98)
- [ ] `BranchTextureRenderer.getGlEmptyTexture(0)`: its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)) (xaero/map/region/texture/BranchTextureRenderer.java:105)

### `ExportLeafRegionTexture`: it extends `LeafRegionTexture`

- [x] `ExportLeafRegionTexture.MIP_MAP_LEVELS` (xaero/map/region/texture/ExportLeafRegionTexture.java:8)
- [ ] `ExportLeafRegionTexture.ExportLeafRegionTexture(1)`: its class isn't translated (it extends `LeafRegionTexture`) (xaero/map/region/texture/ExportLeafRegionTexture.java:10)
- [ ] `ExportLeafRegionTexture.applyHighlights(2)`: its class isn't translated (it extends `LeafRegionTexture`) (xaero/map/region/texture/ExportLeafRegionTexture.java:14)
- [ ] `ExportLeafRegionTexture.getMipMapLevels(0)`: its class isn't translated (it extends `LeafRegionTexture`) (xaero/map/region/texture/ExportLeafRegionTexture.java:18)

### `LeafRegionTexture`: it extends `RegionTexture`

- [ ] `LeafRegionTexture.LeafRegionTexture(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:27)
- [ ] `LeafRegionTexture.postBufferUpdate(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:32)
- [ ] `LeafRegionTexture.preUpload(7)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:37)
- [ ] `LeafRegionTexture.postUpload(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:57)
- [ ] `LeafRegionTexture.canUpload(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:71)
- [ ] `LeafRegionTexture.isUploaded(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:76)
- [ ] `LeafRegionTexture.hasSourceData(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:81)
- [ ] `LeafRegionTexture.uploadNonCache(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:86)
- [ ] `LeafRegionTexture.applyHighlights(3)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:125)
- [ ] `LeafRegionTexture.postBufferWrite(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:156)
- [ ] `LeafRegionTexture.updateTextureVersion(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:167)
- [ ] `LeafRegionTexture.addDebugLines(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:173)
- [ ] `LeafRegionTexture.writeCacheMapData(4)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:190)
- [ ] `LeafRegionTexture.readCacheData(10)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:196)
- [ ] `LeafRegionTexture.resetHeights(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:220)
- [ ] `LeafRegionTexture.shouldBeUsedForBranchUpdate(1)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:225)
- [ ] `LeafRegionTexture.shouldHaveContentForBranchUpdate(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:230)
- [ ] `LeafRegionTexture.deleteTexturesAndBuffers(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:235)
- [ ] `LeafRegionTexture.prepareBuffer(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:248)
- [ ] `LeafRegionTexture.getTileChunk(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:254)
- [ ] `LeafRegionTexture.shouldIncludeInCache(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:258)
- [ ] `LeafRegionTexture.requestHighlightOnlyUpload(0)`: its class isn't translated (it extends `RegionTexture`) (xaero/map/region/texture/LeafRegionTexture.java:263)

### `RegionTexture`: it's generic

- [ ] `RegionTexture.DEFAULT_INTERNAL_FORMAT`: it uses Minecraft's `GpuFormat` (xaero/map/region/texture/RegionTexture.java:37)
- [x] `RegionTexture.PBO_UNPACK_LENGTH` (xaero/map/region/texture/RegionTexture.java:38)
- [x] `RegionTexture.PBO_PACK_LENGTH` (xaero/map/region/texture/RegionTexture.java:39)
- [x] `RegionTexture.ONE_BIOME_PALETTE_DATA` (xaero/map/region/texture/RegionTexture.java:40)
- [ ] `RegionTexture.OLD_HEIGHT_VALUES_SUPPORT`: it uses `ThreadLocal` (xaero/map/region/texture/RegionTexture.java:41)
- [ ] `RegionTexture.RegionTexture(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:60)
- [ ] `RegionTexture.setupTextureParameters(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:68)
- [ ] `RegionTexture.prepareBuffer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:71)
- [ ] `RegionTexture.getMipMapLevels(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:79)
- [ ] `RegionTexture.bindColorTexture(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:83)
- [ ] `RegionTexture.uploadBuffer(6)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:104)
- [ ] `RegionTexture.postBufferWrite(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:125)
- [ ] `RegionTexture.uploadBufferHelper(4)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:128)
- [ ] `RegionTexture.uploadBufferHelper(5)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:134)
- [ ] `RegionTexture.applyHighlights(3)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:208)
- [ ] `RegionTexture.onDownloadedBuffer(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:214)
- [ ] `RegionTexture.endPBODownload(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:225)
- [ ] `RegionTexture.ensurePackPBO(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:235)
- [ ] `RegionTexture.ensureUnpackPBO(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:241)
- [ ] `RegionTexture.writeToUnpackPBO(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:247)
- [ ] `RegionTexture.deleteColorBuffer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:257)
- [ ] `RegionTexture.deletePBOs(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:270)
- [ ] `RegionTexture.writeCacheMapData(4)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:285)
- [ ] `RegionTexture.readCacheData(10)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:306)
- [ ] `RegionTexture.saveBiomeIndexStorage(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:455)
- [ ] `RegionTexture.loadBiomeIndexStorage(3)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:489)
- [ ] `RegionTexture.deleteTexturesAndBuffers(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:539)
- [ ] `RegionTexture.getColorBuffer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:554)
- [ ] `RegionTexture.getDirectColorBuffer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:558)
- [ ] `RegionTexture.setShouldDownloadFromPBO(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:562)
- [ ] `RegionTexture.getColorBufferFormat(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:566)
- [ ] `RegionTexture.shouldDownloadFromPBO(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:570)
- [ ] `RegionTexture.getTimer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:574)
- [ ] `RegionTexture.decTimer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:578)
- [ ] `RegionTexture.resetTimer(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:582)
- [ ] `RegionTexture.getGlColorTexture(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:586)
- [ ] `RegionTexture.onTextureDeletion(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:590)
- [ ] `RegionTexture.shouldUpload(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:594)
- [ ] `RegionTexture.setToUpload(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:598)
- [ ] `RegionTexture.isCachePrepared(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:602)
- [ ] `RegionTexture.setCachePrepared(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:606)
- [ ] `RegionTexture.canUpload(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:610)
- [ ] `RegionTexture.isUploaded(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:614)
- [ ] `RegionTexture.getTextureVersion(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:618)
- [ ] `RegionTexture.getBufferedTextureVersion(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:622)
- [ ] `RegionTexture.setBufferedTextureVersion(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:626)
- [ ] `RegionTexture.getRegion(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:630)
- [ ] `RegionTexture.updateTextureVersion(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:634)
- [ ] `RegionTexture.getHeight(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:638)
- [ ] `RegionTexture.putHeight(3)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:644)
- [ ] `RegionTexture.putHeight(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:649)
- [ ] `RegionTexture.removeHeight(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:654)
- [ ] `RegionTexture.getTopHeight(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:659)
- [ ] `RegionTexture.putTopHeight(3)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:665)
- [ ] `RegionTexture.putTopHeight(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:670)
- [ ] `RegionTexture.removeTopHeight(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:675)
- [ ] `RegionTexture.ensureBiomeIndexStorage(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:680)
- [ ] `RegionTexture.getBiome(2)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:693)
- [ ] `RegionTexture.setBiome(3)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:703)
- [ ] `RegionTexture.getTextureHasLight(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:764)
- [ ] `RegionTexture.addDebugLines(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:768)
- [ ] `RegionTexture.onCacheUploadRequested(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:780)
- [ ] `RegionTexture.shouldBeUsedForBranchUpdate(1)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:783)
- [ ] `RegionTexture.shouldHaveContentForBranchUpdate(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:787)
- [ ] `RegionTexture.shouldIncludeInCache(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:791)
- [ ] `RegionTexture.getBiomes(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:795)
- [ ] `RegionTexture.resetBiomes(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:799)
- [ ] `RegionTexture.hasSourceData(0)`: it has no body (xaero/map/region/texture/RegionTexture.java:803)
- [ ] `RegionTexture.preUpload(7)`: it has no body (xaero/map/region/texture/RegionTexture.java:805)
- [ ] `RegionTexture.postUpload(3)`: it has no body (xaero/map/region/texture/RegionTexture.java:815)
- [ ] `RegionTexture.uploadNonCache(3)`: it has no body (xaero/map/region/texture/RegionTexture.java:817)
- [ ] `RegionTexture.getBufferHasLight(0)`: its class isn't translated (it's generic) (xaero/map/region/texture/RegionTexture.java:819)

### `RegionTextureBiomes`: it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))

- [ ] `RegionTextureBiomes.RegionTextureBiomes(2)`: its class isn't translated (it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))) (xaero/map/region/texture/RegionTextureBiomes.java:12)
- [ ] `RegionTextureBiomes.getBiomeIndexStorage(0)`: its class isn't translated (it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))) (xaero/map/region/texture/RegionTextureBiomes.java:17)
- [ ] `RegionTextureBiomes.getRegionBiomePalette(0)`: its class isn't translated (it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))) (xaero/map/region/texture/RegionTextureBiomes.java:21)

### `XaeroWorldMapServerCore`

- [ ] `XaeroWorldMapServerCore.onServerWorldInfo(1)`: a nullable object kept in a changing static (xaero/map/server/core/XaeroWorldMapServerCore.java:8)

### `ServerEvents`

- [ ] `ServerEvents.ServerEvents(1)`: it stores a second reference to an object (Rust would need it copied or shared) (xaero/map/server/events/ServerEvents.java:8)

### `IMinecraftServer`: it's an interface

- [ ] `IMinecraftServer.getXaeroWorldMapServerData(0)`: it has no body (xaero/map/server/IMinecraftServer.java:4)
- [ ] `IMinecraftServer.setXaeroWorldMapServerData(1)`: it has no body (xaero/map/server/IMinecraftServer.java:6)

### `LevelMapProperties`

- [ ] `LevelMapProperties.write(1)`: it's overloaded with the same number of parameters (xaero/map/server/level/LevelMapProperties.java:13)
- [ ] `LevelMapProperties.read(1)`: it's overloaded with the same number of parameters (xaero/map/server/level/LevelMapProperties.java:17)
- [x] `LevelMapProperties.isUsable(0)` (xaero/map/server/level/LevelMapProperties.java:30)
- [x] `LevelMapProperties.setUsable(1)` (xaero/map/server/level/LevelMapProperties.java:34)
- [x] `LevelMapProperties.getId(0)` (xaero/map/server/level/LevelMapProperties.java:38)
- [ ] `LevelMapProperties.read(1)`: it's overloaded with the same number of parameters (xaero/map/server/level/LevelMapProperties.java:42)
- [ ] `LevelMapProperties.write(1)`: it's overloaded with the same number of parameters (xaero/map/server/level/LevelMapProperties.java:48)

### `LevelMapPropertiesIO`

- [x] `LevelMapPropertiesIO.FILE_NAME` (xaero/map/server/level/LevelMapPropertiesIO.java:16)
- [ ] `LevelMapPropertiesIO.load(2)`: a try/finally (xaero/map/server/level/LevelMapPropertiesIO.java:18)
- [ ] `LevelMapPropertiesIO.save(2)`: a try-with-resources over something other than a file reader or writer (xaero/map/server/level/LevelMapPropertiesIO.java:33)

### `MinecraftServerData`: a Map or Set keyed by a Path

- [ ] `MinecraftServerData.MinecraftServerData(2)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/map/server/MinecraftServerData.java:22)
- [ ] `MinecraftServerData.getLevelProperties(1)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/map/server/MinecraftServerData.java:29)
- [ ] `MinecraftServerData.getSyncedPlayerTrackerSystemManager(0)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/map/server/MinecraftServerData.java:60)
- [ ] `MinecraftServerData.getSyncedPlayerTracker(0)`: its class isn't translated (a Map or Set keyed by a Path) (xaero/map/server/MinecraftServerData.java:64)
- [ ] `MinecraftServerData.get(1)`: it uses MinecraftServerData, which isn't translated (a Map or Set keyed by a Path) (xaero/map/server/MinecraftServerData.java:68)

### `MineraftServerDataInitializer`

- [ ] `MineraftServerDataInitializer.init(1)`: it uses Minecraft's `MinecraftServer` (xaero/map/server/MineraftServerDataInitializer.java:9)

### `ArgonautsSyncedPlayerTrackerSystem`: it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)

- [ ] `ArgonautsSyncedPlayerTrackerSystem.getTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)) (xaero/map/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:11)
- [ ] `ArgonautsSyncedPlayerTrackerSystem.isPartySystem(0)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)) (xaero/map/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:18)
- [ ] `ArgonautsSyncedPlayerTrackerSystem.getPartyTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)) (xaero/map/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:23)
- [ ] `ArgonautsSyncedPlayerTrackerSystem.getGuildTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)) (xaero/map/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java:33)

### `SupportArgonautsServer`: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `SupportArgonautsServer.getSyncedPlayerTrackerSystem(0)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/mods/argonauts/SupportArgonautsServer.java:8)

### `FTBTeamsSyncedPlayerTrackerSystem`: it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)

- [ ] `FTBTeamsSyncedPlayerTrackerSystem.getTrackingLevel(2)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)) (xaero/map/server/mods/ftbteams/FTBTeamsSyncedPlayerTrackerSystem.java:10)
- [ ] `FTBTeamsSyncedPlayerTrackerSystem.isPartySystem(0)`: its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)) (xaero/map/server/mods/ftbteams/FTBTeamsSyncedPlayerTrackerSystem.java:31)

### `SupportFTBTeamsServer`: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `SupportFTBTeamsServer.getSyncedPlayerTrackerSystem(0)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/mods/ftbteams/SupportFTBTeamsServer.java:8)

### `ServerPlayerOpacData`


### `SupportOPACServer`

- [ ] `SupportOPACServer.isPositionSyncAllowed(3)`: it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)) (xaero/map/server/mods/opac/SupportOPACServer.java:11)
- [ ] `SupportOPACServer.getReceiveLocationsFromMutualAlliesConfigValue(1)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/mods/opac/SupportOPACServer.java:24)
- [ ] `SupportOPACServer.getReceiveLocationsFromPartyConfigValue(1)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/mods/opac/SupportOPACServer.java:30)
- [ ] `SupportOPACServer.updateShareLocationConfigValues(2)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/mods/opac/SupportOPACServer.java:36)
- [ ] `SupportOPACServer.getPlayerOpacData(1)`: it uses `Object` (xaero/map/server/mods/opac/SupportOPACServer.java:44)

### `SupportMinimapServer`

- [ ] `SupportMinimapServer.SupportMinimapServer(0)`: it uses `XaeroMinimapServer` (xaero.common.server).SERVER_COMPATIBILITY (xaero/map/server/mods/SupportMinimapServer.java:10)
- [x] `SupportMinimapServer.supportsTrackedPlayers(0)` (xaero/map/server/mods/SupportMinimapServer.java:21)
- [ ] `SupportMinimapServer.playerSupportsTrackedPlayers(1)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/mods/SupportMinimapServer.java:25)

### `SupportServerMods`

- [ ] `SupportServerMods.ftbTeams`: it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/mods/SupportServerMods.java:8)
- [ ] `SupportServerMods.argonauts`: it uses SupportArgonautsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/mods/SupportServerMods.java:9)
- [x] `SupportServerMods.opac` (xaero/map/server/mods/SupportServerMods.java:10)
- [x] `SupportServerMods.minimap` (xaero/map/server/mods/SupportServerMods.java:11)
- [ ] `SupportServerMods.check(0)`: a try/catch (ClassNotFoundException) (xaero/map/server/mods/SupportServerMods.java:13)
- [ ] `SupportServerMods.hasFtbTeams(0)`: it uses SupportServerMods.ftbTeams, which isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/map/server/mods/SupportServerMods.java:39)
- [ ] `SupportServerMods.getFtbTeams(0)`: it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/mods/SupportServerMods.java:43)
- [ ] `SupportServerMods.hasArgonauts(0)`: it uses SupportServerMods.argonauts, which isn't translated (it uses SupportArgonautsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))) (xaero/map/server/mods/SupportServerMods.java:47)
- [ ] `SupportServerMods.getArgonauts(0)`: it uses SupportArgonautsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/mods/SupportServerMods.java:51)
- [x] `SupportServerMods.hasOpac(0)` (xaero/map/server/mods/SupportServerMods.java:55)
- [x] `SupportServerMods.getOpac(0)` (xaero/map/server/mods/SupportServerMods.java:59)
- [x] `SupportServerMods.hasMinimap(0)` (xaero/map/server/mods/SupportServerMods.java:63)
- [x] `SupportServerMods.getMinimap(0)` (xaero/map/server/mods/SupportServerMods.java:67)

### `IServerPlayer`: it's an interface

- [ ] `IServerPlayer.getXaeroWorldMapPlayerData(0)`: it has no body (xaero/map/server/player/IServerPlayer.java:4)
- [ ] `IServerPlayer.setXaeroWorldMapPlayerData(1)`: it has no body (xaero/map/server/player/IServerPlayer.java:6)

### `ServerPlayerData`: it uses `UUID` (java.util)

- [ ] `ServerPlayerData.ServerPlayerData(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:17)
- [ ] `ServerPlayerData.getLastSyncedData(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:21)
- [ ] `ServerPlayerData.ensureLastSyncedData(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:25)
- [ ] `ServerPlayerData.getCurrentlySyncedPlayers(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:33)
- [ ] `ServerPlayerData.ensureCurrentlySyncedPlayers(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:37)
- [ ] `ServerPlayerData.getLastTrackedPlayerSync(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:45)
- [ ] `ServerPlayerData.setLastTrackedPlayerSync(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:49)
- [ ] `ServerPlayerData.get(1)`: it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:53)
- [ ] `ServerPlayerData.hasMod(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:62)
- [ ] `ServerPlayerData.setClientModNetworkVersion(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:66)
- [ ] `ServerPlayerData.getClientModNetworkVersion(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:70)
- [ ] `ServerPlayerData.setOpacData(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:74)
- [ ] `ServerPlayerData.getOpacData(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/player/ServerPlayerData.java:78)

### `ServerPlayerTickHandler`

- [ ] `ServerPlayerTickHandler.tick(1)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/player/ServerPlayerTickHandler.java:8)

### `ISyncedPlayerTrackerSystem`: it's an interface

- [ ] `ISyncedPlayerTrackerSystem.getTrackingLevel(2)`: it has no body (xaero/map/server/radar/tracker/ISyncedPlayerTrackerSystem.java:6)
- [ ] `ISyncedPlayerTrackerSystem.isPartySystem(0)`: it has no body (xaero/map/server/radar/tracker/ISyncedPlayerTrackerSystem.java:8)

### `SyncedPlayerTracker`

- [ ] `SyncedPlayerTracker.onTick(4)`: it uses Minecraft's `MinecraftServer` (xaero/map/server/radar/tracker/SyncedPlayerTracker.java:19)
- [ ] `SyncedPlayerTracker.sendRemovePacket(3)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/radar/tracker/SyncedPlayerTracker.java:112)
- [ ] `SyncedPlayerTracker.sendTrackedPlayerPacket(3)`: it uses Minecraft's `ServerPlayer` (xaero/map/server/radar/tracker/SyncedPlayerTracker.java:117)

### `SyncedPlayerTrackerSystemManager`: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)

- [ ] `SyncedPlayerTrackerSystemManager.register(2)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/radar/tracker/SyncedPlayerTrackerSystemManager.java:10)
- [ ] `SyncedPlayerTrackerSystemManager.getSystems(0)`: its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/server/radar/tracker/SyncedPlayerTrackerSystemManager.java:19)

### `SyncedTrackedPlayer`: it uses `UUID` (java.util)

- [ ] `SyncedTrackedPlayer.SyncedTrackedPlayer(5)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:15)
- [ ] `SyncedTrackedPlayer.setPos(3)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:23)
- [ ] `SyncedTrackedPlayer.setDimension(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:30)
- [ ] `SyncedTrackedPlayer.getId(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:35)
- [ ] `SyncedTrackedPlayer.getX(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:39)
- [ ] `SyncedTrackedPlayer.getY(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:43)
- [ ] `SyncedTrackedPlayer.getZ(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:47)
- [ ] `SyncedTrackedPlayer.getDimension(0)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:51)
- [ ] `SyncedTrackedPlayer.matchesEnough(2)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:55)
- [ ] `SyncedTrackedPlayer.update(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:62)
- [ ] `SyncedTrackedPlayer.copyFrom(1)`: its class isn't translated (it uses `UUID` (java.util)) (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java:66)

### `WorldMapServer`

- [x] `WorldMapServer.LOGGER` (xaero/map/server/WorldMapServer.java:8)
- [x] `WorldMapServer.load(0)` (xaero/map/server/WorldMapServer.java:11)
- [x] `WorldMapServer.loadLater(0)` (xaero/map/server/WorldMapServer.java:15)
- [x] `WorldMapServer.getServerEvents(0)` (xaero/map/server/WorldMapServer.java:19)

### `WorldMapServerFabric`: it extends `WorldMapServer`


### `ModSettings`

- [x] `ModSettings.format` (xaero/map/settings/ModSettings.java:24)
- [ ] `ModSettings.loadDefaultSettings(0)`: it uses `File` (java.io) (xaero/map/settings/ModSettings.java:27)
- [ ] `ModSettings.loadSettings(0)`: it calls ModSettings.loadDefaultSettings, which isn't translated (xaero/map/settings/ModSettings.java:35)
- [ ] `ModSettings.loadSettingsFile(1)`: it uses `File` (java.io) (xaero/map/settings/ModSettings.java:49)
- [x] `ModSettings.getRegionCacheHashCode(0)` (xaero/map/settings/ModSettings.java:202)
- [ ] `ModSettings.updateRegionCacheHashCode(0)`: it calls Minecraft's `Minecraft`.getInstance (xaero/map/settings/ModSettings.java:206)
- [ ] `ModSettings.canEditIngameSettings(0)`: it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/settings/ModSettings.java:249)

### `MapRunnerTask`

- [ ] `MapRunnerTask.run(1)`: it has no body (xaero/map/task/MapRunnerTask.java:6)

### `MapTeleporter`

- [ ] `MapTeleporter.teleport(6)`: it uses Minecraft's `Screen` (xaero/map/teleport/MapTeleporter.java:19)

### `DistanceUtils`

- [ ] `DistanceUtils.getDistanceText(3)`: it calls `NumberFormatUtils` (xaero.lib.common.util).getPrecisionFormat (xaero/map/util/DistanceUtils.java:16)
- [ ] `DistanceUtils.addDistanceRightClickOption(9)`: its body couldn't be read: an anonymous class (xaero/map/util/DistanceUtils.java:22)

### `ILinkedChainNode`: it's an interface

- [ ] `ILinkedChainNode.setNext(1)`: it has no body (xaero/map/util/linked/ILinkedChainNode.java:4)
- [ ] `ILinkedChainNode.setPrevious(1)`: it has no body (xaero/map/util/linked/ILinkedChainNode.java:6)
- [ ] `ILinkedChainNode.getNext(0)`: it has no body (xaero/map/util/linked/ILinkedChainNode.java:8)
- [ ] `ILinkedChainNode.getPrevious(0)`: it has no body (xaero/map/util/linked/ILinkedChainNode.java:10)
- [ ] `ILinkedChainNode.isDestroyed(0)`: it has no body (xaero/map/util/linked/ILinkedChainNode.java:12)
- [ ] `ILinkedChainNode.onDestroyed(0)`: it has no body (xaero/map/util/linked/ILinkedChainNode.java:14)

### `LinkedChain`: it's generic

- [ ] `LinkedChain.add(1)`: its class isn't translated (it's generic) (xaero/map/util/linked/LinkedChain.java:13)
- [ ] `LinkedChain.remove(1)`: its class isn't translated (it's generic) (xaero/map/util/linked/LinkedChain.java:30)
- [ ] `LinkedChain.destroy(0)`: its class isn't translated (it's generic) (xaero/map/util/linked/LinkedChain.java:54)
- [ ] `LinkedChain.reset(0)`: its class isn't translated (it's generic) (xaero/map/util/linked/LinkedChain.java:59)
- [ ] `LinkedChain.iterator(0)`: its body couldn't be read: an anonymous class (xaero/map/util/linked/LinkedChain.java:64)
- [ ] `LinkedChain.stream(0)`: its class isn't translated (it's generic) (xaero/map/util/linked/LinkedChain.java:100)

### `WorldMap`: it uses `ConfigChannel` (xaero.lib.common.config.channel)

- [x] `WorldMap.MOD_ID` (xaero/map/WorldMap.java:57)
- [x] `WorldMap.loaded` (xaero/map/WorldMap.java:58)
- [ ] `WorldMap.INSTANCE`: it uses WorldMap, which isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:59)
- [x] `WorldMap.MINIMAP_COMPATIBILITY_VERSION` (xaero/map/WorldMap.java:60)
- [x] `WorldMap.LOGGER` (xaero/map/WorldMap.java:61)
- [x] `WorldMap.versionID_minecraft` (xaero/map/WorldMap.java:62)
- [x] `WorldMap.newestUpdateID` (xaero/map/WorldMap.java:64)
- [x] `WorldMap.isOutdated` (xaero/map/WorldMap.java:65)
- [x] `WorldMap.latestVersion` (xaero/map/WorldMap.java:66)
- [x] `WorldMap.latestVersionMD5` (xaero/map/WorldMap.java:67)
- [ ] `WorldMap.events`: it uses ClientEvents, which isn't translated (it uses Minecraft's `RealmsServer`) (xaero/map/WorldMap.java:68)
- [x] `WorldMap.modEvents` (xaero/map/WorldMap.java:69)
- [ ] `WorldMap.controlsRegister`: it uses ControlsRegister, which isn't translated (it uses Minecraft's `KeyMapping`) (xaero/map/WorldMap.java:70)
- [ ] `WorldMap.waypointSymbolCreator`: it uses WaypointSymbolCreator, which isn't translated (it uses Minecraft's `Minecraft`) (xaero/map/WorldMap.java:71)
- [ ] `WorldMap.bufferDeallocator`: it uses ByteBufferDeallocator, which isn't translated (it uses `Object`) (xaero/map/WorldMap.java:72)
- [x] `WorldMap.textureUploadBenchmark` (xaero/map/WorldMap.java:73)
- [ ] `WorldMap.overlayManager`: it uses OverlayManager, which isn't translated (it uses Minecraft's `BlockState`) (xaero/map/WorldMap.java:74)
- [ ] `WorldMap.oldFormatSupport`: it uses OldFormatSupport, which isn't translated (it uses Minecraft's `BlockState`) (xaero/map/WorldMap.java:75)
- [ ] `WorldMap.pngExporter`: it uses PNGExporter, which isn't translated (it uses `Calendar` (java.util)) (xaero/map/WorldMap.java:76)
- [ ] `WorldMap.normalTextureUploadPool`: it uses Normal, which isn't translated (it extends `TextureUploadPool`) (xaero/map/WorldMap.java:77)
- [ ] `WorldMap.branchUpdatePool`: it uses BranchUpdate, which isn't translated (it extends `TextureUploadPool`) (xaero/map/WorldMap.java:78)
- [ ] `WorldMap.branchUpdateAllocatePool`: it uses BranchUpdate, which isn't translated (it extends `TextureUploadPool`) (xaero/map/WorldMap.java:79)
- [ ] `WorldMap.branchDownloadPool`: it uses BranchDownload, which isn't translated (it extends `TextureUploadPool`) (xaero/map/WorldMap.java:80)
- [ ] `WorldMap.subsequentNormalTextureUploadPool`: it uses SubsequentNormal, which isn't translated (it extends `TextureUploadPool`) (xaero/map/WorldMap.java:81)
- [ ] `WorldMap.textureDirectBufferPool`: it uses TextureDirectBufferPool, which isn't translated (it extends `MapPool` (xaero.map.pool)) (xaero/map/WorldMap.java:82)
- [ ] `WorldMap.tilePool`: it uses MapTilePool, which isn't translated (it extends `MapPool`) (xaero/map/WorldMap.java:83)
- [ ] `WorldMap.mapLimiter`: it uses MapLimiter, which isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)) (xaero/map/WorldMap.java:84)
- [ ] `WorldMap.unknownBlockStateCache`: it uses UnknownBlockStateCache, which isn't translated (it uses Minecraft's `BlockState`) (xaero/map/WorldMap.java:85)
- [ ] `WorldMap.gpuObjectDeleter`: it uses GpuObjectDeleter, which isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)) (xaero/map/WorldMap.java:86)
- [ ] `WorldMap.mapRunner`: it uses MapRunner, which isn't translated (it implements `Runnable`) (xaero/map/WorldMap.java:87)
- [ ] `WorldMap.mapRunnerThread`: it uses `Thread` (xaero/map/WorldMap.java:88)
- [ ] `WorldMap.crashHandler`: it uses CrashHandler, which isn't translated (it uses `Throwable`) (xaero/map/WorldMap.java:89)
- [x] `WorldMap.guiTextures` (xaero/map/WorldMap.java:90)
- [x] `WorldMap.settings` (xaero/map/WorldMap.java:91)
- [ ] `WorldMap.worldMapClientOnly`: it uses WorldMapClientOnly, which isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/map/WorldMap.java:92)
- [x] `WorldMap.worldmapServer` (xaero/map/WorldMap.java:93)
- [ ] `WorldMap.mapElementRenderHandler`: it uses MapElementRenderHandler, which isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))) (xaero/map/WorldMap.java:94)
- [x] `WorldMap.serverPlayerTickHandler` (xaero/map/WorldMap.java:95)
- [ ] `WorldMap.playerTrackerSystemManager`: it uses PlayerTrackerSystemManager, which isn't translated (it uses IPlayerTrackerSystem, which isn't translated (it's an interface)) (xaero/map/WorldMap.java:96)
- [ ] `WorldMap.trackedPlayerRenderer`: it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)) (xaero/map/WorldMap.java:97)
- [ ] `WorldMap.trackedPlayerMenuRenderer`: it uses PlayerTrackerMenuRenderer, which isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)) (xaero/map/WorldMap.java:98)
- [ ] `WorldMap.messageHandler`: it uses `IPacketHandler` (xaero.lib.common.packet) (xaero/map/WorldMap.java:99)
- [x] `WorldMap.commonEvents` (xaero/map/WorldMap.java:100)
- [x] `WorldMap.modCommonEvents` (xaero/map/WorldMap.java:101)
- [x] `WorldMap.detailed_debug` (xaero/map/WorldMap.java:106)
- [x] `WorldMap.pauseRequests` (xaero/map/WorldMap.java:107)
- [x] `WorldMap.extraDebug` (xaero/map/WorldMap.java:108)
- [ ] `WorldMap.modJAR`: it uses `File` (java.io) (xaero/map/WorldMap.java:109)
- [ ] `WorldMap.configFolder`: it uses `File` (java.io) (xaero/map/WorldMap.java:110)
- [ ] `WorldMap.optionsFile`: it uses `File` (java.io) (xaero/map/WorldMap.java:111)
- [ ] `WorldMap.saveFolder`: it uses `File` (java.io) (xaero/map/WorldMap.java:112)
- [x] `WorldMap.commonConfigIO` (xaero/map/WorldMap.java:113)
- [ ] `WorldMap.WorldMap(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:115)
- [ ] `WorldMap.fetchModFile(0)`: it has no body (xaero/map/WorldMap.java:134)
- [ ] `WorldMap.getFileLayoutID(0)`: it has no body (xaero/map/WorldMap.java:136)
- [ ] `WorldMap.loadClient(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:138)
- [ ] `WorldMap.loadLater(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:218)
- [ ] `WorldMap.loadLaterClientRender(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:249)
- [ ] `WorldMap.loadServer(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:260)
- [ ] `WorldMap.loadLaterServer(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:265)
- [ ] `WorldMap.loadCommon(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:271)
- [ ] `WorldMap.loadLaterCommon(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:279)
- [ ] `WorldMap.getVersionID(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:285)
- [ ] `WorldMap.onSessionFinalized(0)`: a nullable object kept in a changing static (xaero/map/WorldMap.java:289)
- [ ] `WorldMap.getConfigs(0)`: its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)) (xaero/map/WorldMap.java:296)
- [ ] `WorldMap.createSupportMods(0)`: it has no body (xaero/map/WorldMap.java:300)
- [ ] `WorldMap.createClientLoad(0)`: it has no body (xaero/map/WorldMap.java:302)
- [ ] `WorldMap.createServerLoad(0)`: it has no body (xaero/map/WorldMap.java:304)
- [ ] `WorldMap.createWriter(3)`: it has no body (xaero/map/WorldMap.java:306)
- [ ] `WorldMap.getModInfoVersion(0)`: it has no body (xaero/map/WorldMap.java:308)

### `WorldMapClientOnly`: it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))

- [ ] `WorldMapClientOnly.preInit(1)`: its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/map/WorldMapClientOnly.java:14)
- [ ] `WorldMapClientOnly.postInit(0)`: its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/map/WorldMapClientOnly.java:17)
- [ ] `WorldMapClientOnly.loadLaterClientRender(0)`: its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/map/WorldMapClientOnly.java:21)
- [ ] `WorldMapClientOnly.getMapScreenPoseStack(0)`: its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))) (xaero/map/WorldMapClientOnly.java:30)

### `WorldMapClientOnlyFabric`: it extends `WorldMapClientOnly`

- [ ] `WorldMapClientOnlyFabric.preInit(1)`: its class isn't translated (it extends `WorldMapClientOnly`) (xaero/map/WorldMapClientOnlyFabric.java:4)

### `WorldMapFabric`: it extends `WorldMap`

- [ ] `WorldMapFabric.onInitializeClient(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:40)
- [ ] `WorldMapFabric.onInitializeServer(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:49)
- [ ] `WorldMapFabric.registerClientEvents(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:58)
- [ ] `WorldMapFabric.registerCommonEvents(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:63)
- [ ] `WorldMapFabric.loadCommon(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:70)
- [ ] `WorldMapFabric.loadClient(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:82)
- [ ] `WorldMapFabric.loadServer(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:90)
- [ ] `WorldMapFabric.tryLoadLater(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:96)
- [ ] `WorldMapFabric.tryLoadLaterServer(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:110)
- [ ] `WorldMapFabric.fetchModFile(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:123)
- [ ] `WorldMapFabric.getFileLayoutID(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:146)
- [ ] `WorldMapFabric.createSupportMods(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:151)
- [ ] `WorldMapFabric.createClientLoad(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:156)
- [ ] `WorldMapFabric.createServerLoad(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:161)
- [ ] `WorldMapFabric.createWriter(3)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:166)
- [ ] `WorldMapFabric.getModInfoVersion(0)`: its class isn't translated (it extends `WorldMap`) (xaero/map/WorldMapFabric.java:171)

### `WorldMapSession`: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))

- [ ] `WorldMapSession.init(2)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:36)
- [ ] `WorldMapSession.cleanup(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:97)
- [ ] `WorldMapSession.getControlsHandler(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:124)
- [ ] `WorldMapSession.getMapProcessor(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:128)
- [ ] `WorldMapSession.getCurrentSession(0)`: it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:132)
- [ ] `WorldMapSession.getForPlayer(1)`: it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:141)
- [ ] `WorldMapSession.isUsable(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/WorldMapSession.java:145)

### `MapConnectionManager`: a Map or Set keyed by a MapConnectionNode

- [ ] `MapConnectionManager.addConnection(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:16)
- [ ] `MapConnectionManager.addOneWayConnection(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:21)
- [ ] `MapConnectionManager.removeConnection(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:30)
- [ ] `MapConnectionManager.removeOneWayConnection(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:35)
- [ ] `MapConnectionManager.isConnected(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:42)
- [ ] `MapConnectionManager.isEmpty(0)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:55)
- [ ] `MapConnectionManager.save(1)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:59)
- [ ] `MapConnectionManager.swapConnections(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:77)
- [ ] `MapConnectionManager.renameDimension(2)`: its class isn't translated (a Map or Set keyed by a MapConnectionNode) (xaero/map/world/MapConnectionManager.java:98)

### `MapConnectionNode`: it uses Minecraft's `ResourceKey`

- [ ] `MapConnectionNode.MapConnectionNode(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:13)
- [ ] `MapConnectionNode.toString(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:18)
- [ ] `MapConnectionNode.getNamedString(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:27)
- [ ] `MapConnectionNode.fromString(1)`: it uses MapConnectionNode, which isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:32)
- [ ] `MapConnectionNode.equals(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:59)
- [ ] `MapConnectionNode.hashCode(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:70)
- [ ] `MapConnectionNode.getDimId(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:75)
- [ ] `MapConnectionNode.getMw(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapConnectionNode.java:79)

### `MapDimension`: it uses Minecraft's `ResourceKey`

- [x] `MapDimension.CAVE_MODE_TYPES` (xaero/map/world/MapDimension.java:81)
- [ ] `MapDimension.MapDimension(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:83)
- [ ] `MapDimension.getCurrentMultiworld(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:99)
- [ ] `MapDimension.isUsingWorldSave(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:103)
- [ ] `MapDimension.isFutureUsingWorldSaveUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:107)
- [ ] `MapDimension.getMultiworldIdsCopy(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:111)
- [ ] `MapDimension.updateFutureAutomaticUnsynced(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:117)
- [ ] `MapDimension.getFutureCustomSelectedMultiworld(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:135)
- [ ] `MapDimension.getFutureMultiworldUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:139)
- [ ] `MapDimension.switchToFutureUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:143)
- [ ] `MapDimension.switchToFutureMultiworldWritableValueUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:148)
- [ ] `MapDimension.getLayeredMapRegions(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:152)
- [ ] `MapDimension.clear(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:156)
- [ ] `MapDimension.preDetection(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:167)
- [ ] `MapDimension.getMainFolderPath(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:172)
- [ ] `MapDimension.getOldFolderPath(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:179)
- [ ] `MapDimension.saveConfigUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:186)
- [ ] `MapDimension.loadConfigUnsynced(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:214)
- [ ] `MapDimension.pickDefaultCustomMultiworldUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:291)
- [ ] `MapDimension.loadMultiworldsList(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:302)
- [ ] `MapDimension.confirmMultiworldUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:330)
- [ ] `MapDimension.makeCustomSelectedMultiworldAutoUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:342)
- [ ] `MapDimension.bindAutoMultiworld(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:363)
- [ ] `MapDimension.resetCustomMultiworldUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:371)
- [ ] `MapDimension.setMultiworldUnsynced(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:380)
- [ ] `MapDimension.multiworldExists(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:387)
- [ ] `MapDimension.addMultiworldChecked(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:393)
- [ ] `MapDimension.getMultiworldName(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:404)
- [ ] `MapDimension.setMultiworldName(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:431)
- [ ] `MapDimension.getFutureAutoMultiworld(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:435)
- [ ] `MapDimension.getMapWorld(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:445)
- [ ] `MapDimension.deleteMultiworldMapDataUnsynced(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:449)
- [ ] `MapDimension.deleteMultiworldId(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:466)
- [ ] `MapDimension.getDimId(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:476)
- [ ] `MapDimension.hasConfirmedMultiworld(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:480)
- [ ] `MapDimension.isFutureMultiworldServerBased(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:484)
- [ ] `MapDimension.getHighlightHandler(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:488)
- [ ] `MapDimension.onClearCachedHighlightHash(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:492)
- [ ] `MapDimension.onClearCachedHighlightHashes(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:496)
- [ ] `MapDimension.hasDoneRegionDetection(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:500)
- [ ] `MapDimension.addWorldSaveRegionDetection(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:504)
- [ ] `MapDimension.getWorldSaveRegionDetection(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:516)
- [ ] `MapDimension.getCaveModeType(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:525)
- [ ] `MapDimension.toggleCaveModeType(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:529)
- [ ] `MapDimension.getWorldSaveDetectedRegions(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:542)
- [ ] `MapDimension.getLinkedWorldSaveDetectedRegions(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:546)
- [ ] `MapDimension.getFullReloader(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:550)
- [ ] `MapDimension.startFullMapReload(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:554)
- [ ] `MapDimension.clearFullMapReload(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:559)
- [ ] `MapDimension.getDimensionType(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:563)
- [ ] `MapDimension.getDimensionType(3)`: it uses Minecraft's `DimensionType` (xaero/map/world/MapDimension.java:572)
- [ ] `MapDimension.getDimensionType(3)`: it uses Minecraft's `DimensionType` (xaero/map/world/MapDimension.java:600)
- [ ] `MapDimension.onWorldChangeUnsynced(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:604)
- [ ] `MapDimension.isUsingUnknownDimensionType(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:616)
- [ ] `MapDimension.isCacheOnlyMode(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:620)
- [ ] `MapDimension.onCreationUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:624)
- [ ] `MapDimension.getShadowR(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:637)
- [ ] `MapDimension.getShadowG(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:641)
- [ ] `MapDimension.getShadowB(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:645)
- [ ] `MapDimension.getSkyDarken(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:649)
- [ ] `MapDimension.invalidateTickCache(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:675)
- [ ] `MapDimension.calculateDimScale(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:681)
- [ ] `MapDimension.calculateDimDiv(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:686)
- [ ] `MapDimension.getPlayerMapKey(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:690)
- [ ] `MapDimension.getSelectedMapKeyUnsynced(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:699)
- [ ] `MapDimension.isAutoSelected(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:708)
- [ ] `MapDimension.getDimensionTypeId(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:713)
- [ ] `MapDimension.currentMultiworldBlockedByFileLock(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:717)
- [ ] `MapDimension.setCurrentMultiworldBlockedByFileLock(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (xaero/map/world/MapDimension.java:721)

### `MapWorld`: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))

- [ ] `MapWorld.MapWorld(3)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:48)
- [ ] `MapWorld.getDimension(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:59)
- [ ] `MapWorld.createDimensionUnsynced(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:69)
- [ ] `MapWorld.getMainId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:81)
- [ ] `MapWorld.getOldUnfixedMainId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:85)
- [ ] `MapWorld.getCurrentMultiworld(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:89)
- [ ] `MapWorld.getFutureMultiworldUnsynced(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:94)
- [ ] `MapWorld.getCurrentDimension(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:99)
- [ ] `MapWorld.getFutureDimension(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:104)
- [ ] `MapWorld.getCurrentDimensionId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:109)
- [ ] `MapWorld.getFutureDimensionId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:113)
- [ ] `MapWorld.setFutureDimensionId(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:117)
- [ ] `MapWorld.getCustomDimensionId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:121)
- [ ] `MapWorld.setCustomDimensionId(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:125)
- [ ] `MapWorld.switchToFutureUnsynced(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:129)
- [ ] `MapWorld.getDimensionsList(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:134)
- [ ] `MapWorld.getDimensions(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:140)
- [ ] `MapWorld.getCurrentMultiworldType(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:146)
- [ ] `MapWorld.isMultiplayer(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:150)
- [ ] `MapWorld.isCurrentMultiworldTypeConfirmed(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:154)
- [ ] `MapWorld.getFutureMultiworldType(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:158)
- [ ] `MapWorld.toggleMultiworldTypeUnsynced(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:162)
- [ ] `MapWorld.unconfirmMultiworldTypeUnsynced(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:169)
- [ ] `MapWorld.confirmMultiworldTypeUnsynced(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:173)
- [ ] `MapWorld.isFutureMultiworldTypeConfirmed(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:177)
- [ ] `MapWorld.switchToFutureMultiworldTypeUnsynced(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:181)
- [ ] `MapWorld.load(0)`: its body couldn't be read: an anonymous class (xaero/map/world/MapWorld.java:187)
- [ ] `MapWorld.loadConfig(2)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:216)
- [ ] `MapWorld.saveConfig(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:328)
- [ ] `MapWorld.getMapProcessor(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:357)
- [ ] `MapWorld.isIgnoreServerLevelId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:361)
- [ ] `MapWorld.isIgnoreHeightmaps(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:365)
- [ ] `MapWorld.setIgnoreHeightmaps(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:369)
- [ ] `MapWorld.getPlayerTeleportCommandFormat(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:373)
- [ ] `MapWorld.setPlayerTeleportCommandFormat(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:377)
- [ ] `MapWorld.getEffectivePlayerTeleportCommandFormat(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:381)
- [ ] `MapWorld.getTeleportCommandFormat(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:390)
- [ ] `MapWorld.setTeleportCommandFormat(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:394)
- [ ] `MapWorld.getEffectiveTeleportCommandFormat(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:398)
- [ ] `MapWorld.getDimensionTeleportCommandFormat(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:407)
- [ ] `MapWorld.setDimensionTeleportCommandFormat(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:411)
- [ ] `MapWorld.getEffectiveDimensionTeleportCommandFormat(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:415)
- [ ] `MapWorld.clearAllCachedHighlightHashes(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:424)
- [ ] `MapWorld.isUsingCustomDimension(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:432)
- [ ] `MapWorld.isUsingUnknownDimensionType(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:437)
- [ ] `MapWorld.isCacheOnlyMode(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:441)
- [ ] `MapWorld.onWorldChangeUnsynced(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:445)
- [ ] `MapWorld.getPotentialDimId(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:453)
- [ ] `MapWorld.getPlayerMapKey(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:458)
- [ ] `MapWorld.getMapConnections(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:465)
- [ ] `MapWorld.toggleDimension(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:469)
- [x] `MapWorld.convertWorldFolderToRootId(2)` (xaero/map/world/MapWorld.java:485)
- [ ] `MapWorld.isUsingDefaultMapTeleport(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:498)
- [ ] `MapWorld.isUsingDefaultPlayerTeleport(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:502)
- [ ] `MapWorld.setUseDefaultMapTeleport(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:506)
- [ ] `MapWorld.setUseDefaultPlayerTeleport(1)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:510)
- [ ] `MapWorld.invalidateTickCache(0)`: its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))) (xaero/map/world/MapWorld.java:514)
