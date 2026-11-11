<h1 align="center">Horizon Java Bridge</h1>

<p align="center"><b>Real Fabric mods, running inside <a href="https://github.com/horizon-rust/horizon">Horizon</a>, the Rust rewrite of Minecraft: Java Edition 26.3.</b></p>

<p align="center">
  <img src="https://img.shields.io/badge/Fabric-26.3-DBD0B4" alt="Fabric 26.3">
  <img src="https://img.shields.io/badge/status-beta-F0795A" alt="Beta">
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows-2A2470" alt="Linux and Windows">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-3B2F7E" alt="MIT license"></a>
</p>

<p align="center">
  <a href="#get-it"><b>Get it</b></a> ·
  <a href="#what-works"><b>What works</b></a> ·
  <a href="#how-it-works"><b>How it works</b></a> ·
  <a href="docs/java-bridge-beta.md"><b>Testers' guide</b></a> ·
  <a href="https://trello.com/b/2uhi4jkj"><b>Mod tracker</b></a>
</p>

> **Unofficial, opt-in, beta.** Made by [iaxivers](https://github.com/iaxivers), with Windows support by [martipops](https://github.com/martipops). Built on [Horizon](https://github.com/horizon-rust/horizon) by Ludovic, who has agreed to take it into the main repository; that work is underway. Not an official Minecraft product, nor approved by or associated with Mojang or Microsoft. No Minecraft code, assets or data are included: you need your own copy of Java Edition 26.3.

Horizon plays Minecraft in Rust. The bridge lets it play the **mods people already use**: the same Fabric 26.3 jars you'd put in a normal mods folder, unchanged. Horizon keeps running the game (world, mobs, redstone, physics, saving, multiplayer, all rendering); a Java VM inside Horizon runs only the mods' own code, where a mod changes something. Turned off, it's plain Horizon with no Java at all.

## Get it

For normal online play and first-time downloads from Mojang, use a Microsoft account that owns Minecraft: Java Edition. **Offline play is supported** when the Minecraft 26.3 game files are already present on this computer: enable offline play in Settings → Developer options (seven clicks on the sidebar's launcher version), enable Fabric mods for the instance, and press Play without signing in. Offline Fabric play also needs a local Java 25 JDK (including `javac`). The launcher can fetch the Fabric-enabled Horizon build from this project's GitHub releases without Microsoft sign-in; it will not fetch missing Minecraft files or a Java runtime from Mojang while offline.

1. **Build the offline-capable binaries:** open [Actions → Offline Fabric mods build](https://github.com/Creeperuuu/horizon-java-bridge/actions/workflows/offline-fabric-build.yml), run it on the `offline-play` branch, then download the Linux or Windows artifact. It contains the launcher and Fabric-enabled game binary.
2. **Start the launcher** from that artifact. Install a Horizon version and prepare it from your already-installed Minecraft 26.3 files. Do not choose a Mojang download while offline.
3. **Register the built game binary** with `horizon-launcher java-mods use-build <path-to-horizon-client>` (use `horizon-client.exe` on Windows). Enable offline play in Settings → Developer options, then turn on "Fabric mods" for the instance.
4. **Install a local Java 25 JDK** (including `javac`), add Fabric 26.3 jars in the **Java mods** tab, and press **Play**. Fabric's own libraries may be downloaded from Fabric's public servers; no Microsoft sign-in is needed for this offline launch.

Found a bug? Post it with a screenshot or video, your mod list and `logs/latest.log` (in your game folder). The [testers' guide](docs/java-bridge-beta.md) has every step.

<details>
<summary>Build from source instead</summary>

```sh
git clone -b java-bridge https://github.com/iaxivers/horizon-java-bridge && cd horizon-java-bridge
cargo build --profile fast -p horizon-client --features horizon-client/java-mods
cd launcher && cargo build --release    # Fedora needs libxkbcommon-x11-devel
horizon-launcher java-mods use-build ../target/fast/horizon-client
```

Then steps 2–4 above. Start a self-built launcher with `HORIZON_LAUNCHER_NO_SELF_UPDATE=1`.
</details>

## What works

Of the 60 most-downloaded Fabric 26.3 mods, **39 work**, 4 renderer replacements are left out on purpose (Horizon has its own renderer), 16 play with one feature missing and 1 stops the game ([the survey](docs/java-bridge-survey.md)). An 81-jar pack (166 mods) plays on a real PC.

| Area | Status |
|---|---|
| Loading | ✅ 81- to 107-jar packs start (up to 265 mods, 20,000+ recipes); content made once per set of jars, Java's classes cached between runs |
| Blocks, items, recipes, creative tabs | ✅ placed mod blocks show at once; a modpack's bundled resource packs are picked up |
| Machines and their screens | ✅ Farmer's Delight (pot, stove, cutting board, meals), Tom's Storage, Energized Power, CC: Tweaked, XNet, BuildCraft; Horizon's hoppers and droppers fill and empty mod blocks |
| Create | 🚧 motors, shafts, presses, mixers, saws, burners and depots draw and work; value boxes, the toolbox, goggles' overlays, the mechanical arm's points, chain conveyors; items thrown into depots, millstones, crushers and fans; lava with Create's fluids. Not yet: train stations reliably, Ponder, contraptions' collisions |
| Mods' fluids | ✅ drawn with their own textures, flow by the mod's own code |
| JEI, REI, Jade, AppleSkin, tooltips | ✅ JEI's and REI's lists, search and recipes, per player |
| Maps | ✅ Xaero's Minimap and World Map, XaeroPlus, JourneyMap, VoxelMap, MiniHUD |
| Mod mobs | ✅ Naturalist, Friends & Foes, Mutant Monsters, GeckoLib mobs; they fight Horizon's mobs and players both ways |
| Mod worldgen | ✅ Biomes O' Plenty, Terralith, Towns and Towers, Dungeons and Taverns, mods' own structure types and template overrides (The Sift); Tectonic's far terrain; skyblock and one-block worlds; mods' own dimensions (The Sift, Every Mining Dimension) |
| Mods that change vanilla behaviour (mixins) | ✅ Veinminer, Right Click Harvest, Carry On, Serene Seasons, Clumps |
| Player look | ✅ 3D Skin Layers |
| Multiplayer with mods | ✅ LAN and dedicated servers: each player's own maps, recipe viewers and mod settings |
| The launcher | ✅ a **Fabric 26.3 mods (beta)** switch per instance and a **Java mods** tab; offline launches use a local Java 25 JDK and pre-existing Minecraft files |
| Renderer replacements (Sodium, Iris) | ➖ not needed: Horizon has its own renderer, so they have nothing to replace |
| Known not working yet | 🐛 Flashback's recording, Chat Heads' heads, Litematica's overlay in the world; on LAN, the host's own REI list (each player running their mods' client, `HORIZON_JAVA_OWN_CLIENT=1`, is still opt-in). TACZ's guns fire and draw, still rough. Testers track mods on [their Trello](https://trello.com/b/2uhi4jkj) |
| Windows | ✅ builds and plays on Windows 11 (Windows support by [martipops](https://github.com/martipops)); macOS untested |


## How it works

- **Horizon owns the game.** Rust runs the world, ticks, mobs, physics, saving, networking and every frame.
- **Java runs only the mods.** A JVM in Horizon's process loads Fabric Loader, the mods and their mixins against the real 26.3 classes from your own install, and handles only what a mod changes; the result goes back into Horizon's world. Mods' screens and drawings are captured and drawn by Horizon's renderer.
- **Generic, no mod-specific code.** Nothing in the bridge targets one mod's internals; a test fails if a mod's package name appears in its code. Every bug fixed is fixed for every mod.
- **Off by default.** A cargo feature plus a per-instance launcher switch. Off means no Java, no downloads, no cost.

Details for developers: **[docs/java-bridge.md](docs/java-bridge.md)** (where everything is, the tick flow, 26.3 traps, the test tools). One-page overview: [docs/java-bridge-pitch.md](docs/java-bridge-pitch.md). Performance numbers: [docs/java-bridge-perf.md](docs/java-bridge-perf.md). Coming next: NeoForge 26.3 ([plan](docs/java-bridge-neoforge.md)) and Forge 1.12 / SkyFactory 4 on Cleanroom ([plan](docs/java-bridge-112.md)).

**Tests:** `python3 tools/java_bridge/regress.py --jars <jars>` plays every scene that once worked, headless, with screenshots.

---

# About Horizon

This project builds on [Horizon](https://github.com/horizon-rust/horizon) by Ludovic: Minecraft: Java Edition 26.3 rewritten in Rust, checked against the Java game by hundreds of differential harnesses. Everything that isn't the Java bridge is Horizon's own work; its docs, roadmap and status live in [its repository](https://github.com/horizon-rust/horizon) and on [playhorizon.dev](https://playhorizon.dev). For plain Horizon without mods, use [its own launcher](https://playhorizon.dev).

The bridge is being brought into Horizon's main repository: [design note (#10216)](https://github.com/horizon-rust/horizon/pull/10216).

## Licence

MIT, like Horizon ([LICENSE](LICENSE)). Mods keep their own licences and are never included: you bring the jars you downloaded.
