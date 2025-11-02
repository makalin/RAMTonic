# RAMTonic 🧪

[](https://choosealicense.com/licenses/mit/)
[](http://makeapullrequest.com)
[](https://www.google.com/search?q=https://github.com/your-username/RAMTonic)
[](https://www.google.com/search?q=https://github.com/your-username/RAMTonic)

**A cross-platform desktop utility to monitor and manage memory usage for all major web browsers.**

RAMTonic provides a unified dashboard to see exactly how much memory your browsers are consuming, down to the individual tab and extension, and gives you the tools to refresh and reclaim that memory instantly.

-----

## ✨ Features

  - **Unified Dashboard:** View memory usage for all installed browsers in a single, clean interface.
  - **Multi-Browser Support:** Natively supports Google Chrome, Mozilla Firefox, Microsoft Edge, Apple Safari, Brave, and Opera.
  - **Detailed Process View:** Lists all browser-related processes (main process, GPU process, tabs, extensions) with their respective memory ($RAM$) and CPU usage.
  - **Intelligent "Free Memory" Actions:**
      - **Clear Memory:** Safely triggers low-memory notifications to encourage processes to release unused memory gracefully.
      - **Terminate:** Force-close specific, non-responsive, or bloated tabs/processes.
      - **Hibernate Tabs:** (Coming in Phase 2) Unloads memory-intensive tabs without closing them, allowing for one-click restoration.
  - **Advanced Filtering & Search:**
      - Real-time search across process names, types, and browsers
      - Filter by process type (Main Process, Tab, Extension, GPU/Renderer, Utility)
      - Filter by memory range (min/max MB)
      - Combined filters for precise process discovery
  - **Process Sorting:**
      - Sort by memory usage (default)
      - Sort by CPU usage
      - Sort by name or process type
      - Ascending/descending order toggle
  - **Process Statistics:**
      - Total, average, and maximum memory usage
      - CPU usage statistics
      - Breakdown by process type
      - Real-time updates
  - **Data Export:**
      - Export process data to JSON format
      - Export process data to CSV format
      - Save reports for analysis
  - **Settings & Preferences:**
      - Configurable auto-refresh interval
      - Memory usage alert thresholds
      - Default sort preferences
      - UI customization options
      - Persistent settings storage
  - **Memory Alerts:**
      - Configurable memory usage thresholds
      - Automatic alert notifications
      - Helps prevent system memory exhaustion
  - **Auto-Refresh:**
      - Configurable refresh interval
      - Real-time monitoring
      - Optional automatic updates
  - **Cross-Platform:** A single codebase for a native experience on both Windows and macOS.
  - **Lightweight & Efficient:** Built with Rust and Tauri for minimal performance impact on your system.

## 🛠️ Technology Stack

This project uses a modern, cross-platform, and efficient technology stack.

  - **Core Logic:** **Rust**
      - Chosen for its performance, memory safety, and excellent cross-platform compilation capabilities.
      - **Key Crates:**
        - `sysinfo` (v0.29): Gather process and system information efficiently
        - `serde` & `serde_json`: Serialization for data exchange
        - `dirs`: Cross-platform configuration directory access
        - `winapi`: Windows-specific APIs for memory management (Windows only)
  - **Desktop UI Framework:** **Tauri v1.5**
      - Builds on top of the Rust backend, using web technologies (HTML, CSS, JavaScript) for the frontend UI.
      - Creates lightweight, secure, and native-feel applications without the overhead of shipping a full browser engine like Electron.
      - Smaller bundle size and better performance than Electron.
  - **Frontend Framework:** **Svelte 4**
      - Modern, lightweight JavaScript framework for reactive and responsive user interface.
      - Component-based architecture for maintainable code.
      - Compiles to vanilla JavaScript for optimal performance.
  - **Build Tools:**
      - **Vite:** Fast build tool and dev server
      - **TypeScript:** Type-safe JavaScript development
      - **npm:** Package management
  - **Browser Integration (Optional but Recommended):**
      - A companion **WebExtension** (JavaScript) will be developed to communicate with the desktop app. This is crucial for getting detailed tab information (Title, URL) and for advanced actions like "hibernating" specific tabs gracefully.

## 🚀 Getting Started & Installation

This section is for developers who want to contribute or build the project from source.

### Prerequisites

  - **Rust & Cargo:** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
  - **Node.js & npm:** [https://nodejs.org/en/download/](https://nodejs.org/en/download/)
  - **Tauri Prerequisites:** Follow the setup guide for your OS: [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Building from Source

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/makalin/RAMTonic.git
    cd RAMTonic
    ```

2.  **Install frontend dependencies:**

    ```bash
    npm install
    ```

3.  **Install Rust dependencies (automatic on first build):**

    The Rust dependencies will be automatically downloaded when you run `cargo tauri dev` or `cargo tauri build`.

4.  **Run in development mode:**

    ```bash
    cargo tauri dev
    ```

    This will:
    - Start the Vite dev server on `http://localhost:5173`
    - Compile the Rust backend
    - Launch the Tauri application window
    - Enable hot-reload for frontend changes

5.  **Build the application for production:**

    ```bash
    cargo tauri build
    ```

    This will:
    - Create optimized production builds
    - Bundle the application for your platform
    - Generate installers (`.dmg` for macOS, `.msi` for Windows)
    - Output will be in `src-tauri/target/release/bundle/`

## 📖 How to Use (End-User Guide)

### Basic Usage

1.  **Launch RAMTonic.**
2.  The main window will automatically detect and display all running browsers in the left sidebar.
3.  **Select a browser** from the list to see its detailed process breakdown.
4.  The process list shows each tab, extension, and utility process with their memory and CPU usage.

### Viewing and Filtering Processes

- **Search:** Use the search bar to find processes by name, type, or browser.
- **Filter by Type:** Select a process type (Main Process, Tab, Extension, GPU/Renderer, Utility) from the dropdown.
- **Filter by Memory:** Set minimum and maximum memory values (in MB) to narrow down processes.
- **Sort:** Use the toolbar to sort processes by memory, CPU, name, or type.
- **Statistics:** View detailed statistics for the selected browser, including totals and averages.

### Managing Processes

**Select Processes:**
- Click checkboxes to select individual processes
- Use "Select All" to select all visible processes
- Use "Deselect All" to clear selections

**Actions:**
- **🔄 Refresh:** Manually rescan all processes for the latest memory usage
- **▶ Start Auto-Refresh:** Enable automatic updates at configurable intervals
- **Clear Selected:** Triggers graceful memory cleanup via low-memory notifications (safe method)
- **Terminate Selected:** Force-closes the selected processes (use with caution)

### Exporting Data

1. Click the **📥 JSON** or **📥 CSV** button in the toolbar
2. Choose a location to save the export file
3. Your process data will be exported in the selected format

### Settings

1. Click the **⚙️** (Settings) button in the toolbar
2. Configure:
   - **Auto-Refresh Interval:** Set how often the app automatically refreshes (default: 2000ms)
   - **Memory Alert Threshold:** Set the percentage at which memory alerts trigger (default: 85%)
   - **Default Sort:** Choose default sorting column and order
   - **UI Options:** Toggle system stats display and refresh on startup
3. Click "Save Settings" to persist your preferences

### Memory Alerts

- RAMTonic monitors system memory usage
- When memory usage exceeds your configured threshold, an alert will be displayed
- Adjust the threshold in Settings to customize alert sensitivity

## 🔬 Technical Approach: Memory Management

"Freeing" memory from an external process is complex and must be handled carefully to avoid crashing the browser. RAMTonic uses a multi-pronged approach:

1.  **Process Discovery:** ✅ **Implemented**

      - Uses the `sysinfo` Rust crate for cross-platform process discovery.
      - Automatically detects browser processes by name patterns (chrome, firefox, msedge, safari, brave, opera).
      - Identifies process types: Main Process, Tab, Extension, GPU/Renderer, Utility.
      - Works seamlessly on both Windows and macOS.

2.  **Memory Measurement:** ✅ **Implemented**

      - The `sysinfo` crate provides a cross-platform way to get real memory usage (Working Set / Resident Set Size) for each process ID.
      - Displays memory in MB and GB with proper formatting.
      - Tracks CPU usage percentage for each process.

3.  **Memory Clearing Actions:**

      - **Graceful Cleanup (Low Memory Notification):** ✅ **Implemented**
          - On **Windows**, uses the `CreateMemoryResourceNotification` Win32 API to signal low-memory state, encouraging applications to trim memory usage.
          - On **macOS**, uses `memory_pressure` command and system notifications to trigger memory pressure events.
          - This is the safest method and is available through the "Clear Selected" button.
      - **Process Termination:** ✅ **Implemented**
          - Uses standard OS calls to terminate processes by PID.
          - On Unix systems: `kill -9 <pid>`
          - On Windows: `taskkill /F /PID <pid>`
          - Use with caution as this may cause data loss in affected tabs/processes.
      - **Targeted Hibernation (The Extension Bridge):** 🔜 **Coming in Phase 2**
          - For precise tab-level control, a browser extension is planned.
          - The desktop app will run a local WebSocket server.
          - The browser extension will connect to this server.
          - When the user clicks "Hibernate" in the desktop app, it sends a message to the extension.
          - The extension uses browser APIs (`chrome.tabs.discard()`) to perform the action.
          - This will be the most powerful and user-friendly approach for tab-level control.

## 🗺️ Project Roadmap

  - [x] **Phase 1: Core Functionality** ✅ **COMPLETED**
      - [x] Implement process discovery and memory reading for Windows & macOS.
      - [x] Build the basic Tauri UI to display the process list.
      - [x] Implement the "Refresh" and "Terminate Process" actions.
      - [x] Implement graceful memory cleanup with low-memory notifications.
      - [x] Add process filtering and search functionality.
      - [x] Implement process sorting (memory, CPU, name, type).
      - [x] Add process statistics and grouping.
      - [x] Implement data export (JSON/CSV).
      - [x] Create settings/preferences system.
      - [x] Add memory usage alerts and thresholds.
  - [ ] **Phase 2: Advanced Memory Actions & Browser Extension**
      - [ ] Develop the WebSocket bridge between the Rust backend and the extension.
      - [ ] Create the companion browser extension for Chrome/Firefox.
      - [ ] Implement the "Hibernate Tab" feature.
      - [ ] Add tab URL and title display in process list.
      - [ ] Implement one-click tab restoration after hibernation.
  - [ ] **Phase 3: Polish & Release**
      - [x] UI/UX improvements (sorting, filtering, summary stats). ✅
      - [ ] Add an application icon and branding.
      - [ ] Create installers for Windows (`.msi`) and macOS (`.dmg`).
      - [ ] Set up automated builds using GitHub Actions.
      - [ ] Add comprehensive error handling and user feedback.
      - [ ] Performance optimization and testing.
  - [ ] **Phase 4: Future Goals**
      - [ ] Historical memory usage graphs and charts.
      - [ ] Custom browser path configuration.
      - [ ] Automatic hibernation of inactive tabs based on user rules.
      - [ ] Process monitoring alerts and notifications.
      - [ ] Dark mode theme support.
      - [ ] Multi-language support.

## 🙌 How to Contribute

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

Please see the `CONTRIBUTING.md` file for details on our code of conduct and the process for submitting pull requests to us.

## 📝 Recent Updates

### Version 0.1.0 (Current)

- ✅ Core process discovery and monitoring
- ✅ Multi-browser support (Chrome, Firefox, Edge, Safari, Brave, Opera)
- ✅ Memory and CPU usage tracking
- ✅ Process filtering and search
- ✅ Advanced sorting options
- ✅ Process statistics and grouping
- ✅ Data export (JSON/CSV)
- ✅ Settings and preferences
- ✅ Memory usage alerts
- ✅ Graceful memory cleanup
- ✅ Process termination
- ✅ Auto-refresh functionality

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Author

Mehmet T. AKALIN
