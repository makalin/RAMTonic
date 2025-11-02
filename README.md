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
      - **Clear:** Safely prompts the browser's garbage collector to release unused memory.
      - **Hibernate Tabs:** Unloads memory-intensive tabs without closing them, allowing for one-click restoration.
      - **Terminate:** Close specific, non-responsive, or bloated tabs/processes.
  - **Cross-Platform:** A single codebase for a native experience on both Windows and macOS.
  - **Lightweight & Efficient:** Built to have a minimal performance impact on your system while it runs.

## 🛠️ Technology Stack

This project aims for a modern, cross-platform, and efficient stack.

  - **Core Logic:** **Rust**
      - Chosen for its performance, memory safety, and excellent cross-platform compilation capabilities.
      - Crates like `sysinfo` will be used to gather process and system information efficiently.
  - **Desktop UI Framework:** **Tauri**
      - Builds on top of the Rust backend, using web technologies (HTML, CSS, JavaScript) for the frontend UI.
      - Creates lightweight, secure, and native-feel applications without the overhead of shipping a full browser engine like Electron.
  - **Frontend Framework:** **Svelte / React**
      - A modern JavaScript framework will be used to create a reactive and responsive user interface.
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

3.  **Run in development mode:**

    ```bash
    cargo tauri dev
    ```

4.  **Build the application:**

    ```bash
    cargo tauri build
    ```

## 📖 How to Use (End-User Guide)

1.  **Launch RAMTonic.**
2.  The main window will automatically detect and display all running browsers.
3.  **Select a browser** from the list on the left to see its detailed process breakdown.
4.  The list will show each tab, extension, and utility process, sorted by memory usage.
5.  **Select one or more processes** and use the action buttons:
      - **Refresh:** Rescans all processes for the latest memory usage.
      - **Clear Selected:** Attempts a "soft" memory release on the selected processes. This will ask the browser's garbage collector to run.
      - **Hibernate Selected:** (Requires extension) Unloads the selected tab(s) to free memory. The tab remains in your browser, ready to be reloaded with a click.
      - **Terminate Selected:** Force-closes the selected tab or process. Use this for unresponsive or buggy tabs.

## 🔬 Technical Approach: Memory Management

"Freeing" memory from an external process is complex and must be handled carefully to avoid crashing the browser. RAMTonic will use a multi-pronged approach:

1.  **Process Discovery:**

      - On **Windows**, it will use the `CreateToolhelp32Snapshot` Win32 API (via a Rust crate) to find processes with names like `chrome.exe`, `firefox.exe`, etc., and their child processes.
      - On **macOS**, it will parse the output of the `ps aux` command or use the native `libproc` library to identify browser processes by their bundle identifier (e.g., `com.google.Chrome`).

2.  **Memory Measurement:**

      - The `sysinfo` crate in Rust provides a cross-platform way to get the real memory usage (Working Set / Resident Set Size) for each process ID.

3.  **Memory Clearing Actions:**

      - **Graceful Cleanup (Low Memory Notification):** On Windows, we can use the `CreateMemoryResourceNotification` API to signal a low-memory state, which encourages applications to trim their memory usage. A similar approach can be investigated for macOS using its Grand Central Dispatch (GCD) memory pressure events. This is the "safest" method.
      - **Targeted Hibernation/Termination (The Extension Bridge):** For precise control (e.g., "hibernate tab X"), a browser extension is necessary.
          - The desktop app will run a local WebSocket server.
          - The browser extension will connect to this server.
          - When the user clicks "Hibernate" in the desktop app, it sends a message (`{ "action": "hibernate", "tabId": 123 }`) to the extension.
          - The extension uses browser APIs (`chrome.tabs.discard(123)`) to perform the action.
          - This is the most powerful and user-friendly approach for tab-level control.
      - **Process Termination (The Brute-Force Method):** As a last resort, the app will use standard OS calls to terminate a specific Process ID (PID). This is equivalent to using Task Manager or Activity Monitor and may cause data loss in the targeted tab.

## 🗺️ Project Roadmap

  - [ ] **Phase 1: Core Functionality**
      - [ ] Implement process discovery and memory reading for Windows & macOS.
      - [ ] Build the basic Tauri UI to display the process list.
      - [ ] Implement the "Refresh" and "Terminate Process" actions.
  - [ ] **Phase 2: Advanced Memory Actions & Browser Extension**
      - [ ] Develop the WebSocket bridge between the Rust backend and the extension.
      - [ ] Create the companion browser extension for Chrome/Firefox.
      - [ ] Implement the "Hibernate Tab" feature.
      - [ ] Research and implement the "Graceful Cleanup" (low-memory notification) feature.
  - [ ] **Phase 3: Polish & Release**
      - [ ] UI/UX improvements (sorting, filtering, summary stats).
      - [ ] Add an application icon and branding.
      - [ ] Create installers for Windows (`.msi`) and macOS (`.dmg`).
      - [ ] Set up automated builds using GitHub Actions.
  - [ ] **Phase 4: Future Goals**
      - [ ] Historical memory usage graphs.
      - [ ] Configurable settings (e.g., refresh interval, custom browser paths).
      - [ ] Automatic hibernation of inactive tabs based on user rules.

## 🙌 How to Contribute

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

Please see the `CONTRIBUTING.md` file for details on our code of conduct and the process for submitting pull requests to us.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Author

Mehmet T. AKALIN
