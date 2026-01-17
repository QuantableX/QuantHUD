# Testing Guide - Alle Plattformen testen ohne Mac/Linux

## 🎯 Ziel
Du willst **Windows, macOS und Linux** Builds testen, hast aber nur **Windows**.

---

## ✅ Methode 1: Automatisches Artifact-Testing (EMPFOHLEN)

### Voraussetzung: GitHub CLI installieren

```powershell
# Mit winget (Windows 10/11)
winget install GitHub.cli

# Oder von https://cli.github.com/ herunterladen
```

### Artifacts herunterladen und testen

```powershell
# Im Projekt-Ordner
cd C:\projects\QuantView\QuantTools\QuantCalc

# Script ausführen
.\scripts\download-and-test-artifacts.ps1
```

**Das Script:**
- ✅ Lädt automatisch die neuesten Artifacts herunter
- ✅ Zeigt Dateigrößen und Pfade
- ✅ Öffnet den Ordner mit allen Builds
- ✅ Du kannst Windows-Installer direkt testen

---

## 🧪 Methode 2: GitHub Actions Logs prüfen

Die **Test-Steps** im Workflow zeigen dir automatisch:

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
Name: quantcalc.exe
Length: 15.2 MB
```

**Wo findest du das?**
1. GitHub → Actions Tab
2. Klick auf neuesten Workflow
3. Klick auf "macOS" / "Linux" / "Windows" Job
4. Scrolle zu "Test binary" Step

---

## 🖥️ Methode 3: Virtual Machines (Fortgeschritten)

### Linux testen (Einfach!)

**Option A: WSL2 (Windows Subsystem for Linux)**
```powershell
# WSL2 installieren
wsl --install

# Ubuntu starten
wsl

# In WSL: .deb installieren
sudo dpkg -i QuantCalc_1.0.0_amd64.deb

# App starten (braucht X Server auf Windows)
quantcalc
```

**Option B: VirtualBox + Ubuntu**
1. VirtualBox installieren (kostenlos)
2. Ubuntu ISO herunterladen
3. VM erstellen
4. `.deb` in VM installieren

### macOS testen (Schwierig!)

**Option A: Cloud Mac mieten (BESTE LÖSUNG)**
- **MacinCloud**: $30/Monat oder $1/Stunde
  - https://www.macincloud.com/
  - Sofort verfügbar
  - Echter Mac, remote zugreifen
  
- **AWS EC2 Mac**: ~$1/Stunde
  - https://aws.amazon.com/ec2/instance-types/mac/
  - Braucht AWS Account
  
- **MacStadium**: Ab $50/Monat
  - https://www.macstadium.com/

**Option B: Gebrauchten Mac kaufen**
- Mac Mini (2014-2018): ~200-400€ auf eBay
- Nur zum Testen nutzen
- Beste Langzeit-Lösung

**Option C: Freund/Kollege mit Mac** (was du jetzt machst)
- ✅ Kostenlos
- ❌ Nicht immer verfügbar

---

## 📊 Methode 4: Automatische Tests schreiben

### Unit Tests (für Logik)
```bash
# Tests ausführen
npm test
```

### E2E Tests (für UI)
Könnte man mit Playwright/Cypress machen, aber:
- ❌ Komplex für Desktop-Apps
- ❌ Braucht trotzdem echte Plattformen
- ⚠️ Nur sinnvoll für große Projekte

---

## 🎯 Empfohlener Workflow

### Für dich (Windows-Entwickler):

1. **Entwickeln auf Windows**
   ```bash
   npm run tauri:dev
   ```

2. **Lokale Windows-Tests**
   ```bash
   npm run tauri:build
   # Installer testen: src-tauri/target/release/bundle/nsis/*.exe
   ```

3. **Push zu GitHub**
   ```bash
   git push
   ```

4. **Artifacts automatisch testen**
   ```powershell
   .\scripts\download-and-test-artifacts.ps1
   ```

5. **GitHub Actions Logs prüfen**
   - macOS Test-Output ansehen
   - Linux Test-Output ansehen

6. **Finale Tests (1x pro Release)**
   - Freund mit Mac testen lassen
   - Oder Cloud Mac für 1 Stunde mieten ($1)

---

## 💰 Kosten-Vergleich

| **Methode** | **Kosten** | **Aufwand** | **Qualität** |
|-------------|-----------|-------------|--------------|
| GitHub Actions Logs | Kostenlos | Niedrig | Mittel |
| Artifact Download | Kostenlos | Niedrig | Mittel |
| WSL2 (Linux) | Kostenlos | Mittel | Hoch |
| Cloud Mac (1h) | $1 | Niedrig | Hoch |
| Cloud Mac (Monat) | $30 | Niedrig | Hoch |
| Gebrauchter Mac | €300 | Hoch | Sehr Hoch |
| Freund mit Mac | Kostenlos | Mittel | Hoch |

---

## ✅ Meine Empfehlung für dich:

**Während Entwicklung:**
1. ✅ Artifact-Download Script nutzen
2. ✅ GitHub Actions Logs prüfen
3. ✅ Windows lokal testen

**Vor Release:**
1. ✅ Cloud Mac für 1 Stunde mieten ($1)
2. ✅ Alle Plattformen final testen
3. ✅ Oder Freund mit Mac

**Langfristig (wenn Projekt wächst):**
- Gebrauchten Mac Mini kaufen (~€300)
- Oder Cloud Mac Abo ($30/Monat)

---

## 🚀 Quick Start

```powershell
# 1. GitHub CLI installieren
winget install GitHub.cli

# 2. Einloggen
gh auth login

# 3. Artifacts testen
cd C:\projects\QuantView\QuantTools\QuantCalc
.\scripts\download-and-test-artifacts.ps1
```

**Fertig!** Du siehst jetzt alle Builds und kannst Windows direkt testen.

