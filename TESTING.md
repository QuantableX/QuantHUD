# Testing Guide - Test All Platforms Without Mac/Linux

## 🎯 Goal

You want to test **Windows, macOS, and Linux** builds, but only have **Windows**.

---

## ✅ Method 1: Automated Artifact Testing (RECOMMENDED)

### Prerequisite: Install GitHub CLI

```powershell
# With winget (Windows 10/11)
winget install GitHub.cli

# Or download from https://cli.github.com/
```

### Download and Test Artifacts

```powershell
# In project folder
cd C:\projects\QuantView\QuantTools\QuantHUD

# Run script
.\scripts\download-and-test-artifacts.ps1
```

**The script:**

- ✅ Automatically downloads the latest artifacts
- ✅ Shows file sizes and paths
- ✅ Opens the folder with all builds
- ✅ You can test the Windows installer directly

---

## 🧪 Method 2: Check GitHub Actions Logs

The **test steps** in the workflow automatically show you:

### macOS Binary Info:

```
Testing macOS binary...
file: Mach-O universal binary with 2 architectures
  - x86_64 (Intel)
  - arm64 (Apple Silicon)
```

### Linux Binary Info:

```
Testing Linux binary...
file: ELF 64-bit LSB executable
ldd: libwebkit2gtk-4.1.so.0 => found
```

### Windows Binary Info:

```
Testing Windows binary...
Name: quanthub.exe
Length: 15.2 MB
```

**Where to find this?**

1. GitHub → Actions Tab
2. Click on latest workflow
3. Click on "macOS" / "Linux" / "Windows" job
4. Scroll to "Test binary" step

---

## 🖥️ Method 3: Virtual Machines (Advanced)

### Testing Linux (Easy!)

**Option A: WSL2 (Windows Subsystem for Linux)**

```powershell
# Install WSL2
wsl --install

# Start Ubuntu
wsl

# In WSL: Install .deb
sudo dpkg -i QuantHUD_1.0.0_amd64.deb

# Start app (needs X Server on Windows)
quanthub
```

**Option B: VirtualBox + Ubuntu**

1. Install VirtualBox (free)
2. Download Ubuntu ISO
3. Create VM
4. Install `.deb` in VM

### Testing macOS (Difficult!)

**Option A: Rent Cloud Mac (BEST SOLUTION)**

- **MacinCloud**: $30/month or $1/hour
  - https://www.macincloud.com/
  - Available immediately
  - Real Mac, remote access
- **AWS EC2 Mac**: ~$1/hour
  - https://aws.amazon.com/ec2/instance-types/mac/
  - Requires AWS account
- **MacStadium**: From $50/month
  - https://www.macstadium.com/

**Option B: Buy Used Mac**

- Mac Mini (2014-2018): ~$200-400 on eBay
- Use only for testing
- Best long-term solution

**Option C: Friend/Colleague with Mac** (what you're doing now)

- ✅ Free
- ❌ Not always available

---

## 📊 Method 4: Write Automated Tests

### Unit Tests (for logic)

```bash
# Run tests
npm test
```

### E2E Tests (for UI)

Could be done with Playwright/Cypress, but:

- ❌ Complex for desktop apps
- ❌ Still needs real platforms
- ⚠️ Only worthwhile for large projects

---

## 🎯 Recommended Workflow

### For you (Windows developer):

1. **Develop on Windows**

   ```bash
   npm run tauri:dev
   ```

2. **Local Windows tests**

   ```bash
   npm run tauri:build
   # Test installer: src-tauri/target/release/bundle/nsis/*.exe
   ```

3. **Push to GitHub**

   ```bash
   git push
   ```

4. **Test artifacts automatically**

   ```powershell
   .\scripts\download-and-test-artifacts.ps1
   ```

5. **Check GitHub Actions logs**
   - View macOS test output
   - View Linux test output

6. **Final tests (once per release)**
   - Have friend with Mac test
   - Or rent Cloud Mac for 1 hour ($1)

---

## 💰 Cost Comparison

| **Method**          | **Cost** | **Effort** | **Quality** |
| ------------------- | -------- | ---------- | ----------- |
| GitHub Actions Logs | Free     | Low        | Medium      |
| Artifact Download   | Free     | Low        | Medium      |
| WSL2 (Linux)        | Free     | Medium     | High        |
| Cloud Mac (1h)      | $1       | Low        | High        |
| Cloud Mac (month)   | $30      | Low        | High        |
| Used Mac            | $300     | High       | Very High   |
| Friend with Mac     | Free     | Medium     | High        |

---

## ✅ My Recommendation for You:

**During Development:**

1. ✅ Use artifact download script
2. ✅ Check GitHub Actions logs
3. ✅ Test Windows locally

**Before Release:**

1. ✅ Rent Cloud Mac for 1 hour ($1)
2. ✅ Final test all platforms
3. ✅ Or have friend with Mac test

**Long-term (if project grows):**

- Buy used Mac Mini (~$300)
- Or Cloud Mac subscription ($30/month)

---

## 🚀 Quick Start

```powershell
# 1. Install GitHub CLI
winget install GitHub.cli

# 2. Login
gh auth login

# 3. Test artifacts
cd C:\projects\QuantView\QuantTools\QuantHUD
.\scripts\download-and-test-artifacts.ps1
```

**Done!** You can now see all builds and test Windows directly.
