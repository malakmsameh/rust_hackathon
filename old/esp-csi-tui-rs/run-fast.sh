#!/bin/bash
# Fast optimized run (release build, skip checks)
cd "$(dirname "$0")" || exit 1
sudo ./target/release/esp-csi-tui-rs
