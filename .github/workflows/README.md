# GitHub Actions Build Workflow

This workflow automatically builds QuantHUD for **Windows**, **macOS**, and **Linux**.

## 🚀 How does it work?

The build is automatically triggered on:

- **Push** to `main` or `master` branch
- **Pull Requests** to `main` or `master`
- **Tags** starting with `v` (e.g., `v1.0.0`)
- **Manually** via GitHub Actions UI

## 📦 Build Outputs

After a successful build, you can find the files under **Actions** → **Build QuantHUD** → **Artifacts**:

### Windows

- `QuantHUD-Windows.zip` contains:
  - `QuantHUD_1.0.0_x64-setup.exe` (NSIS Installer)
  - `quanthud.exe` (Standalone)

### macOS

- `QuantHUD-macOS.zip` contains:
  - `QuantHUD_1.0.0_x64.dmg` (DMG Installer)
  - `QuantHUD.app` (App Bundle)

### Linux

- `QuantHUD-Linux.zip` contains:
  - `quanthud_1.0.0_amd64.deb` (Debian/Ubuntu)
  - `quanthud_1.0.0_amd64.AppImage` (Universal)
  - `quanthud` (Binary)

## 🎯 Usage

### 1. Push Code

```bash
git add .
git commit -m "Update"
git push
```

### 2. Download Artifacts

1. Go to GitHub → **Actions** tab
2. Click on the latest **Build QuantHUD** workflow
3. Scroll down to **Artifacts**
4. Download the desired platform

### 3. Create Release (Optional)

For an official release with tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

## ⚙️ Manual Builds

You can also start a build manually:

1. Go to **Actions** tab
2. Select **Build QuantHUD** workflow
3. Click **Run workflow**
4. Choose the branch and click **Run workflow**

## 🔧 Troubleshooting

### Build fails?

- Check the logs in the Actions tab
- Make sure `npm run tauri:build` works locally
- Check if all dependencies in `package.json` and `Cargo.toml` are correct

### Artifacts not found?

- Wait until the build is completely finished (✅ green checkmark)
- Artifacts are only available for 90 days
- Create a release for permanent downloads
