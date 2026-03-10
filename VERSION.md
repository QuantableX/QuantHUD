# Release Workflow

To publish a new release, first sync the version across all config files (package.json, Cargo.toml, tauri.conf.json) by running `npm run version:sync X.Y.Z`. Then commit the version bump, tag it, and push with tags. The CI will automatically build for Windows, macOS (universal binary), and Linux, create a GitHub Release with the version tag, and attach all installers. Users will see the update via the in-app "Check for Updates" button in Settings, and can download and install it instantly without leaving the app.

```bash
npm run version:sync
npm run version:sync 1.0.0
git add -A
git commit -m "release: v1.0.0"
git tag v1.0.0
git push origin main --tags
```
