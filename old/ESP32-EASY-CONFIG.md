# ESP32-C3 Configuration - Made Easy!

## Quick Start (The Easy Way)

### Option 1: Use the Configuration Script (Recommended)

```bash
cd ~/Documents/42\ Amman/Hackathons/ConnectedMotion
./esp-config.sh
```

This opens an interactive menu where you can:
- Select "Connect to WiFi" → Enter SSID and password (no typing long commands!)
- Select "Generate Traffic" → Specify frequency
- Select "Start Collection" → Specify duration
- View config with one click

### Option 2: Quick Commands (Still Easy)

If you want to stay in the serial monitor but avoid typing, here are the commands:

**Connect to "42 Students" WiFi:**
```
set-wifi --mode=station --sta-ssid=42_Students --sta-password=12345678 --set-channel=6
```

**Start collecting CSI data:**
```
start --duration=120
```

**Generate some WiFi traffic first (so there's data to collect):**
```
set-traffic --frequency-hz=10
```

---

## The Easiest Way: Pre-Configured Quick-Connect

Create custom quick commands by adding aliases. Edit your `.bashrc` or `.zshrc`:

```bash
# Add these lines to ~/.zshrc
alias esp-connect='echo "set-wifi --mode=station --sta-ssid=42_Students --sta-password=12345678 --set-channel=6" | sudo tee /dev/ttyUSB0'
alias esp-start='echo "start --duration=120" | sudo tee /dev/ttyUSB0'
alias esp-traffic='echo "set-traffic --frequency-hz=10" | sudo tee /dev/ttyUSB0'
```

Then reload:
```bash
source ~/.zshrc
```

Now you can just type:
```bash
esp-connect
esp-traffic
esp-start
```

---

## Configuration Workflow (Step-by-Step)

### Setup WiFi + Collect Data (5 seconds of effort):

1. Run the script:
   ```
   ./esp-config.sh
   ```

2. Select option 1 (Connect to WiFi)
   - Enter: `42 Students`
   - Enter: `12345678`
   - Enter: `6`

3. Select option 3 (Generate Traffic)
   - Enter: `10` (packets per second)

4. Select option 4 (Start Collection)
   - Enter: `120` (seconds)

5. **Done!** Watch the CSI data stream in

---

## Common Configurations Pre-Built

### Config A: Sniffer Mode (Passive Monitoring)
```
./esp-config.sh
→ Select 2 (Start Sniffer)
→ Enter channel: 6
→ Select 4 (Start Collection)
```

### Config B: Active Station (Connected + CSI)
```
./esp-config.sh
→ Select 1 (Connect to WiFi)
→ SSID: 42_Students
→ Password: 12345678
→ Channel: 6
→ Select 3 (Generate Traffic)
→ Frequency: 10
→ Select 4 (Start Collection)
```

### Config C: Custom Mode
```
./esp-config.sh
→ Select 7 (Custom Command)
→ Type your command directly
```

---

## Troubleshooting

**Script says "Port not found"?**
```bash
ls /dev/ttyUSB*
# Then run with custom port:
./esp-config.sh /dev/ttyUSB0
```

**Need to switch to minicom?**
```bash
./esp-config.sh
→ Select 8 (Open serial monitor)
```

**Script not working?**
Try manual approach:
```bash
sudo /home/tuqa-saeed/.cargo/bin/espflash monitor --port /dev/ttyUSB0
```

---

## For Hackathon TUI Application

Once you build the **esp-csi-tui-rs** application, it will handle all this automatically!

But for now, use this script to:
1. Test configurations
2. Collect sample data
3. Verify device is working

---

## Next Steps

1. Use the script to connect and collect some CSI data
2. Watch the output to understand the data format
3. Start building your TUI with the DAY1_STARTER_CODE.md guide
