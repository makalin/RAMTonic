// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browser;
mod process;
mod memory;
mod settings;

use browser::BrowserDetector;
use process::BrowserProcess;
use memory::trigger_low_memory_notification;
use settings::AppSettings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sysinfo::{System, SystemExt, ProcessExt, Pid};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProcessInfo {
    pid: u32,
    name: String,
    memory_mb: f64,
    cpu_percent: f32,
    browser: String,
    process_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BrowserInfo {
    name: String,
    total_memory_mb: f64,
    process_count: usize,
    processes: Vec<ProcessInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    browsers: Vec<BrowserInfo>,
    total_system_memory_mb: u64,
    used_system_memory_mb: u64,
}

#[tauri::command]
fn get_system_info() -> Result<SystemInfo, String> {
    let mut system = System::new_all();
    system.refresh_all();

    let browser_detector = BrowserDetector::new();
    let mut browser_map: HashMap<String, Vec<BrowserProcess>> = HashMap::new();

    // Scan all processes
    for (pid, process) in system.processes() {
        let process_name = process.name();
        if let Some(browser_info) = browser_detector.detect_browser(process_name) {
            let memory_bytes = process.memory();
            let memory_mb = memory_bytes as f64 / 1024.0 / 1024.0;
            let cpu_usage = process.cpu_usage();
            
            let browser_process = BrowserProcess::new(
                pid.as_u32(),
                process_name.to_string(),
                memory_mb,
                cpu_usage,
                browser_info.clone(),
                browser_detector.detect_process_type(process_name),
            );
            browser_map
                .entry(browser_info)
                .or_insert_with(Vec::new)
                .push(browser_process);
        }
    }

    // Convert to BrowserInfo structures
    let browsers: Vec<BrowserInfo> = browser_map
        .into_iter()
        .map(|(browser_name, processes)| {
            let total_memory: f64 = processes.iter().map(|p| p.memory_mb).sum();
            let process_infos: Vec<ProcessInfo> = processes
                .into_iter()
                .map(|p| ProcessInfo {
                    pid: p.pid,
                    name: p.name,
                    memory_mb: p.memory_mb,
                    cpu_percent: p.cpu_percent,
                    browser: p.browser.clone(),
                    process_type: p.process_type,
                })
                .collect();

            BrowserInfo {
                name: browser_name,
                total_memory_mb: total_memory,
                process_count: process_infos.len(),
                processes: process_infos,
            }
        })
        .collect();

    let total_system_memory = system.total_memory() / 1024 / 1024; // MB
    let used_system_memory = system.used_memory() / 1024 / 1024; // MB

    Ok(SystemInfo {
        browsers,
        total_system_memory_mb: total_system_memory,
        used_system_memory_mb: used_system_memory,
    })
}

#[tauri::command]
fn refresh_info() -> Result<SystemInfo, String> {
    get_system_info()
}

#[tauri::command]
fn terminate_process(pid: u32) -> Result<bool, String> {
    let mut system = System::new_all();
    system.refresh_all();

    // On Unix-like systems, we can kill the process
    #[cfg(unix)]
    {
        use std::process::Command;
        match Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    Ok(true)
                } else {
                    Err(format!("Failed to terminate process {}: {}", pid, 
                        String::from_utf8_lossy(&output.stderr)))
                }
            }
            Err(e) => Err(format!("Error executing kill command: {}", e))
        }
    }

    // On Windows
    #[cfg(windows)]
    {
        use std::process::Command;
        match Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    Ok(true)
                } else {
                    Err(format!("Failed to terminate process {}: {}", pid,
                        String::from_utf8_lossy(&output.stderr)))
                }
            }
            Err(e) => Err(format!("Error executing taskkill command: {}", e))
        }
    }
}

#[tauri::command]
fn terminate_processes(pids: Vec<u32>) -> Result<Vec<bool>, String> {
    let mut results = Vec::new();
    for pid in pids {
        match terminate_process(pid) {
            Ok(_) => results.push(true),
            Err(_) => results.push(false),
        }
    }
    Ok(results)
}

#[tauri::command]
fn clear_memory(pids: Vec<u32>) -> Result<Vec<bool>, String> {
    // Trigger low memory notification to encourage processes to free memory
    let _ = trigger_low_memory_notification();
    
    // Return success for all processes (the OS handles the notification)
    Ok(vec![true; pids.len()])
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcessStats {
    total_processes: usize,
    total_memory_mb: f64,
    average_memory_mb: f64,
    max_memory_mb: f64,
    total_cpu_percent: f32,
    by_type: HashMap<String, ProcessTypeStats>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProcessTypeStats {
    count: usize,
    total_memory_mb: f64,
    average_memory_mb: f64,
}

#[tauri::command]
fn get_process_stats(browser_name: Option<String>) -> Result<ProcessStats, String> {
    let system_info = get_system_info()?;
    
    let processes: Vec<&ProcessInfo> = if let Some(browser) = browser_name {
        system_info.browsers
            .iter()
            .find(|b| b.name == browser)
            .map(|b| b.processes.iter().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
            .collect()
    } else {
        system_info.browsers
            .iter()
            .flat_map(|b| b.processes.iter())
            .collect()
    };
    
    if processes.is_empty() {
        return Ok(ProcessStats {
            total_processes: 0,
            total_memory_mb: 0.0,
            average_memory_mb: 0.0,
            max_memory_mb: 0.0,
            total_cpu_percent: 0.0,
            by_type: HashMap::new(),
        });
    }
    
    let total_memory: f64 = processes.iter().map(|p| p.memory_mb).sum();
    let max_memory = processes.iter().map(|p| p.memory_mb).fold(0.0, f64::max);
    let total_cpu: f32 = processes.iter().map(|p| p.cpu_percent).sum();
    let count = processes.len();
    
    let mut by_type: HashMap<String, ProcessTypeStats> = HashMap::new();
    for process in &processes {
        let entry = by_type.entry(process.process_type.clone()).or_insert_with(|| ProcessTypeStats {
            count: 0,
            total_memory_mb: 0.0,
            average_memory_mb: 0.0,
        });
        entry.count += 1;
        entry.total_memory_mb += process.memory_mb;
    }
    
    for stats in by_type.values_mut() {
        if stats.count > 0 {
            stats.average_memory_mb = stats.total_memory_mb / stats.count as f64;
        }
    }
    
    Ok(ProcessStats {
        total_processes: count,
        total_memory_mb: total_memory,
        average_memory_mb: total_memory / count as f64,
        max_memory_mb: max_memory,
        total_cpu_percent: total_cpu,
        by_type,
    })
}

#[tauri::command]
fn export_data(format: String) -> Result<String, String> {
    let system_info = get_system_info()?;
    
    match format.as_str() {
        "json" => {
            serde_json::to_string_pretty(&system_info)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))
        }
        "csv" => {
            let mut csv = String::from("Browser,Process Name,Type,Memory (MB),CPU %,PID\n");
            for browser in &system_info.browsers {
                for process in &browser.processes {
                    csv.push_str(&format!(
                        "{},{},{},{:.2},{:.2},{}\n",
                        browser.name,
                        process.name,
                        process.process_type,
                        process.memory_mb,
                        process.cpu_percent,
                        process.pid
                    ));
                }
            }
            Ok(csv)
        }
        _ => Err("Unsupported format. Use 'json' or 'csv'".to_string())
    }
}

#[tauri::command]
fn get_settings() -> Result<AppSettings, String> {
    AppSettings::load()
}

#[tauri::command]
fn save_settings(settings: AppSettings) -> Result<bool, String> {
    settings.save()
}

#[tauri::command]
fn filter_processes(
    browser_name: Option<String>,
    process_type: Option<String>,
    min_memory_mb: Option<f64>,
    max_memory_mb: Option<f64>,
    search_query: Option<String>,
) -> Result<Vec<ProcessInfo>, String> {
    let system_info = get_system_info()?;
    
    let mut processes: Vec<ProcessInfo> = system_info.browsers
        .iter()
        .filter(|b| browser_name.is_none() || browser_name.as_ref().unwrap() == &b.name)
        .flat_map(|b| b.processes.clone())
        .collect();
    
    // Filter by process type
    if let Some(ptype) = process_type {
        processes.retain(|p| p.process_type == ptype);
    }
    
    // Filter by memory range
    if let Some(min) = min_memory_mb {
        processes.retain(|p| p.memory_mb >= min);
    }
    if let Some(max) = max_memory_mb {
        processes.retain(|p| p.memory_mb <= max);
    }
    
    // Filter by search query
    if let Some(query) = search_query {
        let query_lower = query.to_lowercase();
        processes.retain(|p| {
            p.name.to_lowercase().contains(&query_lower) ||
            p.process_type.to_lowercase().contains(&query_lower) ||
            p.browser.to_lowercase().contains(&query_lower)
        });
    }
    
    Ok(processes)
}

#[tauri::command]
fn sort_processes(
    processes: Vec<ProcessInfo>,
    sort_by: String,
    ascending: bool,
) -> Result<Vec<ProcessInfo>, String> {
    let mut sorted = processes;
    
    match sort_by.as_str() {
        "memory" => {
            sorted.sort_by(|a, b| {
                if ascending {
                    a.memory_mb.partial_cmp(&b.memory_mb).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    b.memory_mb.partial_cmp(&a.memory_mb).unwrap_or(std::cmp::Ordering::Equal)
                }
            });
        }
        "cpu" => {
            sorted.sort_by(|a, b| {
                if ascending {
                    a.cpu_percent.partial_cmp(&b.cpu_percent).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal)
                }
            });
        }
        "name" => {
            sorted.sort_by(|a, b| {
                if ascending {
                    a.name.cmp(&b.name)
                } else {
                    b.name.cmp(&a.name)
                }
            });
        }
        "type" => {
            sorted.sort_by(|a, b| {
                if ascending {
                    a.process_type.cmp(&b.process_type)
                } else {
                    b.process_type.cmp(&a.process_type)
                }
            });
        }
        _ => return Err("Invalid sort_by parameter. Use 'memory', 'cpu', 'name', or 'type'".to_string())
    }
    
    Ok(sorted)
}

#[tauri::command]
fn check_memory_threshold(threshold_percent: f64) -> Result<bool, String> {
    let system_info = get_system_info()?;
    let usage_percent = (system_info.used_system_memory_mb as f64 / system_info.total_system_memory_mb as f64) * 100.0;
    Ok(usage_percent >= threshold_percent)
}

#[tauri::command]
fn get_process_by_pid(pid: u32) -> Result<Option<ProcessInfo>, String> {
    let system_info = get_system_info()?;
    
    for browser in system_info.browsers {
        if let Some(process) = browser.processes.into_iter().find(|p| p.pid == pid) {
            return Ok(Some(process));
        }
    }
    
    Ok(None)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            refresh_info,
            terminate_process,
            terminate_processes,
            clear_memory,
            get_process_stats,
            export_data,
            get_settings,
            save_settings,
            filter_processes,
            sort_processes,
            check_memory_threshold,
            get_process_by_pid
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

