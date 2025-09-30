#!/usr/bin/env bash
set -euo pipefail
APP_DIR="/opt/reptilesim"
BIN="$APP_DIR/ReptileSim-riscv64"
PCK="$APP_DIR/ReptileSim.pck"

if [[ -x "$BIN" ]]; then
  exec "$BIN" --rendering-driver Vulkan -- --path "$APP_DIR" --fullscreen
else
  exec godot4 --path "$APP_DIR" --fullscreen
fi
