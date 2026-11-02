// Xaero's World Map, ported to Horizon by horizon-port from xaeroworldmap-fabric-26.3-1.47.2.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_xaeroworldmap). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "xaeroworldmap";
pub const NAME: &str = "Xaero's World Map";
pub const VERSION: &str = "1.47.2";
pub const DESCRIPTION: &str = "A self-writing fullscreen map which also works as an add-on to Xaero's Minimap.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[
    ["world_map", "World Map", "key.keyboard.m"],
    ["zoom_in", "Zoom In", "key.keyboard.equal"],
    ["zoom_out", "Zoom Out", "key.keyboard.minus"],
    ["recenter", "Recenter", "key.keyboard.c"],
];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `Animation` (xaero/map/animation/Animation.java)
    pub mod animation {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Animation {
            pub start: i64,
            pub time: i64,
            pub from: f64,
            pub off: f64,
        }
        impl Animation {
            pub fn new(from: f64, to: f64, time: i64) -> Self {
                let mut s = Self { start: 0i64, time: time, from: from, off: 0.0f64 };
                s.from = from;
                s.off = to - from;
                s.time = time;
                s.start = j_millis();
                s
            }
            pub fn get_current(&self) -> f64 {
                return self.from + ((1.0f64.min(((j_millis() - self.start) as f64) / (self.time as f64))) * self.off);
            }
            pub fn get_destination(&self) -> f64 {
                return self.from + self.off;
            }
            pub fn get_passed(&self) -> i64 {
                return j_millis() - self.start;
            }
        }
    }

    /// `SinAnimation` (xaero/map/animation/SinAnimation.java)
    pub mod sin_animation {
        use super::*;
        // The class itself isn't translated: it extends `Animation`. Its static parts are below.
        // SinAnimation.SinAnimation(3): not translated, its class isn't translated (it extends `Animation`).
        // SinAnimation.getCurrent(0): not translated, its class isn't translated (it extends `Animation`).
    }

    /// `SlowingAnimation` (xaero/map/animation/SlowingAnimation.java)
    pub mod slowing_animation {
        use super::*;
        // The class itself isn't translated: it extends `Animation`. Its static parts are below.
        pub const ANIMATION_THING: f64 = 16.666666666666668f64;
        // SlowingAnimation.SlowingAnimation(4): not translated, its class isn't translated (it extends `Animation`).
        // SlowingAnimation.getCurrent(0): not translated, its class isn't translated (it extends `Animation`).
        // SlowingAnimation.getDestination(0): not translated, its class isn't translated (it extends `Animation`).
    }

    /// `BiomeBlendCalculator` (xaero/map/biome/BiomeBlendCalculator.java)
    pub mod biome_blend_calculator {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockAndTintGetter`. Its static parts are below.
        // BiomeBlendCalculator.setWorld(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getLightEngine(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.cardinalLighting(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BiomeBlendCalculator.getBlockTint(2): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
    }

    /// `BiomeColorCalculator` (xaero/map/biome/BiomeColorCalculator.java)
    pub mod biome_color_calculator {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // BiomeColorCalculator.prepare(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeColorCalculator.getBiomeColor(7): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeColorCalculator.getBiomeAtPos(4): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `BiomeGetter` (xaero/map/biome/BiomeGetter.java)
    pub mod biome_getter {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // BiomeGetter.BiomeGetter(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeGetter.getBiome(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `BlockTintProvider` (xaero/map/biome/BlockTintProvider.java)
    pub mod block_tint_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockAndTintGetter`. Its static parts are below.
        // BlockTintProvider.BlockTintProvider(5): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getLightEngine(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.cardinalLighting(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getBlockTint(2): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // BlockTintProvider.getBiomeColor(6): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
    }

    /// `SingleBlockAndTintGetter` (xaero/map/block/SingleBlockAndTintGetter.java)
    pub mod single_block_and_tint_getter {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockAndTintGetter`. Its static parts are below.
        // SingleBlockAndTintGetter.SingleBlockAndTintGetter(3): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getLightEngine(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.cardinalLighting(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getBlockTint(2): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
        // SingleBlockAndTintGetter.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockAndTintGetter`).
    }

    /// `BlockStateShortShapeCache` (xaero/map/cache/BlockStateShortShapeCache.java)
    pub mod block_state_short_shape_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // BlockStateShortShapeCache.BlockStateShortShapeCache(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BlockStateShortShapeCache.scheduleAndGet(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BlockStateShortShapeCache.isShort(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BlockStateShortShapeCache.reset(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BlockStateShortShapeCache.supplyForIOThread(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `BrokenBlockTintCache` (xaero/map/cache/BrokenBlockTintCache.java)
    pub mod broken_block_tint_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // BrokenBlockTintCache.BrokenBlockTintCache(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BrokenBlockTintCache.setBroken(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BrokenBlockTintCache.isBroken(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // BrokenBlockTintCache.getSize(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `PlaceholderBlockGetter` (xaero/map/cache/placeholder/PlaceholderBlockGetter.java)
    pub mod placeholder_block_getter {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockGetter`. Its static parts are below.
        // PlaceholderBlockGetter.setPlaceholderState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getBlockEntity(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getBlockState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getFluidState(1): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getHeight(0): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
        // PlaceholderBlockGetter.getMinY(0): not translated, its class isn't translated (it implements Minecraft's `BlockGetter`).
    }

    /// `UnknownBlockStateCache` (xaero/map/cache/UnknownBlockStateCache.java)
    pub mod unknown_block_state_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // UnknownBlockStateCache.getBlockStateFromNBT(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `CapabilityGetter` (xaero/map/capabilities/CapabilityGetter.java)
    pub mod capability_getter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CapabilityGetter {
        }
        impl CapabilityGetter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CapabilityGetter.getServerWorldCapabilities(1): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `ServerWorldCapabilities` (xaero/map/capabilities/ServerWorldCapabilities.java)
    pub mod server_world_capabilities {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerWorldCapabilities {
            pub loaded: bool,
        }
        impl ServerWorldCapabilities {
            pub fn new() -> Self {
                let mut s = Self { loaded: true };
                s
            }
        }
    }

    /// `WorldMapChannelCommonRegistryHandler` (xaero/map/common/config/channel/register/handler/WorldMapChannelCommonRegistryHandler.java)
    pub mod world_map_channel_common_registry_handler {
        use super::*;
        // The class itself isn't translated: it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler). Its static parts are below.
        // WorldMapChannelCommonRegistryHandler.registerPrimaryCommonOptions(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
        // WorldMapChannelCommonRegistryHandler.registerProfiledOptions(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
        // WorldMapChannelCommonRegistryHandler.registerServerOptionChangeHandlers(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
        // WorldMapChannelCommonRegistryHandler.registerOptionServerRedirectors(1): not translated, its class isn't translated (it implements `IConfigChannelCommonRegistryHandler` (xaero.lib.common.config.channel.register.handler)).
    }

    /// `LegacyCommonConfigInit` (xaero/map/common/config/LegacyCommonConfigInit.java)
    pub mod legacy_common_config_init {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LegacyCommonConfigInit {
        }
        impl LegacyCommonConfigInit {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // LegacyCommonConfigInit.init(1): not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        }
    }

    /// `LegacyCommonConfigIO` (xaero/map/common/config/LegacyCommonConfigIO.java)
    pub mod legacy_common_config_io {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LegacyCommonConfigIO {
            pub config_file_path: std::path::PathBuf,
            pub allow_cave_mode_on_server: bool,
            pub allow_nether_cave_mode_on_server: bool,
            pub should_enable_everyone_tracks_everyone: bool,
        }
        impl LegacyCommonConfigIO {
            pub fn new(config_file_path: &std::path::Path) -> Self {
                let mut s = Self { config_file_path: config_file_path.to_path_buf(), allow_cave_mode_on_server: false, allow_nether_cave_mode_on_server: false, should_enable_everyone_tracks_everyone: false };
                s.config_file_path = config_file_path.to_path_buf();
                s
            }
            // LegacyCommonConfigIO.load(0): not translated, it uses `ConfigProfile` (xaero.lib.common.config.profile).
            // LegacyCommonConfigIO.readLine(1): not translated, a nullable object kept in a changing static.
            pub fn should_enable_everyone_tracks_everyone(&self) -> bool {
                return self.should_enable_everyone_tracks_everyone;
            }
        }
    }

    /// `WorldMapConfigOptionServerHandlers` (xaero/map/common/config/listener/handler/WorldMapConfigOptionServerHandlers.java)
    pub mod world_map_config_option_server_handlers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigOptionServerHandlers {
        }
        impl WorldMapConfigOptionServerHandlers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapConfigOptionServerHandlers.registerAll(1): not translated, it uses `HandlerBasedConfigChangeListener` (xaero.lib.common.config.listener.handler).
    }

    /// `WorldMapConfigOptionServerRedirectors` (xaero/map/common/config/option/value/redirect/WorldMapConfigOptionServerRedirectors.java)
    pub mod world_map_config_option_server_redirectors {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigOptionServerRedirectors {
        }
        impl WorldMapConfigOptionServerRedirectors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapConfigOptionServerRedirectors.registerAll(1): not translated, it uses `OptionValueRedirectorManager` (xaero.lib.common.config.option.value.redirect).
    }

    /// `WorldMapProfiledConfigOptions` (xaero/map/common/config/option/WorldMapProfiledConfigOptions.java)
    pub mod world_map_profiled_config_options {
        use super::*;
        // ALL: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // LIGHTING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // BLOCK_COLORS: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // LOAD_NEW_CHUNKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // UPDATE_CHUNKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TERRAIN_DEPTH: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // TERRAIN_SLOPES: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // FOOTSTEPS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // COORDINATES: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // RENDER_WAYPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_BACKGROUNDS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WAYPOINT_SCALE: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // BIOME_BLENDING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // BIOME_COLORS_IN_VANILLA: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // MIN_ZOOM_LOCAL_WAYPOINTS: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // ADJUST_HEIGHT_FOR_SHORT_BLOCKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // FLOWERS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // STAINED_GLASS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_ALLOWED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_ALLOWED_DIMENSIONS: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_DEPTH: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // LEGIBLE_CAVE_MAPS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // AUTO_CAVE_MODE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_TOGGLE_TIMER: not translated, it uses `SteppedConfigOption` (xaero.lib.common.config.option).
        // DEFAULT_CAVE_MODE_TYPE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_CAVE_MODE_START: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // WRITING_DISTANCE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // ARROW: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OPENING_ANIMATION: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ARROW_COLOR: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_ZOOM: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_HOVERED_BIOME: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ZOOM_BUTTONS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DETECT_AMBIGUOUS_Y: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DEFAULT_MAP_TELEPORT_FORMAT: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // DEFAULT_MAP_TELEPORT_DIMENSION_FORMAT: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // DEFAULT_PLAYER_TELEPORT_FORMAT: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MAP_TELEPORT_ALLOWED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // PARTIAL_Y_TELEPORT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OPAC_CLAIMS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // OPAC_CLAIMS_BORDER_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // OPAC_CLAIMS_FILL_OPACITY: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // MAP_ITEM: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MINIMAP_RADAR: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_TRACKED_PLAYERS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // UNLIMITED_ZOOM_IN: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // UNLIMITED_ZOOM_OUT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // SKIP_WORLD_RENDER: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // SERVER_CHUNK_RADIUS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // FORCE_FAST_WRITING: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISTANCES: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISTANCE_TO_KM_AT: not translated, it uses `IndexedConfigOption` (xaero.lib.common.config.option).
        // DISTANCE_PRECISION: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        #[derive(Clone, Debug)]
        pub struct WorldMapProfiledConfigOptions {
        }
        impl WorldMapProfiledConfigOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapProfiledConfigOptions.registerAll(1): not translated, it uses `ConfigOptionManager` (xaero.lib.common.config.option).
    }

    /// `WorldMapPrimaryCommonConfigOptions` (xaero/map/common/config/primary/option/WorldMapPrimaryCommonConfigOptions.java)
    pub mod world_map_primary_common_config_options {
        use super::*;
        // ALL: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // REGISTER_EFFECTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        #[derive(Clone, Debug)]
        pub struct WorldMapPrimaryCommonConfigOptions {
        }
        impl WorldMapPrimaryCommonConfigOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapPrimaryCommonConfigOptions.registerAll(1): not translated, it uses `ConfigOptionManager` (xaero.lib.common.config.option).
    }

    /// `WorldMapConfigUtils` (xaero/map/common/config/util/WorldMapConfigUtils.java)
    pub mod world_map_config_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigUtils {
        }
        impl WorldMapConfigUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `WorldMapConfigConstants` (xaero/map/common/config/WorldMapConfigConstants.java)
    pub mod world_map_config_constants {
        use super::*;
        // INGAME_TOOLTIP: a game value, so it's in game_side.rs.
        // LEGACY_PLUGIN_TOOLTIP: a game value, so it's in game_side.rs.
        // FAIRPLAY_TOOLTIP: a game value, so it's in game_side.rs.
        // EFFECT_TOOLTIP: a game value, so it's in game_side.rs.
        // MINIMAP_TOOLTIP: a game value, so it's in game_side.rs.
        // BLOCK_COLORS_NAMES: a game value, so it's in game_side.rs.
        // TERRAIN_SLOPES_NAMES: a game value, so it's in game_side.rs.
        // SEC: a game value, so it's in game_side.rs.
        // MILLISEC: a game value, so it's in game_side.rs.
        // DEFAULT_CAVE_MODE_TYPE_NAMES: a game value, so it's in game_side.rs.
        // ARROW_COLOR_NAMES: a game value, so it's in game_side.rs.
        pub static ARROW_COLORS: LazyLock<JCell<Vec<Vec<f32>>>> = LazyLock::new(|| JCell::new(vec![vec![0.8f32, 0.1f32, 0.1f32, 1.0f32], vec![0.09f32, 0.57f32, 0.0f32, 1.0f32], vec![0.0f32, 0.55f32, 1.0f32, 1.0f32], vec![1.0f32, 0.93f32, 0.0f32, 1.0f32], vec![0.73f32, 0.33f32, 0.83f32, 1.0f32], vec![1.0f32, 1.0f32, 1.0f32, 1.0f32], vec![0.0f32, 0.0f32, 0.0f32, 1.0f32], vec![0.4588f32, 0.0f32, 0.0f32, 1.0f32]]));
        // CONVERT_DISTANCE_NEVER_COMPONENT: a game value, so it's in game_side.rs.
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigConstants {
        }
        impl WorldMapConfigConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `WorldMapChannelClientRegistryHandler` (xaero/map/config/channel/register/handler/WorldMapChannelClientRegistryHandler.java)
    pub mod world_map_channel_client_registry_handler {
        use super::*;
        // The class itself isn't translated: it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler). Its static parts are below.
        // WorldMapChannelClientRegistryHandler.registerPrimaryClientOptions(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
        // WorldMapChannelClientRegistryHandler.registerConfigOptionUITypes(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
        // WorldMapChannelClientRegistryHandler.registerClientOptionChangeHandlers(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
        // WorldMapChannelClientRegistryHandler.registerOptionClientRedirectors(1): not translated, its class isn't translated (it implements `IConfigChannelClientRegistryHandler` (xaero.lib.client.config.channel.register.handler)).
    }

    /// `WorldMapConfigOptionClientHandlers` (xaero/map/config/listener/handler/WorldMapConfigOptionClientHandlers.java)
    pub mod world_map_config_option_client_handlers {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigOptionClientHandlers {
        }
        impl WorldMapConfigOptionClientHandlers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapConfigOptionClientHandlers.handleBlockColors(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleTerrainDepth(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleTerrainSlopes(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleBiomeBlending(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleBiomesInVanilla(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleAdjustShortBlockHeight(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleStainedGlass(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleLegibleCaveMaps(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleFlowers(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.clearAllCachedHighlightHashes(0): not translated, it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapConfigOptionClientHandlers.handleOpacClaims(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleOpacClaimFillOpacity(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleOpacClaimBorderOpacity(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleReloadViewed(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.handleMapItem(1): not translated, it uses `Config` (xaero.lib.common.config).
        // WorldMapConfigOptionClientHandlers.registerAll(1): not translated, it uses `ClientConfigChangeListener` (xaero.lib.client.config.listener).
    }

    /// `WorldMapConfigOptionUITypes` (xaero/map/config/option/ui/type/WorldMapConfigOptionUITypes.java)
    pub mod world_map_config_option_ui_types {
        use super::*;
        // DEFAULT_MAP_TP_COMMAND: not translated, it uses `ConfigOptionUIType` (xaero.lib.client.config.option.ui.type).
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigOptionUITypes {
        }
        impl WorldMapConfigOptionUITypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `WorldMapConfigOptionUIRegister` (xaero/map/config/option/ui/WorldMapConfigOptionUIRegister.java)
    pub mod world_map_config_option_ui_register {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigOptionUIRegister {
        }
        impl WorldMapConfigOptionUIRegister {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapConfigOptionUIRegister.registerAll(1): not translated, it uses `ConfigOptionUITypeManager` (xaero.lib.client.config.option.ui).
    }

    /// `WorldMapConfigOptionClientRedirectors` (xaero/map/config/option/value/redirect/WorldMapConfigOptionClientRedirectors.java)
    pub mod world_map_config_option_client_redirectors {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapConfigOptionClientRedirectors {
        }
        impl WorldMapConfigOptionClientRedirectors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapConfigOptionClientRedirectors.registerAll(1): not translated, it uses `ClientOptionValueRedirectorManager` (xaero.lib.client.config.option.value.redirect).
    }

    /// `WorldMapPrimaryClientConfigOptions` (xaero/map/config/primary/option/WorldMapPrimaryClientConfigOptions.java)
    pub mod world_map_primary_client_config_options {
        use super::*;
        // ALL: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // IGNORED_UPDATE: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // UPDATE_NOTIFICATIONS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // RELOAD_VIEWED: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // RELOAD_VIEWED_VERSION: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // DEBUG: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CAVE_MODE_START: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // EXPORT_MULTIPLE_IMAGES: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // NIGHT_EXPORT: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // EXPORT_SCALE_DOWN_SQUARE: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // EXPORT_HIGHLIGHTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DIFFERENTIATE_BY_SERVER_ADDRESS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // DISPLAY_DISABLED_WAYPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // CLOSE_WAYPOINTS_AFTER_HOP: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // ONLY_CURRENT_MAP_WAYPOINTS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        // GLOBAL_VERSION: not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // MAX_LOADED_REGIONS: not translated, it uses `RangeConfigOption` (xaero.lib.common.config.option).
        // IGNORE_MAP_FILE_LOCKS: not translated, it uses `BooleanConfigOption` (xaero.lib.common.config.option).
        #[derive(Clone, Debug)]
        pub struct WorldMapPrimaryClientConfigOptions {
        }
        impl WorldMapPrimaryClientConfigOptions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapPrimaryClientConfigOptions.registerAll(1): not translated, it uses `ClientConfigOptionManager` (xaero.lib.client.config.option).
    }

    /// `WorldMapClientConfigUtils` (xaero/map/config/util/WorldMapClientConfigUtils.java)
    pub mod world_map_client_config_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapClientConfigUtils {
        }
        impl WorldMapClientConfigUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapClientConfigUtils.isFairPlay(0): not translated, a nullable object kept in a changing static.
        // WorldMapClientConfigUtils.isCaveModeDisabledLegacy(0): not translated, a nullable object kept in a changing static.
        // WorldMapClientConfigUtils.getEffectiveCaveModeAllowed(0): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // WorldMapClientConfigUtils.getDebug(0): not translated, it uses `ClientConfigManager` (xaero.lib.client.config).
        // WorldMapClientConfigUtils.togglePrimaryOption(1): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // WorldMapClientConfigUtils.tryTogglingCurrentProfileOption(1): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
        // WorldMapClientConfigUtils.isOptionServerEnforced(1): not translated, it uses `ConfigOption` (xaero.lib.common.config.option).
    }

    /// `ControlsHandler` (xaero/map/controls/ControlsHandler.java)
    pub mod controls_handler {
        use super::*;
        // The class itself isn't translated: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))). Its static parts are below.
        // ControlsHandler.ControlsHandler(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // ControlsHandler.eventExists(1): not translated, it uses Minecraft's `KeyMapping`.
        // ControlsHandler.oldEventExists(1): not translated, it uses Minecraft's `KeyMapping`.
        // ControlsHandler.isKeyRepeat(1): not translated, it uses Minecraft's `KeyMapping`.
        // ControlsHandler.keyDown(3): not translated, it uses Minecraft's `KeyMapping`.
        // ControlsHandler.keyUp(2): not translated, it uses Minecraft's `KeyMapping`.
        // ControlsHandler.handleKeyEvents(0): not translated, it uses Minecraft's `Minecraft`.
        // ControlsHandler.onKeyInput(1): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `ControlsRegister` (xaero/map/controls/ControlsRegister.java)
    pub mod controls_register {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `KeyMapping`. Its static parts are below.
        // CATEGORY: not translated, it uses Minecraft's `Category`.
        // keyOpenMap: not translated, it uses Minecraft's `KeyMapping`.
        // keyOpenSettings: not translated, it uses Minecraft's `KeyMapping`.
        // keyOpenServerSettings: not translated, it uses Minecraft's `KeyMapping`.
        // keyZoomIn: not translated, it uses Minecraft's `KeyMapping`.
        // keyZoomOut: not translated, it uses Minecraft's `KeyMapping`.
        // keyQuickConfirm: not translated, it uses Minecraft's `KeyMapping`.
        // keyToggleDimension: not translated, it uses Minecraft's `KeyMapping`.
        // keyToggleTrackedPlayers: not translated, it uses Minecraft's `KeyMapping`.
        // keyTogglePacChunkClaims: not translated, it uses Minecraft's `KeyMapping`.
        // ControlsRegister.register(2): not translated, its class isn't translated (it uses Minecraft's `KeyMapping`).
    }

    /// `KeyEvent` (xaero/map/controls/KeyEvent.java)
    pub mod key_event {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `KeyMapping`. Its static parts are below.
        // KeyEvent.KeyEvent(4): not translated, its class isn't translated (it uses Minecraft's `KeyMapping`).
        // KeyEvent.getKb(0): not translated, its class isn't translated (it uses Minecraft's `KeyMapping`).
        // KeyEvent.isTickEnd(0): not translated, its class isn't translated (it uses Minecraft's `KeyMapping`).
        // KeyEvent.isRepeat(0): not translated, its class isn't translated (it uses Minecraft's `KeyMapping`).
        // KeyEvent.isKeyDown(0): not translated, its class isn't translated (it uses Minecraft's `KeyMapping`).
    }

    /// `IGuiRenderer` (xaero/map/core/IGuiRenderer.java)
    pub mod i_gui_renderer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IGuiRenderer.xaero_wm_getGuiProjection(0): not translated, it has no body.
        // IGuiRenderer.xaero_wm_getGuiProjectionMatrixBuffer(0): not translated, it has no body.
    }

    /// `IWorldMapClientPlayNetHandler` (xaero/map/core/IWorldMapClientPlayNetHandler.java)
    pub mod i_world_map_client_play_net_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWorldMapClientPlayNetHandler.getXaero_worldmapSession(0): not translated, it has no body.
        // IWorldMapClientPlayNetHandler.setXaero_worldmapSession(1): not translated, it has no body.
        // IWorldMapClientPlayNetHandler.getXaero_serverChunkRadius(0): not translated, it has no body.
    }

    /// `IWorldMapGlBuffer` (xaero/map/core/IWorldMapGlBuffer.java)
    pub mod i_world_map_gl_buffer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWorldMapGlBuffer.xaero_wm_getHandle(0): not translated, it has no body.
    }

    /// `IWorldMapMinecraftClient` (xaero/map/core/IWorldMapMinecraftClient.java)
    pub mod i_world_map_minecraft_client {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWorldMapMinecraftClient.getXaeroWorldMap_fps(0): not translated, it has no body.
    }

    /// `IWorldMapServerLevel` (xaero/map/core/IWorldMapServerLevel.java)
    pub mod i_world_map_server_level {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWorldMapServerLevel.getXaero_wm_capabilities(0): not translated, it has no body.
        // IWorldMapServerLevel.setXaero_wm_capabilities(1): not translated, it has no body.
    }

    /// `IWorldMapSMultiBlockChangePacket` (xaero/map/core/IWorldMapSMultiBlockChangePacket.java)
    pub mod i_world_map_s_multi_block_change_packet {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWorldMapSMultiBlockChangePacket.xaero_wm_getSectionPos(0): not translated, it has no body.
    }

    /// `XaeroWorldMapCore` (xaero/map/core/XaeroWorldMapCore.java)
    pub mod xaero_world_map_core {
        use super::*;
        // chunkCleanField: not translated, it uses `Field` (java.lang.reflect).
        // currentSession: not translated, it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        #[derive(Clone, Debug)]
        pub struct XaeroWorldMapCore {
        }
        impl XaeroWorldMapCore {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroWorldMapCore.ensureField(0): not translated, it uses XaeroWorldMapCore.chunkCleanField, which isn't translated (it uses `Field` (java.lang.reflect)).
        // XaeroWorldMapCore.chunkUpdateCallback(2): not translated, it calls XaeroWorldMapCore.ensureField, which isn't translated.
        // XaeroWorldMapCore.onHandleLevelChunkWithLight(1): not translated, it uses Minecraft's `ClientboundLevelChunkWithLightPacket`.
        // XaeroWorldMapCore.onHandleLightUpdatePacket(1): not translated, it uses Minecraft's `ClientboundLightUpdatePacket`.
        // XaeroWorldMapCore.onQueueLightRemoval(1): not translated, it uses Minecraft's `ClientboundForgetLevelChunkPacket`.
        // XaeroWorldMapCore.onBlockChange(1): not translated, it uses Minecraft's `ClientboundBlockUpdatePacket`.
        // XaeroWorldMapCore.onMultiBlockChange(1): not translated, it uses Minecraft's `ClientboundSectionBlocksUpdatePacket`.
        // XaeroWorldMapCore.onPlayNetHandler(2): not translated, it uses Minecraft's `ClientPacketListener`.
        // XaeroWorldMapCore.cleanupCurrentSession(0): not translated, a try/finally.
        // XaeroWorldMapCore.onPlayNetHandlerCleanup(1): not translated, it uses Minecraft's `ClientPacketListener`.
        // XaeroWorldMapCore.onDeleteWorld(1): not translated, it uses Minecraft's `LevelStorageAccess`.
        pub fn on_minecraft_run_tick() {
            // (`if` always false in Horizon: only the else part)
        }
        // XaeroWorldMapCore.onSystemChat(1): not translated, it uses Minecraft's `Component`.
        // XaeroWorldMapCore.onHandleDisguisedChatMessage(2): not translated, it uses Minecraft's `Bound`.
        // XaeroWorldMapCore.onRenderCall(1): not translated, a nullable object kept in a changing static.
        // XaeroWorldMapCore.handlePlayerSetSpawnPacket(1): not translated, it uses Minecraft's `ClientboundSetDefaultSpawnPositionPacket`.
        // XaeroWorldMapCore.onRenderCrosshair(1): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `XaeroWorldMapCoreFabric` (xaero/map/core/XaeroWorldMapCoreFabric.java)
    pub mod xaero_world_map_core_fabric {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroWorldMapCoreFabric {
        }
        impl XaeroWorldMapCoreFabric {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroWorldMapCoreFabric.onPlayNetHandler(2): not translated, it uses Minecraft's `ClientPacketListener`.
        // XaeroWorldMapCoreFabric.onMinecraftRunTick(0): not translated, it uses WorldMapFabric, which isn't translated (it extends `WorldMap`).
    }

    /// `CrashHandler` (xaero/map/CrashHandler.java)
    pub mod crash_handler {
        use super::*;
        // The class itself isn't translated: it uses `Throwable`. Its static parts are below.
        // CrashHandler.checkForCrashes(0): not translated, its class isn't translated (it uses `Throwable`).
        // CrashHandler.getCrashedBy(0): not translated, its class isn't translated (it uses `Throwable`).
        // CrashHandler.setCrashedBy(1): not translated, its class isn't translated (it uses `Throwable`).
    }

    /// `ByteBufferDeallocator` (xaero/map/deallocator/ByteBufferDeallocator.java)
    pub mod byte_buffer_deallocator {
        use super::*;
        // The class itself isn't translated: it uses `Object`. Its static parts are below.
        // ByteBufferDeallocator.ByteBufferDeallocator(0): not translated, its class isn't translated (it uses `Object`).
        // ByteBufferDeallocator.deallocate(2): not translated, its class isn't translated (it uses `Object`).
        // ByteBufferDeallocator.reportException(1): not translated, its class isn't translated (it uses `Object`).
    }

    /// `Effects` (xaero/map/effects/Effects.java)
    pub mod effects {
        use super::*;
        // NO_WORLD_MAP_UNHELD: not translated, it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`).
        // NO_WORLD_MAP_HARMFUL_UNHELD: not translated, it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`).
        // NO_CAVE_MAPS_UNHELD: not translated, it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`).
        // NO_CAVE_MAPS_HARMFUL_UNHELD: not translated, it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`).
        // NO_WORLD_MAP: not translated, it uses Minecraft's `Holder`.
        // NO_WORLD_MAP_HARMFUL: not translated, it uses Minecraft's `Holder`.
        // NO_CAVE_MAPS: not translated, it uses Minecraft's `Holder`.
        // NO_CAVE_MAPS_HARMFUL: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct Effects {
        }
        impl Effects {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Effects.init(0): not translated, it uses Effects.NO_WORLD_MAP, which isn't translated (it uses Minecraft's `Holder`).
    }

    /// `EffectsRegister` (xaero/map/effects/EffectsRegister.java)
    pub mod effects_register {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EffectsRegister {
        }
        impl EffectsRegister {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // EffectsRegister.registerEffects(1): not translated, it uses WorldMapStatusEffect, which isn't translated (it extends Minecraft's `MobEffect`).
        }
    }

    /// `NoCaveMapsEffect` (xaero/map/effects/NoCaveMapsEffect.java)
    pub mod no_cave_maps_effect {
        use super::*;
        // The class itself isn't translated: it extends `WorldMapStatusEffect`. Its static parts are below.
        // NoCaveMapsEffect.NoCaveMapsEffect(1): not translated, its class isn't translated (it extends `WorldMapStatusEffect`).
    }

    /// `NoWorldMapEffect` (xaero/map/effects/NoWorldMapEffect.java)
    pub mod no_world_map_effect {
        use super::*;
        // The class itself isn't translated: it extends `WorldMapStatusEffect`. Its static parts are below.
        // NoWorldMapEffect.NoWorldMapEffect(1): not translated, its class isn't translated (it extends `WorldMapStatusEffect`).
    }

    /// `WorldMapStatusEffect` (xaero/map/effects/WorldMapStatusEffect.java)
    pub mod world_map_status_effect {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobEffect`. Its static parts are below.
        // WorldMapStatusEffect.WorldMapStatusEffect(3): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // WorldMapStatusEffect.setRegistryName(1): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
        // WorldMapStatusEffect.getRegistryName(0): not translated, its class isn't translated (it extends Minecraft's `MobEffect`).
    }

    /// `HoveredMapElementHolder` (xaero/map/element/HoveredMapElementHolder.java)
    pub mod hovered_map_element_holder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // HoveredMapElementHolder.HoveredMapElementHolder(2): not translated, its class isn't translated (it's generic).
        // HoveredMapElementHolder.getRightClickOptions(0): not translated, its class isn't translated (it's generic).
        // HoveredMapElementHolder.isRightClickValid(0): not translated, its class isn't translated (it's generic).
        // HoveredMapElementHolder.getRightClickTitleBackgroundColor(0): not translated, its class isn't translated (it's generic).
        // HoveredMapElementHolder.getElement(0): not translated, its class isn't translated (it's generic).
        // HoveredMapElementHolder.getRenderer(0): not translated, its class isn't translated (it's generic).
        // HoveredMapElementHolder.is(1): not translated, its class isn't translated (it's generic).
    }

    /// `MapElementGraphics` (xaero/map/element/MapElementGraphics.java)
    pub mod map_element_graphics {
        use super::*;
        // The class itself isn't translated: it uses `XaeroBufferProvider` (xaero.lib.client.graphics). Its static parts are below.
        // MapElementGraphics.MapElementGraphics(1): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.fill(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.fill(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.fillGradient(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.blit(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.blit(9): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.blit(11): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.blit(11): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawCenteredString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawCenteredString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawCenteredString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawString(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawString(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawString(5): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.drawString(6): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.getBufferSource(0): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.pose(0): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
        // MapElementGraphics.flush(0): not translated, its class isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
    }

    /// `MapElementMenuHitbox` (xaero/map/element/MapElementMenuHitbox.java)
    pub mod map_element_menu_hitbox {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapElementMenuHitbox {
            pub x: i32,
            pub y: i32,
            pub w: i32,
            pub h: i32,
        }
        impl MapElementMenuHitbox {
            pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
                let mut s = Self { x: x, y: y, w: w, h: h };
                s.x = x;
                s.y = y;
                s.w = w;
                s.h = h;
                s
            }
            pub fn get_x(&self) -> i32 {
                return self.x;
            }
            pub fn get_y(&self) -> i32 {
                return self.y;
            }
            pub fn get_w(&self) -> i32 {
                return self.w;
            }
            pub fn get_h(&self) -> i32 {
                return self.h;
            }
            pub fn set_y(&mut self, y: i32) {
                self.y = y;
            }
            pub fn set_h(&mut self, h: i32) {
                self.h = h;
            }
        }
    }

    /// `MapElementMenuRenderer` (xaero/map/element/MapElementMenuRenderer.java)
    pub mod map_element_menu_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const MENU_RIGHT_PADDING: i32 = 27;
        pub const MAX_MENU_SIZE: i32 = 10;
        pub const MIN_MENU_SIZE: i32 = 2;
        // MapElementMenuRenderer.MapElementMenuRenderer(2): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.onMapInit(4): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.renderMenu(11): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.beforeMenuRender(0): not translated, it has no body.
        // MapElementMenuRenderer.afterMenuRender(0): not translated, it has no body.
        // MapElementMenuRenderer.postMapRender(7): not translated, its class isn't translated (it's generic).
        pub fn get_menu_element_count(menu_start_pos: i32) -> i32 {
            return (10i32).min((2i32).max(((menu_start_pos - 34) / 16) - 2));
        }
        // MapElementMenuRenderer.renderMenuElement(11): not translated, it's generic.
        // MapElementMenuRenderer.renderMenuElement(13): not translated, it's generic.
        // MapElementMenuRenderer.onMapMouseRelease(1): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.releaseScroll(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.renderScroll(10): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.updateSearch(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.charTyped(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.keyPressed(2): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.mouseScrolled(1): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.tick(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.unfocusAll(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.onMenuClosed(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.scroll(1): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.getSearchPattern(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.getSearchStartPattern(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.updateFilteredList(0): not translated, its class isn't translated (it's generic).
        // MapElementMenuRenderer.getAnyRenderer(1): not translated, it's generic.
        // MapElementMenuRenderer.getRenderer(1): not translated, it has no body.
        // MapElementMenuRenderer.menuStartPos(1): not translated, it has no body.
        // MapElementMenuRenderer.menuSearchPadding(0): not translated, it has no body.
        // MapElementMenuRenderer.renderInMenu(11): not translated, it has no body.
        // MapElementMenuRenderer.getFilterPlaceholder(0): not translated, it has no body.
        // MapElementMenuRenderer.beforeFiltering(0): not translated, it has no body.
    }

    /// `MapElementMenuScroll` (xaero/map/element/MapElementMenuScroll.java)
    pub mod map_element_menu_scroll {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapElementMenuScroll {
            pub name: String,
            pub icon: String,
            pub direction: i32,
            pub last_scroll: i64,
        }
        impl MapElementMenuScroll {
            pub fn new(name: &str, icon: &str, direction: i32) -> Self {
                let mut s = Self { name: String::new(), icon: String::new(), direction: direction, last_scroll: 0i64 };
                s.name = name.to_string();
                s.icon = icon.to_string();
                s.direction = direction;
                s
            }
            pub fn get_direction(&self) -> i32 {
                return self.direction;
            }
            pub fn scroll(&mut self) -> i32 {
                let current_time: i64 = j_millis();
                if (self.last_scroll != 0i64) && ((current_time - self.last_scroll) <= 100i64) {
                    return 0;
                }
                self.last_scroll = current_time;
                return self.direction;
            }
            pub fn on_mouse_release(&mut self) {
                self.last_scroll = 0i64;
            }
            pub fn get_icon(&self) -> String {
                return self.icon.to_string();
            }
            pub fn get_name(&self) -> String {
                return self.name.to_string();
            }
        }
    }

    /// `MapElementRenderHandler` (xaero/map/element/MapElementRenderHandler.java)
    pub mod map_element_render_handler {
        use super::*;
        // The class itself isn't translated: it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)). Its static parts are below.
        // MapElementRenderHandler.MapElementRenderHandler(3): not translated, its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))).
        // MapElementRenderHandler.add(1): not translated, its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))).
        // MapElementRenderHandler.createResult(2): not translated, it's generic.
        // MapElementRenderHandler.getRenderer(1): not translated, it's generic.
        // MapElementRenderHandler.render(17): not translated, its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))).
        // MapElementRenderHandler.renderHoveredWithRenderer(9): not translated, it's generic.
        // MapElementRenderHandler.renderWithRenderer(11): not translated, it's generic.
        // MapElementRenderHandler.transformAndRenderElement(11): not translated, it's generic.
        // MapElementRenderHandler.getElementIndexDepth(2): not translated, its class isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))).
    }

    /// `MapElementRenderHandler.Builder` (xaero/map/element/MapElementRenderHandler.java)
    pub mod map_element_render_handler_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub pose_stack: JPoseStack,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { pose_stack: JPoseStack::new() };
                s
            }
            // MapElementRenderHandler.Builder.setDefault(0): not translated, `null` (Rust has no null: this needs an Option).
            // MapElementRenderHandler.Builder.setPoseStack(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            // MapElementRenderHandler.Builder.build(0): not translated, it uses MapElementRenderHandler, which isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))).
        }
        pub fn begin() -> map_element_render_handler_builder::Builder {
            return map_element_render_handler_builder::Builder::new();
        }
    }

    /// `MapElementRenderLocation` (xaero/map/element/MapElementRenderLocation.java)
    pub mod map_element_render_location {
        use super::*;
        pub const UNKNOWN: i32 = -1;
        pub const IN_MINIMAP: i32 = 0;
        pub const OVER_MINIMAP: i32 = 1;
        pub const IN_GAME: i32 = 2;
        pub const WORLD_MAP: i32 = 3;
        pub const WORLD_MAP_MENU: i32 = 4;
        #[derive(Clone, Debug)]
        pub struct MapElementRenderLocation {
        }
        impl MapElementRenderLocation {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn from_minimap(location: i32) -> i32 {
            return if (location <= 4) && (location >= (-1)) { location } else { -1 };
        }
    }

    /// `MenuHitboxReader` (xaero/map/element/MenuHitboxReader.java)
    pub mod menu_hitbox_reader {
        use super::*;
        // The class itself isn't translated: it extends `MenuOnlyElementReader`. Its static parts are below.
        // MenuHitboxReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuHitboxReader.isMouseOverMenuElement(6): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuHitboxReader.getMenuName(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuHitboxReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuHitboxReader.getFilterName(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuHitboxReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuHitboxReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
    }

    /// `MenuOnlyElementReader` (xaero/map/element/MenuOnlyElementReader.java)
    pub mod menu_only_element_reader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MenuOnlyElementReader.isHidden(2): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getRenderX(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getRenderZ(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getInteractionBoxRight(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getInteractionBoxTop(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getRenderBoxLeft(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getRenderBoxRight(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getRenderBoxTop(3): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementReader.getRenderBoxBottom(3): not translated, its class isn't translated (it's generic).
    }

    /// `MenuOnlyElementRenderer` (xaero/map/element/MenuOnlyElementRenderer.java)
    pub mod menu_only_element_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MenuOnlyElementRenderer.MenuOnlyElementRenderer(1): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementRenderer.shouldRender(2): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementRenderer.preRender(4): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementRenderer.postRender(4): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementRenderer.renderElementShadow(9): not translated, its class isn't translated (it's generic).
        // MenuOnlyElementRenderer.renderElement(10): not translated, its class isn't translated (it's generic).
    }

    /// `MenuScrollReader` (xaero/map/element/MenuScrollReader.java)
    pub mod menu_scroll_reader {
        use super::*;
        // The class itself isn't translated: it extends `MenuOnlyElementReader`. Its static parts are below.
        // MenuScrollReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuScrollReader.getMenuName(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuScrollReader.getFilterName(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuScrollReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuScrollReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
        // MenuScrollReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `MenuOnlyElementReader`).
    }

    /// `ElementReader` (xaero/map/element/render/ElementReader.java)
    pub mod element_reader {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ElementReader.isHidden(2): not translated, it has no body.
        // ElementReader.getRenderX(3): not translated, it has no body.
        // ElementReader.getRenderZ(3): not translated, it has no body.
        // ElementReader.getInteractionBoxLeft(3): not translated, it has no body.
        // ElementReader.getInteractionBoxRight(3): not translated, it has no body.
        // ElementReader.getInteractionBoxTop(3): not translated, it has no body.
        // ElementReader.getInteractionBoxBottom(3): not translated, it has no body.
        // ElementReader.getRenderBoxLeft(3): not translated, it has no body.
        // ElementReader.getRenderBoxRight(3): not translated, it has no body.
        // ElementReader.getRenderBoxTop(3): not translated, it has no body.
        // ElementReader.getRenderBoxBottom(3): not translated, it has no body.
        // ElementReader.getLeftSideLength(2): not translated, it has no body.
        // ElementReader.getMenuName(1): not translated, it has no body.
        // ElementReader.getFilterName(1): not translated, it has no body.
        // ElementReader.getMenuTextFillLeftPadding(1): not translated, it has no body.
        // ElementReader.getRightClickTitleBackgroundColor(1): not translated, it has no body.
        // ElementReader.shouldScaleBoxWithOptionalScale(0): not translated, it has no body.
        // ElementReader.isInteractable(2): not translated, its class isn't translated (it's generic).
        // ElementReader.getBoxScale(3): not translated, its class isn't translated (it's generic).
        // ElementReader.isMouseOverMenuElement(6): not translated, its class isn't translated (it's generic).
        // ElementReader.isHoveredOnMap(9): not translated, its class isn't translated (it's generic).
        // ElementReader.isOnScreen(10): not translated, its class isn't translated (it's generic).
        // ElementReader.getRightClickOptions(2): not translated, its class isn't translated (it's generic).
        // ElementReader.isRightClickValid(1): not translated, its class isn't translated (it's generic).
        // ElementReader.getTooltip(3): not translated, its class isn't translated (it's generic).
        // ElementReader.getRenderY(3): not translated, its class isn't translated (it's generic).
        // ElementReader.hasYCoordinate(0): not translated, its class isn't translated (it's generic).
    }

    /// `ElementRenderer` (xaero/map/element/render/ElementRenderer.java)
    pub mod element_renderer {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ElementRenderer.ElementRenderer(3): not translated, its class isn't translated (it's generic).
        // ElementRenderer.shouldRenderHovered(1): not translated, its class isn't translated (it's generic).
        // ElementRenderer.getReader(0): not translated, its class isn't translated (it's generic).
        // ElementRenderer.getContext(0): not translated, its class isn't translated (it's generic).
        // ElementRenderer.getProvider(0): not translated, its class isn't translated (it's generic).
        // ElementRenderer.getOrder(0): not translated, its class isn't translated (it's generic).
        // ElementRenderer.shouldBeDimScaled(0): not translated, its class isn't translated (it's generic).
        // ElementRenderer.compareTo(1): not translated, its class isn't translated (it's generic).
        // ElementRenderer.preRender(4): not translated, it has no body.
        // ElementRenderer.postRender(4): not translated, it has no body.
        // ElementRenderer.renderElementShadow(9): not translated, it has no body.
        // ElementRenderer.renderElement(10): not translated, it has no body.
        // ElementRenderer.shouldRender(2): not translated, it has no body.
    }

    /// `ElementRenderInfo` (xaero/map/element/render/ElementRenderInfo.java)
    pub mod element_render_info {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Entity`. Its static parts are below.
        // ElementRenderInfo.ElementRenderInfo(14): not translated, its class isn't translated (it uses Minecraft's `Entity`).
    }

    /// `ElementRenderLocation` (xaero/map/element/render/ElementRenderLocation.java)
    pub mod element_render_location {
        use super::*;
        // ALL: not translated, it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints).
        // UNKNOWN: not translated, it creates a ElementRenderLocation with a constructor that isn't translated.
        // IN_MINIMAP: not translated, it creates a ElementRenderLocation with a constructor that isn't translated.
        // OVER_MINIMAP: not translated, it creates a ElementRenderLocation with a constructor that isn't translated.
        // IN_WORLD: not translated, it creates a ElementRenderLocation with a constructor that isn't translated.
        // WORLD_MAP: not translated, it creates a ElementRenderLocation with a constructor that isn't translated.
        // WORLD_MAP_MENU: not translated, it creates a ElementRenderLocation with a constructor that isn't translated.
        #[derive(Clone, Debug)]
        pub struct ElementRenderLocation {
            pub index: i32,
        }
        impl ElementRenderLocation {
            // ElementRenderLocation.ElementRenderLocation(1): not translated, it uses ElementRenderLocation.ALL, which isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
            pub fn get_index(&self) -> i32 {
                return self.index;
            }
        }
        // ElementRenderLocation.fromIndex(1): not translated, it uses ElementRenderLocation.ALL, which isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
    }

    /// `ElementRenderProvider` (xaero/map/element/render/ElementRenderProvider.java)
    pub mod element_render_provider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ElementRenderProvider.begin(2): not translated, it has no body.
        // ElementRenderProvider.hasNext(2): not translated, it has no body.
        // ElementRenderProvider.getNext(2): not translated, it has no body.
        // ElementRenderProvider.setupContextAndGetNext(2): not translated, its class isn't translated (it's generic).
        // ElementRenderProvider.end(2): not translated, it has no body.
    }

    /// `EntityUtil` (xaero/map/entity/util/EntityUtil.java)
    pub mod entity_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EntityUtil {
        }
        impl EntityUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityUtil.getEntityX(2): not translated, it uses Minecraft's `Entity`.
        // EntityUtil.getEntityY(2): not translated, it uses Minecraft's `Entity`.
        // EntityUtil.getEntityZ(2): not translated, it uses Minecraft's `Entity`.
        // EntityUtil.getEntityPos(2): not translated, it uses Minecraft's `Vec3`.
    }

    /// `ClientEvents` (xaero/map/events/ClientEvents.java)
    pub mod client_events {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `RealmsServer`. Its static parts are below.
        // ClientEvents.handleGuiOpen(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.onMinecraftRunTickStart(0): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleRenderTick(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleDrawScreen(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handlePlayerSetSpawnEvent(2): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleWorldUnload(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.getLatestRealm(0): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleRenderCrosshairOverlay(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleClientPlayerChatReceivedEvent(3): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleClientSystemChatReceivedEvent(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleChatMessage(2): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handlePlayerTickStart(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleClientTickStart(0): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleClientRunTickStart(0): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
        // ClientEvents.handleTextureStitchEventPost(1): not translated, its class isn't translated (it uses Minecraft's `RealmsServer`).
    }

    /// `ClientEventsFabric` (xaero/map/events/ClientEventsFabric.java)
    pub mod client_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `ClientEvents`. Its static parts are below.
    }

    /// `CommonEvents` (xaero/map/events/CommonEvents.java)
    pub mod common_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonEvents {
        }
        impl CommonEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // CommonEvents.onPlayerClone(3): not translated, it uses Minecraft's `Player`.
            // CommonEvents.onServerStarting(1): not translated, it uses Minecraft's `MinecraftServer`.
            // CommonEvents.onServerStopped(1): not translated, it uses Minecraft's `MinecraftServer`.
            // CommonEvents.onPlayerLogIn(1): not translated, it uses Minecraft's `Player`.
            // CommonEvents.onPlayerWorldJoin(1): not translated, it uses Minecraft's `ServerPlayer`.
            // CommonEvents.handlePlayerTickStart(1): not translated, it uses Minecraft's `Player`.
        }
    }

    /// `CommonEventsFabric` (xaero/map/events/CommonEventsFabric.java)
    pub mod common_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `CommonEvents`. Its static parts are below.
        // CommonEventsFabric.register(0): not translated, its class isn't translated (it extends `CommonEvents`).
    }

    /// `ModClientEvents` (xaero/map/events/ModClientEvents.java)
    pub mod mod_client_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModClientEvents {
        }
        impl ModClientEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModClientEventsFabric` (xaero/map/events/ModClientEventsFabric.java)
    pub mod mod_client_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `ModClientEvents`. Its static parts are below.
    }

    /// `ModCommonEvents` (xaero/map/events/ModCommonEvents.java)
    pub mod mod_common_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModCommonEvents {
        }
        impl ModCommonEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModCommonEventsFabric` (xaero/map/events/ModCommonEventsFabric.java)
    pub mod mod_common_events_fabric {
        use super::*;
        // The class itself isn't translated: it extends `ModCommonEvents`. Its static parts are below.
    }

    /// `Executor` (xaero/map/executor/Executor.java)
    pub mod executor {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockableEventLoop`. Its static parts are below.
        // Executor.Executor(3): not translated, its class isn't translated (it extends Minecraft's `BlockableEventLoop`).
        // Executor.wrapRunnable(1): not translated, its class isn't translated (it extends Minecraft's `BlockableEventLoop`).
        // Executor.shouldRun(1): not translated, its class isn't translated (it extends Minecraft's `BlockableEventLoop`).
        // Executor.getRunningThread(0): not translated, its class isn't translated (it extends Minecraft's `BlockableEventLoop`).
        // Executor.runAllTasks(0): not translated, its class isn't translated (it extends Minecraft's `BlockableEventLoop`).
    }

    /// `PNGExporter` (xaero/map/file/export/PNGExporter.java)
    pub mod png_exporter {
        use super::*;
        // The class itself isn't translated: it uses `Calendar` (java.util). Its static parts are below.
        // PNGExporter.PNGExporter(1): not translated, its class isn't translated (it uses `Calendar` (java.util)).
        // PNGExporter.export(5): not translated, its class isn't translated (it uses `Calendar` (java.util)).
        // PNGExporter.saveImage(4): not translated, its class isn't translated (it uses `Calendar` (java.util)).
        // PNGExporter.getExportBaseName(0): not translated, its class isn't translated (it uses `Calendar` (java.util)).
    }

    /// `PNGExportResult` (xaero/map/file/export/PNGExportResult.java)
    pub mod png_export_result {
        use super::*;
        // The class itself isn't translated: it uses PNGExportResultType, which isn't translated (its constants carry values). Its static parts are below.
        // PNGExportResult.PNGExportResult(2): not translated, its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)).
        // PNGExportResult.getType(0): not translated, its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)).
        // PNGExportResult.getFolderToOpen(0): not translated, its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)).
        // PNGExportResult.getMessage(0): not translated, its class isn't translated (it uses PNGExportResultType, which isn't translated (its constants carry values)).
    }

    /// `PNGExportResultType` (xaero/map/file/export/PNGExportResultType.java)
    pub mod png_export_result_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // PNGExportResultType.PNGExportResultType(2): not translated, its class isn't translated (its constants carry values).
        // PNGExportResultType.getMessage(0): not translated, its class isn't translated (its constants carry values).
        // PNGExportResultType.isSuccess(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `IOHelper` (xaero/map/file/IOHelper.java)
    pub mod io_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IOHelper {
        }
        impl IOHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // IOHelper.readToBuffer(3): not translated, it uses `DataInputStream` (java.io).
    }

    /// `MapRegionInfo` (xaero/map/file/MapRegionInfo.java)
    pub mod map_region_info {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MapRegionInfo.shouldCache(0): not translated, it has no body.
        // MapRegionInfo.getRegionFile(0): not translated, it has no body.
        // MapRegionInfo.getCacheFile(0): not translated, it has no body.
        // MapRegionInfo.getWorldId(0): not translated, it has no body.
        // MapRegionInfo.getDimId(0): not translated, it has no body.
        // MapRegionInfo.getMwId(0): not translated, it has no body.
        // MapRegionInfo.getRegionX(0): not translated, it has no body.
        // MapRegionInfo.getRegionZ(0): not translated, it has no body.
        // MapRegionInfo.setShouldCache(2): not translated, it has no body.
        // MapRegionInfo.setCacheFile(1): not translated, it has no body.
        // MapRegionInfo.hasLookedForCache(0): not translated, it has no body.
    }

    /// `MapSaveLoad` (xaero/map/file/MapSaveLoad.java)
    pub mod map_save_load {
        use super::*;
        // The class itself isn't translated: it uses MapRegion, which isn't translated (it extends `LeveledRegion`). Its static parts are below.
        pub const CURRENT_SAVE_MAJOR_VERSION: i32 = 8;
        pub const CURRENT_SAVE_MINOR_VERSION: i32 = 8;
        pub const SAVE_TIME: i32 = 60000;
        pub const CURRENT_CACHE_SAVE_MAJOR_VERSION: i32 = 3;
        pub const CURRENT_CACHE_SAVE_MINOR_VERSION: i32 = 25;
        // MapSaveLoad.MapSaveLoad(4): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.setMapProcessor(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.exportPNG(2): not translated, its body couldn't be read: an anonymous class.
        // MapSaveLoad.getSecondaryFile(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getTempFile(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getCacheFolder(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getCacheFile(4): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getFile(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getNormalFile(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getRootFolder(1): not translated, it uses `File` (java.io).
        // MapSaveLoad.getMainFolder(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getMWSubFolder(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getCaveLayerFolder(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getMWSubFolder(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getOldFolder(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.xaeroToZip(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.detectRegions(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.detectRegionsFromFiles(11): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.saveRegion(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getBackupFolder(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.backupFile(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.loadRegion(7): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.beingSaved(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.requestLoad(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.requestLoad(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.requestBranchCache(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.requestBranchCache(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.addToLoad(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.removeToLoad(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.clearToLoad(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getSizeOfToLoad(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.saveExists(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.updateSave(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.run(5): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.removeTempCacheRequest(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.addTempCacheRequest(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.cleanupLoadedCache(6): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.savePixel(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.loadPixel(9): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.saveOverlay(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.loadOverlay(7): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.isRegionDetectionComplete(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.setRegionDetectionComplete(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.requestCache(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.removeToCache(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.removeToCache(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.toCacheContains(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getToSave(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getNextToLoadByViewing(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.setNextToLoadByViewing(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getOldFormatSupport(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.safeDelete(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.safeMoveAndReplace(4): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapSaveLoad.getSizeOfToLoadBranchCache(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
    }

    /// `OldFormatSupport` (xaero/map/file/OldFormatSupport.java)
    pub mod old_format_support {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // OldFormatSupport.OldFormatSupport(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.loadVanillaStates(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.loadModdedStates(4): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.loadStates(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.loadStates(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.putState(4): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.getStateForId(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.getStateForId(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBlock(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBlockName1314(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBlock1516(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBlock1617(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBlock1215(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBlockState262_263(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBiome(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBiome(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBiome(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OldFormatSupport.fixBiome1718(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `OldFormatSupport.Fixer` (xaero/map/file/OldFormatSupport.java)
    pub mod old_format_support_fixer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // OldFormatSupport.Fixer.fix(1): not translated, it has no body.
    }

    /// `RegionDetection` (xaero/map/file/RegionDetection.java)
    pub mod region_detection {
        use super::*;
        // The class itself isn't translated: it implements `MapRegionInfo`. Its static parts are below.
        // RegionDetection.RegionDetection(8): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getInitialVersion(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.shouldCache(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getCacheFile(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.setShouldCache(2): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.setCacheFile(1): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getWorldId(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getDimId(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getMwId(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getRegionX(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getRegionZ(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.transferInfoTo(1): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.transferInfoPostAddTo(2): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.transferInfoFrom(1): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getRegionFile(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.isHasHadTerrain(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.hasLookedForCache(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.setRemoved(1): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.isRemoved(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.setNext(1): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.setPrevious(1): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getNext(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.getPrevious(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.isDestroyed(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
        // RegionDetection.onDestroyed(0): not translated, its class isn't translated (it implements `MapRegionInfo`).
    }

    /// `WorldDataBiomeManager` (xaero/map/file/worldsave/biome/WorldDataBiomeManager.java)
    pub mod world_data_biome_manager {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BiomeResolver`. Its static parts are below.
        // WorldDataBiomeManager.WorldDataBiomeManager(0): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
        // WorldDataBiomeManager.resetChunkBiomeData(4): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
        // WorldDataBiomeManager.clear(0): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
        // WorldDataBiomeManager.getChunkBiomeData(2): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
        // WorldDataBiomeManager.addBiomeSectionForRegionChunk(4): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
        // WorldDataBiomeManager.getBiome(4): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
        // WorldDataBiomeManager.getNoiseBiome(3): not translated, its class isn't translated (it implements Minecraft's `BiomeResolver`).
    }

    /// `WorldDataReaderChunkBiomeData` (xaero/map/file/worldsave/biome/WorldDataReaderChunkBiomeData.java)
    pub mod world_data_reader_chunk_biome_data {
        use super::*;
        // The class itself isn't translated: it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints). Its static parts are below.
        // WorldDataReaderChunkBiomeData.addSection(2): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // WorldDataReaderChunkBiomeData.getNoiseBiome(5): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // WorldDataReaderChunkBiomeData.clear(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
    }

    /// `WorldDataReaderSectionBiomeData` (xaero/map/file/worldsave/biome/WorldDataReaderSectionBiomeData.java)
    pub mod world_data_reader_section_biome_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ListTag`. Its static parts are below.
        // WorldDataReaderSectionBiomeData.WorldDataReaderSectionBiomeData(2): not translated, its class isn't translated (it uses Minecraft's `ListTag`).
        // WorldDataReaderSectionBiomeData.hasDifferentBiomes(0): not translated, its class isn't translated (it uses Minecraft's `ListTag`).
        // WorldDataReaderSectionBiomeData.get(3): not translated, its class isn't translated (it uses Minecraft's `ListTag`).
    }

    /// `WorldDataChunkTileEntityLookup` (xaero/map/file/worldsave/WorldDataChunkTileEntityLookup.java)
    pub mod world_data_chunk_tile_entity_lookup {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ListTag`. Its static parts are below.
        // WorldDataChunkTileEntityLookup.WorldDataChunkTileEntityLookup(1): not translated, its class isn't translated (it uses Minecraft's `ListTag`).
        // WorldDataChunkTileEntityLookup.loadIfNeeded(0): not translated, its class isn't translated (it uses Minecraft's `ListTag`).
        // WorldDataChunkTileEntityLookup.getTileEntityNbt(3): not translated, its class isn't translated (it uses Minecraft's `ListTag`).
    }

    /// `WorldDataHandler` (xaero/map/file/worldsave/WorldDataHandler.java)
    pub mod world_data_handler {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ServerLevel`. Its static parts are below.
        // WorldDataHandler.WorldDataHandler(2): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.handleRenderExecutor(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.prepareSingleplayer(2): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.buildRegion(6): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.onServerWorldUnload(1): not translated, it uses Minecraft's `ServerLevel`.
        // WorldDataHandler.getWorldServer(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.getWorldDataReader(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.getWorldDir(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.getWorldDataRenderExecutor(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
        // WorldDataHandler.onResourceReload(0): not translated, its class isn't translated (it uses Minecraft's `ServerLevel`).
    }

    /// `WorldDataHandler.Result` (xaero/map/file/worldsave/WorldDataHandler.java)
    pub mod world_data_handler_result {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Result_ {
            Success,
            Fail,
            Cancel,
        }
        impl Result_ {
            pub const VALUES: [Result_; 3] = [Result_::Success, Result_::Fail, Result_::Cancel, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Result_::Success => "SUCCESS",
                    Result_::Fail => "FAIL",
                    Result_::Cancel => "CANCEL",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "SUCCESS" => Result_::Success,
                    "FAIL" => Result_::Fail,
                    "CANCEL" => Result_::Cancel,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `WorldDataReader` (xaero/map/file/worldsave/WorldDataReader.java)
    pub mod world_data_reader {
        use super::*;
        // The class itself isn't translated: it uses MapBlock, which isn't translated (it extends `MapPixel`). Its static parts are below.
        // WorldDataReader.WorldDataReader(5): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.setMapProcessor(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.updateHeightArray(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.buildRegion(8): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.readChunkNBTCompounds(2): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.readChunk(2): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.buildTileChunk(14): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.buildTile(19): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.buildPixel(20): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.buildPixelHelp(18): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.handleTileChunkOutsideRegion(9): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.handleTileOutsideRegion(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.handleChunkBiomes(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.fillBiomes(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.transferFilledBiomes(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.shouldOverlayCached(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.nibbleValue(2): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // WorldDataReader.onResourceReload(0): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
    }

    /// `CustomRenderTypes` (xaero/map/graphics/CustomRenderTypes.java)
    pub mod custom_render_types {
        use super::*;
        // GUI_NEAREST: not translated, it uses Minecraft's `RenderType`.
        // GUI_BILINEAR: not translated, it uses Minecraft's `RenderType`.
        // GUI_BILINEAR_PRE: not translated, it uses Minecraft's `RenderType`.
        // MAP_BILINEAR: not translated, it uses Minecraft's `RenderType`.
        // MAP: not translated, it uses Minecraft's `RenderType`.
        // MAP_EXPORT: not translated, it uses Minecraft's `RenderType`.
        // MAP_BRANCH: not translated, it uses Minecraft's `RenderType`.
        // MAP_COLOR_OVERLAY: not translated, it uses Minecraft's `RenderType`.
        // MAP_FRAME: not translated, it uses Minecraft's `RenderType`.
        // MAP_COLOR_FILLER: not translated, it uses Minecraft's `RenderType`.
        // MAP_ELEMENT_TEXT_BG: not translated, it uses Minecraft's `RenderType`.
        #[derive(Clone, Debug)]
        pub struct CustomRenderTypes {
        }
        impl CustomRenderTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CustomRenderTypes.applyFixedOrder(0): not translated, it uses `XaeroBufferProvider` (xaero.lib.client.graphics).
    }

    /// `GpuObjectDeleter` (xaero/map/graphics/GpuObjectDeleter.java)
    pub mod gpu_object_deleter {
        use super::*;
        // The class itself isn't translated: it uses `GpuTextureAndView` (xaero.lib.client.graphics). Its static parts are below.
        pub const DELETE_PER_FRAME: i32 = 5;
        // GpuObjectDeleter.work(0): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // GpuObjectDeleter.requestTextureDeletion(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // GpuObjectDeleter.requestBufferToDelete(1): not translated, its class isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // GpuObjectDeleter.deleteBuffers(2): not translated, it uses Minecraft's `GpuBuffer`.
        // GpuObjectDeleter.deleteTextures(2): not translated, it uses `GpuTextureAndView` (xaero.lib.client.graphics).
    }

    /// `ImprovedFramebuffer` (xaero/map/graphics/ImprovedFramebuffer.java)
    pub mod improved_framebuffer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TextureTarget`. Its static parts are below.
        // MAIN_RENDER_TARGET_FIELD: not translated, it uses `Field` (java.lang.reflect).
        // mainRenderTargetBackup: not translated, it uses Minecraft's `RenderTarget`.
        // outputColorTextureOverrideBU: not translated, it uses Minecraft's `GpuTextureView`.
        // outputDepthTextureOverrideBU: not translated, it uses Minecraft's `GpuTextureView`.
        // ImprovedFramebuffer.ImprovedFramebuffer(4): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.bindDefaultFramebuffer(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.forceAsMainRenderTarget(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        pub fn restore_main_render_target() {
            // (`if` always false in Horizon: only the else part)
        }
        // ImprovedFramebuffer.bindAsMainTarget(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setColorTexture(2): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setColorTexture(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setDepthTexture(2): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.getSampler(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.setSampler(1): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.closeColorTexture(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.closeDepthTexture(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.getTas(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
        // ImprovedFramebuffer.getFormat(0): not translated, its class isn't translated (it extends Minecraft's `TextureTarget`).
    }

    /// `MapRenderHelper` (xaero/map/graphics/MapRenderHelper.java)
    pub mod map_render_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapRenderHelper {
        }
        impl MapRenderHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MapRenderHelper.renderBranchUpdate(12): not translated, it uses Minecraft's `GpuTextureView`.
        // MapRenderHelper.fillIntoExistingBuffer(10): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.blitIntoExistingBuffer(16): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.blitIntoExistingBuffer(12): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.blitIntoMultiTextureRenderer(17): not translated, it uses Minecraft's `GpuTextureView`.
        // MapRenderHelper.blitIntoMultiTextureRenderer(15): not translated, it uses Minecraft's `GpuTextureView`.
        // MapRenderHelper.blitIntoMultiTextureRenderer(13): not translated, it uses Minecraft's `GpuTextureView`.
        // MapRenderHelper.renderDynamicHighlight(16): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.renderDynamicChunkHighlight(16): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.drawCenteredStringWithBackground(10): not translated, it's overloaded with the same number of parameters.
        // MapRenderHelper.drawStringWithBackground(10): not translated, it's overloaded with the same number of parameters.
        // MapRenderHelper.drawCenteredStringWithBackground(10): not translated, it's overloaded with the same number of parameters.
        // MapRenderHelper.drawStringWithBackground(10): not translated, it's overloaded with the same number of parameters.
        // MapRenderHelper.blitIntoExistingBuffer(10): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.blitIntoExistingBuffer(8): not translated, it uses Minecraft's `VertexConsumer`.
        // MapRenderHelper.blitIntoMultiTextureRenderer(11): not translated, it uses Minecraft's `GpuTextureView`.
        // MapRenderHelper.blitIntoMultiTextureRenderer(9): not translated, it uses Minecraft's `GpuTextureView`.
        pub fn restore_default_shader_blend_state() {
        }
    }

    /// `OpenGlHelper` (xaero/map/graphics/OpenGlHelper.java)
    pub mod open_gl_helper {
        use super::*;
        // deviceCheckCache: not translated, it uses Minecraft's `GpuDevice`.
        pub static CACHED_DEVICE_IS_OPEN_GL: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct OpenGlHelper {
        }
        impl OpenGlHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // OpenGlHelper.isUsingOpenGL(0): not translated, it uses Minecraft's `GpuDevice`.
    }

    /// `PixelBuffers` (xaero/map/graphics/PixelBuffers.java)
    pub mod pixel_buffers {
        use super::*;
        pub static BUFFERS_TYPE: JCell<i32> = JCell::new(0);
        #[derive(Clone, Debug)]
        pub struct PixelBuffers {
        }
        impl PixelBuffers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PixelBuffers.glBindBuffer(2): not translated, it calls `GL15` (org.lwjgl.opengl).glBindBuffer.
    }

    /// `MultiTextureRenderTypeRenderer` (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java)
    pub mod multi_texture_render_type_renderer {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ByteBufferBuilder`. Its static parts are below.
        pub const RENDER_PASS_NAME: &str = "xaero wm multitexture render pass";
        // MultiTextureRenderTypeRenderer.MultiTextureRenderTypeRenderer(0): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.init(3): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.draw(0): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.uploadImmediateVertexBuffer(1): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.endBuffer(1): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
        // MultiTextureRenderTypeRenderer.begin(1): not translated, its class isn't translated (it uses Minecraft's `ByteBufferBuilder`).
    }

    /// `MultiTextureRenderTypeRenderer.DrawCall` (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRenderer.java)
    pub mod multi_texture_render_type_renderer_draw_call {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuBufferSlice`. Its static parts are below.
        // MultiTextureRenderTypeRenderer.DrawCall.DrawCall(5): not translated, its class isn't translated (it uses Minecraft's `GpuBufferSlice`).
    }

    /// `MultiTextureRenderTypeRendererProvider` (xaero/map/graphics/renderer/multitexture/MultiTextureRenderTypeRendererProvider.java)
    pub mod multi_texture_render_type_renderer_provider {
        use super::*;
        // The class itself isn't translated: it uses `Deque` (java.util). Its static parts are below.
        // MultiTextureRenderTypeRendererProvider.MultiTextureRenderTypeRendererProvider(1): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(1): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(2): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.getRenderer(3): not translated, its class isn't translated (it uses `Deque` (java.util)).
        // MultiTextureRenderTypeRendererProvider.draw(1): not translated, its class isn't translated (it uses `Deque` (java.util)).
    }

    /// `TextureUpload` (xaero/map/graphics/TextureUpload.java)
    pub mod texture_upload {
        use super::*;
        // The class itself isn't translated: it implements `PoolUnit` (xaero.map.pool). Its static parts are below.
        // DOWNLOAD_CALLBACK: not translated, a function kept in a variable or field (only parameters can be).
        // TextureUpload.create(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // TextureUpload.run(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // TextureUpload.upload(0): not translated, it has no body.
        // TextureUpload.getUploadType(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
    }

    /// `TextureUpload.BranchDownload` (xaero/map/graphics/TextureUpload.java)
    pub mod texture_upload_branch_download {
        use super::*;
        // The class itself isn't translated: it extends `TextureUpload`. Its static parts are below.
        // TextureUpload.BranchDownload.BranchDownload(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.BranchDownload.BranchDownload(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.BranchDownload.upload(0): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.BranchDownload.create(1): not translated, its class isn't translated (it extends `TextureUpload`).
    }

    /// `TextureUpload.BranchUpdate` (xaero/map/graphics/TextureUpload.java)
    pub mod texture_upload_branch_update {
        use super::*;
        // The class itself isn't translated: it extends `TextureUpload`. Its static parts are below.
        // TextureUpload.BranchUpdate.BranchUpdate(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.BranchUpdate.BranchUpdate(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.BranchUpdate.upload(0): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.BranchUpdate.create(1): not translated, its class isn't translated (it extends `TextureUpload`).
    }

    /// `TextureUpload.Normal` (xaero/map/graphics/TextureUpload.java)
    pub mod texture_upload_normal {
        use super::*;
        // The class itself isn't translated: it extends `TextureUpload`. Its static parts are below.
        // TextureUpload.Normal.Normal(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.Normal.Normal(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.Normal.upload(0): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.Normal.create(1): not translated, its class isn't translated (it extends `TextureUpload`).
    }

    /// `TextureUpload.SubsequentNormal` (xaero/map/graphics/TextureUpload.java)
    pub mod texture_upload_subsequent_normal {
        use super::*;
        // The class itself isn't translated: it extends `TextureUpload`. Its static parts are below.
        // TextureUpload.SubsequentNormal.SubsequentNormal(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.SubsequentNormal.SubsequentNormal(1): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.SubsequentNormal.upload(0): not translated, its class isn't translated (it extends `TextureUpload`).
        // TextureUpload.SubsequentNormal.create(1): not translated, its class isn't translated (it extends `TextureUpload`).
    }

    /// `TextureUploadBenchmark` (xaero/map/graphics/TextureUploadBenchmark.java)
    pub mod texture_upload_benchmark {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TextureUploadBenchmark {
            pub accumulators: Vec<i64>,
            pub results: Vec<i64>,
            pub totals: Vec<i32>,
            pub finished: Vec<bool>,
            pub n_of_elements: Vec<i32>,
            pub n_of_finished: i32,
            pub all_finished: bool,
        }
        impl TextureUploadBenchmark {
            pub fn new(n_of_elements: &[i32]) -> Self {
                let mut s = Self { accumulators: Vec::new(), results: Vec::new(), totals: Vec::new(), finished: Vec::new(), n_of_elements: Vec::new(), n_of_finished: 0i32, all_finished: false };
                let n_of_types: i32 = (n_of_elements.len() as i32);
                s.accumulators = vec![0i64; n_of_types as usize];
                s.totals = vec![0i32; n_of_types as usize];
                s.results = vec![0i64; n_of_types as usize];
                s.finished = vec![false; n_of_types as usize];
                s.n_of_elements = n_of_elements.to_vec();
                s
            }
            pub fn pre(&self) {
                misc::timer_pre();
            }
            // TextureUploadBenchmark.post(1): not translated, it uses Minecraft's `GpuFence`.
            pub fn finish(&mut self, r#type: i32) {
                self.results[(r#type) as usize] = self.accumulators[(r#type) as usize] / (self.totals[(r#type) as usize] as i64);
                self.finished[(r#type) as usize] = true;
                self.n_of_finished += 1;
                if self.n_of_finished == (self.finished.len() as i32) {
                    self.all_finished = true;
                }
            }
            pub fn is_finished_0(&self) -> bool {
                return self.all_finished;
            }
            pub fn is_finished_1(&self, r#type: i32) -> bool {
                return self.finished[(r#type) as usize];
            }
            pub fn get_average(&self, r#type: i32) -> i64 {
                return if self.finished[(r#type) as usize] { self.results[(r#type) as usize] } else { self.accumulators[(r#type) as usize] / (self.totals[(r#type) as usize] as i64) };
            }
            // TextureUploadBenchmark.getTotalsString(0): not translated, it calls `Arrays` (java.util).toString.
        }
    }

    /// `TextureUploader` (xaero/map/graphics/TextureUploader.java)
    pub mod texture_uploader {
        use super::*;
        // The class itself isn't translated: it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool)). Its static parts are below.
        pub const NORMAL: i32 = 0;
        pub const NORMALDOWNLOAD: i32 = 1;
        pub const BRANCHUPDATE: i32 = 3;
        pub const BRANCHUPDATE_ALLOCATE: i32 = 4;
        pub const BRANCHDOWNLOAD: i32 = 5;
        pub const SUBSEQUENT_NORMAL: i32 = 6;
        pub const DEFAULT_NORMAL_TIME: i32 = 1000000;
        pub const DEFAULT_COMPRESSED_TIME: i32 = 1000000;
        pub const DEFAULT_BRANCHUPDATED_TIME: i32 = 3000000;
        pub const DEFAULT_BRANCHUPDATE_ALLOCATE_TIME: i32 = 4000000;
        pub const DEFAULT_BRANCHDOWNLOAD_TIME: i32 = 1000000;
        pub const DEFAULT_SUBSEQUENT_NORMAL_TIME: i32 = 1000000;
        // TextureUploader.TextureUploader(6): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.requestUpload(1): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.requestNormal(8): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.requestSubsequentNormal(9): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.requestBranchUpdate(16): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.requestBranchDownload(3): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.finishNewestRequestImmediately(0): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.uploadTextures(0): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
        // TextureUploader.addToPool(1): not translated, its class isn't translated (it uses TextureUpload, which isn't translated (it implements `PoolUnit` (xaero.map.pool))).
    }

    /// `ConfirmScreenBase` (xaero/map/gui/ConfirmScreenBase.java)
    pub mod confirm_screen_base {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ConfirmScreen`. Its static parts are below.
        // ConfirmScreenBase.ConfirmScreenBase(6): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.ConfirmScreenBase(8): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.onExit(1): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.goBack(0): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.onClose(0): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.renderEscapeScreen(4): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.init(0): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.shouldSkipWorldRender(0): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.onDropdownOpen(1): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.onDropdownClosed(1): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
        // ConfirmScreenBase.getEscape(0): not translated, its class isn't translated (it extends Minecraft's `ConfirmScreen`).
    }

    /// `CursorBox` (xaero/map/gui/CursorBox.java)
    pub mod cursor_box {
        use super::*;
        // The class itself isn't translated: it extends `Tooltip` (xaero.lib.client.gui.widget). Its static parts are below.
        // CursorBox.CursorBox(1): not translated, its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)).
        // CursorBox.CursorBox(2): not translated, its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)).
        // CursorBox.CursorBox(3): not translated, its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)).
        // CursorBox.CursorBox(1): not translated, its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)).
        // CursorBox.CursorBox(2): not translated, its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)).
        // CursorBox.CursorBox(1): not translated, its class isn't translated (it extends `Tooltip` (xaero.lib.client.gui.widget)).
    }

    /// `GuiRightClickMenu` (xaero/map/gui/dropdown/rightclick/GuiRightClickMenu.java)
    pub mod gui_right_click_menu {
        use super::*;
        // The class itself isn't translated: it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown). Its static parts are below.
        // GuiRightClickMenu.GuiRightClickMenu(8): not translated, its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        pub fn should_open_left(option_count: i32, x: i32, w: i32, screen_width: i32) -> bool {
            return ((x + w) - screen_width) > 0;
        }
        pub fn should_open_up(option_count: i32, y: i32, screen_height: i32) -> bool {
            let potential_height: i32 = 11 * option_count;
            return ((y + potential_height) - screen_height) > (potential_height / 2);
        }
        // GuiRightClickMenu.setClosed(1): not translated, its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiRightClickMenu.selectId(2): not translated, its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiRightClickMenu.getMenu(5): not translated, it uses GuiRightClickMenu, which isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiRightClickMenu.getTarget(0): not translated, its class isn't translated (it extends `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
    }

    /// `RightClickOption` (xaero/map/gui/dropdown/rightclick/RightClickOption.java)
    pub mod right_click_option {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Style`. Its static parts are below.
        // RightClickOption.RightClickOption(4): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.RightClickOption(3): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.onAction(1): not translated, it has no body.
        // RightClickOption.onSelected(1): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.getName(0): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.getDisplayName(0): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.isActive(0): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.setActive(1): not translated, its class isn't translated (it uses Minecraft's `Style`).
        // RightClickOption.setNameFormatArgs(1): not translated, its class isn't translated (it uses Minecraft's `Style`).
    }

    /// `EditDefaultTpCommandScreen` (xaero/map/gui/EditDefaultTpCommandScreen.java)
    pub mod edit_default_tp_command_screen {
        use super::*;
        // The class itself isn't translated: it extends `EditStringConfigOptionScreen` (xaero.lib.client.gui.config). Its static parts are below.
        // EditDefaultTpCommandScreen.EditDefaultTpCommandScreen(8): not translated, its class isn't translated (it extends `EditStringConfigOptionScreen` (xaero.lib.client.gui.config)).
        // EditDefaultTpCommandScreen.extractBackground(4): not translated, its class isn't translated (it extends `EditStringConfigOptionScreen` (xaero.lib.client.gui.config)).
    }

    /// `ExportScreen` (xaero/map/gui/ExportScreen.java)
    pub mod export_screen {
        use super::*;
        // The class itself isn't translated: it extends `GuiSettings` (xaero.lib.client.gui). Its static parts are below.
        // EXPORTING_MESSAGE: a game value, so it's in game_side.rs.
        // ExportScreen.ExportScreen(4): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // ExportScreen.init(0): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // ExportScreen.confirm(0): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // ExportScreen.extractBackground(4): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // ExportScreen.extractRenderState(4): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // ExportScreen.primaryOptionEntry(1): not translated, it's generic.
        // ExportScreen.onExportDone(1): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
        // ExportScreen.getSelection(0): not translated, its class isn't translated (it extends `GuiSettings` (xaero.lib.client.gui)).
    }

    /// `GuiCaveModeOptions` (xaero/map/gui/GuiCaveModeOptions.java)
    pub mod gui_cave_mode_options {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `AbstractWidget`. Its static parts are below.
        // GuiCaveModeOptions.onInit(2): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.onCaveModeTypeButton(2): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.createField(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.getCaveStart(0): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.setCaveStart(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.createSlider(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.updateField(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.updateSlider(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.toggle(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.onCaveModeStartSet(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.tick(1): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.unfocusAll(0): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.getCaveModeTypeButtonMessage(0): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
        // GuiCaveModeOptions.isEnabled(0): not translated, its class isn't translated (it uses Minecraft's `AbstractWidget`).
    }

    /// `GuiDimensionOptions` (xaero/map/gui/GuiDimensionOptions.java)
    pub mod gui_dimension_options {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // GuiDimensionOptions.GuiDimensionOptions(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `GuiMap` (xaero/map/gui/GuiMap.java)
    pub mod gui_map {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // FULL_RELOAD_IN_PROGRESS: a game value, so it's in game_side.rs.
        // UNKNOWN_DIMENSION_TYPE1: a game value, so it's in game_side.rs.
        // UNKNOWN_DIMENSION_TYPE2: a game value, so it's in game_side.rs.
        pub const ZOOM_STEP: f64 = 1.2f64;
        pub const WHITE: i32 = -1;
        pub const BLACK: i32 = -16777216;
        // black4f: not translated, it uses `Vector4f` (org.joml).
        pub static LAST_AMOUNT_OF_REGIONS_VIEWED: JCell<i32> = JCell::new(1);
        pub static DEST_SCALE: JCell<f64> = JCell::new(3.0f64);
        // primaryScaleFBO: not translated, it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`).
        // immediateRenderFBO: not translated, it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`).
        pub static ATTACHED_CAMERA: JCell<bool> = JCell::new(false);
        pub static HIDDEN_UI: JCell<bool> = JCell::new(false);
        pub static IDENTITY_MATRIX: LazyLock<JMat4> = LazyLock::new(|| JMat4::IDENTITY);
        // GuiMap.GuiMap(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getScaleMultiplier(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.addRenderableWidget(1): not translated, it's generic.
        // GuiMap.addButton(1): not translated, it's generic.
        // GuiMap.addWidget(1): not translated, it's generic.
        // GuiMap.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.setInitialFocus(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onRenderWaypointsButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onCaveModeButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onDimensionToggleButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onSettingsButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onKeybindingsButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onExportButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.toggleWaypointMenu(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.togglePlayerMenu(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onPlayersButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onClaimsButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onWaypointsButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onRadarButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onZoomInButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onZoomOutButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onHopButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.activateHopToCoords(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onAttachedCameraButton(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.mouseClicked(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.mouseReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.mouseScrolled(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.changeZoom(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.applyZoomLimits(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.removed(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.renderPreDropdown(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.hoveredElementTooltipHelper(2): not translated, it's generic.
        // GuiMap.renderLoadingScreen(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.renderMessageScreen(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.renderMessageScreen(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.drawDotOnMap(6): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.drawArrowOnMap(6): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.drawFarArrowOnMap(6): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.drawObjectOnMap(12): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.renderTexturedModalRectWithLighting3(8): not translated, it uses Minecraft's `GpuTextureView`.
        // GuiMap.renderTexturedModalSubRectWithLighting(12): not translated, it uses Minecraft's `GpuTextureView`.
        // GuiMap.buildTexturedModalRectWithLighting(6): not translated, it uses Minecraft's `BufferBuilder`.
        // GuiMap.buildTexturedModalSubRectWithLighting(10): not translated, it uses Minecraft's `BufferBuilder`.
        // GuiMap.renderTexturedModalRect(14): not translated, it uses Minecraft's `VertexConsumer`.
        // GuiMap.mapClicked(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.handleRightClick(3): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.charTyped(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.keyReleased(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        pub fn bytes_to_mb(bytes: i64) -> i64 {
            return (bytes / 1024i64) / 1024i64;
        }
        // GuiMap.setColourBuffer(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.isUsingTextField(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getTrackedPlayerKeyBinding(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onInputPress(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getCurrentMapCoordinateScale(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onInputRelease(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getHoverTarget(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.unfocusAll(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.closeRightClick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onRightClickClosed(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.closeDropdowns(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getRightClickOptions(0): not translated, its body couldn't be read: an anonymous class.
        // GuiMap.isRightClickValid(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getRightClickTitleBackgroundColor(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.shouldSkipWorldRender(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getUserScale(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getRadarButton(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onDropdownOpen(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onDropdownClosed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.onCaveModeStartSet(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getDimensionOnInit(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.getMapProcessor(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.enableCaveModeOptions(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.removeWidget(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.toggleHiddenUI(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.isMapNavigationInput(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.isExitInput(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMap.renderServerChunkRadius(7): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiMapName` (xaero/map/gui/GuiMapName.java)
    pub mod gui_map_name {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiMapName.GuiMapName(5): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.setInitialFocus(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.canConfirm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.updateConfirmButton(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapName.isInGameUi(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiMapSwitching` (xaero/map/gui/GuiMapSwitching.java)
    pub mod gui_map_switching {
        use super::*;
        // The class itself isn't translated: it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown). Its static parts are below.
        // CONNECT_MAP: a game value, so it's in game_side.rs.
        // DISCONNECT_MAP: a game value, so it's in game_side.rs.
        // GuiMapSwitching.GuiMapSwitching(1): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.init(4): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.getSortedDimensionOptions(1): not translated, it uses GuiDimensionOptions, which isn't translated (it uses Minecraft's `ResourceKey`).
        // GuiMapSwitching.createDimensionDropdown(4): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.createMapDropdown(4): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.isUIPaused(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.isMapSelectionOptionEnabled(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.canToggleThisScreen(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.canDeleteMap(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.canRenameMap(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.canConnectMap(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.canConfirm(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.getConnectButtonLabel(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.updateButtons(3): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.getMultiworldTypeButtonMessage(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.confirm(4): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.getDropdownSelectionIdFromValue(2): not translated, it's generic.
        // GuiMapSwitching.preMapRender(4): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.refreshScreen(3): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.renderText(6): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.postMapRender(6): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
        // GuiMapSwitching.getCreatedDimensionDropdown(0): not translated, its class isn't translated (it uses `DropDownWidget` (xaero.lib.client.gui.widget.dropdown)).
    }

    /// `GuiMapSwitchingButton` (xaero/map/gui/GuiMapSwitchingButton.java)
    pub mod gui_map_switching_button {
        use super::*;
        // The class itself isn't translated: it extends `GuiTexturedButton`. Its static parts are below.
        // TOOLTIP: not translated, it uses `Tooltip` (xaero.lib.client.gui.widget).
        // GuiMapSwitchingButton.GuiMapSwitchingButton(4): not translated, its class isn't translated (it extends `GuiTexturedButton`).
    }

    /// `GuiMapTpCommand` (xaero/map/gui/GuiMapTpCommand.java)
    pub mod gui_map_tp_command {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiMapTpCommand.GuiMapTpCommand(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapTpCommand.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapTpCommand.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapTpCommand.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapTpCommand.canConfirm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapTpCommand.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiMapTpCommand.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiPlayerTpCommand` (xaero/map/gui/GuiPlayerTpCommand.java)
    pub mod gui_player_tp_command {
        use super::*;
        // The class itself isn't translated: it extends `ScreenBase` (xaero.lib.client.gui). Its static parts are below.
        // GuiPlayerTpCommand.GuiPlayerTpCommand(2): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiPlayerTpCommand.init(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiPlayerTpCommand.extractBackground(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiPlayerTpCommand.extractRenderState(4): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiPlayerTpCommand.canConfirm(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiPlayerTpCommand.tick(0): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
        // GuiPlayerTpCommand.keyPressed(1): not translated, its class isn't translated (it extends `ScreenBase` (xaero.lib.client.gui)).
    }

    /// `GuiTexturedButton` (xaero/map/gui/GuiTexturedButton.java)
    pub mod gui_textured_button {
        use super::*;
        // The class itself isn't translated: it extends `TooltipButton`. Its static parts are below.
        // GuiTexturedButton.GuiTexturedButton(13): not translated, its class isn't translated (it extends `TooltipButton`).
        // GuiTexturedButton.getMessage(0): not translated, its class isn't translated (it extends `TooltipButton`).
        // GuiTexturedButton.extractContents(4): not translated, its class isn't translated (it extends `TooltipButton`).
    }

    /// `GuiWorldMapSettings` (xaero/map/gui/GuiWorldMapSettings.java)
    pub mod gui_world_map_settings {
        use super::*;
        // The class itself isn't translated: it extends `EditConfigScreen` (xaero.lib.client.gui.config). Its static parts are below.
        // PLAYER_TELEPORT_COMMAND_TOOLTIP: a game value, so it's in game_side.rs.
        // MAP_TELEPORT_COMMAND_TOOLTIP: a game value, so it's in game_side.rs.
        // GuiWorldMapSettings.GuiWorldMapSettings(1): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.GuiWorldMapSettings(2): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.GuiWorldMapSettings(3): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.getEntryForFullReloadOption(1): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.resetConfirmResult(3): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.init(0): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.onSettingsButton(1): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
        // GuiWorldMapSettings.keyPressed(1): not translated, its class isn't translated (it extends `EditConfigScreen` (xaero.lib.client.gui.config)).
    }

    /// `IRightClickableElement` (xaero/map/gui/IRightClickableElement.java)
    pub mod i_right_clickable_element {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRightClickableElement.getRightClickOptions(0): not translated, it has no body.
        // IRightClickableElement.isRightClickValid(0): not translated, it has no body.
        // IRightClickableElement.getRightClickTitleBackgroundColor(0): not translated, its class isn't translated (it's an interface).
    }

    /// `MapMouseButtonPress` (xaero/map/gui/MapMouseButtonPress.java)
    pub mod map_mouse_button_press {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapMouseButtonPress {
            pub is_down: bool,
            pub clicked: bool,
            pub pressed_at_x: i32,
            pub pressed_at_y: i32,
        }
        impl MapMouseButtonPress {
            pub fn new() -> Self {
                let mut s = Self { is_down: false, clicked: false, pressed_at_x: -1, pressed_at_y: -1 };
                s
            }
        }
    }

    /// `MapTileSelection` (xaero/map/gui/MapTileSelection.java)
    pub mod map_tile_selection {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapTileSelection {
            pub start_x: i32,
            pub start_z: i32,
            pub end_x: i32,
            pub end_z: i32,
        }
        impl MapTileSelection {
            pub fn new(start_x: i32, start_z: i32) -> Self {
                let mut s = Self { start_x: start_x, start_z: start_z, end_x: 0i32, end_z: 0i32 };
                s.start_x = start_x;
                s.start_z = start_z;
                s
            }
            pub fn set_end(&mut self, end_x: i32, end_z: i32) {
                self.end_x = end_x;
                self.end_z = end_z;
            }
            pub fn get_start_x(&self) -> i32 {
                return self.start_x;
            }
            pub fn get_start_z(&self) -> i32 {
                return self.start_z;
            }
            pub fn get_end_x(&self) -> i32 {
                return self.end_x;
            }
            pub fn get_end_z(&self) -> i32 {
                return self.end_z;
            }
            pub fn get_left(&self) -> i32 {
                return self.start_x.min(self.end_x);
            }
            pub fn get_right(&self) -> i32 {
                return self.start_x.max(self.end_x);
            }
            pub fn get_top(&self) -> i32 {
                return self.start_z.min(self.end_z);
            }
            pub fn get_bottom(&self) -> i32 {
                return self.start_z.max(self.end_z);
            }
        }
    }

    /// `Message` (xaero/map/gui/message/Message.java)
    pub mod message {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // Message.Message(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // Message.getText(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // Message.getAdditionTime(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `MessageBox` (xaero/map/gui/message/MessageBox.java)
    pub mod message_box {
        use super::*;
        // The class itself isn't translated: it uses Message, which isn't translated (it uses Minecraft's `Component`). Its static parts are below.
        // MessageBox.MessageBox(3): not translated, its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
        // MessageBox.addMessageLine(1): not translated, its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
        // MessageBox.addMessage(1): not translated, its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
        // MessageBox.addMessageWithSource(2): not translated, its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
        // MessageBox.getCapacity(0): not translated, its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
        // MessageBox.getIterator(0): not translated, its class isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
    }

    /// `MessageBox.Builder` (xaero/map/gui/message/MessageBox.java)
    pub mod message_box_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub width: i32,
            pub capacity: i32,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { width: 0i32, capacity: 0i32 };
                s
            }
            // MessageBox.Builder.setDefault(0): not translated, `this` used as a value.
            // MessageBox.Builder.setWidth(1): not translated, `this` used as a value.
            // MessageBox.Builder.setCapacity(1): not translated, `this` used as a value.
            // MessageBox.Builder.build(0): not translated, it uses MessageBox, which isn't translated (it uses Message, which isn't translated (it uses Minecraft's `Component`)).
        }
        // MessageBox.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `MessageBoxRenderer` (xaero/map/gui/message/render/MessageBoxRenderer.java)
    pub mod message_box_renderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MessageBoxRenderer {
            pub opaque_for: i32,
            pub fade_for: i32,
        }
        impl MessageBoxRenderer {
            pub fn new() -> Self {
                let mut s = Self { opaque_for: 8000, fade_for: 3000 };
                s
            }
            // MessageBoxRenderer.render(6): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        }
    }

    /// `ScreenSwitchSettingEntry` (xaero/map/gui/ScreenSwitchSettingEntry.java)
    pub mod screen_switch_setting_entry {
        use super::*;
        // The class itself isn't translated: it implements `ISettingEntry` (xaero.lib.client.gui). Its static parts are below.
        // ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(4): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.ScreenSwitchSettingEntry(5): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.getStringForSearch(0): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.createWidget(3): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
        // ScreenSwitchSettingEntry.getScreenFactory(0): not translated, its class isn't translated (it implements `ISettingEntry` (xaero.lib.client.gui)).
    }

    /// `TooltipButton` (xaero/map/gui/TooltipButton.java)
    pub mod tooltip_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Plain`. Its static parts are below.
        // TooltipButton.TooltipButton(7): not translated, its class isn't translated (it extends Minecraft's `Plain`).
        // TooltipButton.getXaero_tooltip(0): not translated, its class isn't translated (it extends Minecraft's `Plain`).
    }

    /// `TooltipSlider` (xaero/map/gui/TooltipSlider.java)
    pub mod tooltip_slider {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSliderButton`. Its static parts are below.
        // TooltipSlider.TooltipSlider(9): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TooltipSlider.getXaero_tooltip(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TooltipSlider.updateMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
        // TooltipSlider.applyValue(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSliderButton`).
    }

    /// `GuiUtils` (xaero/map/gui/util/GuiUtils.java)
    pub mod gui_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct GuiUtils {
        }
        impl GuiUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // GuiUtils.getBoundKeyComponent(1): not translated, it uses Minecraft's `Component`.
        // GuiUtils.getBoundKeyComponent(1): not translated, it uses Minecraft's `Component`.
    }

    /// `AbstractHighlighter` (xaero/map/highlight/AbstractHighlighter.java)
    pub mod abstract_highlighter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AbstractHighlighter {
            pub covering_outside_discovered: bool,
            pub result_store: Vec<i32>,
        }
        impl AbstractHighlighter {
            pub fn new(covering_outside_discovered: bool) -> Self {
                let mut s = Self { covering_outside_discovered: covering_outside_discovered, result_store: vec![0i32; 256] };
                s.covering_outside_discovered = covering_outside_discovered;
                s
            }
            // AbstractHighlighter.calculateRegionHash(3): not translated, it has no body.
            // AbstractHighlighter.regionHasHighlights(3): not translated, it has no body.
            // AbstractHighlighter.chunkIsHighlit(3): not translated, it has no body.
            // AbstractHighlighter.getChunkHighlitColor(3): not translated, it has no body.
            // AbstractHighlighter.getBlockHighlightSubtleTooltip(3): not translated, it has no body.
            // AbstractHighlighter.getBlockHighlightBluntTooltip(3): not translated, it has no body.
            // AbstractHighlighter.addMinimapBlockHighlightTooltips(5): not translated, it has no body.
            pub fn set_result(&mut self, x: i32, z: i32, color: i32) {
                self.result_store[((z << 4) | x) as usize] = color;
            }
            pub fn get_blend(&self, color1: i32, color2: i32) -> i32 {
                if color1 == color2 {
                    return color1;
                }
                let red1: i32 = (color1 >> 8) & 0xFF;
                let green1: i32 = (color1 >> 16) & 0xFF;
                let blue1: i32 = (color1 >> 24) & 0xFF;
                let alpha1: i32 = color1 & 0xFF;
                let red2: i32 = (color2 >> 8) & 0xFF;
                let green2: i32 = (color2 >> 16) & 0xFF;
                let blue2: i32 = (color2 >> 24) & 0xFF;
                let alpha2: i32 = color2 & 0xFF;
                let red: i32 = (red1 + red2) >> 1;
                let green: i32 = (green1 + green2) >> 1;
                let blue: i32 = (blue1 + blue2) >> 1;
                let alpha: i32 = (alpha1 + alpha2) >> 1;
                return (((blue << 24) | (green << 16)) | (red << 8)) | alpha;
            }
            pub fn is_covering_outside_discovered(&self) -> bool {
                return self.covering_outside_discovered;
            }
        }
    }

    /// `ChunkHighlighter` (xaero/map/highlight/ChunkHighlighter.java)
    pub mod chunk_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `AbstractHighlighter`. Its static parts are below.
        // ChunkHighlighter.ChunkHighlighter(1): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getColors(3): not translated, it has no body.
        // ChunkHighlighter.getChunkHighlitColor(3): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getSideBlend(3): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getBlockHighlightBluntTooltip(3): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getBlockHighlightSubtleTooltip(3): not translated, its class isn't translated (it extends `AbstractHighlighter`).
        // ChunkHighlighter.getChunkHighlightSubtleTooltip(3): not translated, it has no body.
        // ChunkHighlighter.getChunkHighlightBluntTooltip(3): not translated, it has no body.
    }

    /// `DimensionHighlighterHandler` (xaero/map/highlight/DimensionHighlighterHandler.java)
    pub mod dimension_highlighter_handler {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // DimensionHighlighterHandler.DimensionHighlighterHandler(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.getRegionHash(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyRegionHighlights(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyTileChunkHighlights(5): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyTileChunkHighlights(6): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.shouldApplyTileChunkHighlightsHelp(6): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.applyChunkHighlightColors(9): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.recalculateHash(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.clearCachedHash(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.clearCachedHashes(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.getBlockHighlightSubtleTooltip(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.getBlockHighlightBluntTooltip(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // DimensionHighlighterHandler.getBlockHighlightTooltip(4): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        pub fn get_key(region_x: i32, region_z: i32) -> i64 {
            return ((region_z as i64) << 32) | ((region_x as i64) & 4294967295i64);
        }
        pub fn get_x_from_key(key: i64) -> i32 {
            return ((key & (-1i64)) as i32);
        }
        pub fn get_z_from_key(key: i64) -> i32 {
            return ((key >> 32) as i32);
        }
    }

    /// `HighlighterRegistry` (xaero/map/highlight/HighlighterRegistry.java)
    pub mod highlighter_registry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct HighlighterRegistry {
            pub highlighters: Vec<abstract_highlighter::AbstractHighlighter>,
        }
        impl HighlighterRegistry {
            pub fn new() -> Self {
                let mut s = Self { highlighters: Vec::new() };
                s
            }
            // HighlighterRegistry.register(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
            pub fn end(&mut self) {
                self.highlighters = self.highlighters.to_vec();
            }
            pub fn get_highlighters(&self) -> Vec<abstract_highlighter::AbstractHighlighter> {
                return self.highlighters.to_vec();
            }
        }
    }

    /// `MapRegionHighlightsPreparer` (xaero/map/highlight/MapRegionHighlightsPreparer.java)
    pub mod map_region_highlights_preparer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapRegionHighlightsPreparer {
        }
        impl MapRegionHighlightsPreparer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // MapRegionHighlightsPreparer.prepare(2): not translated, it uses MapRegion, which isn't translated (it extends `LeveledRegion`).
            // MapRegionHighlightsPreparer.prepare(5): not translated, it uses MapRegion, which isn't translated (it extends `LeveledRegion`).
            // MapRegionHighlightsPreparer.prepare(4): not translated, it uses MapRegion, which isn't translated (it extends `LeveledRegion`).
        }
    }

    /// `RegionHighlightExistenceTracker` (xaero/map/highlight/RegionHighlightExistenceTracker.java)
    pub mod region_highlight_existence_tracker {
        use super::*;
        // The class itself isn't translated: it uses `LongSet` (it.unimi.dsi.fastutil.longs). Its static parts are below.
        // RegionHighlightExistenceTracker.RegionHighlightExistenceTracker(2): not translated, its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)).
        // RegionHighlightExistenceTracker.requestBranchUpdates(2): not translated, its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)).
        // RegionHighlightExistenceTracker.onClearCachedHash(2): not translated, its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)).
        // RegionHighlightExistenceTracker.onClearCachedHashes(0): not translated, its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)).
        // RegionHighlightExistenceTracker.track(2): not translated, its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)).
        // RegionHighlightExistenceTracker.stopTracking(2): not translated, its class isn't translated (it uses `LongSet` (it.unimi.dsi.fastutil.longs)).
    }

    /// `TestHighlighter` (xaero/map/highlight/TestHighlighter.java)
    pub mod test_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `ChunkHighlighter`. Its static parts are below.
        // TestHighlighter.TestHighlighter(0): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.regionHasHighlights(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.getColors(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.calculateRegionHash(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.chunkIsHighlit(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.getChunkHighlightSubtleTooltip(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.getChunkHighlightBluntTooltip(3): not translated, its class isn't translated (it extends `ChunkHighlighter`).
        // TestHighlighter.addMinimapBlockHighlightTooltips(5): not translated, its class isn't translated (it extends `ChunkHighlighter`).
    }

    /// `XaeroIcon` (xaero/map/icon/XaeroIcon.java)
    pub mod xaero_icon {
        use super::*;
        // The class itself isn't translated: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`). Its static parts are below.
        // XaeroIcon.XaeroIcon(3): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // XaeroIcon.getTextureAtlas(0): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // XaeroIcon.getOffsetX(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIcon.getOffsetY(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
    }

    /// `XaeroIconAtlas` (xaero/map/icon/XaeroIconAtlas.java)
    pub mod xaero_icon_atlas {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuTexture`. Its static parts are below.
        // XaeroIconAtlas.XaeroIconAtlas(4): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getTextureId(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getTextureView(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.close(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getWidth(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.getCurrentIndex(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.isFull(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.createIcon(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
    }

    /// `XaeroIconAtlas.Builder` (xaero/map/icon/XaeroIconAtlas.java)
    pub mod xaero_icon_atlas_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GpuTexture`. Its static parts are below.
        // XaeroIconAtlas.Builder.Builder(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setDefault(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setPreparedTexture(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setWidth(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.setIconWidth(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.createGlTexture(1): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.build(0): not translated, its class isn't translated (it uses Minecraft's `GpuTexture`).
        // XaeroIconAtlas.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses Minecraft's `GpuTexture`).
    }

    /// `XaeroIconAtlasManager` (xaero/map/icon/XaeroIconAtlasManager.java)
    pub mod xaero_icon_atlas_manager {
        use super::*;
        // The class itself isn't translated: it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`). Its static parts are below.
        // XaeroIconAtlasManager.XaeroIconAtlasManager(3): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIconAtlasManager.clearAtlases(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
        // XaeroIconAtlasManager.getCurrentAtlas(0): not translated, its class isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)).
    }

    /// `MapFullReloader` (xaero/map/MapFullReloader.java)
    pub mod map_full_reloader {
        use super::*;
        // The class itself isn't translated: it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`). Its static parts are below.
        pub const CONVERTED_WORLD_SAVE_MW: &str = "cm$converted";
        // MapFullReloader.MapFullReloader(5): not translated, its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)).
        // MapFullReloader.onRenderProcess(0): not translated, its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)).
        // MapFullReloader.isPartOfReload(1): not translated, its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)).
        // MapFullReloader.isResave(0): not translated, its class isn't translated (it uses RegionDetection, which isn't translated (it implements `MapRegionInfo`)).
    }

    /// `MapLimiter` (xaero/map/MapLimiter.java)
    pub mod map_limiter {
        use super::*;
        // The class itself isn't translated: it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`). Its static parts are below.
        pub const MIN_LIMIT: i32 = 53;
        // MapLimiter.getRegionsFitOnScreen(0): not translated, its class isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)).
        // MapLimiter.regionsFitOnScreen(1): not translated, its class isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)).
        // MapLimiter.applyLimit(2): not translated, it uses WorldMapPrimaryClientConfigOptions.MAX_LOADED_REGIONS, which isn't translated (it uses `RangeConfigOption` (xaero.lib.common.config.option)).
        // MapLimiter.onSessionFinalized(0): not translated, its class isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)).
    }

    /// `MapProcessor` (xaero/map/MapProcessor.java)
    pub mod map_processor {
        use super::*;
        // The class itself isn't translated: it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)). Its static parts are below.
        pub const ROOT_FOLDER_FORMAT: i32 = 5;
        pub const DEFAULT_LIGHT_LEVELS: i32 = 4;
        pub const SPAWNPOINT_TIMEOUT: i32 = 3000;
        // MapProcessor.MapProcessor(20): not translated, its body couldn't be read: an anonymous class.
        // MapProcessor.onInit(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.run(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.shouldSkipWorldRender(0): not translated, `instanceof`.
        // MapProcessor.actuallySkippingWorldRender(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateConfigValuesForMultipleThreads(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMaxLoadedRegionsConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isReloadViewedConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getReloadViewedVersionConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isFlowersConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isAdjustHeightForCarpetLikeBlocksConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.onRenderProcess(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateCaveStart(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.ignoreWorld(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getDimensionName(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getDimensionIdForFolder(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.waitForLoadingToFinish(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.changeWorld(6): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateVisitedDimension(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateDimension(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMainId(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMainId(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.toggleMultiworldType(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.quickConfirmMultiworld(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.confirmMultiworld(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.setMultiworld(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.canQuickConfirmUnsynced(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCrosshairMessage(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.checkForWorldUpdate(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateWorld(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateWorldSynced(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateFootprints(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.addToRefresh(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.removeToRefresh(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.clearToRefresh(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.handleRefresh(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.regionExists(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.regionDetectionExists(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.removeMapRegion(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getLeveledRegion(4): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.initMinimapRender(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.beforeMinimapRegionRender(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.finalizeMinimapRender(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getLeafMapRegion(4): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMinimapMapRegion(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapChunk(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapTile(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateWorldSpawn(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.onServerLevelId(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.onWorldUnload(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.onClientTickStart(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateRenderStartTime(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.pushWriterPause(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.popWriterPause(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.pushRenderPause(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.popRenderPause(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.pushIsLoading(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.popIsLoading(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.pushUIPause(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.popUIPause(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isUIPaused(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isWritingPaused(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isRenderingPaused(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isUploadingPaused(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isProcessingPaused(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isProcessed(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.addToProcess(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.removeToProcess(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getProcessedCount(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getAffectingLoadingFrequencyCount(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapSaveLoad(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getWorld(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getNewWorld(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCurrentWorldId(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCurrentDimId(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCurrentMWId(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapWriter(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapLimiter(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getFootprints(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getBufferDeallocator(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getTilePool(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getOverlayManager(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getGlobalVersion(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getRenderStartTime(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.resetRenderStartTime(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getRenderStartTimeUpdater(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isWaitingForWorldUpdate(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getWorldDataHandler(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.setMainValues(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getBrightness(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getBrightness(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getBrightness(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getAmbientBrightness(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        pub fn is_world_realms(world: &str) -> bool {
            return world.starts_with("Realms_");
        }
        pub fn is_world_multiplayer(realms: bool, world: &str) -> bool {
            return realms || (world.starts_with("Multiplayer_"));
        }
        // MapProcessor.getMapWorld(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isCurrentMultiworldWritable(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCurrentDimension(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.requestCurrentMapDeletion(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isFinalizing(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.stop(0): not translated, its body couldn't be read: an anonymous class.
        // MapProcessor.forceClean(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isMapWorldUsable(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getAutoIdBase(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getUsedAutoIdBase(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.setUsedAutoIdBase(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.removeUsedAutoIdBase(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.hasServerLevelId(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isEqual(3): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isFinished(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMultiTextureRenderTypeRenderers(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isCurrentMapLocked(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.releaseLocksIfNeeded(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCaveLayer(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCurrentCaveLayer(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getBlockStateShortShapeCache(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getWorldBlockTintProvider(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getHighlighterRegistry(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapRegionHighlightsPreparer(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMessageBox(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMessageBoxRenderer(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getWorldBlockRegistry(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getWorldBlockLookup(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isConsideringNetherFairPlay(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.setConsideringNetherFairPlayMessage(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getBiomeColorCalculator(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getClientSyncedTrackedPlayerManager(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.serverHasMod(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.setServerModNetworkVersion(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getServerModNetworkVersion(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getWorldDimensionTypeRegistry(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.checkFootstepsReset(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.fixRootFolder(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.fixRootFolder(2): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.fairplayMessageWasReceived(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.setFairplayMessageReceived(1): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.updateMapItem(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getMapItem(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isDebugConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isFootstepsConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getCaveModeDepthConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isIgnoreMapFileLocksConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.isForceFastWritingConfig(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        // MapProcessor.getSaveTime(0): not translated, its class isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
    }

    /// `MapRunner` (xaero/map/MapRunner.java)
    pub mod map_runner {
        use super::*;
        // The class itself isn't translated: it implements `Runnable`. Its static parts are below.
        // MapRunner.run(0): not translated, its class isn't translated (it implements `Runnable`).
        // MapRunner.doTasks(1): not translated, its body couldn't be read: a labelled statement.
        // MapRunner.addTask(1): not translated, its class isn't translated (it implements `Runnable`).
        // MapRunner.stop(0): not translated, its class isn't translated (it implements `Runnable`).
    }

    /// `MapWriter` (xaero/map/MapWriter.java)
    pub mod map_writer {
        use super::*;
        // The class itself isn't translated: it uses MapBlock, which isn't translated (it extends `MapPixel`). Its static parts are below.
        pub const NO_Y_VALUE: i32 = 32767;
        pub const MAX_TRANSPARENCY_BLEND_DEPTH: i32 = 5;
        pub static DEFAULT_RESOURCE: LazyLock<Vec<String>> = LazyLock::new(|| vec!["minecraft".to_string(), "".to_string()]);
        // MapWriter.MapWriter(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.onRender(2): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.getWriteDistance(0): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.writeMap(17): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.writeChunk(23): not translated, its body couldn't be read: a labelled statement.
        // MapWriter.updateBottomRightTile(5): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.getSectionBasedHeight(2): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.isGlowing(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.shouldOverlayCached(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.shouldOverlay(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.isInvisible(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.hasVanillaColor(4): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.unpackFramedBlocks(3): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.loadPixel(18): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.loadPixelHelp(19): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.getQuads(5): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.getParticleIcon(5): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.loadBlockColourFromTexture(5): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.getUpdateCounter(0): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.resetPosition(0): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.requestCachedColoursClear(0): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.setMapProcessor(1): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.getMapProcessor(0): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
        // MapWriter.setDirtyInWriteDistance(2): not translated, its class isn't translated (it uses MapBlock, which isn't translated (it extends `MapPixel`)).
    }

    /// `MapWriterFabric` (xaero/map/MapWriterFabric.java)
    pub mod map_writer_fabric {
        use super::*;
        // The class itself isn't translated: it extends `MapWriter`. Its static parts are below.
        // MapWriterFabric.MapWriterFabric(3): not translated, its class isn't translated (it extends `MapWriter`).
    }

    /// `IWorldMapClientWorld` (xaero/map/mcworld/IWorldMapClientWorld.java)
    pub mod i_world_map_client_world {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWorldMapClientWorld.getXaero_worldmapData(0): not translated, it has no body.
        // IWorldMapClientWorld.setXaero_worldmapData(1): not translated, it has no body.
    }

    /// `WorldMapClientWorldData` (xaero/map/mcworld/WorldMapClientWorldData.java)
    pub mod world_map_client_world_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // WorldMapClientWorldData.WorldMapClientWorldData(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // WorldMapClientWorldData.setServerModNetworkVersion(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // WorldMapClientWorldData.getServerModNetworkVersion(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // WorldMapClientWorldData.setSyncedRules(1): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // WorldMapClientWorldData.getSyncedRules(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `WorldMapClientWorldDataHelper` (xaero/map/mcworld/WorldMapClientWorldDataHelper.java)
    pub mod world_map_client_world_data_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapClientWorldDataHelper {
        }
        impl WorldMapClientWorldDataHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WorldMapClientWorldDataHelper.getCurrentWorldData(0): not translated, it uses WorldMapClientWorldData, which isn't translated (it uses Minecraft's `BlockPos`).
        // WorldMapClientWorldDataHelper.getWorldData(1): not translated, it uses WorldMapClientWorldData, which isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `ClientboundRulesPacket` (xaero/map/message/basic/ClientboundRulesPacket.java)
    pub mod clientbound_rules_packet {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientboundRulesPacket {
            pub allow_cave_mode_on_server: bool,
            pub allow_nether_cave_mode_on_server: bool,
        }
        impl ClientboundRulesPacket {
            pub fn new(allow_cave_mode_on_server: bool, allow_nether_cave_mode_on_server: bool) -> Self {
                let mut s = Self { allow_cave_mode_on_server: allow_cave_mode_on_server, allow_nether_cave_mode_on_server: allow_nether_cave_mode_on_server };
                s.allow_cave_mode_on_server = allow_cave_mode_on_server;
                s.allow_nether_cave_mode_on_server = allow_nether_cave_mode_on_server;
                s
            }
            // ClientboundRulesPacket.write(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
        }
        // ClientboundRulesPacket.read(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
    }

    /// `ClientboundRulesPacket.ClientHandler` (xaero/map/message/basic/ClientboundRulesPacket.java)
    pub mod clientbound_rules_packet_client_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // ClientboundRulesPacket.ClientHandler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `HandshakePacket` (xaero/map/message/basic/HandshakePacket.java)
    pub mod handshake_packet {
        use super::*;
        pub const NETWORK_COMPATIBILITY: i32 = 3;
        #[derive(Clone, Debug)]
        pub struct HandshakePacket {
            pub network_version: i32,
        }
        impl HandshakePacket {
            pub fn new_1(network_version: i32) -> Self {
                let mut s = Self { network_version: network_version };
                s.network_version = network_version;
                s
            }
            pub fn new() -> Self {
                let mut s = Self::new_1(3);
                s
            }
            // HandshakePacket.write(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
        }
        // HandshakePacket.read(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
    }

    /// `HandshakePacket.ClientHandler` (xaero/map/message/basic/HandshakePacket.java)
    pub mod handshake_packet_client_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // HandshakePacket.ClientHandler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `HandshakePacket.ServerHandler` (xaero/map/message/basic/HandshakePacket.java)
    pub mod handshake_packet_server_handler {
        use super::*;
        // The class itself isn't translated: it implements `BiConsumer` (java.util.function). Its static parts are below.
        // HandshakePacket.ServerHandler.accept(2): not translated, its class isn't translated (it implements `BiConsumer` (java.util.function)).
    }

    /// `LevelMapPropertiesConsumer` (xaero/map/message/LevelMapPropertiesConsumer.java)
    pub mod level_map_properties_consumer {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // LevelMapPropertiesConsumer.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `ClientboundPlayerTrackerResetPacket` (xaero/map/message/tracker/ClientboundPlayerTrackerResetPacket.java)
    pub mod clientbound_player_tracker_reset_packet {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientboundPlayerTrackerResetPacket {
        }
        impl ClientboundPlayerTrackerResetPacket {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ClientboundPlayerTrackerResetPacket.write(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
        }
        // ClientboundPlayerTrackerResetPacket.read(1): not translated, it uses Minecraft's `FriendlyByteBuf`.
    }

    /// `ClientboundPlayerTrackerResetPacket.Handler` (xaero/map/message/tracker/ClientboundPlayerTrackerResetPacket.java)
    pub mod clientbound_player_tracker_reset_packet_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // ClientboundPlayerTrackerResetPacket.Handler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `ClientboundTrackedPlayerPacket` (xaero/map/message/tracker/ClientboundTrackedPlayerPacket.java)
    pub mod clientbound_tracked_player_packet {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // ClientboundTrackedPlayerPacket.ClientboundTrackedPlayerPacket(7): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientboundTrackedPlayerPacket.write(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientboundTrackedPlayerPacket.read(1): not translated, it uses ClientboundTrackedPlayerPacket, which isn't translated (it uses `UUID` (java.util)).
    }

    /// `ClientboundTrackedPlayerPacket.Handler` (xaero/map/message/tracker/ClientboundTrackedPlayerPacket.java)
    pub mod clientbound_tracked_player_packet_handler {
        use super::*;
        // The class itself isn't translated: it implements `Consumer` (java.util.function). Its static parts are below.
        // ClientboundTrackedPlayerPacket.Handler.accept(1): not translated, its class isn't translated (it implements `Consumer` (java.util.function)).
    }

    /// `WorldMapMessageRegister` (xaero/map/message/WorldMapMessageRegister.java)
    pub mod world_map_message_register {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WorldMapMessageRegister {
        }
        impl WorldMapMessageRegister {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // WorldMapMessageRegister.register(1): not translated, it uses `IPacketHandler` (xaero.lib.common.packet).
        }
    }

    /// `MinimapRenderListener` (xaero/map/minimap/MinimapRenderListener.java)
    pub mod minimap_render_listener {
        use super::*;
        // The class itself isn't translated: it uses MapRegion, which isn't translated (it extends `LeveledRegion`). Its static parts are below.
        // MinimapRenderListener.init(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.beforeMinimapRender(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.finalize(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.getRenderedCaveLayer(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.shouldRequestLoading(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.isPlayerMoving(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.isCacheOnlyMode(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.getGlobalRegionCacheHashCode(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.isReloadEverything(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.getGlobalVersion(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.getGlobalReloadVersion(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.getGlobalCaveStart(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MinimapRenderListener.getGlobalCaveDepth(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
    }

    /// `Area` (xaero/map/misc/Area.java)
    pub mod area {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Area {
            pub left: i32,
            pub top: i32,
            pub right: i32,
            pub bottom: i32,
        }
        impl Area {
            pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
                let mut s = Self { left: left, top: top, right: right, bottom: bottom };
                s.left = left;
                s.top = top;
                s.right = right;
                s.bottom = bottom;
                s
            }
            pub fn get_left(&self) -> i32 {
                return self.left;
            }
            pub fn get_top(&self) -> i32 {
                return self.top;
            }
            pub fn get_right(&self) -> i32 {
                return self.right;
            }
            pub fn get_bottom(&self) -> i32 {
                return self.bottom;
            }
            pub fn hash_code(&self) -> i32 {
                let mut hash: i32 = self.left;
                hash = (hash * 37) + self.top;
                hash = (hash * 37) + self.right;
                hash = (hash * 37) + self.bottom;
                hash = (hash * 37) + self.right;
                hash = (hash * 37) + self.top;
                return (hash * 37) + self.left;
            }
            // Area.equals(1): not translated, it uses `Object`.
        }
    }

    /// `CachedFunction` (xaero/map/misc/CachedFunction.java)
    pub mod cached_function {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CachedFunction.CachedFunction(1): not translated, its class isn't translated (it's generic).
        // CachedFunction.apply(1): not translated, its class isn't translated (it's generic).
        // CachedFunction.clear(0): not translated, its class isn't translated (it's generic).
    }

    /// `CaveStartCalculator` (xaero/map/misc/CaveStartCalculator.java)
    pub mod cave_start_calculator {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `MutableBlockPos`. Its static parts are below.
        // CaveStartCalculator.CaveStartCalculator(1): not translated, its class isn't translated (it uses Minecraft's `MutableBlockPos`).
        // CaveStartCalculator.getCaving(4): not translated, its body couldn't be read: a labelled statement.
    }

    /// `ConsistentBitArray` (xaero/map/misc/ConsistentBitArray.java)
    pub mod consistent_bit_array {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConsistentBitArray {
            pub inside_a_long: i32,
            pub bits_per_entry: i32,
            pub entries: i32,
            pub data: Option<Vec<i64>>,
            pub entry_mask: i32,
        }
        impl ConsistentBitArray {
            pub fn new_2(bits_per_entry: i32, entries: i32) -> Self {
                let mut s = Self::new_3(bits_per_entry, entries, None);
                s
            }
            pub fn new_3(bits_per_entry: i32, entries: i32, data: Option<Vec<i64>>) -> Self {
                let mut s = Self { inside_a_long: 0i32, bits_per_entry: bits_per_entry, entries: entries, data: None, entry_mask: 0i32 };
                if bits_per_entry > 32 {
                    panic!("{}", format!("Entry size too big for int! {}", bits_per_entry));
                }
                s.inside_a_long = 64 / bits_per_entry;
                let longs: i32 = ((entries + s.inside_a_long) - 1) / s.inside_a_long;
                if data.is_some() {
                    if ((data.as_ref().expect("null")).len() as i32) != longs {
                        panic!("{}", format!("Incorrect data length: {} VS {}", ((data.as_ref().expect("null")).len() as i32), longs));
                    }
                    s.data = data.clone();
                } else {
                    s.data = Some(vec![0i64; longs as usize]);
                }
                s.bits_per_entry = bits_per_entry;
                s.entries = entries;
                s.entry_mask = (1 << bits_per_entry) - 1;
                s
            }
            pub fn get(&self, index: i32) -> i32 {
                if index >= self.entries {
                    panic!("ArrayIndexOutOfBoundsException");
                }
                let long_index: i32 = index / self.inside_a_long;
                let inside_index: i32 = index % self.inside_a_long;
                return (((((self.data.as_ref().expect("null"))[long_index as usize]) >> (inside_index * self.bits_per_entry)) & (self.entry_mask as i64)) as i32);
            }
            pub fn set(&mut self, index: i32, value: i32) {
                if index >= self.entries {
                    panic!("ArrayIndexOutOfBoundsException");
                }
                let long_index: i32 = index / self.inside_a_long;
                let inside_index: i32 = index % self.inside_a_long;
                let current_long: i64 = (self.data.as_ref().expect("null"))[long_index as usize];
                let offset: i32 = inside_index * self.bits_per_entry;
                let shifted_mask: i64 = (self.entry_mask as i64) << offset;
                let shifted_value: i64 = ((value & self.entry_mask) as i64) << offset;
                (self.data.as_mut().expect("null"))[long_index as usize] = (current_long & (!shifted_mask)) | shifted_value;
            }
            // ConsistentBitArray.write(1): not translated, it uses `DataOutputStream` (java.io).
            pub fn get_data(&self) -> Option<Vec<i64>> {
                return self.data.clone();
            }
            pub fn set_data(&mut self, data: &[i64]) {
                self.data = Some(data.to_vec());
            }
            pub fn get_bits_per_entry(&self) -> i32 {
                return self.bits_per_entry;
            }
        }
    }

    /// `Internet` (xaero/map/misc/Internet.java)
    pub mod internet {
        use super::*;
        // cipher: not translated, it uses `Cipher` (javax.crypto).
        #[derive(Clone, Debug)]
        pub struct Internet {
        }
        impl Internet {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Internet.checkModVersion(0): not translated, it uses `XaeroLib` (xaero.lib).INSTANCE.
    }

    /// `Misc` (xaero/map/misc/Misc.java)
    pub mod misc {
        use super::*;
        pub static ZERO_LONG_1024: LazyLock<Vec<i64>> = LazyLock::new(|| vec![0i64; 1024]);
        pub static CPU_TIMER_PRE_TIME: JCell<i64> = JCell::new(0i64);
        pub static GL_TIMER_PRE_TIME: JCell<i64> = JCell::new(0i64);
        pub const OUTDATED_FILE_EXT: &str = ".outdated";
        #[derive(Clone, Debug)]
        pub struct Misc {
        }
        impl Misc {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Misc.getStateById(1): not translated, it uses Minecraft's `BlockState`.
        // Misc.getDefaultBlockStateForStateId(1): not translated, it uses Minecraft's `BlockState`.
        pub fn timer_pre() {
            misc::CPU_TIMER_PRE_TIME.set(j_nanos());
        }
        pub fn timer_result() -> i32 {
            return ((j_nanos() - misc::CPU_TIMER_PRE_TIME.get()) as i32);
        }
        // Misc.getMouseX(2): not translated, it uses Minecraft's `Minecraft`.
        // Misc.getMouseY(2): not translated, it uses Minecraft's `Minecraft`.
        // Misc.minecraftOrtho(2): not translated, it uses Minecraft's `Minecraft`.
        pub fn clear_heights_data1024(data: &mut [i64]) {
            data[0..0 + 1024].copy_from_slice(&(*misc::ZERO_LONG_1024)[0..0 + 1024]);
        }
        // Misc.addToListOfSmallest(3): not translated, it's generic.
        // Misc.convertToOutdated(2): not translated, it calls Misc.convertToOutdated, which isn't translated.
        // Misc.convertToOutdated(3): not translated, a catch block that uses its exception.
        // Misc.hasItem(2): not translated, it uses Minecraft's `Player`.
        // Misc.hasItem(3): not translated, it uses Minecraft's `NonNullList`.
        // Misc.getTeamColour(1): not translated, it uses Minecraft's `Entity`.
        // Misc.hasEffect(2): not translated, it uses Minecraft's `Player`.
        // Misc.hasEffect(1): not translated, it uses Minecraft's `Holder`.
    }

    /// `OptimizedMath` (xaero/map/misc/OptimizedMath.java)
    pub mod optimized_math {
        use super::*;
        pub static XP: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(1.0f32, 0.0f32, 0.0f32));
        pub static YP: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(0.0f32, 1.0f32, 0.0f32));
        pub static ZP: LazyLock<JVec3> = LazyLock::new(|| JVec3::new(0.0f32, 0.0f32, 1.0f32));
        #[derive(Clone, Debug)]
        pub struct OptimizedMath {
        }
        impl OptimizedMath {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn my_floor(d: f64) -> i32 {
            let mut as_int: i32 = (d as i32);
            if ((as_int as f64) != d) && (d < 0.0f64) {
                as_int -= 1;
            }
            return as_int;
        }
        // OptimizedMath.rotatePose(3): not translated, it uses Minecraft's `Pose`.
    }

    /// `Waypoint` (xaero/map/mods/gui/Waypoint.java)
    pub mod waypoint {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        pub static MINIMAP_TEXTURES: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaerobetterpvp", "gui/guis.png"));
        pub const WHITE: i32 = -1;
        // Waypoint.Waypoint(4): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getName(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.toString(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getX(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getY(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getZ(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isDisabled(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setDisabled(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getType(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getPurpose(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getYaw(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setYaw(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isRotation(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setRotation(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isEditable(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getOriginal(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getSymbol(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setTemporary(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setGlobal(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getSetName(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getComparisonName(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getColor(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isGlobal(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getRenderX(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getRenderZ(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isTemporary(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getDestAlpha(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setDestAlpha(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getAlphaAnim(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setAlphaAnim(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getAlpha(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.setAlpha(1): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isyIncluded(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getCachedNameLength(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.getThirdPartyOrigin(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isThirdParty(0): not translated, its class isn't translated (it implements `Comparable`).
        // Waypoint.isThirdPartyDeleted(0): not translated, its class isn't translated (it implements `Comparable`).
    }

    /// `WaypointMenuRenderContext` (xaero/map/mods/gui/WaypointMenuRenderContext.java)
    pub mod waypoint_menu_render_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct WaypointMenuRenderContext {
        }
        impl WaypointMenuRenderContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `WaypointMenuRenderer` (xaero/map/mods/gui/WaypointMenuRenderer.java)
    pub mod waypoint_menu_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MapElementMenuRenderer` (xaero.map.element). Its static parts are below.
        // WaypointMenuRenderer.WaypointMenuRenderer(3): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.onMapInit(7): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.onFullMenuButton(2): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.onRenderAllSetsButton(4): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.onShowDisabledButton(4): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.onCloseMenuWhenHoppingButton(4): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.onCurrentMapWaypointsButton(4): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.renderInMenu(11): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.menuStartPos(1): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.menuSearchPadding(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.getFilterPlaceholder(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.getRenderer(1): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.beforeFiltering(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.beforeMenuRender(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // WaypointMenuRenderer.afterMenuRender(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
    }

    /// `WaypointMenuRenderProvider` (xaero/map/mods/gui/WaypointMenuRenderProvider.java)
    pub mod waypoint_menu_render_provider {
        use super::*;
        // The class itself isn't translated: it extends `ElementRenderProvider` (xaero.map.element.render). Its static parts are below.
        // WaypointMenuRenderProvider.WaypointMenuRenderProvider(1): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointMenuRenderProvider.begin(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointMenuRenderProvider.hasNext(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointMenuRenderProvider.getNext(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointMenuRenderProvider.end(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
    }

    /// `WaypointReader` (xaero/map/mods/gui/WaypointReader.java)
    pub mod waypoint_reader {
        use super::*;
        // The class itself isn't translated: it extends `ElementReader` (xaero.map.element.render). Its static parts are below.
        // WaypointReader.waypointIsGood(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.isHidden(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.isInteractable(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getBoxScale(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderX(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderY(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.hasYCoordinate(0): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderZ(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getMenuName(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getFilterName(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRightClickOptions(2): not translated, its body couldn't be read: an anonymous class.
        // WaypointReader.isRightClickValid(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // WaypointReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
    }

    /// `WaypointRenderContext` (xaero/map/mods/gui/WaypointRenderContext.java)
    pub mod waypoint_render_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `VertexConsumer`. Its static parts are below.
    }

    /// `WaypointRenderer` (xaero/map/mods/gui/WaypointRenderer.java)
    pub mod waypoint_renderer {
        use super::*;
        // The class itself isn't translated: it extends `ElementRenderer` (xaero.map.element.render). Its static parts are below.
        // WaypointRenderer.WaypointRenderer(5): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.getSymbolCreator(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.renderElementShadow(9): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.shouldRender(2): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.renderElement(10): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.preRender(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.postRender(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.getOrder(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.shouldBeDimScaled(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
    }

    /// `WaypointRenderer.Builder` (xaero/map/mods/gui/WaypointRenderer.java)
    pub mod waypoint_renderer_builder {
        use super::*;
        // The class itself isn't translated: it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common)). Its static parts are below.
        // WaypointRenderer.Builder.Builder(0): not translated, its class isn't translated (it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common))).
        // WaypointRenderer.Builder.setDefault(0): not translated, `this` used as a value.
        // WaypointRenderer.Builder.setMinimap(1): not translated, `this` used as a value.
        // WaypointRenderer.Builder.setSymbolCreator(1): not translated, `this` used as a value.
        // WaypointRenderer.Builder.build(0): not translated, it uses WaypointRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // WaypointRenderer.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `WaypointRenderProvider` (xaero/map/mods/gui/WaypointRenderProvider.java)
    pub mod waypoint_render_provider {
        use super::*;
        // The class itself isn't translated: it extends `ElementRenderProvider` (xaero.map.element.render). Its static parts are below.
        // WaypointRenderProvider.WaypointRenderProvider(1): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointRenderProvider.begin(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointRenderProvider.hasNext(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointRenderProvider.getNext(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // WaypointRenderProvider.end(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
    }

    /// `WaypointSymbolCreator` (xaero/map/mods/gui/WaypointSymbolCreator.java)
    pub mod waypoint_symbol_creator {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Minecraft`. Its static parts are below.
        pub const PREFERRED_ATLAS_WIDTH: i32 = 1024;
        pub const ICON_WIDTH: i32 = 64;
        pub static MINIMAP_TEXTURES: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaerobetterpvp", "gui/guis.png"));
        pub const WHITE: i32 = -1;
        // WaypointSymbolCreator.getDeathSymbolTexture(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // WaypointSymbolCreator.createDeathSymbolTexture(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // WaypointSymbolCreator.getSymbolTexture(2): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // WaypointSymbolCreator.createCharSymbol(3): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // WaypointSymbolCreator.resetChars(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
    }

    /// `MinimapElementGraphicsWrapper` (xaero/map/mods/minimap/element/MinimapElementGraphicsWrapper.java)
    pub mod minimap_element_graphics_wrapper {
        use super::*;
        // The class itself isn't translated: it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render). Its static parts are below.
        // MinimapElementGraphicsWrapper.MinimapElementGraphicsWrapper(0): not translated, its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)).
        // MinimapElementGraphicsWrapper.setGraphics(1): not translated, its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)).
        // MinimapElementGraphicsWrapper.pose(0): not translated, its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)).
        // MinimapElementGraphicsWrapper.getBufferSource(0): not translated, its class isn't translated (it extends `MinimapElementGraphics` (xaero.hud.minimap.element.render)).
    }

    /// `MinimapElementReaderWrapper` (xaero/map/mods/minimap/element/MinimapElementReaderWrapper.java)
    pub mod minimap_element_reader_wrapper {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementReaderWrapper.MinimapElementReaderWrapper(1): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.isHidden(2): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getBoxScale(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRenderX(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRenderZ(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getInteractionBoxLeft(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getInteractionBoxRight(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getInteractionBoxTop(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getInteractionBoxBottom(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getLeftSideLength(2): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getMenuName(1): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getFilterName(1): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRenderBoxLeft(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRenderBoxRight(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRenderBoxTop(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.getRenderBoxBottom(3): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.isInteractable(2): not translated, its class isn't translated (it's generic).
        // MinimapElementReaderWrapper.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it's generic).
    }

    /// `MinimapElementRendererWrapper` (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java)
    pub mod minimap_element_renderer_wrapper {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementRendererWrapper.MinimapElementRendererWrapper(7): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.getRenderLocation(1): not translated, it uses `MinimapElementRenderLocation` (xaero.hud.minimap.element.render).
        // MinimapElementRendererWrapper.convertRenderInfo(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.preRender(4): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.postRender(4): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.renderElement(10): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.renderElementShadow(9): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.shouldRender(2): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.getOrder(0): not translated, its class isn't translated (it's generic).
    }

    /// `MinimapElementRendererWrapper.Builder` (xaero/map/mods/minimap/element/MinimapElementRendererWrapper.java)
    pub mod minimap_element_renderer_wrapper_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementRendererWrapper.Builder.Builder(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.Builder.setModMain(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.Builder.setShouldRenderSupplier(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.Builder.setOrder(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.Builder.build(0): not translated, its class isn't translated (it's generic).
        // MinimapElementRendererWrapper.Builder.begin(1): not translated, it's generic.
    }

    /// `MinimapElementRenderProviderWrapper` (xaero/map/mods/minimap/element/MinimapElementRenderProviderWrapper.java)
    pub mod minimap_element_render_provider_wrapper {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MinimapElementRenderProviderWrapper.MinimapElementRenderProviderWrapper(1): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderProviderWrapper.begin(2): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderProviderWrapper.hasNext(2): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderProviderWrapper.setupContextAndGetNext(2): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderProviderWrapper.getNext(2): not translated, its class isn't translated (it's generic).
        // MinimapElementRenderProviderWrapper.end(2): not translated, its class isn't translated (it's generic).
    }

    /// `RadarRendererWrapperHelper` (xaero/map/mods/minimap/element/RadarRendererWrapperHelper.java)
    pub mod radar_renderer_wrapper_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RadarRendererWrapperHelper {
        }
        impl RadarRendererWrapperHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // RadarRendererWrapperHelper.createWrapper(2): not translated, it uses `IXaeroMinimap` (xaero.common).
        }
    }

    /// `MinimapSyncedPlayerTrackerSystem` (xaero/map/mods/minimap/tracker/system/MinimapSyncedPlayerTrackerSystem.java)
    pub mod minimap_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system). Its static parts are below.
        // MinimapSyncedPlayerTrackerSystem.MinimapSyncedPlayerTrackerSystem(1): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
        // MinimapSyncedPlayerTrackerSystem.getReader(0): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
        // MinimapSyncedPlayerTrackerSystem.getTrackedPlayerIterator(0): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
    }

    /// `MinimapSyncedTrackedPlayerReader` (xaero/map/mods/minimap/tracker/system/MinimapSyncedTrackedPlayerReader.java)
    pub mod minimap_synced_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system). Its static parts are below.
        // MinimapSyncedTrackedPlayerReader.MinimapSyncedTrackedPlayerReader(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // MinimapSyncedTrackedPlayerReader.getId(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // MinimapSyncedTrackedPlayerReader.getX(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // MinimapSyncedTrackedPlayerReader.getY(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // MinimapSyncedTrackedPlayerReader.getZ(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // MinimapSyncedTrackedPlayerReader.getDimension(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
    }

    /// `ClientClaimChangeListener` (xaero/map/mods/pac/ClientClaimChangeListener.java)
    pub mod client_claim_change_listener {
        use super::*;
        // The class itself isn't translated: it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api). Its static parts are below.
        // ClientClaimChangeListener.onWholeRegionChange(3): not translated, its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)).
        // ClientClaimChangeListener.onChunkChange(4): not translated, its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)).
        // ClientClaimChangeListener.onDimensionChange(1): not translated, its class isn't translated (it implements `IClaimsManagerListenerAPI` (xaero.pac.common.claims.tracker.api)).
    }

    /// `ClaimResultElement` (xaero/map/mods/pac/gui/claim/ClaimResultElement.java)
    pub mod claim_result_element {
        use super::*;
        // The class itself isn't translated: it uses `AreaClaimResult` (xaero.pac.common.claims.result.api). Its static parts are below.
        // ClaimResultElement.ClaimResultElement(8): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getKey(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getFadeOutStartTime(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getCreationTime(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.setFadeOutStartTime(1): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getDimension(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getLeft(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getTop(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getRight(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getBottom(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getTooltip(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.hasNegative(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.hasPositive(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.getFilteredResultTypeIterator(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
    }

    /// `ClaimResultElement.Builder` (xaero/map/mods/pac/gui/claim/ClaimResultElement.java)
    pub mod claim_result_element_builder {
        use super::*;
        // The class itself isn't translated: it uses `AreaClaimResult` (xaero.pac.common.claims.result.api). Its static parts are below.
        // ClaimResultElement.Builder.Builder(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.Builder.setDefault(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.Builder.setKey(1): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.Builder.setResult(1): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.Builder.build(0): not translated, its class isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
        // ClaimResultElement.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)).
    }

    /// `ClaimResultElementManager` (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java)
    pub mod claim_result_element_manager {
        use super::*;
        // The class itself isn't translated: it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)). Its static parts are below.
        // ClaimResultElementManager.ClaimResultElementManager(1): not translated, its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))).
        pub fn get_long_coordinates_for(x: i32, z: i32) -> i64 {
            return ((x as i64) << 32) | ((z as i64) & 4294967295i64);
        }
        pub fn get_x_from_long_coordinates(key: i64) -> i32 {
            return ((key >> 32) as i32);
        }
        pub fn get_z_from_long_coordinates(key: i64) -> i32 {
            return ((key & (-1i64)) as i32);
        }
        // ClaimResultElementManager.add(1): not translated, its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))).
        // ClaimResultElementManager.remove(1): not translated, its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))).
        // ClaimResultElementManager.getIterator(0): not translated, its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))).
        // ClaimResultElementManager.clear(0): not translated, its class isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))).
    }

    /// `ClaimResultElementManager.Builder` (xaero/map/mods/pac/gui/claim/ClaimResultElementManager.java)
    pub mod claim_result_element_manager_builder {
        use super::*;
        // The class itself isn't translated: it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)). Its static parts are below.
        // ClaimResultElementManager.Builder.Builder(0): not translated, its class isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api))).
        // ClaimResultElementManager.Builder.setDefault(0): not translated, `this` used as a value.
        // ClaimResultElementManager.Builder.setPac(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // ClaimResultElementManager.Builder.build(0): not translated, it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))).
        // ClaimResultElementManager.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `ClaimResultElementRenderContext` (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderContext.java)
    pub mod claim_result_element_render_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `VertexConsumer`. Its static parts are below.
    }

    /// `ClaimResultElementRenderer` (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java)
    pub mod claim_result_element_renderer {
        use super::*;
        // The class itself isn't translated: it extends `ElementRenderer` (xaero.map.element.render). Its static parts are below.
        // ClaimResultElementRenderer.ClaimResultElementRenderer(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.preRender(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.postRender(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.renderElementShadow(9): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.renderElement(10): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.shouldRender(2): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.getOrder(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // ClaimResultElementRenderer.shouldBeDimScaled(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
    }

    /// `ClaimResultElementRenderer.Builder` (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderer.java)
    pub mod claim_result_element_renderer_builder {
        use super::*;
        // The class itself isn't translated: it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api))). Its static parts are below.
        // ClaimResultElementRenderer.Builder.Builder(0): not translated, its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))).
        // ClaimResultElementRenderer.Builder.setDefault(0): not translated, its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))).
        // ClaimResultElementRenderer.Builder.setManager(1): not translated, its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))).
        // ClaimResultElementRenderer.Builder.build(0): not translated, its class isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))).
        // ClaimResultElementRenderer.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses ClaimResultElementManager, which isn't translated (it uses ClaimResultElement, which isn't translated (it uses `AreaClaimResult` (xaero.pac.common.claims.result.api)))).
    }

    /// `ClaimResultElementRenderProvider` (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderProvider.java)
    pub mod claim_result_element_render_provider {
        use super::*;
        // The class itself isn't translated: it extends `ElementRenderProvider` (xaero.map.element.render). Its static parts are below.
        // ClaimResultElementRenderProvider.ClaimResultElementRenderProvider(1): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // ClaimResultElementRenderProvider.begin(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // ClaimResultElementRenderProvider.hasNext(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // ClaimResultElementRenderProvider.getNext(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
        // ClaimResultElementRenderProvider.end(2): not translated, its class isn't translated (it extends `ElementRenderProvider` (xaero.map.element.render)).
    }

    /// `ClaimResultElementRenderReader` (xaero/map/mods/pac/gui/claim/element/ClaimResultElementRenderReader.java)
    pub mod claim_result_element_render_reader {
        use super::*;
        // The class itself isn't translated: it extends `ElementReader` (xaero.map.element.render). Its static parts are below.
        // ClaimResultElementRenderReader.isHidden(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRenderX(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRenderZ(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getMenuName(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getFilterName(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.isInteractable(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // ClaimResultElementRenderReader.getTooltip(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
    }

    /// `ClientClaimResultListener` (xaero/map/mods/pac/gui/claim/result/ClientClaimResultListener.java)
    pub mod client_claim_result_listener {
        use super::*;
        // The class itself isn't translated: it implements `IClaimsManagerClaimResultListenerAPI` (xaero.pac.client.claims.tracker.result.api). Its static parts are below.
        // ClientClaimResultListener.onClaimResult(1): not translated, its class isn't translated (it implements `IClaimsManagerClaimResultListenerAPI` (xaero.pac.client.claims.tracker.result.api)).
    }

    /// `ClaimsHighlighter` (xaero/map/mods/pac/highlight/ClaimsHighlighter.java)
    pub mod claims_highlighter {
        use super::*;
        // The class itself isn't translated: it extends `ChunkHighlighter` (xaero.map.highlight). Its static parts are below.
        // ClaimsHighlighter.ClaimsHighlighter(1): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.regionHasHighlights(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.getColors(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.getChunkHighlitColor(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.applyAnchorPixel(4): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.calculateRegionHash(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.accountClaim(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.chunkIsHighlit(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.getChunkHighlightSubtleTooltip(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.getChunkHighlightBluntTooltip(3): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.addMinimapBlockHighlightTooltips(5): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.getClaimsCustomName(2): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
        // ClaimsHighlighter.getClaimsColor(2): not translated, its class isn't translated (it extends `ChunkHighlighter` (xaero.map.highlight)).
    }

    /// `OPACPlayerTrackerSystem` (xaero/map/mods/pac/party/OPACPlayerTrackerSystem.java)
    pub mod opac_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system). Its static parts are below.
        // OPACPlayerTrackerSystem.OPACPlayerTrackerSystem(1): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
        // OPACPlayerTrackerSystem.getReader(0): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
        // OPACPlayerTrackerSystem.getTrackedPlayerIterator(0): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
    }

    /// `OPACTrackedPlayerReader` (xaero/map/mods/pac/party/OPACTrackedPlayerReader.java)
    pub mod opac_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system). Its static parts are below.
        // OPACTrackedPlayerReader.getId(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // OPACTrackedPlayerReader.getX(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // OPACTrackedPlayerReader.getY(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // OPACTrackedPlayerReader.getZ(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // OPACTrackedPlayerReader.getDimension(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
    }

    /// `SupportOpenPartiesAndClaims` (xaero/map/mods/pac/SupportOpenPartiesAndClaims.java)
    pub mod support_open_parties_and_claims {
        use super::*;
        // The class itself isn't translated: it uses `OpenPACClientAPI` (xaero.pac.client.api). Its static parts are below.
        // CLAIMS_MESSAGE_SOURCE: a game value, so it's in game_side.rs.
        // SupportOpenPartiesAndClaims.register(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.claimAt(3): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.onMapRender(8): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.isFromParty(1): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.openPlayerConfigScreen(3): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.onMapKeyPressed(3): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.getControlsTooltip(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.registerHighlighters(1): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.getClaimDistance(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.getClaimResultElementManager(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.getCaimResultElementRenderer(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.onMapChange(1): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.addRightClickOptions(5): not translated, its body couldn't be read: an anonymous class.
        // SupportOpenPartiesAndClaims.getPacClaimsKeyBinding(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.resetDetection(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.detectStuff(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.serverHasMod(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        // SupportOpenPartiesAndClaims.getAllyIterator(0): not translated, its class isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
    }

    /// `SupportAmecs` (xaero/map/mods/SupportAmecs.java)
    pub mod support_amecs {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportAmecs {
        }
        impl SupportAmecs {
            // SupportAmecs.SupportAmecs(1): not translated, it uses `Logger` (org.apache.logging.log4j).
            // SupportAmecs.modifiersArePressed(1): not translated, it uses Minecraft's `KeyMapping`.
        }
    }

    /// `SupportFramedBlocks` (xaero/map/mods/SupportFramedBlocks.java)
    pub mod support_framed_blocks {
        use super::*;
        // The class itself isn't translated: it uses `Class`. Its static parts are below.
        // SupportFramedBlocks.SupportFramedBlocks(0): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.onWorldChange(0): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.findFramedBlocks(2): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.isFrameBlock(3): not translated, its class isn't translated (it uses `Class`).
        // SupportFramedBlocks.unpackFramedBlock(4): not translated, its class isn't translated (it uses `Class`).
    }

    /// `SupportIris` (xaero/map/mods/SupportIris.java)
    pub mod support_iris {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportIris {
        }
        impl SupportIris {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `SupportMods` (xaero/map/mods/SupportMods.java)
    pub mod support_mods {
        use super::*;
        // xaeroMinimap: not translated, it uses SupportXaeroMinimap, which isn't translated (it uses `HudMod` (xaero.common)).
        // xaeroPac: not translated, it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api)).
        pub static OPTIFINE: JCell<bool> = JCell::new(false);
        pub static VIVECRAFT: JCell<bool> = JCell::new(false);
        pub static IRIS: JCell<bool> = JCell::new(false);
        pub static SUPPORT_IRIS: LazyLock<JCell<Option<support_iris::SupportIris>>> = LazyLock::new(|| JCell::new(None));
        // supportFramedBlocks: not translated, it uses SupportFramedBlocks, which isn't translated (it uses `Class`).
        #[derive(Clone, Debug)]
        pub struct SupportMods {
        }
        impl SupportMods {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SupportMods.load(0): not translated, a try/catch (ClassNotFoundException).
        }
        // SupportMods.minimap(0): not translated, a nullable object kept in a changing static.
        // SupportMods.framedBlocks(0): not translated, it uses SupportMods.supportFramedBlocks, which isn't translated (it uses SupportFramedBlocks, which isn't translated (it uses `Class`)).
        // SupportMods.pac(0): not translated, it uses SupportMods.xaeroPac, which isn't translated (it uses SupportOpenPartiesAndClaims, which isn't translated (it uses `OpenPACClientAPI` (xaero.pac.client.api))).
    }

    /// `SupportModsFabric` (xaero/map/mods/SupportModsFabric.java)
    pub mod support_mods_fabric {
        use super::*;
        // The class itself isn't translated: it extends `SupportMods`. Its static parts are below.
        pub static AMECS: LazyLock<JCell<Option<support_amecs::SupportAmecs>>> = LazyLock::new(|| JCell::new(None));
        // SupportModsFabric.load(0): not translated, its class isn't translated (it extends `SupportMods`).
        pub fn amecs() -> bool {
            return support_mods_fabric::AMECS.with(|o| o.is_some());
        }
    }

    /// `SupportXaeroMinimap` (xaero/map/mods/SupportXaeroMinimap.java)
    pub mod support_xaero_minimap {
        use super::*;
        // The class itself isn't translated: it uses `HudMod` (xaero.common). Its static parts are below.
        // SupportXaeroMinimap.SupportXaeroMinimap(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.register(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.convertWaypoints(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.convertSet(3): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.convertWaypoint(4): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.openWaypoint(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.createWaypoint(6): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.createTempWaypoint(5): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.canTeleport(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.teleportToWaypoint(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.teleportToWaypoint(3): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.disableWaypoint(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.deleteWaypoint(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.checkWaypoints(8): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getDimensionDivision(5): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.renderWaypointsMenu(11): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.requestWaypointsRefresh(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getWaypointKeyBinding(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getTempWaypointKeyBinding(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getTempWaypointsMenuKeyBinding(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onMapKeyPressed(3): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onMapKeyReleased(3): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.handleMinimapKeyBinding(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.drawSetChange(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getSettingsScreen(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getControlsTooltip(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onMapMouseRelease(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onMapConstruct(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onMapInit(4): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getWaypointsSorted(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.waypointExists(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.toggleTemporaryWaypoint(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.openWaypointsMenu(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.hidingWaypointCoordinates(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.shareWaypoint(3): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.shareLocation(4): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getMapWaypointWorld(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getWaypointWorld(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getDimDiv(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getArrowColorIndex(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getArrowColor(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getSubWorldNameToRender(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.registerMinimapHighlighters(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getWaypoints(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getDeathpoints(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getWaypointRenderer(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getWaypointMenuRenderer(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onClearHighlightHash(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.createRadarRendererWrapper(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getToggleRadarKey(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onClearHighlightHashes(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getToggleAllyPlayersKey(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getToggleClaimsKey(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getToggleWaypointsKeybinding(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onSessionFinalized(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.openWaypointWorldTeleportCommandScreen(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getMinimapListPlayersAlternative(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getCaveStart(2): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getUsedCaving(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.isFairPlay(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.getMinimapSyncedPlayerTrackerSystem(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onBlockHover(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.onRightClick(0): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
        // SupportXaeroMinimap.wrapElementGraphics(1): not translated, its class isn't translated (it uses `HudMod` (xaero.common)).
    }

    /// `XaeroWorldMapModMenu` (xaero/map/mods/XaeroWorldMapModMenu.java)
    pub mod xaero_world_map_mod_menu {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // XaeroWorldMapModMenu.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `XaeroNbtUtils` (xaero/map/nbt/util/XaeroNbtUtils.java)
    pub mod xaero_nbt_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroNbtUtils {
        }
        impl XaeroNbtUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroNbtUtils.getUUID(2): not translated, it uses `UUID` (java.util).
        // XaeroNbtUtils.putUUID(3): not translated, it uses Minecraft's `CompoundTag`.
        // XaeroNbtUtils.putUUIDAsLongArray(3): not translated, it uses Minecraft's `CompoundTag`.
        // XaeroNbtUtils.createUUIDTag(1): not translated, it uses Minecraft's `IntArrayTag`.
    }

    /// `FastIntPalette` (xaero/map/palette/FastIntPalette.java)
    pub mod fast_int_palette {
        use super::*;
        // The class itself isn't translated: it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints). Its static parts are below.
        // FastIntPalette.FastIntPalette(3): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.get(2): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.add(1): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.add(2): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.append(2): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.getIndex(1): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.count(2): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.getCount(1): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.remove(1): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.replace(2): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.replaceAtIndex(2): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.addNull(0): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.getSize(0): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        // FastIntPalette.getNonNullCount(0): not translated, its class isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
    }

    /// `FastIntPalette.Builder` (xaero/map/palette/FastIntPalette.java)
    pub mod fast_int_palette_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub max_count_per_element: i32,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { max_count_per_element: 0i32 };
                s
            }
            // FastIntPalette.Builder.setDefault(0): not translated, `this` used as a value.
            // FastIntPalette.Builder.setMaxCountPerElement(1): not translated, `this` used as a value.
            // FastIntPalette.Builder.build(0): not translated, it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)).
        }
        // FastIntPalette.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `FastIntPalette.Element` (xaero/map/palette/FastIntPalette.java)
    pub mod fast_int_palette_element {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Element {
            pub value: i32,
            pub count: i16,
        }
        impl Element {
            pub fn new(element_value: i32) -> Self {
                let mut s = Self { value: element_value, count: 0i16 };
                s.value = element_value;
                s
            }
            pub fn set_value(&mut self, element_value: i32) {
                self.value = element_value;
            }
            pub fn get_value(&self) -> i32 {
                return self.value;
            }
            pub fn get_count(&self) -> i32 {
                return (self.count as i32) & 65535;
            }
            pub fn count(&mut self, up: bool, max_count: i32) {
                if ((!up) || ((self.count as i32) != max_count)) && (up || ((self.count as i32) != 0)) {
                    self.count = (((self.count as i32) + (if up { 1 } else { -1 })) as i16);
                } else {
                    panic!("IllegalStateException");
                }
            }
        }
    }

    /// `FastPalette` (xaero/map/palette/FastPalette.java)
    pub mod fast_palette {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FastPalette.FastPalette(3): not translated, its class isn't translated (it's generic).
        // FastPalette.get(1): not translated, its class isn't translated (it's generic).
        // FastPalette.add(1): not translated, its class isn't translated (it's generic).
        // FastPalette.add(2): not translated, its class isn't translated (it's generic).
        // FastPalette.append(2): not translated, its class isn't translated (it's generic).
        // FastPalette.getIndex(1): not translated, its class isn't translated (it's generic).
        // FastPalette.count(2): not translated, its class isn't translated (it's generic).
        // FastPalette.getCount(1): not translated, its class isn't translated (it's generic).
        // FastPalette.remove(1): not translated, its class isn't translated (it's generic).
        // FastPalette.replace(2): not translated, its class isn't translated (it's generic).
        // FastPalette.replace(2): not translated, its class isn't translated (it's generic).
        // FastPalette.addNull(0): not translated, its class isn't translated (it's generic).
        // FastPalette.getSize(0): not translated, its class isn't translated (it's generic).
        // FastPalette.getNonNullCount(0): not translated, its class isn't translated (it's generic).
    }

    /// `FastPalette.Builder` (xaero/map/palette/FastPalette.java)
    pub mod fast_palette_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FastPalette.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // FastPalette.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // FastPalette.Builder.setMaxCountPerElement(1): not translated, its class isn't translated (it's generic).
        // FastPalette.Builder.build(0): not translated, its class isn't translated (it's generic).
        // FastPalette.Builder.begin(0): not translated, it's generic.
    }

    /// `FastPalette.Element` (xaero/map/palette/FastPalette.java)
    pub mod fast_palette_element {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FastPalette.Element.Element(1): not translated, its class isn't translated (it's generic).
        // FastPalette.Element.setObject(1): not translated, its class isn't translated (it's generic).
        // FastPalette.Element.getObject(0): not translated, its class isn't translated (it's generic).
        // FastPalette.Element.getCount(0): not translated, its class isn't translated (it's generic).
        // FastPalette.Element.count(2): not translated, its class isn't translated (it's generic).
    }

    /// `Paletted2DFastBitArrayIntStorage` (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java)
    pub mod paletted2_d_fast_bit_array_int_storage {
        use super::*;
        // The class itself isn't translated: it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)). Its static parts are below.
        // Paletted2DFastBitArrayIntStorage.Paletted2DFastBitArrayIntStorage(6): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.checkRange(2): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getIndex(2): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.get(2): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getRaw(2): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.set(3): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.writeData(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.contains(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getPaletteSize(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getPaletteNonNullCount(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getPaletteElement(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getPaletteElementCount(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getDefaultValueCount(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.getBiomePaletteDebug(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
    }

    /// `Paletted2DFastBitArrayIntStorage.Builder` (xaero/map/palette/Paletted2DFastBitArrayIntStorage.java)
    pub mod paletted2_d_fast_bit_array_int_storage_builder {
        use super::*;
        // The class itself isn't translated: it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)). Its static parts are below.
        // Paletted2DFastBitArrayIntStorage.Builder.Builder(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setDefault(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setWidth(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setHeight(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setMaxPaletteElements(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setDefaultValue(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setPalette(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setData(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.setDefaultValueCount(1): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.build(0): not translated, its class isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))).
        // Paletted2DFastBitArrayIntStorage.Builder.begin(0): not translated, it's generic.
    }

    /// `Paletted2DFastBitArrayStorage` (xaero/map/palette/Paletted2DFastBitArrayStorage.java)
    pub mod paletted2_d_fast_bit_array_storage {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Paletted2DFastBitArrayStorage.Paletted2DFastBitArrayStorage(6): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.checkRange(2): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.getIndex(2): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.get(2): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.set(3): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.contains(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.getPaletteSize(0): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.getPaletteNonNullCount(0): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.getPaletteElement(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.getPaletteElementCount(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.getDefaultValueCount(0): not translated, its class isn't translated (it's generic).
    }

    /// `Paletted2DFastBitArrayStorage.Builder` (xaero/map/palette/Paletted2DFastBitArrayStorage.java)
    pub mod paletted2_d_fast_bit_array_storage_builder {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Paletted2DFastBitArrayStorage.Builder.Builder(0): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setDefault(0): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setWidth(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setHeight(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setMaxPaletteElements(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setDefaultValue(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setPalette(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setData(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.setDefaultValueCount(1): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.build(0): not translated, its class isn't translated (it's generic).
        // Paletted2DFastBitArrayStorage.Builder.begin(0): not translated, it's generic.
    }

    /// `Services` (xaero/map/platform/Services.java)
    pub mod services {
        use super::*;
        // PLATFORM: not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Services {
        }
        impl Services {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Services.load(1): not translated, it's generic.
    }

    /// `FabricPlatformHelper` (xaero/map/platform/services/FabricPlatformHelper.java)
    pub mod fabric_platform_helper {
        use super::*;
        // The class itself isn't translated: it implements `IPlatformHelper`. Its static parts are below.
        // FabricPlatformHelper.getPlatformName(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.isModLoaded(1): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.isDevelopmentEnvironment(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.isDedicatedServer(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.getGameDir(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
        // FabricPlatformHelper.getConfigDir(0): not translated, its class isn't translated (it implements `IPlatformHelper`).
    }

    /// `IPlatformHelper` (xaero/map/platform/services/IPlatformHelper.java)
    pub mod i_platform_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPlatformHelper.getPlatformName(0): not translated, it has no body.
        // IPlatformHelper.isModLoaded(1): not translated, it has no body.
        // IPlatformHelper.checkModForMixin(1): not translated, its class isn't translated (it's an interface).
        // IPlatformHelper.isDevelopmentEnvironment(0): not translated, it has no body.
        // IPlatformHelper.getEnvironmentName(0): not translated, its class isn't translated (it's an interface).
        // IPlatformHelper.isDedicatedServer(0): not translated, it has no body.
        // IPlatformHelper.getGameDir(0): not translated, it has no body.
        // IPlatformHelper.getConfigDir(0): not translated, it has no body.
    }

    /// `PoolTextureDirectBufferUnit` (xaero/map/pool/buffer/PoolTextureDirectBufferUnit.java)
    pub mod pool_texture_direct_buffer_unit {
        use super::*;
        // The class itself isn't translated: it implements `PoolUnit` (xaero.map.pool). Its static parts are below.
        // PoolTextureDirectBufferUnit.PoolTextureDirectBufferUnit(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // PoolTextureDirectBufferUnit.getDirectBuffer(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // PoolTextureDirectBufferUnit.reset(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // PoolTextureDirectBufferUnit.create(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // PoolTextureDirectBufferUnit.createBuffer(0): not translated, it uses `ByteBuffer` (java.nio).
    }

    /// `TextureDirectBufferPool` (xaero/map/pool/buffer/TextureDirectBufferPool.java)
    pub mod texture_direct_buffer_pool {
        use super::*;
        // The class itself isn't translated: it extends `MapPool` (xaero.map.pool). Its static parts are below.
        // TextureDirectBufferPool.TextureDirectBufferPool(0): not translated, its class isn't translated (it extends `MapPool` (xaero.map.pool)).
        // TextureDirectBufferPool.construct(1): not translated, its class isn't translated (it extends `MapPool` (xaero.map.pool)).
        // TextureDirectBufferPool.get(1): not translated, its class isn't translated (it extends `MapPool` (xaero.map.pool)).
    }

    /// `MapPool` (xaero/map/pool/MapPool.java)
    pub mod map_pool {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MapPool.MapPool(1): not translated, its class isn't translated (it's generic).
        // MapPool.get(1): not translated, its class isn't translated (it's generic).
        // MapPool.addToPool(1): not translated, its class isn't translated (it's generic).
        // MapPool.takeFromPool(0): not translated, its class isn't translated (it's generic).
        // MapPool.size(0): not translated, its class isn't translated (it's generic).
        // MapPool.construct(1): not translated, it has no body.
    }

    /// `MapTilePool` (xaero/map/pool/MapTilePool.java)
    pub mod map_tile_pool {
        use super::*;
        // The class itself isn't translated: it extends `MapPool`. Its static parts are below.
        // MapTilePool.MapTilePool(0): not translated, its class isn't translated (it extends `MapPool`).
        // MapTilePool.construct(1): not translated, its class isn't translated (it extends `MapPool`).
        // MapTilePool.get(3): not translated, its class isn't translated (it extends `MapPool`).
    }

    /// `PoolUnit` (xaero/map/pool/PoolUnit.java)
    pub mod pool_unit {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PoolUnit.create(1): not translated, it has no body.
    }

    /// `TextureUploadPool` (xaero/map/pool/TextureUploadPool.java)
    pub mod texture_upload_pool {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TextureUploadPool.TextureUploadPool(1): not translated, its class isn't translated (it's generic).
    }

    /// `TextureUploadPool.BranchDownload` (xaero/map/pool/TextureUploadPool.java)
    pub mod texture_upload_pool_branch_download {
        use super::*;
        // The class itself isn't translated: it extends `TextureUploadPool`. Its static parts are below.
        // TextureUploadPool.BranchDownload.BranchDownload(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.BranchDownload.construct(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.BranchDownload.get(3): not translated, its class isn't translated (it extends `TextureUploadPool`).
    }

    /// `TextureUploadPool.BranchUpdate` (xaero/map/pool/TextureUploadPool.java)
    pub mod texture_upload_pool_branch_update {
        use super::*;
        // The class itself isn't translated: it extends `TextureUploadPool`. Its static parts are below.
        // TextureUploadPool.BranchUpdate.BranchUpdate(2): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.BranchUpdate.construct(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.BranchUpdate.get(15): not translated, its class isn't translated (it extends `TextureUploadPool`).
    }

    /// `TextureUploadPool.Normal` (xaero/map/pool/TextureUploadPool.java)
    pub mod texture_upload_pool_normal {
        use super::*;
        // The class itself isn't translated: it extends `TextureUploadPool`. Its static parts are below.
        // TextureUploadPool.Normal.Normal(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.Normal.construct(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.Normal.get(8): not translated, its class isn't translated (it extends `TextureUploadPool`).
    }

    /// `TextureUploadPool.SubsequentNormal` (xaero/map/pool/TextureUploadPool.java)
    pub mod texture_upload_pool_subsequent_normal {
        use super::*;
        // The class itself isn't translated: it extends `TextureUploadPool`. Its static parts are below.
        // TextureUploadPool.SubsequentNormal.SubsequentNormal(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.SubsequentNormal.construct(1): not translated, its class isn't translated (it extends `TextureUploadPool`).
        // TextureUploadPool.SubsequentNormal.get(9): not translated, its class isn't translated (it extends `TextureUploadPool`).
    }

    /// `SimpleProfiler` (xaero/map/profile/SimpleProfiler.java)
    pub mod simple_profiler {
        use super::*;
        // The class itself isn't translated: it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects). Its static parts are below.
        // SimpleProfiler.reset(0): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // SimpleProfiler.addTime(2): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // SimpleProfiler.section(1): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // SimpleProfiler.end(0): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
        // SimpleProfiler.debug(0): not translated, its class isn't translated (it uses `Object2LongMap` (it.unimi.dsi.fastutil.objects)).
    }

    /// `PlayerTeleporter` (xaero/map/radar/tracker/PlayerTeleporter.java)
    pub mod player_teleporter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlayerTeleporter {
        }
        impl PlayerTeleporter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlayerTeleporter.teleport(6): not translated, it uses Minecraft's `Screen`.
            // PlayerTeleporter.teleportToPlayer(3): not translated, it uses Minecraft's `Screen`.
        }
    }

    /// `PlayerTrackerIconRenderer` (xaero/map/radar/tracker/PlayerTrackerIconRenderer.java)
    pub mod player_tracker_icon_renderer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlayerTrackerIconRenderer {
        }
        impl PlayerTrackerIconRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlayerTrackerIconRenderer.isPlayerUpsideDown(1): not translated, it uses Minecraft's `Player`.
            // PlayerTrackerIconRenderer.renderIcon(3): not translated, it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics)).
            // PlayerTrackerIconRenderer.renderIconGUI(3): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        }
        pub fn is_upside_down_name(player_name: &str) -> bool {
            return j_eq("Dinnerbone", &player_name) || j_eq("Grumm", &player_name);
        }
    }

    /// `PlayerTrackerMapElement` (xaero/map/radar/tracker/PlayerTrackerMapElement.java)
    pub mod player_tracker_map_element {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PlayerTrackerMapElement.PlayerTrackerMapElement(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getPlayerId(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getX(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getY(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getZ(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getDimension(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getPlayer(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.setRenderedOnRadar(1): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.wasRenderedOnRadar(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getFadeAnim(0): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.setFadeAnim(1): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElement.getSystem(0): not translated, its class isn't translated (it's generic).
    }

    /// `PlayerTrackerMapElementCollector` (xaero/map/radar/tracker/PlayerTrackerMapElementCollector.java)
    pub mod player_tracker_map_element_collector {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // PlayerTrackerMapElementCollector.PlayerTrackerMapElementCollector(2): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMapElementCollector.update(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMapElementCollector.updateForSystem(3): not translated, it's generic.
        // PlayerTrackerMapElementCollector.playerExists(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMapElementCollector.getElements(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMapElementCollector.resetRenderedOnRadarFlags(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // PlayerTrackerMapElementCollector.confirmPlayerRadarRender(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `PlayerTrackerMapElementReader` (xaero/map/radar/tracker/PlayerTrackerMapElementReader.java)
    pub mod player_tracker_map_element_reader {
        use super::*;
        // The class itself isn't translated: it extends `ElementReader` (xaero.map.element.render). Its static parts are below.
        // PlayerTrackerMapElementReader.isHidden(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderX(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderY(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.hasYCoordinate(0): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderZ(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getInteractionBoxLeft(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getInteractionBoxRight(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getInteractionBoxTop(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getInteractionBoxBottom(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderBoxLeft(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderBoxRight(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderBoxTop(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRenderBoxBottom(3): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getLeftSideLength(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getMenuName(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getFilterName(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getMenuTextFillLeftPadding(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRightClickTitleBackgroundColor(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.shouldScaleBoxWithOptionalScale(0): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.isRightClickValid(1): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
        // PlayerTrackerMapElementReader.getRightClickOptions(2): not translated, its body couldn't be read: an anonymous class.
        // PlayerTrackerMapElementReader.isInteractable(2): not translated, its class isn't translated (it extends `ElementReader` (xaero.map.element.render)).
    }

    /// `PlayerTrackerMapElementRenderContext` (xaero/map/radar/tracker/PlayerTrackerMapElementRenderContext.java)
    pub mod player_tracker_map_element_render_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `VertexConsumer`. Its static parts are below.
    }

    /// `PlayerTrackerMapElementRenderer` (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java)
    pub mod player_tracker_map_element_renderer {
        use super::*;
        // The class itself isn't translated: it extends `ElementRenderer` (xaero.map.element.render). Its static parts are below.
        // PlayerTrackerMapElementRenderer.PlayerTrackerMapElementRenderer(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.getTrackedPlayerIconManager(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.preRender(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.postRender(4): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.renderElementShadow(9): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.renderElement(10): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.shouldRender(2): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.getOrder(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.getCollector(0): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // PlayerTrackerMapElementRenderer.update(1): not translated, its class isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
    }

    /// `PlayerTrackerMapElementRenderer.Builder` (xaero/map/radar/tracker/PlayerTrackerMapElementRenderer.java)
    pub mod player_tracker_map_element_renderer_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // PlayerTrackerMapElementRenderer.Builder.setDefault(0): not translated, `this` used as a value.
            // PlayerTrackerMapElementRenderer.Builder.build(0): not translated, it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        }
        // PlayerTrackerMapElementRenderer.Builder.begin(0): not translated, it calls Builder.setDefault, which isn't translated.
    }

    /// `PlayerTrackerMapElementRenderProvider` (xaero/map/radar/tracker/PlayerTrackerMapElementRenderProvider.java)
    pub mod player_tracker_map_element_render_provider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PlayerTrackerMapElementRenderProvider.PlayerTrackerMapElementRenderProvider(1): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElementRenderProvider.begin(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElementRenderProvider.hasNext(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElementRenderProvider.getNext(2): not translated, its class isn't translated (it's generic).
        // PlayerTrackerMapElementRenderProvider.end(2): not translated, its class isn't translated (it's generic).
    }

    /// `PlayerTrackerMenuRenderContext` (xaero/map/radar/tracker/PlayerTrackerMenuRenderContext.java)
    pub mod player_tracker_menu_render_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlayerTrackerMenuRenderContext {
        }
        impl PlayerTrackerMenuRenderContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `PlayerTrackerMenuRenderer` (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java)
    pub mod player_tracker_menu_renderer {
        use super::*;
        // The class itself isn't translated: it extends `MapElementMenuRenderer` (xaero.map.element). Its static parts are below.
        // PlayerTrackerMenuRenderer.PlayerTrackerMenuRenderer(4): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.onMapInit(4): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.onShowPlayersButton(3): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.beforeMenuRender(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.afterMenuRender(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.renderInMenu(11): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.beforeFiltering(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.menuStartPos(1): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.menuSearchPadding(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.getFilterPlaceholder(0): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.getRenderer(1): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // PlayerTrackerMenuRenderer.canJumpTo(1): not translated, its class isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
    }

    /// `PlayerTrackerMenuRenderer.Builder` (xaero/map/radar/tracker/PlayerTrackerMenuRenderer.java)
    pub mod player_tracker_menu_renderer_builder {
        use super::*;
        // The class itself isn't translated: it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)). Its static parts are below.
        // PlayerTrackerMenuRenderer.Builder.Builder(0): not translated, its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))).
        // PlayerTrackerMenuRenderer.Builder.setDefault(0): not translated, its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))).
        // PlayerTrackerMenuRenderer.Builder.setRenderer(1): not translated, its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))).
        // PlayerTrackerMenuRenderer.Builder.build(0): not translated, its class isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))).
        // PlayerTrackerMenuRenderer.Builder.begin(0): not translated, it uses Builder, which isn't translated (it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render))).
    }

    /// `ClientSyncedTrackedPlayerManager` (xaero/map/radar/tracker/synced/ClientSyncedTrackedPlayerManager.java)
    pub mod client_synced_tracked_player_manager {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // ClientSyncedTrackedPlayerManager.remove(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.update(5): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.getPlayers(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ClientSyncedTrackedPlayerManager.reset(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `SyncedPlayerTrackerSystem` (xaero/map/radar/tracker/system/impl/SyncedPlayerTrackerSystem.java)
    pub mod synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system). Its static parts are below.
        // SyncedPlayerTrackerSystem.getReader(0): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
        // SyncedPlayerTrackerSystem.getTrackedPlayerIterator(0): not translated, its class isn't translated (it implements `IPlayerTrackerSystem` (xaero.map.radar.tracker.system)).
    }

    /// `SyncedTrackedPlayerReader` (xaero/map/radar/tracker/system/impl/SyncedTrackedPlayerReader.java)
    pub mod synced_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system). Its static parts are below.
        // SyncedTrackedPlayerReader.getId(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // SyncedTrackedPlayerReader.getX(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // SyncedTrackedPlayerReader.getY(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // SyncedTrackedPlayerReader.getZ(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
        // SyncedTrackedPlayerReader.getDimension(1): not translated, its class isn't translated (it implements `ITrackedPlayerReader` (xaero.map.radar.tracker.system)).
    }

    /// `IPlayerTrackerSystem` (xaero/map/radar/tracker/system/IPlayerTrackerSystem.java)
    pub mod i_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPlayerTrackerSystem.getReader(0): not translated, it has no body.
        // IPlayerTrackerSystem.getTrackedPlayerIterator(0): not translated, it has no body.
    }

    /// `ITrackedPlayerReader` (xaero/map/radar/tracker/system/ITrackedPlayerReader.java)
    pub mod i_tracked_player_reader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ITrackedPlayerReader.getId(1): not translated, it has no body.
        // ITrackedPlayerReader.getX(1): not translated, it has no body.
        // ITrackedPlayerReader.getY(1): not translated, it has no body.
        // ITrackedPlayerReader.getZ(1): not translated, it has no body.
        // ITrackedPlayerReader.getDimension(1): not translated, it has no body.
    }

    /// `PlayerTrackerSystemManager` (xaero/map/radar/tracker/system/PlayerTrackerSystemManager.java)
    pub mod player_tracker_system_manager {
        use super::*;
        // The class itself isn't translated: it uses IPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // PlayerTrackerSystemManager.register(2): not translated, its class isn't translated (it uses IPlayerTrackerSystem, which isn't translated (it's an interface)).
        // PlayerTrackerSystemManager.getSystems(0): not translated, its class isn't translated (it uses IPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `TrackedPlayerIconManager` (xaero/map/radar/tracker/TrackedPlayerIconManager.java)
    pub mod tracked_player_icon_manager {
        use super::*;
        // The class itself isn't translated: it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`)). Its static parts are below.
        pub const ICON_WIDTH: i32 = 32;
        pub const PREFERRED_ATLAS_WIDTH: i32 = 1024;
        // TrackedPlayerIconManager.TrackedPlayerIconManager(4): not translated, its class isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))).
        // TrackedPlayerIconManager.getPlayerSkin(2): not translated, its class isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))).
        // TrackedPlayerIconManager.getIcon(4): not translated, its class isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))).
    }

    /// `TrackedPlayerIconManager.Builder` (xaero/map/radar/tracker/TrackedPlayerIconManager.java)
    pub mod tracked_player_icon_manager_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // TrackedPlayerIconManager.Builder.build(0): not translated, it uses TrackedPlayerIconManager, which isn't translated (it uses XaeroIconAtlasManager, which isn't translated (it uses XaeroIconAtlas, which isn't translated (it uses Minecraft's `GpuTexture`))).
        }
        pub fn begin() -> tracked_player_icon_manager_builder::Builder {
            return tracked_player_icon_manager_builder::Builder::new();
        }
    }

    /// `TrackedPlayerIconPrerenderer` (xaero/map/radar/tracker/TrackedPlayerIconPrerenderer.java)
    pub mod tracked_player_icon_prerenderer {
        use super::*;
        // The class itself isn't translated: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`). Its static parts are below.
        // TrackedPlayerIconPrerenderer.prerender(6): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
    }

    /// `BranchLeveledRegion` (xaero/map/region/BranchLeveledRegion.java)
    pub mod branch_leveled_region {
        use super::*;
        // The class itself isn't translated: it extends `LeveledRegion`. Its static parts are below.
        pub const CHILD_LENGTH_IN_TEXTURES: i32 = 4;
        pub const MAX_COORD_WITHIN_CHILD: i32 = 3;
        // BranchLeveledRegion.BranchLeveledRegion(9): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.reset(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.checkAndTrackRegionExistence(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.checkForUpdates(9): not translated, its body couldn't be read: a labelled statement.
        // BranchLeveledRegion.putTexture(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.getTexture(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.hasTextures(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.isEmpty(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.preCacheLoad(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.putLeaf(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.get(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.remove(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.loadingAnimation(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.addDebugLines(4): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.shouldEndProcessingAfterUpload(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.setLoaded(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.isLoaded(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.cleanAndCacheRequestsBlocked(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.onProcessingEnd(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.shouldBeProcessed(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.preCache(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.postCache(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.skipCaching(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.findCacheFile(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.onCurrentDimFinish(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.onLimiterRemoval(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.afterLimiterRemoval(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.createTexture(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.setShouldCheckForUpdatesRecursive(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.setShouldCheckForUpdatesSingle(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.startDownloadingTexturesForCache(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.postTextureUpdate(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.countTextureUpdate(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.eligibleForSaving(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // BranchLeveledRegion.onCacheLoadFailed(1): not translated, its class isn't translated (it extends `LeveledRegion`).
    }

    /// `ExportMapRegion` (xaero/map/region/ExportMapRegion.java)
    pub mod export_map_region {
        use super::*;
        // The class itself isn't translated: it extends `MapRegion`. Its static parts are below.
        // ExportMapRegion.ExportMapRegion(5): not translated, its class isn't translated (it extends `MapRegion`).
        // ExportMapRegion.createTileChunk(2): not translated, its class isn't translated (it extends `MapRegion`).
        // ExportMapRegion.getTexture(2): not translated, its class isn't translated (it extends `MapRegion`).
        // ExportMapRegion.getChunk(2): not translated, its class isn't translated (it extends `MapRegion`).
    }

    /// `ExportMapTileChunk` (xaero/map/region/ExportMapTileChunk.java)
    pub mod export_map_tile_chunk {
        use super::*;
        // The class itself isn't translated: it extends `MapTileChunk`. Its static parts are below.
        // ExportMapTileChunk.ExportMapTileChunk(3): not translated, its class isn't translated (it extends `MapTileChunk`).
        // ExportMapTileChunk.createLeafTexture(0): not translated, its class isn't translated (it extends `MapTileChunk`).
        // ExportMapTileChunk.getLeafTexture(0): not translated, its class isn't translated (it extends `MapTileChunk`).
    }

    /// `LayeredRegionManager` (xaero/map/region/LayeredRegionManager.java)
    pub mod layered_region_manager {
        use super::*;
        // The class itself isn't translated: it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints). Its static parts are below.
        // LayeredRegionManager.LayeredRegionManager(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.putLeaf(3): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.getLeaf(3): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.get(4): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.remove(4): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.getLayer(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.clear(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.loadedCount(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.removeListRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.addListRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.bumpLoadedRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.bumpLoadedRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.getLoadedListUnsynced(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.getLoadedRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.addLoadedRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.removeLoadedRegion(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.size(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.getUnsyncedSet(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.onClearCachedHighlightHash(2): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.onClearCachedHighlightHashes(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.applyToEachLoadedLayer(1): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
        // LayeredRegionManager.preDetection(0): not translated, its class isn't translated (it uses `Int2ObjectMap` (it.unimi.dsi.fastutil.ints)).
    }

    /// `LeveledRegion` (xaero/map/region/LeveledRegion.java)
    pub mod leveled_region {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const SIDE_LENGTH: i32 = 8;
        pub static COMPARISON_X: JCell<i32> = JCell::new(0);
        pub static COMPARISON_Z: JCell<i32> = JCell::new(0);
        pub static COMPARISON_LEVEL: JCell<i32> = JCell::new(0i32);
        pub static COMPARISON_LEAF_X: JCell<i32> = JCell::new(0);
        pub static COMPARISON_LEAF_Z: JCell<i32> = JCell::new(0);
        // LeveledRegion.LeveledRegion(9): not translated, its class isn't translated (it's generic).
        // LeveledRegion.onDimensionClear(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.deleteTexturesAndBuffers(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.hasTextures(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.deleteBuffers(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.deleteGLBuffers(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.isAllCachePrepared(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.setAllCachePrepared(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getRegionX(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getRegionZ(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.shouldCache(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getLevel(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.setShouldCache(2): not translated, its class isn't translated (it's generic).
        // LeveledRegion.recacheHasBeenRequested(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.setRecacheHasBeenRequested(2): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getCacheFile(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.setCacheFile(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getDim(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.toString(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.reloadHasBeenRequested(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.setReloadHasBeenRequested(2): not translated, its class isn't translated (it's generic).
        pub fn set_comparison(x: i32, z: i32, level: i32, leaf_x: i32, leaf_z: i32) {
            leveled_region::COMPARISON_X.set(x);
            leveled_region::COMPARISON_Z.set(z);
            leveled_region::COMPARISON_LEVEL.set(level);
            leveled_region::COMPARISON_LEAF_X.set(leaf_x);
            leveled_region::COMPARISON_LEAF_Z.set(leaf_z);
        }
        // LeveledRegion.distanceFromPlayer(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.leafDistanceFromPlayer(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.calculateSortingDistance(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.chunkDistanceFromPlayer(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.calculateSortingChunkDistance(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.compareTo(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.onProcessingEnd(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.addDebugLines(4): not translated, its class isn't translated (it's generic).
        // LeveledRegion.writeCacheMetaData(3): not translated, its class isn't translated (it's generic).
        // LeveledRegion.saveCacheTextures(3): not translated, its class isn't translated (it's generic).
        // LeveledRegion.readCacheMetaData(7): not translated, its class isn't translated (it's generic).
        // LeveledRegion.loadCacheTextures(9): not translated, its class isn't translated (it's generic).
        // LeveledRegion.onCacheLoadFailed(1): not translated, it has no body.
        // LeveledRegion.saveBiomePalette(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.loadBiomePalette(6): not translated, its class isn't translated (it's generic).
        // LeveledRegion.shouldLeafAffectCache(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.readCacheInput(9): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getAndResetCachedTextureVersion(2): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getParent(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.shouldAffectLoadingRequestFrequency(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.preCacheLoad(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.processWhenLoadedChunksExist(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.isMetaLoaded(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.confirmMetaLoaded(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getRootRegion(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.checkForUpdates(9): not translated, its class isn't translated (it's generic).
        // LeveledRegion.ensureBiomePalette(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getBiomeKey(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getBiomePaletteIndex(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.onBiomeAddedToTexture(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.onBiomeRemovedFromTexture(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.uncountTextureBiomes(1): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getBiomePaletteSize(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getBiomePalette(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.resetBiomePalette(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.isRefreshing(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.shouldAllowAnotherRegionToLoad(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.shouldEndProcessingAfterUpload(0): not translated, it has no body.
        // LeveledRegion.createTexture(2): not translated, it has no body.
        // LeveledRegion.putTexture(3): not translated, it has no body.
        // LeveledRegion.getTexture(2): not translated, it has no body.
        // LeveledRegion.putLeaf(3): not translated, it has no body.
        // LeveledRegion.remove(3): not translated, it has no body.
        // LeveledRegion.get(3): not translated, it has no body.
        // LeveledRegion.loadingAnimation(0): not translated, it has no body.
        // LeveledRegion.cleanAndCacheRequestsBlocked(0): not translated, it has no body.
        // LeveledRegion.shouldBeProcessed(0): not translated, it has no body.
        // LeveledRegion.isLoaded(0): not translated, it has no body.
        // LeveledRegion.preCache(0): not translated, it has no body.
        // LeveledRegion.postCache(3): not translated, it has no body.
        // LeveledRegion.skipCaching(1): not translated, it has no body.
        // LeveledRegion.findCacheFile(1): not translated, it has no body.
        // LeveledRegion.onCurrentDimFinish(2): not translated, it has no body.
        // LeveledRegion.onLimiterRemoval(1): not translated, it has no body.
        // LeveledRegion.afterLimiterRemoval(1): not translated, it has no body.
        // LeveledRegion.getExtraInfo(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.updateLeafTextureVersion(3): not translated, its class isn't translated (it's generic).
        // LeveledRegion.hasRemovableSourceData(0): not translated, its class isn't translated (it's generic).
        // LeveledRegion.getCaveLayer(0): not translated, its class isn't translated (it's generic).
    }

    /// `LeveledRegionManager` (xaero/map/region/LeveledRegionManager.java)
    pub mod leveled_region_manager {
        use super::*;
        // The class itself isn't translated: it uses LeveledRegion, which isn't translated (it's generic). Its static parts are below.
        pub const MAX_LEVEL: i32 = 3;
        // LeveledRegionManager.putLeaf(3): not translated, its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)).
        // LeveledRegionManager.getLeaf(2): not translated, its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)).
        // LeveledRegionManager.get(3): not translated, its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)).
        // LeveledRegionManager.remove(3): not translated, its class isn't translated (it uses LeveledRegion, which isn't translated (it's generic)).
    }

    /// `MapBlock` (xaero/map/region/MapBlock.java)
    pub mod map_block {
        use super::*;
        // The class itself isn't translated: it extends `MapPixel`. Its static parts are below.
        // MapBlock.isGrass(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getParametres(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getPixelColour(27): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.toRenderString(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.equalsSlopesExcluded(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.equals(2): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.fixHeightType(11): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.prepareForWriting(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.write(7): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.addOverlay(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getHeight(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.setHeight(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getTopHeight(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.setTopHeight(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getEffectiveHeight(2): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getEffectiveHeight(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getEffectiveTopHeight(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getBiome(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.setBiome(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getOverlays(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getVerticalSlope(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.setVerticalSlope(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getDiagonalSlope(0): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.setDiagonalSlope(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.setSlopeUnknown(1): not translated, its class isn't translated (it extends `MapPixel`).
        // MapBlock.getNumberOfOverlays(0): not translated, its class isn't translated (it extends `MapPixel`).
    }

    /// `MapLayer` (xaero/map/region/MapLayer.java)
    pub mod map_layer {
        use super::*;
        // The class itself isn't translated: it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic)). Its static parts are below.
        // MapLayer.MapLayer(2): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.regionDetectionExists(2): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.addRegionDetection(1): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getCompleteRegionDetection(2): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.completeRegionDetectionContains(1): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.tryAddingToCompleteRegionDetection(1): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getRegionDetection(2): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.removeRegionDetection(2): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getRegionHighlightExistenceTracker(0): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getMapRegions(0): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getDetectedRegions(0): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getLinkedCompleteWorldSaveDetectedRegions(0): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.preDetection(0): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.getCaveStart(0): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
        // MapLayer.setCaveStart(1): not translated, its class isn't translated (it uses LeveledRegionManager, which isn't translated (it uses LeveledRegion, which isn't translated (it's generic))).
    }

    /// `MapPixel` (xaero/map/region/MapPixel.java)
    pub mod map_pixel {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        pub const VOID_COLOR: i32 = -16121833;
        pub const DEFAULT_AMBIENT_LIGHT: f32 = 0.7f32;
        pub const DEFAULT_AMBIENT_LIGHT_COLORED: f32 = 0.2f32;
        pub const DEFAULT_AMBIENT_LIGHT_WHITE: f32 = 0.5f32;
        pub const DEFAULT_MAX_DIRECT_LIGHT: f32 = 0.6666667f32;
        pub const GLOWING_MAX_DIRECT_LIGHT: f32 = 0.22222224f32;
        // MapPixel.getVanillaTransparency(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.getPixelColours(29): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.getBlockBrightness(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.getPixelLight(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.getState(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.setState(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.setLight(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // MapPixel.setGlowing(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `MapRegion` (xaero/map/region/MapRegion.java)
    pub mod map_region {
        use super::*;
        // The class itself isn't translated: it extends `LeveledRegion`. Its static parts are below.
        pub const SIDE_LENGTH: i32 = 8;
        // MapRegion.MapRegion(10): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setParent(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.destroyBufferUpdateObjects(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.restoreBufferUpdateObjects(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.requestRefresh(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.requestRefresh(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.cancelRefresh(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.distanceFromPlayer(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.leafDistanceFromPlayer(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.clean(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.writeCacheMetaData(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.readCacheMetaData(7): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.clearRegion(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isResting(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getWorldId(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getDimId(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getMwId(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getVersion(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setVersion(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isBeingWritten(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setBeingWritten(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getLoadState(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setLoadState(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getChunk(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setChunk(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getInitialVersion(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setInitialVersion(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getPixelResultBuffer(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getMutableGlobalPos(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getRegionFile(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setRegionFile(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getSaveExists(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setSaveExists(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getLastSaveTime(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setLastSaveTime(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isRefreshing(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setRefreshing(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isWritingPaused(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.pushWriterPause(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.popWriterPause(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.hasVersion(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isNormalMapData(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getLastVisited(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getTimeSinceVisit(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.registerVisit(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.countChunks(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getBiomeRegistry(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.putLeaf(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.putTexture(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getTexture(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.get(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.remove(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.loadingAnimation(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.cleanAndCacheRequestsBlocked(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.shouldBeProcessed(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isLoaded(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.shouldEndProcessingAfterUpload(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.onProcessingEnd(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.preCache(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.postCache(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.skipCaching(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.findCacheFile(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.onCurrentDimFinish(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.onLimiterRemoval(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.afterLimiterRemoval(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.addDebugLines(4): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getExtraInfo(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.createTexture(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.createTileChunk(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.checkForUpdates(9): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getReloadVersion(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setReloadVersion(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setCacheHashCode(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getCacheHashCode(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setCaveStart(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getCaveStart(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.processWhenLoadedChunksExist(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.updateLeafTextureVersion(3): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.onDimensionClear(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.restoreMetaData(7): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.loadCacheTextures(9): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.onCacheLoadFailed(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.convertCacheToOutdated(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.shouldAffectLoadingRequestFrequency(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.hasRemovableSourceData(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getHighlightsHash(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setHighlightsHash(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.shouldLeafAffectCache(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.updateTargetHighlightsHash(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getTargetHighlightsHash(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setHasHadTerrain(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.unsetHasHadTerrain(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.hasHadTerrain(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.hasLookedForCache(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setLookedForCache(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.caveStartOutdated(2): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.updateCaveMode(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getUpToDateCaveStart(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.shouldConvertCacheToOutdatedOnFinishDim(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setOutdatedWithOtherLayers(1): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isOutdatedWithOtherLayers(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.getCaveDepth(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.canRequestReload_unsynced(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.isResaving(0): not translated, its class isn't translated (it extends `LeveledRegion`).
        // MapRegion.setResaving(1): not translated, its class isn't translated (it extends `LeveledRegion`).
    }

    /// `MapTile` (xaero/map/region/MapTile.java)
    pub mod map_tile {
        use super::*;
        // The class itself isn't translated: it implements `PoolUnit` (xaero.map.pool). Its static parts are below.
        pub const CURRENT_WORLD_INTERPRETATION_VERSION: i32 = 1;
        // MapTile.MapTile(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.create(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.isLoaded(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.setLoaded(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getBlock(2): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getBlockColumn(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.setBlock(3): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getChunkX(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getChunkZ(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.wasWrittenOnce(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.setWrittenOnce(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getWorldInterpretationVersion(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.setWorldInterpretationVersion(1): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getWrittenCaveStart(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.setWrittenCave(2): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
        // MapTile.getWrittenCaveDepth(0): not translated, its class isn't translated (it implements `PoolUnit` (xaero.map.pool)).
    }

    /// `MapTileChunk` (xaero/map/region/MapTileChunk.java)
    pub mod map_tile_chunk {
        use super::*;
        // The class itself isn't translated: it uses MapRegion, which isn't translated (it extends `LeveledRegion`). Its static parts are below.
        pub const SIDE_LENGTH: i32 = 4;
        // MapTileChunk.MapTileChunk(3): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.createLeafTexture(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.updateBuffers(6): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.putColour(8): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getNeighbourTileChunk(4): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.clean(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getX(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getZ(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getTileGridsCache(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getLoadState(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setLoadState(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getTile(2): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setTile(5): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getInRegion(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.wasChanged(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setChanged(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getTimer(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.decTimer(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.includeInSave(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.unincludeInSave(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.resetHeights(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getToUpdateBuffers(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setToUpdateBuffers(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.getLeafTexture(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.writeCacheData(4): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.readCacheData(8): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.toString(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.hasHadTerrain(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setHasHadTerrain(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.unsetHasHadTerrain(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.hasHighlights(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setHasHighlights(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.hasHighlightsIfUndiscovered(0): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
        // MapTileChunk.setHasHighlightsIfUndiscovered(1): not translated, its class isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)).
    }

    /// `MapUpdateFastConfig` (xaero/map/region/MapUpdateFastConfig.java)
    pub mod map_update_fast_config {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapUpdateFastConfig {
            pub block_colors: i32,
            pub biome_blending: bool,
            pub biome_colors_in_vanilla: bool,
            pub terrain_slopes: i32,
            pub terrain_depth: bool,
            pub stained_glass: bool,
            pub legible_cave_maps: bool,
            pub adjust_height_for_short_blocks: bool,
        }
        impl MapUpdateFastConfig {
            // MapUpdateFastConfig.MapUpdateFastConfig(1): not translated, it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
        }
    }

    /// `Overlay` (xaero/map/region/Overlay.java)
    pub mod overlay {
        use super::*;
        // The class itself isn't translated: it extends `MapPixel`. Its static parts are below.
        // Overlay.Overlay(3): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.write(3): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.isWater(0): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.getParametres(0): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.getPixelColour(25): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.toRenderString(0): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.equals(1): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.fillManagerKeyHolder(1): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.increaseOpacity(1): not translated, its class isn't translated (it extends `MapPixel`).
        // Overlay.getOpacity(0): not translated, its class isn't translated (it extends `MapPixel`).
    }

    /// `OverlayBuilder` (xaero/map/region/OverlayBuilder.java)
    pub mod overlay_builder {
        use super::*;
        // The class itself isn't translated: it uses Overlay, which isn't translated (it extends `MapPixel`). Its static parts are below.
        pub const MAX_OVERLAYS: i32 = 10;
        // OverlayBuilder.OverlayBuilder(1): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.startBuilding(0): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.build(5): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.isEmpty(0): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.getCurrentOverlay(0): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.finishBuilding(1): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.getOverlayBiome(0): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
        // OverlayBuilder.setOverlayBiome(1): not translated, its class isn't translated (it uses Overlay, which isn't translated (it extends `MapPixel`)).
    }

    /// `OverlayManager` (xaero/map/region/OverlayManager.java)
    pub mod overlay_manager {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // OverlayManager.OverlayManager(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayManager.getOriginal(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayManager.getOriginal(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayManager.getNumberOfUniqueOverlays(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `UnknownBlockState` (xaero/map/region/state/UnknownBlockState.java)
    pub mod unknown_block_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockState`. Its static parts are below.
        // UnknownBlockState.UnknownBlockState(1): not translated, its class isn't translated (it extends Minecraft's `BlockState`).
        // UnknownBlockState.write(1): not translated, its class isn't translated (it extends Minecraft's `BlockState`).
        // UnknownBlockState.toString(0): not translated, its class isn't translated (it extends Minecraft's `BlockState`).
    }

    /// `BranchRegionTexture` (xaero/map/region/texture/BranchRegionTexture.java)
    pub mod branch_region_texture {
        use super::*;
        // The class itself isn't translated: it extends `RegionTexture`. Its static parts are below.
        // BranchRegionTexture.BranchRegionTexture(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.reset(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.checkForUpdates(5): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.isChildUpdated(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.preUpload(7): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.postUpload(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.uploadBuffer(6): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.copyNonColorData(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.uploadNonCache(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.onCacheUploadRequested(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.onDownloadedBuffer(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.endPBODownload(2): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.hasSourceData(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.addDebugLines(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.onTextureDeletion(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.requestDownload(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.writeCacheMapData(4): not translated, its class isn't translated (it extends `RegionTexture`).
        // BranchRegionTexture.readCacheData(10): not translated, its class isn't translated (it extends `RegionTexture`).
    }

    /// `BranchRegionTexture.ChildTextureInfo` (xaero/map/region/texture/BranchRegionTexture.java)
    pub mod branch_region_texture_child_texture_info {
        use super::*;
        // The class itself isn't translated: it uses RegionTexture, which isn't translated (it's generic). Its static parts are below.
        // BranchRegionTexture.ChildTextureInfo.getColorTextureForUpdate(1): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
        // BranchRegionTexture.ChildTextureInfo.getTextureVersion(0): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
        // BranchRegionTexture.ChildTextureInfo.hasLight(0): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
        // BranchRegionTexture.ChildTextureInfo.onUpdate(1): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
        // BranchRegionTexture.ChildTextureInfo.onParentDeletion(0): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
        // BranchRegionTexture.ChildTextureInfo.getReferenceColorTexture(0): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
        // BranchRegionTexture.ChildTextureInfo.toString(0): not translated, its class isn't translated (it uses RegionTexture, which isn't translated (it's generic)).
    }

    /// `BranchTextureRenderer` (xaero/map/region/texture/BranchTextureRenderer.java)
    pub mod branch_texture_renderer {
        use super::*;
        // The class itself isn't translated: it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`). Its static parts are below.
        // BranchTextureRenderer.BranchTextureRenderer(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // BranchTextureRenderer.getEmptyTexture(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // BranchTextureRenderer.ensureRenderTarget(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // BranchTextureRenderer.render(7): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // BranchTextureRenderer.renderCorner(4): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
        // BranchTextureRenderer.getGlEmptyTexture(0): not translated, its class isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)).
    }

    /// `ExportLeafRegionTexture` (xaero/map/region/texture/ExportLeafRegionTexture.java)
    pub mod export_leaf_region_texture {
        use super::*;
        // The class itself isn't translated: it extends `LeafRegionTexture`. Its static parts are below.
        pub static MIP_MAP_LEVELS: JCell<i32> = JCell::new(7);
        // ExportLeafRegionTexture.ExportLeafRegionTexture(1): not translated, its class isn't translated (it extends `LeafRegionTexture`).
        // ExportLeafRegionTexture.applyHighlights(2): not translated, its class isn't translated (it extends `LeafRegionTexture`).
        // ExportLeafRegionTexture.getMipMapLevels(0): not translated, its class isn't translated (it extends `LeafRegionTexture`).
    }

    /// `LeafRegionTexture` (xaero/map/region/texture/LeafRegionTexture.java)
    pub mod leaf_region_texture {
        use super::*;
        // The class itself isn't translated: it extends `RegionTexture`. Its static parts are below.
        // LeafRegionTexture.LeafRegionTexture(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.postBufferUpdate(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.preUpload(7): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.postUpload(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.canUpload(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.isUploaded(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.hasSourceData(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.uploadNonCache(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.applyHighlights(3): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.postBufferWrite(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.updateTextureVersion(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.addDebugLines(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.writeCacheMapData(4): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.readCacheData(10): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.resetHeights(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.shouldBeUsedForBranchUpdate(1): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.shouldHaveContentForBranchUpdate(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.deleteTexturesAndBuffers(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.prepareBuffer(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.getTileChunk(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.shouldIncludeInCache(0): not translated, its class isn't translated (it extends `RegionTexture`).
        // LeafRegionTexture.requestHighlightOnlyUpload(0): not translated, its class isn't translated (it extends `RegionTexture`).
    }

    /// `RegionTexture` (xaero/map/region/texture/RegionTexture.java)
    pub mod region_texture {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // DEFAULT_INTERNAL_FORMAT: not translated, it uses Minecraft's `GpuFormat`.
        pub const PBO_UNPACK_LENGTH: i32 = 16384;
        pub const PBO_PACK_LENGTH: i32 = 16384;
        pub static ONE_BIOME_PALETTE_DATA: LazyLock<Option<Vec<i64>>> = LazyLock::new(|| None);
        // OLD_HEIGHT_VALUES_SUPPORT: not translated, it uses `ThreadLocal`.
        // RegionTexture.RegionTexture(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.setupTextureParameters(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.prepareBuffer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getMipMapLevels(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.bindColorTexture(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.uploadBuffer(6): not translated, its class isn't translated (it's generic).
        // RegionTexture.postBufferWrite(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.uploadBufferHelper(4): not translated, its class isn't translated (it's generic).
        // RegionTexture.uploadBufferHelper(5): not translated, its class isn't translated (it's generic).
        // RegionTexture.applyHighlights(3): not translated, its class isn't translated (it's generic).
        // RegionTexture.onDownloadedBuffer(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.endPBODownload(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.ensurePackPBO(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.ensureUnpackPBO(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.writeToUnpackPBO(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.deleteColorBuffer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.deletePBOs(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.writeCacheMapData(4): not translated, its class isn't translated (it's generic).
        // RegionTexture.readCacheData(10): not translated, its class isn't translated (it's generic).
        // RegionTexture.saveBiomeIndexStorage(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.loadBiomeIndexStorage(3): not translated, its class isn't translated (it's generic).
        // RegionTexture.deleteTexturesAndBuffers(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getColorBuffer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getDirectColorBuffer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.setShouldDownloadFromPBO(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.getColorBufferFormat(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.shouldDownloadFromPBO(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getTimer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.decTimer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.resetTimer(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getGlColorTexture(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.onTextureDeletion(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.shouldUpload(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.setToUpload(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.isCachePrepared(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.setCachePrepared(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.canUpload(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.isUploaded(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getTextureVersion(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getBufferedTextureVersion(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.setBufferedTextureVersion(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.getRegion(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.updateTextureVersion(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.getHeight(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.putHeight(3): not translated, its class isn't translated (it's generic).
        // RegionTexture.putHeight(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.removeHeight(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.getTopHeight(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.putTopHeight(3): not translated, its class isn't translated (it's generic).
        // RegionTexture.putTopHeight(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.removeTopHeight(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.ensureBiomeIndexStorage(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getBiome(2): not translated, its class isn't translated (it's generic).
        // RegionTexture.setBiome(3): not translated, its class isn't translated (it's generic).
        // RegionTexture.getTextureHasLight(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.addDebugLines(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.onCacheUploadRequested(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.shouldBeUsedForBranchUpdate(1): not translated, its class isn't translated (it's generic).
        // RegionTexture.shouldHaveContentForBranchUpdate(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.shouldIncludeInCache(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.getBiomes(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.resetBiomes(0): not translated, its class isn't translated (it's generic).
        // RegionTexture.hasSourceData(0): not translated, it has no body.
        // RegionTexture.preUpload(7): not translated, it has no body.
        // RegionTexture.postUpload(3): not translated, it has no body.
        // RegionTexture.uploadNonCache(3): not translated, it has no body.
        // RegionTexture.getBufferHasLight(0): not translated, its class isn't translated (it's generic).
    }

    /// `RegionTextureBiomes` (xaero/map/region/texture/RegionTextureBiomes.java)
    pub mod region_texture_biomes {
        use super::*;
        // The class itself isn't translated: it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints))). Its static parts are below.
        // RegionTextureBiomes.RegionTextureBiomes(2): not translated, its class isn't translated (it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))).
        // RegionTextureBiomes.getBiomeIndexStorage(0): not translated, its class isn't translated (it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))).
        // RegionTextureBiomes.getRegionBiomePalette(0): not translated, its class isn't translated (it uses Paletted2DFastBitArrayIntStorage, which isn't translated (it uses FastIntPalette, which isn't translated (it uses `Int2IntMap` (it.unimi.dsi.fastutil.ints)))).
    }

    /// `XaeroWorldMapServerCore` (xaero/map/server/core/XaeroWorldMapServerCore.java)
    pub mod xaero_world_map_server_core {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct XaeroWorldMapServerCore {
        }
        impl XaeroWorldMapServerCore {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // XaeroWorldMapServerCore.onServerWorldInfo(1): not translated, it uses Minecraft's `Player`.
    }

    /// `ServerEvents` (xaero/map/server/events/ServerEvents.java)
    pub mod server_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerEvents {
            pub worldmap_server: Box<world_map_server::WorldMapServer>,
        }
        impl ServerEvents {
            // ServerEvents.ServerEvents(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        }
    }

    /// `IMinecraftServer` (xaero/map/server/IMinecraftServer.java)
    pub mod i_minecraft_server {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IMinecraftServer.getXaeroWorldMapServerData(0): not translated, it has no body.
        // IMinecraftServer.setXaeroWorldMapServerData(1): not translated, it has no body.
    }

    /// `LevelMapProperties` (xaero/map/server/level/LevelMapProperties.java)
    pub mod level_map_properties {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LevelMapProperties {
            pub id: i32,
            pub usable: bool,
        }
        impl LevelMapProperties {
            // LevelMapProperties.write(1): not translated, it's overloaded with the same number of parameters.
            // LevelMapProperties.read(1): not translated, it's overloaded with the same number of parameters.
            pub fn is_usable(&self) -> bool {
                return self.usable;
            }
            pub fn set_usable(&mut self, usable: bool) {
                self.usable = usable;
            }
            pub fn get_id(&self) -> i32 {
                return self.id;
            }
            // LevelMapProperties.write(1): not translated, it's overloaded with the same number of parameters.
        }
        // LevelMapProperties.read(1): not translated, it's overloaded with the same number of parameters.
    }

    /// `LevelMapPropertiesIO` (xaero/map/server/level/LevelMapPropertiesIO.java)
    pub mod level_map_properties_io {
        use super::*;
        pub const FILE_NAME: &str = "xaeromap.txt";
        #[derive(Clone, Debug)]
        pub struct LevelMapPropertiesIO {
        }
        impl LevelMapPropertiesIO {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // LevelMapPropertiesIO.load(2): not translated, a try/finally.
            // LevelMapPropertiesIO.save(2): not translated, a try-with-resources over something other than a file reader or writer.
        }
    }

    /// `MinecraftServerData` (xaero/map/server/MinecraftServerData.java)
    pub mod minecraft_server_data {
        use super::*;
        // The class itself isn't translated: a Map or Set keyed by a Path. Its static parts are below.
        // MinecraftServerData.MinecraftServerData(2): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getLevelProperties(1): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getSyncedPlayerTrackerSystemManager(0): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.getSyncedPlayerTracker(0): not translated, its class isn't translated (a Map or Set keyed by a Path).
        // MinecraftServerData.get(1): not translated, it uses MinecraftServerData, which isn't translated (a Map or Set keyed by a Path).
    }

    /// `MineraftServerDataInitializer` (xaero/map/server/MineraftServerDataInitializer.java)
    pub mod mineraft_server_data_initializer {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MineraftServerDataInitializer {
        }
        impl MineraftServerDataInitializer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // MineraftServerDataInitializer.init(1): not translated, it uses Minecraft's `MinecraftServer`.
        }
    }

    /// `ArgonautsSyncedPlayerTrackerSystem` (xaero/map/server/mods/argonauts/ArgonautsSyncedPlayerTrackerSystem.java)
    pub mod argonauts_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker). Its static parts are below.
        // ArgonautsSyncedPlayerTrackerSystem.getTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)).
        // ArgonautsSyncedPlayerTrackerSystem.isPartySystem(0): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)).
        // ArgonautsSyncedPlayerTrackerSystem.getPartyTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)).
        // ArgonautsSyncedPlayerTrackerSystem.getGuildTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)).
    }

    /// `SupportArgonautsServer` (xaero/map/server/mods/argonauts/SupportArgonautsServer.java)
    pub mod support_argonauts_server {
        use super::*;
        // The class itself isn't translated: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // SupportArgonautsServer.getSyncedPlayerTrackerSystem(0): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `FTBTeamsSyncedPlayerTrackerSystem` (xaero/map/server/mods/ftbteams/FTBTeamsSyncedPlayerTrackerSystem.java)
    pub mod ftb_teams_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker). Its static parts are below.
        // FTBTeamsSyncedPlayerTrackerSystem.getTrackingLevel(2): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)).
        // FTBTeamsSyncedPlayerTrackerSystem.isPartySystem(0): not translated, its class isn't translated (it implements `ISyncedPlayerTrackerSystem` (xaero.map.server.radar.tracker)).
    }

    /// `SupportFTBTeamsServer` (xaero/map/server/mods/ftbteams/SupportFTBTeamsServer.java)
    pub mod support_ftb_teams_server {
        use super::*;
        // The class itself isn't translated: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // SupportFTBTeamsServer.getSyncedPlayerTrackerSystem(0): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `ServerPlayerOpacData` (xaero/map/server/mods/opac/ServerPlayerOpacData.java)
    pub mod server_player_opac_data {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerPlayerOpacData {
            pub share_location_with_party: bool,
            pub share_location_with_mutual_allies: bool,
        }
        impl ServerPlayerOpacData {
            pub fn new() -> Self {
                let mut s = Self { share_location_with_party: false, share_location_with_mutual_allies: false };
                s
            }
        }
    }

    /// `SupportOPACServer` (xaero/map/server/mods/opac/SupportOPACServer.java)
    pub mod support_opac_server {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportOPACServer {
        }
        impl SupportOPACServer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SupportOPACServer.isPositionSyncAllowed(3): not translated, it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)).
            // SupportOPACServer.getReceiveLocationsFromMutualAlliesConfigValue(1): not translated, it uses Minecraft's `ServerPlayer`.
            // SupportOPACServer.getReceiveLocationsFromPartyConfigValue(1): not translated, it uses Minecraft's `ServerPlayer`.
            // SupportOPACServer.updateShareLocationConfigValues(2): not translated, it uses Minecraft's `ServerPlayer`.
            // SupportOPACServer.getPlayerOpacData(1): not translated, it uses `Object`.
        }
    }

    /// `SupportMinimapServer` (xaero/map/server/mods/SupportMinimapServer.java)
    pub mod support_minimap_server {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SupportMinimapServer {
            pub compatibility_version: i32,
        }
        impl SupportMinimapServer {
            // SupportMinimapServer.SupportMinimapServer(0): not translated, it uses `XaeroMinimapServer` (xaero.common.server).SERVER_COMPATIBILITY.
            pub fn supports_tracked_players(&self) -> bool {
                return self.compatibility_version >= 1;
            }
            // SupportMinimapServer.playerSupportsTrackedPlayers(1): not translated, it uses Minecraft's `ServerPlayer`.
        }
    }

    /// `SupportServerMods` (xaero/map/server/mods/SupportServerMods.java)
    pub mod support_server_mods {
        use super::*;
        // ftbTeams: not translated, it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
        // argonauts: not translated, it uses SupportArgonautsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
        pub static OPAC: LazyLock<JCell<Option<support_opac_server::SupportOPACServer>>> = LazyLock::new(|| JCell::new(None));
        pub static MINIMAP: LazyLock<JCell<Option<support_minimap_server::SupportMinimapServer>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct SupportServerMods {
        }
        impl SupportServerMods {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SupportServerMods.check(0): not translated, a try/catch (ClassNotFoundException).
        // SupportServerMods.hasFtbTeams(0): not translated, it uses SupportServerMods.ftbTeams, which isn't translated (it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.getFtbTeams(0): not translated, it uses SupportFTBTeamsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
        // SupportServerMods.hasArgonauts(0): not translated, it uses SupportServerMods.argonauts, which isn't translated (it uses SupportArgonautsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface))).
        // SupportServerMods.getArgonauts(0): not translated, it uses SupportArgonautsServer, which isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
        pub fn has_opac() -> bool {
            return support_server_mods::OPAC.with(|o| o.is_some());
        }
        pub fn get_opac() -> Option<support_opac_server::SupportOPACServer> {
            return support_server_mods::OPAC.with(|o| o.clone());
        }
        pub fn has_minimap() -> bool {
            return support_server_mods::MINIMAP.with(|o| o.is_some());
        }
        pub fn get_minimap() -> Option<support_minimap_server::SupportMinimapServer> {
            return support_server_mods::MINIMAP.with(|o| o.clone());
        }
    }

    /// `IServerPlayer` (xaero/map/server/player/IServerPlayer.java)
    pub mod i_server_player {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IServerPlayer.getXaeroWorldMapPlayerData(0): not translated, it has no body.
        // IServerPlayer.setXaeroWorldMapPlayerData(1): not translated, it has no body.
    }

    /// `ServerPlayerData` (xaero/map/server/player/ServerPlayerData.java)
    pub mod server_player_data {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // ServerPlayerData.ServerPlayerData(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getLastSyncedData(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.ensureLastSyncedData(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getCurrentlySyncedPlayers(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.ensureCurrentlySyncedPlayers(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getLastTrackedPlayerSync(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.setLastTrackedPlayerSync(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.get(1): not translated, it uses ServerPlayerData, which isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.hasMod(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.setClientModNetworkVersion(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getClientModNetworkVersion(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.setOpacData(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // ServerPlayerData.getOpacData(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `ServerPlayerTickHandler` (xaero/map/server/player/ServerPlayerTickHandler.java)
    pub mod server_player_tick_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ServerPlayerTickHandler {
        }
        impl ServerPlayerTickHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // ServerPlayerTickHandler.tick(1): not translated, it uses Minecraft's `ServerPlayer`.
        }
    }

    /// `ISyncedPlayerTrackerSystem` (xaero/map/server/radar/tracker/ISyncedPlayerTrackerSystem.java)
    pub mod i_synced_player_tracker_system {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ISyncedPlayerTrackerSystem.getTrackingLevel(2): not translated, it has no body.
        // ISyncedPlayerTrackerSystem.isPartySystem(0): not translated, it has no body.
    }

    /// `SyncedPlayerTracker` (xaero/map/server/radar/tracker/SyncedPlayerTracker.java)
    pub mod synced_player_tracker {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct SyncedPlayerTracker {
        }
        impl SyncedPlayerTracker {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // SyncedPlayerTracker.onTick(4): not translated, it uses Minecraft's `MinecraftServer`.
            // SyncedPlayerTracker.sendRemovePacket(3): not translated, it uses Minecraft's `ServerPlayer`.
            // SyncedPlayerTracker.sendTrackedPlayerPacket(3): not translated, it uses Minecraft's `ServerPlayer`.
        }
    }

    /// `SyncedPlayerTrackerSystemManager` (xaero/map/server/radar/tracker/SyncedPlayerTrackerSystemManager.java)
    pub mod synced_player_tracker_system_manager {
        use super::*;
        // The class itself isn't translated: it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface). Its static parts are below.
        // SyncedPlayerTrackerSystemManager.register(2): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
        // SyncedPlayerTrackerSystemManager.getSystems(0): not translated, its class isn't translated (it uses ISyncedPlayerTrackerSystem, which isn't translated (it's an interface)).
    }

    /// `SyncedTrackedPlayer` (xaero/map/server/radar/tracker/SyncedTrackedPlayer.java)
    pub mod synced_tracked_player {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // SyncedTrackedPlayer.SyncedTrackedPlayer(5): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.setPos(3): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.setDimension(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getId(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getX(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getY(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getZ(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.getDimension(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.matchesEnough(2): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.update(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // SyncedTrackedPlayer.copyFrom(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `WorldMapServer` (xaero/map/server/WorldMapServer.java)
    pub mod world_map_server {
        use super::*;
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct WorldMapServer {
            pub server_events: Option<server_events::ServerEvents>,
        }
        impl WorldMapServer {
            pub fn new() -> Self {
                let mut s = Self { server_events: None };
                s
            }
            pub fn load(&self) {
                // log: "Loading Xaero's World Map - Stage 1/2 (Server)"
            }
            pub fn load_later(&self) {
                // log: "Loading Xaero's World Map - Stage 2/2 (Server)"
            }
            pub fn get_server_events(&self) -> Option<server_events::ServerEvents> {
                return self.server_events.clone();
            }
        }
    }

    /// `WorldMapServerFabric` (xaero/map/server/WorldMapServerFabric.java)
    pub mod world_map_server_fabric {
        use super::*;
        // The class itself isn't translated: it extends `WorldMapServer`. Its static parts are below.
    }

    /// `ModSettings` (xaero/map/settings/ModSettings.java)
    pub mod mod_settings {
        use super::*;
        pub const FORMAT: &str = "\u{a7}";
        #[derive(Clone, Debug)]
        pub struct ModSettings {
            pub region_cache_hash_code: i32,
        }
        impl ModSettings {
            pub fn new() -> Self {
                let mut s = Self { region_cache_hash_code: 0i32 };
                s
            }
            // ModSettings.loadDefaultSettings(0): not translated, it uses `File` (java.io).
            // ModSettings.loadSettings(0): not translated, it calls ModSettings.loadDefaultSettings, which isn't translated.
            // ModSettings.loadSettingsFile(1): not translated, it uses `File` (java.io).
            pub fn get_region_cache_hash_code(&self) -> i32 {
                return self.region_cache_hash_code;
            }
            // ModSettings.updateRegionCacheHashCode(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
        }
        // ModSettings.canEditIngameSettings(0): not translated, it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))).
    }

    /// `MapRunnerTask` (xaero/map/task/MapRunnerTask.java)
    pub mod map_runner_task {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapRunnerTask {
        }
        impl MapRunnerTask {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // MapRunnerTask.run(1): not translated, it has no body.
        }
    }

    /// `MapTeleporter` (xaero/map/teleport/MapTeleporter.java)
    pub mod map_teleporter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MapTeleporter {
        }
        impl MapTeleporter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // MapTeleporter.teleport(6): not translated, it uses Minecraft's `Screen`.
        }
    }

    /// `DistanceUtils` (xaero/map/util/DistanceUtils.java)
    pub mod distance_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DistanceUtils {
        }
        impl DistanceUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DistanceUtils.getDistanceText(3): not translated, it calls `NumberFormatUtils` (xaero.lib.common.util).getPrecisionFormat.
        // DistanceUtils.addDistanceRightClickOption(9): not translated, its body couldn't be read: an anonymous class.
    }

    /// `ILinkedChainNode` (xaero/map/util/linked/ILinkedChainNode.java)
    pub mod i_linked_chain_node {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ILinkedChainNode.setNext(1): not translated, it has no body.
        // ILinkedChainNode.setPrevious(1): not translated, it has no body.
        // ILinkedChainNode.getNext(0): not translated, it has no body.
        // ILinkedChainNode.getPrevious(0): not translated, it has no body.
        // ILinkedChainNode.isDestroyed(0): not translated, it has no body.
        // ILinkedChainNode.onDestroyed(0): not translated, it has no body.
    }

    /// `LinkedChain` (xaero/map/util/linked/LinkedChain.java)
    pub mod linked_chain {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LinkedChain.add(1): not translated, its class isn't translated (it's generic).
        // LinkedChain.remove(1): not translated, its class isn't translated (it's generic).
        // LinkedChain.destroy(0): not translated, its class isn't translated (it's generic).
        // LinkedChain.reset(0): not translated, its class isn't translated (it's generic).
        // LinkedChain.iterator(0): not translated, its body couldn't be read: an anonymous class.
        // LinkedChain.stream(0): not translated, its class isn't translated (it's generic).
    }

    /// `WorldMap` (xaero/map/WorldMap.java)
    pub mod world_map {
        use super::*;
        // The class itself isn't translated: it uses `ConfigChannel` (xaero.lib.common.config.channel). Its static parts are below.
        pub const MOD_ID: &str = "xaeroworldmap";
        pub static LOADED: JCell<bool> = JCell::new(false);
        // INSTANCE: not translated, it uses WorldMap, which isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        pub static MINIMAP_COMPATIBILITY_VERSION: JCell<i32> = JCell::new(26);
        // LOGGER: the logger. Log calls become comments.
        pub const VERSION_ID_MINECRAFT: &str = "26.3";
        pub static NEWEST_UPDATE_ID: JCell<i32> = JCell::new(0i32);
        pub static IS_OUTDATED: JCell<bool> = JCell::new(false);
        pub static LATEST_VERSION: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(None));
        pub static LATEST_VERSION_MD5: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(None));
        // events: not translated, it uses ClientEvents, which isn't translated (it uses Minecraft's `RealmsServer`).
        pub static MOD_EVENTS: LazyLock<JCell<Option<mod_client_events::ModClientEvents>>> = LazyLock::new(|| JCell::new(None));
        // controlsRegister: not translated, it uses ControlsRegister, which isn't translated (it uses Minecraft's `KeyMapping`).
        // waypointSymbolCreator: not translated, it uses WaypointSymbolCreator, which isn't translated (it uses Minecraft's `Minecraft`).
        // bufferDeallocator: not translated, it uses ByteBufferDeallocator, which isn't translated (it uses `Object`).
        pub static TEXTURE_UPLOAD_BENCHMARK: LazyLock<JCell<Option<texture_upload_benchmark::TextureUploadBenchmark>>> = LazyLock::new(|| JCell::new(None));
        // overlayManager: not translated, it uses OverlayManager, which isn't translated (it uses Minecraft's `BlockState`).
        // oldFormatSupport: not translated, it uses OldFormatSupport, which isn't translated (it uses Minecraft's `BlockState`).
        // pngExporter: not translated, it uses PNGExporter, which isn't translated (it uses `Calendar` (java.util)).
        // normalTextureUploadPool: not translated, it uses Normal, which isn't translated (it extends `TextureUploadPool`).
        // branchUpdatePool: not translated, it uses BranchUpdate, which isn't translated (it extends `TextureUploadPool`).
        // branchUpdateAllocatePool: not translated, it uses BranchUpdate, which isn't translated (it extends `TextureUploadPool`).
        // branchDownloadPool: not translated, it uses BranchDownload, which isn't translated (it extends `TextureUploadPool`).
        // subsequentNormalTextureUploadPool: not translated, it uses SubsequentNormal, which isn't translated (it extends `TextureUploadPool`).
        // textureDirectBufferPool: not translated, it uses TextureDirectBufferPool, which isn't translated (it extends `MapPool` (xaero.map.pool)).
        // tilePool: not translated, it uses MapTilePool, which isn't translated (it extends `MapPool`).
        // mapLimiter: not translated, it uses MapLimiter, which isn't translated (it uses MapDimension, which isn't translated (it uses Minecraft's `ResourceKey`)).
        // unknownBlockStateCache: not translated, it uses UnknownBlockStateCache, which isn't translated (it uses Minecraft's `BlockState`).
        // gpuObjectDeleter: not translated, it uses GpuObjectDeleter, which isn't translated (it uses `GpuTextureAndView` (xaero.lib.client.graphics)).
        // mapRunner: not translated, it uses MapRunner, which isn't translated (it implements `Runnable`).
        // mapRunnerThread: not translated, it uses `Thread`.
        // crashHandler: not translated, it uses CrashHandler, which isn't translated (it uses `Throwable`).
        pub static GUI_TEXTURES: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("xaeroworldmap", "gui/gui.png"));
        pub static SETTINGS: LazyLock<JCell<Option<mod_settings::ModSettings>>> = LazyLock::new(|| JCell::new(None));
        // worldMapClientOnly: not translated, it uses WorldMapClientOnly, which isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        pub static WORLDMAP_SERVER: LazyLock<JCell<Option<world_map_server::WorldMapServer>>> = LazyLock::new(|| JCell::new(None));
        // mapElementRenderHandler: not translated, it uses MapElementRenderHandler, which isn't translated (it uses MapElementGraphics, which isn't translated (it uses `XaeroBufferProvider` (xaero.lib.client.graphics))).
        pub static SERVER_PLAYER_TICK_HANDLER: LazyLock<JCell<Option<server_player_tick_handler::ServerPlayerTickHandler>>> = LazyLock::new(|| JCell::new(None));
        // playerTrackerSystemManager: not translated, it uses PlayerTrackerSystemManager, which isn't translated (it uses IPlayerTrackerSystem, which isn't translated (it's an interface)).
        // trackedPlayerRenderer: not translated, it uses PlayerTrackerMapElementRenderer, which isn't translated (it extends `ElementRenderer` (xaero.map.element.render)).
        // trackedPlayerMenuRenderer: not translated, it uses PlayerTrackerMenuRenderer, which isn't translated (it extends `MapElementMenuRenderer` (xaero.map.element)).
        // messageHandler: not translated, it uses `IPacketHandler` (xaero.lib.common.packet).
        pub static COMMON_EVENTS: LazyLock<JCell<Option<common_events::CommonEvents>>> = LazyLock::new(|| JCell::new(None));
        pub static MOD_COMMON_EVENTS: LazyLock<JCell<Option<mod_common_events::ModCommonEvents>>> = LazyLock::new(|| JCell::new(None));
        pub static DETAILED_DEBUG: JCell<bool> = JCell::new(false);
        pub static PAUSE_REQUESTS: JCell<bool> = JCell::new(false);
        pub static EXTRA_DEBUG: JCell<bool> = JCell::new(false);
        // modJAR: not translated, it uses `File` (java.io).
        // configFolder: not translated, it uses `File` (java.io).
        // optionsFile: not translated, it uses `File` (java.io).
        // saveFolder: not translated, it uses `File` (java.io).
        pub static COMMON_CONFIG_IO: LazyLock<JCell<Option<legacy_common_config_io::LegacyCommonConfigIO>>> = LazyLock::new(|| JCell::new(None));
        // WorldMap.WorldMap(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.fetchModFile(0): not translated, it has no body.
        // WorldMap.getFileLayoutID(0): not translated, it has no body.
        // WorldMap.loadClient(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.loadLater(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.loadLaterClientRender(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.loadServer(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.loadLaterServer(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.loadCommon(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.loadLaterCommon(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.getVersionID(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.onSessionFinalized(0): not translated, a nullable object kept in a changing static.
        // WorldMap.getConfigs(0): not translated, its class isn't translated (it uses `ConfigChannel` (xaero.lib.common.config.channel)).
        // WorldMap.createSupportMods(0): not translated, it has no body.
        // WorldMap.createClientLoad(0): not translated, it has no body.
        // WorldMap.createServerLoad(0): not translated, it has no body.
        // WorldMap.createWriter(3): not translated, it has no body.
        // WorldMap.getModInfoVersion(0): not translated, it has no body.
    }

    /// `WorldMapClientOnly` (xaero/map/WorldMapClientOnly.java)
    pub mod world_map_client_only {
        use super::*;
        // The class itself isn't translated: it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`)). Its static parts are below.
        // WorldMapClientOnly.preInit(1): not translated, its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // WorldMapClientOnly.postInit(0): not translated, its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // WorldMapClientOnly.loadLaterClientRender(0): not translated, its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
        // WorldMapClientOnly.getMapScreenPoseStack(0): not translated, its class isn't translated (it uses BranchTextureRenderer, which isn't translated (it uses ImprovedFramebuffer, which isn't translated (it extends Minecraft's `TextureTarget`))).
    }

    /// `WorldMapClientOnlyFabric` (xaero/map/WorldMapClientOnlyFabric.java)
    pub mod world_map_client_only_fabric {
        use super::*;
        // The class itself isn't translated: it extends `WorldMapClientOnly`. Its static parts are below.
        // WorldMapClientOnlyFabric.preInit(1): not translated, its class isn't translated (it extends `WorldMapClientOnly`).
    }

    /// `WorldMapFabric` (xaero/map/WorldMapFabric.java)
    pub mod world_map_fabric {
        use super::*;
        // The class itself isn't translated: it extends `WorldMap`. Its static parts are below.
        // WorldMapFabric.onInitializeClient(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.onInitializeServer(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.registerClientEvents(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.registerCommonEvents(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.loadCommon(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.loadClient(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.loadServer(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.tryLoadLater(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.tryLoadLaterServer(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.fetchModFile(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.getFileLayoutID(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.createSupportMods(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.createClientLoad(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.createServerLoad(0): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.createWriter(3): not translated, its class isn't translated (it extends `WorldMap`).
        // WorldMapFabric.getModInfoVersion(0): not translated, its class isn't translated (it extends `WorldMap`).
    }

    /// `WorldMapSession` (xaero/map/WorldMapSession.java)
    pub mod world_map_session {
        use super::*;
        // The class itself isn't translated: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))). Its static parts are below.
        // WorldMapSession.init(2): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapSession.cleanup(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapSession.getControlsHandler(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapSession.getMapProcessor(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapSession.getCurrentSession(0): not translated, it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapSession.getForPlayer(1): not translated, it uses WorldMapSession, which isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // WorldMapSession.isUsable(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
    }

    /// `MapConnectionManager` (xaero/map/world/MapConnectionManager.java)
    pub mod map_connection_manager {
        use super::*;
        // The class itself isn't translated: a Map or Set keyed by a MapConnectionNode. Its static parts are below.
        // MapConnectionManager.addConnection(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.addOneWayConnection(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.removeConnection(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.removeOneWayConnection(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.isConnected(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.isEmpty(0): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.save(1): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.swapConnections(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
        // MapConnectionManager.renameDimension(2): not translated, its class isn't translated (a Map or Set keyed by a MapConnectionNode).
    }

    /// `MapConnectionNode` (xaero/map/world/MapConnectionNode.java)
    pub mod map_connection_node {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // MapConnectionNode.MapConnectionNode(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.toString(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.getNamedString(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.fromString(1): not translated, it uses MapConnectionNode, which isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.equals(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.getDimId(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapConnectionNode.getMw(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `MapDimension` (xaero/map/world/MapDimension.java)
    pub mod map_dimension {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        pub const CAVE_MODE_TYPES: i32 = 3;
        // MapDimension.MapDimension(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getCurrentMultiworld(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.isUsingWorldSave(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.isFutureUsingWorldSaveUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getMultiworldIdsCopy(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.updateFutureAutomaticUnsynced(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getFutureCustomSelectedMultiworld(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getFutureMultiworldUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.switchToFutureUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.switchToFutureMultiworldWritableValueUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getLayeredMapRegions(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.clear(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.preDetection(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getMainFolderPath(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getOldFolderPath(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.saveConfigUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.loadConfigUnsynced(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.pickDefaultCustomMultiworldUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.loadMultiworldsList(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.confirmMultiworldUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.makeCustomSelectedMultiworldAutoUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.bindAutoMultiworld(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.resetCustomMultiworldUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.setMultiworldUnsynced(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.multiworldExists(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.addMultiworldChecked(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getMultiworldName(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.setMultiworldName(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getFutureAutoMultiworld(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getMapWorld(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.deleteMultiworldMapDataUnsynced(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.deleteMultiworldId(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getDimId(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.hasConfirmedMultiworld(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.isFutureMultiworldServerBased(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getHighlightHandler(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.onClearCachedHighlightHash(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.onClearCachedHighlightHashes(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.hasDoneRegionDetection(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.addWorldSaveRegionDetection(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getWorldSaveRegionDetection(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getCaveModeType(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.toggleCaveModeType(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getWorldSaveDetectedRegions(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getLinkedWorldSaveDetectedRegions(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getFullReloader(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.startFullMapReload(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.clearFullMapReload(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getDimensionType(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getDimensionType(3): not translated, it uses Minecraft's `DimensionType`.
        // MapDimension.getDimensionType(3): not translated, it uses Minecraft's `DimensionType`.
        // MapDimension.onWorldChangeUnsynced(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.isUsingUnknownDimensionType(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.isCacheOnlyMode(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.onCreationUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getShadowR(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getShadowG(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getShadowB(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getSkyDarken(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.invalidateTickCache(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.calculateDimScale(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.calculateDimDiv(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getPlayerMapKey(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getSelectedMapKeyUnsynced(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.isAutoSelected(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.getDimensionTypeId(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.currentMultiworldBlockedByFileLock(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // MapDimension.setCurrentMultiworldBlockedByFileLock(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `MapWorld` (xaero/map/world/MapWorld.java)
    pub mod map_world {
        use super::*;
        // The class itself isn't translated: it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`))). Its static parts are below.
        // MapWorld.MapWorld(3): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getDimension(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.createDimensionUnsynced(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getMainId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getOldUnfixedMainId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getCurrentMultiworld(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getFutureMultiworldUnsynced(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getCurrentDimension(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getFutureDimension(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getCurrentDimensionId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getFutureDimensionId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setFutureDimensionId(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getCustomDimensionId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setCustomDimensionId(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.switchToFutureUnsynced(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getDimensionsList(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getDimensions(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getCurrentMultiworldType(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isMultiplayer(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isCurrentMultiworldTypeConfirmed(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getFutureMultiworldType(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.toggleMultiworldTypeUnsynced(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.unconfirmMultiworldTypeUnsynced(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.confirmMultiworldTypeUnsynced(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isFutureMultiworldTypeConfirmed(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.switchToFutureMultiworldTypeUnsynced(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.load(0): not translated, its body couldn't be read: an anonymous class.
        // MapWorld.loadConfig(2): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.saveConfig(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getMapProcessor(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isIgnoreServerLevelId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isIgnoreHeightmaps(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setIgnoreHeightmaps(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getPlayerTeleportCommandFormat(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setPlayerTeleportCommandFormat(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getEffectivePlayerTeleportCommandFormat(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getTeleportCommandFormat(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setTeleportCommandFormat(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getEffectiveTeleportCommandFormat(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getDimensionTeleportCommandFormat(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setDimensionTeleportCommandFormat(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getEffectiveDimensionTeleportCommandFormat(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.clearAllCachedHighlightHashes(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isUsingCustomDimension(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isUsingUnknownDimensionType(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isCacheOnlyMode(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.onWorldChangeUnsynced(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getPotentialDimId(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getPlayerMapKey(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.getMapConnections(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.toggleDimension(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        pub fn convert_world_folder_to_root_id(version: i32, world_folder: &str) -> String {
            let mut root_id: String = { static RE: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("_")); &*RE }.replace_all(&world_folder, "^us^");
            if map_processor::is_world_multiplayer(map_processor::is_world_realms(&root_id), &root_id) {
                root_id = format!("^e^{}", root_id);
            }
            if version >= 3 {
                root_id = (root_id.replace("[", "%lb%")).replace("]", "%rb%");
            }
            return root_id.to_string();
        }
        // MapWorld.isUsingDefaultMapTeleport(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.isUsingDefaultPlayerTeleport(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setUseDefaultMapTeleport(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.setUseDefaultPlayerTeleport(1): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
        // MapWorld.invalidateTickCache(0): not translated, its class isn't translated (it uses MapProcessor, which isn't translated (it uses MapSaveLoad, which isn't translated (it uses MapRegion, which isn't translated (it extends `LeveledRegion`)))).
    }

    /// JOML's Vector3f as a value (Java's methods change the vector; the translation writes the result back).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct JVec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl JVec3 {
        pub const fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn add(self, o: JVec3) -> Self {
            Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
        }
        pub fn add3(self, x: f32, y: f32, z: f32) -> Self {
            Self::new(self.x + x, self.y + y, self.z + z)
        }
        pub fn sub(self, o: JVec3) -> Self {
            Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
        }
        pub fn mul_s(self, s: f32) -> Self {
            Self::new(self.x * s, self.y * s, self.z * s)
        }
        pub fn mul_v(self, o: JVec3) -> Self {
            Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
        }
        pub fn negate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z)
        }
        pub fn dot(self, o: JVec3) -> f32 {
            self.x * o.x + self.y * o.y + self.z * o.z
        }
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }
        pub fn length_squared(self) -> f32 {
            self.dot(self)
        }
        /// JOML's normalize: divides by the length (a zero vector becomes NaNs, as in Java).
        pub fn normalize(self) -> Self {
            let inv = 1.0 / self.length();
            self.mul_s(inv)
        }
        pub fn cross(self, o: JVec3) -> Self {
            Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
        }
        /// JOML's lerp: this + (other - this) * t.
        pub fn lerp(self, o: JVec3, t: f32) -> Self {
            Self::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
        }
        pub fn distance(self, o: JVec3) -> f32 {
            self.sub(o).length()
        }
    }

    /// JOML's Quaternionf (x, y, z, w).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JQuat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl Default for JQuat {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JQuat {
        pub const IDENTITY: JQuat = JQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
        /// `rotationX(angle)`: a turn about +X by `angle` radians (Axis.XP.rotation).
        pub fn rotation_x(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(s, 0.0, 0.0, c)
        }
        pub fn rotation_y(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, s, 0.0, c)
        }
        pub fn rotation_z(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, 0.0, s, c)
        }
        pub fn normalize(self) -> Self {
            let inv = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
            Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
        /// this * q.
        pub fn mul(self, q: JQuat) -> Self {
            Self::new(
                self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
                self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
                self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
                self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            )
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.mul(Self::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.mul(Self::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.mul(Self::rotation_z(a))
        }
    }

    /// JOML's Matrix3f, column-major, from three column vectors.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat3 {
        pub m: [f32; 9],
    }
    impl JMat3 {
        pub fn from_cols(c0: JVec3, c1: JVec3, c2: JVec3) -> Self {
            Self { m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z] }
        }
    }

    /// JOML's Matrix4f: column-major, `m[col * 4 + row]`; `a.mul(b)` is a * b, and translate/rotate/scale
    /// multiply on the right, as JOML does.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat4 {
        pub m: [f32; 16],
    }
    impl Default for JMat4 {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JMat4 {
        pub const IDENTITY: JMat4 = JMat4 { m: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        pub fn identity() -> Self {
            Self::IDENTITY
        }
        fn at(&self, row: usize, col: usize) -> f32 {
            self.m[col * 4 + row]
        }
        pub fn mul(self, r: JMat4) -> Self {
            let mut out = [0.0f32; 16];
            for col in 0..4 {
                for row in 0..4 {
                    let mut v = 0.0;
                    for k in 0..4 {
                        v += self.at(row, k) * r.at(k, col);
                    }
                    out[col * 4 + row] = v;
                }
            }
            Self { m: out }
        }
        pub fn translation(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[12] = x;
            t.m[13] = y;
            t.m[14] = z;
            t
        }
        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::translation(x, y, z))
        }
        pub fn translate_v(self, v: JVec3) -> Self {
            self.translate(v.x, v.y, v.z)
        }
        pub fn scaling(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[0] = x;
            t.m[5] = y;
            t.m[10] = z;
            t
        }
        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::scaling(x, y, z))
        }
        pub fn scale_s(self, s: f32) -> Self {
            self.scale(s, s, s)
        }
        /// The rotation a unit quaternion stands for.
        pub fn rotation(q: JQuat) -> Self {
            let (x, y, z, w) = (q.x, q.y, q.z, q.w);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (xy, xz, yz, wx, wy, wz) = (x * y, x * z, y * z, w * x, w * y, w * z);
            Self {
                m: [
                    1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
                    2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0,
                    2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            }
        }
        pub fn rotate(self, q: JQuat) -> Self {
            self.mul(Self::rotation(q))
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_z(a))
        }
        /// JOML's rotateXYZ: about X, then Y, then Z.
        pub fn rotate_xyz(self, x: f32, y: f32, z: f32) -> Self {
            self.rotate_x(x).rotate_y(y).rotate_z(z)
        }
        /// JOML's rotateAround(q, ox, oy, oz): about the point (ox, oy, oz).
        pub fn rotate_around(self, q: JQuat, ox: f32, oy: f32, oz: f32) -> Self {
            self.translate(ox, oy, oz).rotate(q).translate(-ox, -oy, -oz)
        }
        /// The general inverse (JOML's invert); a singular matrix gives infinities, as in Java.
        pub fn invert(self) -> Self {
            let a = &self.m;
            let mut inv = [0.0f32; 16];
            inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
            inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
            inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
            inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
            inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
            inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
            inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
            inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
            inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
            inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
            inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
            inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
            inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
            inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
            inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
            inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
            let det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
            let d = 1.0 / det;
            for v in inv.iter_mut() {
                *v *= d;
            }
            Self { m: inv }
        }
        /// The point v moved by this matrix (w = 1).
        pub fn transform_position(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z + self.at(0, 3),
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z + self.at(1, 3),
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z + self.at(2, 3),
            )
        }
        /// The direction v turned by this matrix (w = 0).
        pub fn transform_direction(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z,
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z,
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z,
            )
        }
        /// JOML's set3x3: the upper-left 3x3 from `m`, the rest kept.
        pub fn set3x3(self, m: JMat3) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = m.m[col * 3 + row];
                }
            }
            out
        }
        /// JOML's transpose3x3: the upper-left 3x3 transposed, the rest kept.
        pub fn transpose3x3(self) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = self.m[row * 4 + col];
                }
            }
            out
        }
        pub fn get_translation(&self) -> JVec3 {
            JVec3::new(self.m[12], self.m[13], self.m[14])
        }
    }

    /// Minecraft's PoseStack: a stack of matrices (normals aren't kept: Horizon's hand renderer takes one matrix).
    #[derive(Clone, Debug)]
    pub struct JPoseStack {
        pub stack: Vec<JMat4>,
    }
    impl Default for JPoseStack {
        fn default() -> Self {
            Self { stack: vec![JMat4::IDENTITY] }
        }
    }
    impl JPoseStack {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn of(m: JMat4) -> Self {
            Self { stack: vec![m] }
        }
        fn top(&mut self) -> &mut JMat4 {
            self.stack.last_mut().expect("PoseStack is empty")
        }
        pub fn push_pose(&mut self) {
            let m = *self.top();
            self.stack.push(m);
        }
        pub fn pop_pose(&mut self) {
            self.stack.pop();
        }
        /// Writes the top matrix (Java changed `pose.last().pose()` in place).
        pub fn set_last(&mut self, m: JMat4) {
            *self.top() = m;
        }
        pub fn last_pose(&self) -> JMat4 {
            *self.stack.last().expect("PoseStack is empty")
        }
        pub fn set_identity(&mut self) {
            *self.top() = JMat4::IDENTITY;
        }
        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.translate(x, y, z);
        }
        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.scale(x, y, z);
        }
        pub fn mul_pose(&mut self, q: JQuat) {
            let m = self.top();
            *m = m.rotate(q);
        }
        pub fn mul_pose_matrix(&mut self, r: JMat4) {
            let m = self.top();
            *m = m.mul(r);
        }
        pub fn rotate_around(&mut self, q: JQuat, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.rotate_around(q, x, y, z);
        }
    }

    #[cfg(test)]
    mod joml_tests {
        use super::*;

        fn close(a: JVec3, b: JVec3) -> bool {
            (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
        }

        #[test]
        fn rotations_turn_axes_as_joml_does() {
            let q = JQuat::rotation_x(std::f32::consts::FRAC_PI_2);
            // A positive turn about X takes +Y to +Z (right-handed), as JOML's rotationX.
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(0.0, 0.0, 1.0)));
            let q = JQuat::rotation_y(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 0.0, 1.0)), JVec3::new(1.0, 0.0, 0.0)));
            let q = JQuat::rotation_z(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
        }

        #[test]
        fn translate_then_rotate_multiplies_on_the_right() {
            // JOML: new Matrix4f().translate(1, 0, 0).rotateZ(90) moves (1, 0, 0) to (1, 1, 0).
            let m = JMat4::identity().translate(1.0, 0.0, 0.0).rotate_z(std::f32::consts::FRAC_PI_2);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(1.0, 1.0, 0.0)));
        }

        #[test]
        fn invert_undoes() {
            let m = JMat4::identity().translate(1.0, 2.0, 3.0).rotate_x(0.7).rotate_y(-0.3).scale(2.0, 0.5, 1.5);
            let p = JVec3::new(0.3, -1.2, 4.0);
            assert!(close(m.invert().transform_position(m.transform_position(p)), p));
            assert!(close(m.mul(m.invert()).transform_position(p), p));
        }

        #[test]
        fn rotate_around_keeps_its_point() {
            let m = JMat4::identity().rotate_around(JQuat::rotation_y(1.1), 0.5, 0.25, 0.0);
            assert!(close(m.transform_position(JVec3::new(0.5, 0.25, 0.0)), JVec3::new(0.5, 0.25, 0.0)));
        }

        #[test]
        fn pose_stack_push_pop() {
            let mut p = JPoseStack::new();
            p.translate(1.0, 0.0, 0.0);
            p.push_pose();
            p.scale(2.0, 2.0, 2.0);
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(3.0, 0.0, 0.0)));
            p.pop_pose();
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(2.0, 0.0, 0.0)));
        }

        #[test]
        fn quaternion_matrix_agrees_with_axis_turns() {
            let q = JQuat::rotation_x(0.4).mul(JQuat::rotation_y(-0.9));
            let a = JMat4::rotation(q);
            let b = JMat4::identity().rotate_x(0.4).rotate_y(-0.9);
            let p = JVec3::new(1.0, 2.0, 3.0);
            assert!(close(a.transform_position(p), b.transform_position(p)));
        }

        #[test]
        fn set3x3_and_transpose() {
            let m3 = JMat3::from_cols(JVec3::new(0.0, 1.0, 0.0), JVec3::new(-1.0, 0.0, 0.0), JVec3::new(0.0, 0.0, 1.0));
            let m = JMat4::identity().set3x3(m3);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
            assert!(close(m.transpose3x3().transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(1.0, 0.0, 0.0)));
        }
    }


    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.

    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }

    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }

    type JCaps = Vec<Option<(usize, usize)>>;

    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }

    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }

    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }

    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }

    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }

        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }

        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }

        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }

        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }

        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }

        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }

        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }

        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }

        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }

        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }

        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }

        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }

        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }

    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }

    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }

    #[cfg(test)]
    mod jregex_tests {
        use super::*;

        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }

        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }

        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "xaeroworldmap");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
