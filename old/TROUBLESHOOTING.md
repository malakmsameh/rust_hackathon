# Troubleshooting: ESP32 Not Connecting to TUI

## Step 1: Check if Device is Responsive

First, verify the device is actually working:

```bash
sudo /home/tuqa-saeed/.cargo/bin/espflash monitor --port /dev/ttyUSB0
```

**What to look for:**
- You should see boot messages like `ESP-IDF v4.4.1`
- Device info and some startup text

**If you see nothing:**
- Device might be hung or need flashing
- Try: Hold BOOT button while plugging in USB

---

## Step 2: Reset the Device

If device seems unresponsive, reset it:

```bash
# Option A: Unplug USB cable, wait 5 seconds, plug back in

# Option B: Use espflash to reset (if it responds at all)
sudo /home/tuqa-saeed/.cargo/bin/espflash erase-flash --port /dev/ttyUSB0
```

Then wait 10 seconds for device to reboot.

---

## Step 3: Re-flash the Firmware

If reset doesn't help, re-flash the binary:

```bash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-cli-rs

# Flash the firmware
sudo /home/tuqa-saeed/.cargo/bin/espflash flash --port /dev/ttyUSB0 target/riscv32imc-unknown-none-elf/release/async_main

# Wait 30 seconds for flashing and boot
```

Then try the TUI again:

```bash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-tui-rs
./run.sh
```

---

## Step 4: Test with Direct Monitor

Don't use the TUI yet. Just test with espflash monitor:

```bash
sudo /home/tuqa-saeed/.cargo/bin/espflash monitor --port /dev/ttyUSB0

# Once in monitor, try sending a simple command:
help

# You should see device respond with help text
```

**If this works** → Device is fine, TUI may have an issue
**If this doesn't work** → Device needs flashing or reset

---

## Step 5: Once Device Works, Try TUI

Once you can see device output in espflash monitor:

```bash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-tui-rs
sudo cargo run
```

Press **h** to send help command and verify it works.

---

## Common Issues

### "Device always shows Connecting..."
- Usually means serial port can't be opened
- **Solution:** Make sure you're running with sudo

### "Device doesn't respond to commands"
- Device might be in wrong mode or hung
- **Solution:** Reset it (unplug/plug USB cable)

### "See espflash boot messages but TUI shows nothing"
- Device is fine, TUI might have timeout issue
- **Solution:** Try running directly: `sudo ./target/debug/esp-csi-tui-rs`

### "Can't even connect with espflash monitor"
- Device needs to be reflashed
- **Solution:** Follow "Re-flash the Firmware" section above

---

## Quick Checklist

- [ ] ESP32 plugged in via USB
- [ ] LED blinking or powered (check device)
- [ ] Device appears in `/dev/ttyUSB*` (check: `ls /dev/ttyUSB*`)
- [ ] espflash monitor shows boot messages (check: `sudo espflash monitor --port /dev/ttyUSB0`)
- [ ] Device responds to `help` command
- [ ] TUI runs with sudo (check: `sudo ./run.sh`)

If all above pass, TUI should connect!

---

## Nuclear Option: Complete Flash Reset

If nothing works, do a complete reset and reflash:

```bash
# 1. Erase entire flash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion/esp-csi-cli-rs
sudo /home/tuqa-saeed/.cargo/bin/espflash erase-flash --port /dev/ttyUSB0

# 2. Build release binary
cargo build --release --features=esp32c3 --target=riscv32imc-unknown-none-elf

# 3. Flash it
sudo /home/tuqa-saeed/.cargo/bin/espflash flash --port /dev/ttyUSB0 target/riscv32imc-unknown-none-elf/release/async_main

# 4. Wait 30 seconds
# 5. Test with monitor
sudo /home/tuqa-saeed/.cargo/bin/espflash monitor --port /dev/ttyUSB0
```

---

**Still stuck?** Check:
1. USB cable - try different port
2. Device drivers - might need esp32 drivers
3. Device might be bricked - try different USB hub
