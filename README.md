<p align="center"><img width="500" alt="image" src="https://github.com/user-attachments/assets/de5e6599-e4bb-48e0-857c-c861ecef3ded" /></p>
  
<h1 align="center">PostCrab</h1>
<p align="center">A simple and fast REST client built with Rust, Relm4, GTK4, and GtkSourceView5.</p>

</br>

## Screenshots
<img width="1233" height="936" alt="image" src="https://github.com/user-attachments/assets/c16fadff-c96c-425b-9d25-b71c23b1926e\" />
<img width="1233" height="936" alt="image" src="https://github.com/user-attachments/assets/ea6305e4-19d2-445b-852c-8a22b95217dc\" />

---

## Releases & Downloads

Self-contained packages are automatically built and published on the [GitHub Releases](https://github.com/abelosbert06/PostCrab/releases) page for each new release tag (`v*`):

| Platform | Format | Description |
|---|---|---|
| **macOS** | `.dmg` | Mountable Apple Silicon disk image containing `PostCrab.app` (drag-and-drop to `/Applications`) |
| **Linux** | `.AppImage` | Standalone executable AppImage bundled with desktop integration, icons, and GLib schemas |
| **Windows** | `.exe` | Self-contained Windows installer bundled with all required GTK4 runtime DLLs and schemas |

> [!NOTE]
> **macOS Gatekeeper:** Because PostCrab is an open-source project without a paid Apple Developer ID certificate, macOS Gatekeeper may flag the downloaded application when opened directly from the web. If you see a prompt indicating the app cannot be opened or is damaged:
> - Either right-click (Control-click) `PostCrab.app` and select **Open**; or
> - Navigate to **System Settings > Privacy & Security**, scroll down to **Security**, and click **Open Anyway**; or
> - Run the following command in Terminal to clear the download quarantine attribute:
>   ```bash
>   xattr -cr /Applications/PostCrab.app
>   ```

---

## Cross-Platform Setup & Build Instructions

### macOS (Apple Silicon & Intel)

1. **Install Homebrew dependencies:**
   ```bash
   brew install gtk4 gtksourceview5 adwaita-icon-theme pkg-config
   ```

2. **Ensure `PKG_CONFIG_PATH` is configured** (usually automatic on Homebrew, but export if using custom shells/IDEs):
   ```bash
   # Apple Silicon (M1/M2/M3/M4)
   export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:/opt/homebrew/share/pkgconfig:$PKG_CONFIG_PATH"

   # Intel Mac
   export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/share/pkgconfig:$PKG_CONFIG_PATH"
   ```

3. **Build and run:**
   ```bash
   cargo build --release
   ./target/release/postcrab-r4
   ```

---

### Windows

GTK4 applications on Windows require GTK4 and GtkSourceView5 C libraries. The recommended workflow is via **MSYS2 (UCRT64)**:

1. **Install MSYS2:**
   Download and install from [msys2.org](https://www.msys2.org/).

2. **Install GTK4 and GtkSourceView5 in the UCRT64 shell:**
   Launch the **MSYS2 UCRT64** terminal and run:
   ```bash
   pacman -S mingw-w64-ucrt-x86_64-gtk4 \
             mingw-w64-ucrt-x86_64-gtksourceview5 \
             mingw-w64-ucrt-x86_64-pkg-config \
             mingw-w64-ucrt-x86_64-gcc
   ```

3. **Set up Rust for UCRT64:**
   Install Rust toolchain with the GNU target:
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

4. **Build:**
   Add `C:\msys64\ucrt64\bin` to your `PATH` and set `PKG_CONFIG_PATH`:
   ```cmd
   set PATH=C:\msys64\ucrt64\bin;%PATH%
   set PKG_CONFIG_PATH=C:\msys64\ucrt64\lib\pkgconfig
   cargo build --release --target x86_64-pc-windows-gnu
   ```

5. **Packaging for Distribution:**
   Copy the required DLLs (`gtk-4-1.dll`, `libgtksourceview-5-0.dll`, etc.) and the `share/` directory (`share/glib-2.0/schemas/`, `share/gtksourceview-5/`) adjacent to the `.exe`.

---

### Linux

1. **Install dependencies:**
   - **Ubuntu/Debian:**
     ```bash
     sudo apt install libgtk-4-dev libgtksourceview-5-dev libadwaita-1-dev pkg-config
     ```
   - **Fedora/RHEL:**
     ```bash
     sudo dnf install gtk4-devel gtksourceview5-devel pkgconf
     ```
   - **Arch Linux:**
     ```bash
     sudo pacman -S gtk4 gtksourceview5 pkgconf
     ```

2. **Build and run:**
   ```bash
   cargo run --release
   ```

---

## Testing

Run the test suite with:
```bash
cargo test -- --nocapture
```
