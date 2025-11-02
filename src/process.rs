pub struct BrowserProcess {
    pub pid: u32,
    pub name: String,
    pub memory_mb: f64,
    pub cpu_percent: f32,
    pub browser: String,
    pub process_type: String,
}

#[derive(Debug, Clone)]
pub enum ProcessAction {
    Clear,
    Hibernate,
    Terminate,
}

impl BrowserProcess {
    pub fn new(
        pid: u32,
        name: String,
        memory_mb: f64,
        cpu_percent: f32,
        browser: String,
        process_type: String,
    ) -> Self {
        Self {
            pid,
            name,
            memory_mb,
            cpu_percent,
            browser,
            process_type,
        }
    }
}

