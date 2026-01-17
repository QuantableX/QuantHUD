# macOS Installation Troubleshooting

## ❌ Error: "Application is not supported on this Mac"

### Possible Causes:

#### 1. **macOS Version Too Old**

**Minimum Requirement:** macOS 10.13 (High Sierra) or newer

**Check:**

```bash
sw_vers
```

**Solution:**

- Update macOS to at least 10.13
- Or use a newer Mac

---

#### 2. **Architecture Issue (Intel vs. Apple Silicon)**

**Check:**

```bash
uname -m
```

**Result:**

- `x86_64` = Intel Mac
- `arm64` = Apple Silicon (M1/M2/M3/M4)

**Solution for Apple Silicon:**
Install Rosetta 2 (for Intel apps):

```bash
softwareupdate --install-rosetta
```

---

#### 3. **Gatekeeper Blocking the App**

**Symptom:** "App can't be opened because it is from an unidentified developer"

**Solution:**

1. **Right-click** on `QuantCalc.app`
2. Select **"Open"**
3. Confirm **"Open"** again in the dialog

**Alternative (Terminal):**

```bash
xattr -cr /Applications/QuantCalc.app
```

---

#### 4. **App is Damaged**

**Symptom:** "App is damaged and can't be opened"

**Solution:**

```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /Applications/QuantCalc.app

# Or remove all attributes
xattr -cr /Applications/QuantCalc.app
```

---

## 🔍 Run Diagnostic Script

Download and run the diagnostic script:

```bash
# Make script executable
chmod +x scripts/check-macos-compatibility.sh

# Run script
./scripts/check-macos-compatibility.sh
```

**Send the output to the developer!**

---

## 📋 Manual Check

### Collect System Information:

```bash
# macOS Version
sw_vers

# CPU Architecture
uname -m

# Gatekeeper Status
spctl --status

# App Information
file /Applications/QuantCalc.app/Contents/MacOS/QuantCalc

# Check App Attributes
xattr -l /Applications/QuantCalc.app
```

---

## ✅ Successful Installation

After successful installation, the app should:

1. Be in the **Applications folder**
2. Show a **security warning** on first launch (normal!)
3. Start **normally** after confirmation

---

## 🆘 Need More Help?

If nothing works, send this information:

```bash
# Write all info to a file
{
  echo "=== System Info ==="
  sw_vers
  echo ""
  echo "=== Architecture ==="
  uname -m
  echo ""
  echo "=== App File Info ==="
  file /Applications/QuantCalc.app/Contents/MacOS/QuantCalc
  echo ""
  echo "=== App Attributes ==="
  xattr -l /Applications/QuantCalc.app
  echo ""
  echo "=== Gatekeeper ==="
  spctl --status
} > ~/Desktop/quantcalc-debug.txt

echo "Debug info saved: ~/Desktop/quantcalc-debug.txt"
```

Then send the `quantcalc-debug.txt` file from your Desktop.
