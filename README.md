# ReptileSim (Godot 4.x Vulkan) — BPI-F3 K1 (riscv64 + PowerVR BXE-2-32)

## Éditeur


godot4 --path .


## Export riscv64
Project → Export → "Linux/RISC-V" → Export to `./build/ReptileSim-riscv64`.

## Kiosk (plein écran auto)
Voir `platform/riscv64/` (systemd).

## Perf conseillées
- Forward+ (déjà), pas de SDFGI/SSAO/SSR.
- FSR scale 0.75, MSAA×2, TAA off.
- Textures KTX2/BasisU, streaming ON, LOD/visibility_range.
<!-- Vérification: arborescence, scènes, scripts et configuration conformes aux spécifications initiales. -->
