use super::{CPU_PRIORITY_PENDING_PID, CPU_TREE_EXPANDED, WinchiselApp};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

static CPU_SORT_STATE: LazyLock<Mutex<(CpuSortColumn, bool)>> =
    LazyLock::new(|| Mutex::new((CpuSortColumn::Name, true)));

#[derive(Clone)]
pub(crate) struct CpuState {
    pub(crate) cpu_filter_active_only: bool,
    pub(crate) cpu_visible_count: usize,
    pub(crate) cpu_total_usage: String,
    pub(crate) cpu_processes_all: Vec<CpuProcessRow>,
    pub(crate) cpu_processes: Vec<CpuProcessRow>,
    pub(crate) cpu_selected_pid: i32,
    pub(crate) cpu_selected_name: String,
    pub(crate) cpu_affinity_dialog_visible: bool,
    pub(crate) cpu_affinity_dialog_pid: i32,
    pub(crate) cpu_affinity_dialog_name: String,
    pub(crate) cpu_affinity_dialog_mask: String,
    pub(crate) cpu_affinity_cores: Vec<CpuAffinityCoreRow>,
    pub(crate) cpu_last_refresh: Option<Instant>,
    pub(crate) cpu_reload_ready_at: Option<Instant>,
    pub(crate) cpu_reload_pending: bool,
    pub(crate) cpu_last_error: Option<String>,
    pub(crate) cpu_pending_action: Option<CpuAction>,
    pub(crate) cpu_realtime_confirm_visible: bool,
    pub(crate) cpu_realtime_pending_pid: i32,
    pub(crate) cpu_realtime_pending_name: String,
}

#[derive(Clone, Default)]
pub(crate) struct CpuProcessRow {
    pub(crate) pid: i32,
    pub(crate) ppid: i32,
    pub(crate) depth: usize,
    pub(crate) has_children: bool,
    pub(crate) expanded: bool,
    pub(crate) name: String,
    pub(crate) name_lc: String,
    pub(crate) cpu: String,
    pub(crate) priority: String,
    pub(crate) affinity: String,
    pub(crate) status: String,
}

#[derive(Clone, Default)]
pub(crate) enum CpuAction {
    #[default]
    Reload,
    ToggleTree(i32),
    OpenAffinity(i32, String),
    SetCpuCurrent(i32, i32),
    SetCpuAlways(String, u32),
    SetIoCurrent(i32, i32),
    SetIoAlways(String, u32),
    SetAffinityCurrent(i32, i32),
}

#[derive(Clone, Default)]
pub(crate) struct CpuAffinityCoreRow {
    pub(crate) idx: i32,
    pub(crate) col: i32,
    pub(crate) label: String,
    pub(crate) checked: bool,
    pub(crate) enabled: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CpuSortColumn {
    Pid,
    Name,
    Cpu,
    Priority,
    Affinity,
    Status,
}

pub(crate) struct CpuLoadResult {
    pub(crate) all_rows: Vec<CpuProcessRow>,
    pub(crate) rows: Vec<CpuProcessRow>,
    pub(crate) total_cpu: f32,
}

pub(crate) struct CpuLoadWorker {
    pub(crate) rx: Receiver<CpuLoadResult>,
}

impl WinchiselApp {
    pub(crate) fn start_cpu_load(&mut self) {
        if self.cpu_load_worker.is_some() {
            return;
        }
        self.cpu_load_worker = Some(Self::spawn_cpu_worker(
            self.state.cpu.cpu_filter_active_only,
        ));
    }

    pub(crate) fn spawn_cpu_worker(active_only: bool) -> CpuLoadWorker {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let result = Self::load_cpu_processes(active_only);
            let _ = tx.send(result);
        });
        CpuLoadWorker { rx }
    }

    pub(crate) fn request_cpu_reload(&mut self) {
        if self.state.active_tab != super::Tab::Processes {
            return;
        }
        self.state.cpu.cpu_reload_pending = true;
        self.state.cpu.cpu_reload_ready_at =
            Some(Instant::now() + Duration::from_millis(200));
    }

    pub(crate) fn queue_cpu_action(&mut self, action: CpuAction) {
        self.state.cpu.cpu_pending_action = Some(action);
    }

    pub(crate) fn process_cpu_actions(&mut self) {
        let Some(action) = self.state.cpu.cpu_pending_action.take() else {
            return;
        };
        match action {
            CpuAction::ToggleTree(pid) => {
                Self::toggle_cpu_tree(pid);
                self.rebuild_cpu_visible_rows();
            }
            CpuAction::OpenAffinity(pid, name) => self.open_affinity_editor(pid, name),
            CpuAction::SetCpuCurrent(pid, level) => {
                if level == 5 {
                    self.state.cpu.cpu_realtime_confirm_visible = true;
                    self.state.cpu.cpu_realtime_pending_pid = pid;
                    self.state.cpu.cpu_realtime_pending_name = self
                        .state
                        .cpu
                        .cpu_processes
                        .iter()
                        .find(|r| r.pid == pid)
                        .map(|r| r.name.clone())
                        .unwrap_or_default();
                    return;
                }
                let _ = cpu_set_process_priority_class(pid, level);
                self.request_cpu_reload();
            }
            CpuAction::SetCpuAlways(name, value) => {
                let _ = cpu_set_cpu_always_registry(&name, value);
                self.request_cpu_reload();
            }
            CpuAction::SetIoCurrent(pid, idx) => {
                let _ = cpu_set_process_io_priority(pid, idx);
                self.request_cpu_reload();
            }
            CpuAction::SetIoAlways(name, value) => {
                let _ = cpu_set_io_always_registry(&name, value);
                self.request_cpu_reload();
            }
            CpuAction::SetAffinityCurrent(pid, mode) => {
                let _ = self.set_process_affinity_mode(pid, mode);
                self.request_cpu_reload();
            }
            CpuAction::Reload => self.request_cpu_reload(),
        }
    }

    pub(crate) fn toggle_cpu_tree(pid: i32) {
        if let Ok(mut set_opt) = CPU_TREE_EXPANDED.lock() {
            let set = set_opt.get_or_insert_with(HashSet::new);
            if !set.insert(pid) {
                set.remove(&pid);
            }
        }
    }

    pub(crate) fn set_cpu_sort(column: CpuSortColumn) {
        if let Ok(mut state) = CPU_SORT_STATE.lock() {
            if state.0 == column {
                state.1 = !state.1;
            } else {
                *state = (column, true);
            }
        }
    }

    pub(crate) fn cpu_sort_state() -> (CpuSortColumn, bool) {
        CPU_SORT_STATE
            .lock()
            .ok()
            .map(|s| *s)
            .unwrap_or((CpuSortColumn::Name, true))
    }

    pub(crate) fn rebuild_cpu_visible_rows(&mut self) {
        self.state.cpu.cpu_processes = Self::build_cpu_tree_rows(
            &self.state.cpu.cpu_processes_all,
            self.state.cpu.cpu_filter_active_only,
        );
        self.state.cpu.cpu_visible_count = self.state.cpu.cpu_processes.len();
        self.retain_cpu_selection();
    }

    fn retain_cpu_selection(&mut self) {
        let selected_pid = self.state.cpu.cpu_selected_pid;
        if selected_pid <= 0 {
            return;
        }
        if let Some(row) = self
            .state
            .cpu
            .cpu_processes
            .iter()
            .find(|r| r.pid == selected_pid)
        {
            self.state.cpu.cpu_selected_name = row.name.clone();
        } else {
            self.state.cpu.cpu_selected_pid = -1;
            self.state.cpu.cpu_selected_name.clear();
        }
    }

    pub(crate) fn open_affinity_editor(&mut self, pid: i32, name: String) {
        if let Ok((process_mask, system_mask)) = cpu_read_process_affinity_masks(pid) {
            if let Ok(mut p) = CPU_PRIORITY_PENDING_PID.lock() {
                *p = pid;
            }
            self.state.cpu.cpu_affinity_dialog_pid = pid;
            self.state.cpu.cpu_affinity_dialog_name = name;
            self.state.cpu.cpu_affinity_dialog_mask = format!("{:X}", process_mask);
            self.state.cpu.cpu_affinity_cores =
                cpu_build_affinity_core_rows(process_mask, system_mask);
            self.state.cpu.cpu_affinity_dialog_visible = true;
        }
    }

    pub(crate) fn toggle_cpu_affinity_core(&mut self, idx: i32, checked: bool) {
        let pid = self.state.cpu.cpu_affinity_dialog_pid;
        if pid <= 0 {
            return;
        }
        if let Ok((mut process_mask, system_mask)) = cpu_read_process_affinity_masks(pid) {
            let bit = 1usize << (idx as usize);
            if (system_mask & bit) == 0 {
                return;
            }
            if checked {
                process_mask |= bit;
            } else {
                process_mask &= !bit;
            }
            if process_mask == 0 {
                process_mask = bit;
            }
            self.state.cpu.cpu_affinity_dialog_mask = format!("{:X}", process_mask);
            self.state.cpu.cpu_affinity_cores =
                cpu_build_affinity_core_rows(process_mask, system_mask);
            if let Ok(mut p) = CPU_PRIORITY_PENDING_PID.lock() {
                *p = pid;
            }
        }
    }

    pub(crate) fn invert_cpu_affinity_selection(&mut self) {
        let pid = self.state.cpu.cpu_affinity_dialog_pid;
        if pid <= 0 {
            return;
        }
        if let Ok((process_mask, system_mask)) = cpu_read_process_affinity_masks(pid) {
            let mut next = (!process_mask) & system_mask;
            if next == 0 {
                next = system_mask;
            }
            self.state.cpu.cpu_affinity_dialog_mask = format!("{:X}", next);
            self.state.cpu.cpu_affinity_cores = cpu_build_affinity_core_rows(next, system_mask);
        }
    }

    pub(crate) fn clear_cpu_affinity_selection(&mut self) {
        let pid = self.state.cpu.cpu_affinity_dialog_pid;
        if pid <= 0 {
            return;
        }
        if let Ok((_, system_mask)) = cpu_read_process_affinity_masks(pid) {
            self.state.cpu.cpu_affinity_dialog_mask = format!("{:X}", system_mask);
            self.state.cpu.cpu_affinity_cores =
                cpu_build_affinity_core_rows(system_mask, system_mask);
        }
    }

    pub(crate) fn apply_cpu_affinity_selection(&mut self) {
        let pid = self.state.cpu.cpu_affinity_dialog_pid;
        if pid <= 0 {
            return;
        }
        if let Ok((process_mask, _)) = cpu_read_process_affinity_masks(pid) {
            let target = usize::from_str_radix(&self.state.cpu.cpu_affinity_dialog_mask, 16)
                .unwrap_or(process_mask);
            let _ = self.set_process_affinity_mask(pid, target);
            self.state.cpu.cpu_affinity_dialog_visible = false;
            self.start_cpu_load();
        }
    }

    pub(crate) fn set_process_affinity_mask(&self, pid: i32, mask: usize) -> Result<(), String> {
        use windows::Win32::System::Threading::{
            OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, SetProcessAffinityMask,
        };
        unsafe {
            let handle = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
                false,
                pid as u32,
            )
            .map_err(|e| format!("OpenProcess failed for PID {}: {}", pid, e))?;
            let ok = SetProcessAffinityMask(handle, mask).is_ok();
            let _ = windows::Win32::Foundation::CloseHandle(handle);
            if ok {
                Ok(())
            } else {
                Err(format!("SetProcessAffinityMask failed for PID {}", pid))
            }
        }
    }

    pub(crate) fn set_process_affinity_mode(&self, pid: i32, mode: i32) -> Result<(), String> {
        use windows::Win32::System::Threading::{
            GetProcessAffinityMask, OpenProcess, PROCESS_QUERY_INFORMATION,
            PROCESS_SET_INFORMATION, SetProcessAffinityMask,
        };
        unsafe {
            let handle = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
                false,
                pid as u32,
            )
            .map_err(|e| format!("OpenProcess failed for PID {}: {}", pid, e))?;

            let mut process_mask: usize = 0;
            let mut system_mask: usize = 0;
            GetProcessAffinityMask(handle, &mut process_mask, &mut system_mask)
                .map_err(|e| format!("GetProcessAffinityMask failed for PID {}: {}", pid, e))?;

            if system_mask == 0 {
                let _ = windows::Win32::Foundation::CloseHandle(handle);
                return Err(format!("Invalid system affinity mask for PID {}", pid));
            }

            let mut target_mask: usize = 0;
            match mode {
                0 => target_mask = system_mask,
                1 => {
                    for i in 0..(usize::BITS as usize) {
                        let bit = 1usize << i;
                        if (system_mask & bit) != 0 && i.is_multiple_of(2) {
                            target_mask |= bit;
                        }
                    }
                }
                2 => {
                    for i in 0..(usize::BITS as usize) {
                        let bit = 1usize << i;
                        if (system_mask & bit) != 0 && i % 2 == 1 {
                            target_mask |= bit;
                        }
                    }
                }
                3 | 4 => {
                    let mut cores: Vec<usize> = Vec::new();
                    for i in 0..(usize::BITS as usize) {
                        let bit = 1usize << i;
                        if (system_mask & bit) != 0 {
                            cores.push(i);
                        }
                    }
                    if !cores.is_empty() {
                        let split = cores.len().div_ceil(2);
                        let range = if mode == 3 {
                            0..split
                        } else {
                            split..cores.len()
                        };
                        for idx in range {
                            target_mask |= 1usize << cores[idx];
                        }
                    }
                }
                _ => {
                    let _ = windows::Win32::Foundation::CloseHandle(handle);
                    return Err("Invalid affinity mode".to_string());
                }
            }

            if target_mask == 0 {
                target_mask = system_mask;
            }
            target_mask &= system_mask;

            let ok = SetProcessAffinityMask(handle, target_mask).is_ok();
            let _ = windows::Win32::Foundation::CloseHandle(handle);
            if ok {
                Ok(())
            } else {
                Err(format!("SetProcessAffinityMask failed for PID {}", pid))
            }
        }
    }

    pub(crate) fn poll_cpu_load(&mut self) {
        let Some(worker) = self.cpu_load_worker.as_ref() else {
            return;
        };
        match worker.rx.try_recv() {
            Ok(result) => {
                let now = Instant::now();
                self.state.cpu.cpu_visible_count = result.rows.len();
                self.state.cpu.cpu_total_usage = format!("{:.1}%", result.total_cpu);
                self.state.cpu.cpu_processes_all = result.all_rows;
                self.state.cpu.cpu_processes = result.rows;
                self.state.cpu.cpu_last_refresh = Some(now);
                self.state.cpu.cpu_reload_pending = false;
                self.state.cpu.cpu_last_error = None;
                self.retain_cpu_selection();
                self.cpu_load_worker = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.state.cpu.cpu_last_error = Some("Process scan failed".to_string());
                self.state.cpu.cpu_reload_pending = false;
                self.state.cpu.cpu_reload_ready_at = None;
                self.cpu_load_worker = None;
            }
        }
    }

    fn load_cpu_processes(active_only: bool) -> CpuLoadResult {
        use sysinfo::{ProcessesToUpdate, System};
        let mut system = System::new_all();
        system.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(220));
        let _ = system.refresh_processes(ProcessesToUpdate::All, true);
        let total_cpu = system.global_cpu_usage();

        let rows: Vec<CpuProcessRow> = system
            .processes()
            .iter()
            .map(|(pid, p)| CpuProcessRow {
                pid: pid.as_u32() as i32,
                ppid: p.parent().map(|x| x.as_u32() as i32).unwrap_or(0),
                depth: 0,
                has_children: false,
                expanded: false,
                name: p.name().to_string_lossy().into_owned(),
                name_lc: p.name().to_string_lossy().to_lowercase(),
                cpu: format!("{:.1}%", p.cpu_usage()),
                priority: cpu_get_process_priority_label(pid.as_u32() as i32),
                affinity: cpu_get_process_affinity_label(pid.as_u32() as i32),
                status: format!("{:?}", p.status()),
            })
            .collect();
        let rows_tree = Self::build_cpu_tree_rows(&rows, active_only);
        CpuLoadResult {
            all_rows: rows,
            rows: rows_tree,
            total_cpu,
        }
    }

    fn build_cpu_tree_rows(rows: &[CpuProcessRow], active_only: bool) -> Vec<CpuProcessRow> {
        let mut by_pid: HashMap<i32, CpuProcessRow> = HashMap::new();
        let mut children: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows.iter().cloned() {
            if row.ppid != row.pid {
                children.entry(row.ppid).or_default().push(row.pid);
            }
            by_pid.insert(row.pid, row);
        }
        let (sort_column, ascending) = Self::cpu_sort_state();
        Self::sort_cpu_children(&mut children, &by_pid, sort_column, ascending);
        let expanded = CPU_TREE_EXPANDED
            .lock()
            .ok()
            .and_then(|v| v.clone())
            .unwrap_or_default();
        let active_set = if active_only {
            Self::compute_active_cpu_set(&by_pid, &children)
        } else {
            HashSet::new()
        };

        let mut roots: Vec<i32> = by_pid
            .values()
            .filter(|r| r.ppid <= 0 || !by_pid.contains_key(&r.ppid))
            .map(|r| r.pid)
            .collect();
        Self::sort_cpu_pids(&mut roots, &by_pid, sort_column, ascending);

        let mut out = Vec::new();
        for pid in roots {
            let mut path = HashSet::new();
            Self::flatten_cpu_tree(
                pid,
                0,
                &children,
                &by_pid,
                &expanded,
                if active_only { Some(&active_set) } else { None },
                &mut path,
                &mut out,
            );
            if out.len() >= 1000 {
                break;
            }
        }
        out.truncate(1000);
        out
    }

    fn sort_cpu_children(
        children: &mut HashMap<i32, Vec<i32>>,
        by_pid: &HashMap<i32, CpuProcessRow>,
        sort_column: CpuSortColumn,
        ascending: bool,
    ) {
        for bucket in children.values_mut() {
            Self::sort_cpu_pids(bucket, by_pid, sort_column, ascending);
        }
    }

    fn sort_cpu_pids(
        pids: &mut Vec<i32>,
        by_pid: &HashMap<i32, CpuProcessRow>,
        sort_column: CpuSortColumn,
        ascending: bool,
    ) {
        pids.sort_by(|a, b| {
            let ra = by_pid.get(a);
            let rb = by_pid.get(b);
            let ord = match sort_column {
                CpuSortColumn::Pid => a.cmp(b),
                CpuSortColumn::Name => ra
                    .map(|r| r.name_lc.as_str())
                    .cmp(&rb.map(|r| r.name_lc.as_str())),
                CpuSortColumn::Cpu => Self::cpu_value(ra)
                    .partial_cmp(&Self::cpu_value(rb))
                    .unwrap_or(std::cmp::Ordering::Equal),
                CpuSortColumn::Priority => ra.map(|r| r.priority.as_str()).cmp(&rb.map(|r| r.priority.as_str())),
                CpuSortColumn::Affinity => ra.map(|r| r.affinity.as_str()).cmp(&rb.map(|r| r.affinity.as_str())),
                CpuSortColumn::Status => ra.map(|r| r.status.as_str()).cmp(&rb.map(|r| r.status.as_str())),
            };
            if ascending { ord } else { ord.reverse() }
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn flatten_cpu_tree(
        pid: i32,
        depth: usize,
        children: &HashMap<i32, Vec<i32>>,
        by_pid: &HashMap<i32, CpuProcessRow>,
        expanded: &HashSet<i32>,
        active_set: Option<&HashSet<i32>>,
        path: &mut HashSet<i32>,
        out: &mut Vec<CpuProcessRow>,
    ) {
        if !path.insert(pid) {
            return;
        }
        let Some(mut row) = by_pid.get(&pid).cloned() else {
            path.remove(&pid);
            return;
        };
        if let Some(set) = active_set && !set.contains(&pid) {
            path.remove(&pid);
            return;
        }
        row.depth = depth;
        let kids = children.get(&pid);
        row.has_children = kids.is_some_and(|k| !k.is_empty());
        row.expanded = expanded.contains(&pid);
        out.push(row);
        if out.len() >= 1000 || kids.is_none_or(|k| k.is_empty()) || expanded.contains(&pid) {
            path.remove(&pid);
            return;
        }
        if let Some(kids) = kids {
            for &child in kids {
                Self::flatten_cpu_tree(
                    child,
                    depth + 1,
                    children,
                    by_pid,
                    expanded,
                    active_set,
                    path,
                    out,
                );
                if out.len() >= 1000 {
                    break;
                }
            }
        }
        path.remove(&pid);
    }

fn compute_active_cpu_set(
        by_pid: &HashMap<i32, CpuProcessRow>,
        children: &HashMap<i32, Vec<i32>>,
    ) -> HashSet<i32> {
        let mut active = HashSet::new();
        for row in by_pid.values() {
            let cpu = row.cpu.trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
            if cpu > 0.0 {
                let mut p = row.pid;
                loop {
                    if !active.insert(p) {
                        break;
                    }
                    let parent = by_pid.get(&p).map(|r| r.ppid).unwrap_or(0);
                    if parent <= 0 || !by_pid.contains_key(&parent) {
                        break;
                    }
                    p = parent;
                }
            }
        }
        let mut stack: Vec<i32> = active.iter().copied().collect();
        while let Some(p) = stack.pop() {
            if let Some(kids) = children.get(&p) {
                for &k in kids {
                    if active.insert(k) {
                        stack.push(k);
                    }
                }
            }
        }
    active
}

    fn cpu_value(row: Option<&CpuProcessRow>) -> f32 {
        row.and_then(|r| r.cpu.trim_end_matches('%').parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    pub(crate) fn render_processes_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Processes");
                ui.label("Inspect processes, affinity and priority settings.");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let loading = self.cpu_load_worker.is_some();
                let button = egui::Button::new("Refresh")
                    .fill(egui::Color32::from_rgb(35, 88, 55))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 145, 92)));
                let response = ui.add_sized([120.0, 34.0], button);
                if loading {
                    let spinner_rect = egui::Rect::from_min_size(
                        response.rect.left_center() + egui::vec2(8.0, -8.0),
                        egui::vec2(14.0, 14.0),
                    );
                    ui.put(spinner_rect, egui::Spinner::new().size(12.0));
                }
                if response.clicked() && !loading {
                    self.request_cpu_reload();
                    ui.ctx().request_repaint();
                }
            });
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(format!("Visible: {}", self.state.cpu.cpu_visible_count));
            ui.separator();
            ui.label(format!("Total CPU: {}", self.state.cpu.cpu_total_usage));
            ui.separator();
            let mut active_only = self.state.cpu.cpu_filter_active_only;
            ui.horizontal(|ui| {
                let resp = ui
                    .add_enabled_ui(true, |ui| Self::native_toggle_switch(ui, &mut active_only))
                    .inner;
                if resp.changed() {
                    self.state.cpu.cpu_filter_active_only = active_only;
                    self.request_cpu_reload();
                    ui.ctx().request_repaint();
                }
                let label = if active_only {
                    egui::RichText::new("Active only")
                        .strong()
                        .color(egui::Color32::from_rgb(96, 181, 103))
                } else {
                    egui::RichText::new("Active only")
                        .color(egui::Color32::from_rgb(210, 80, 80))
                };
                ui.label(label);
            });
            ui.separator();
            if self.cpu_load_worker.is_some() {
                ui.add(egui::Spinner::new().size(16.0));
                ui.add_space(6.0);
                ui.colored_label(
                    egui::Color32::from_rgb(149, 194, 255),
                    "Refreshing process list...",
                );
            } else if let Some(err) = self.state.cpu.cpu_last_error.as_ref() {
                ui.colored_label(egui::Color32::from_rgb(210, 80, 80), err);
            } else if let Some(last) = self.state.cpu.cpu_last_refresh {
                let elapsed = last.elapsed();
                ui.label(format!("Last refresh: {}s ago", elapsed.as_secs()));
            } else {
                ui.label("Waiting for first refresh...");
            }
            if self.state.cpu.cpu_reload_pending {
                ui.separator();
                ui.label("Reload queued");
            }
        });

        ui.add_space(10.0);

        TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::exact(64.0))
                    .column(Column::remainder())
                    .column(Column::exact(64.0))
                    .column(Column::exact(64.0))
                    .column(Column::exact(92.0))
                    .column(Column::exact(72.0))
                    .header(22.0, |mut header| {
                        let (sort_column, ascending) = Self::cpu_sort_state();
                        let mut header_button =
                            |ui: &mut egui::Ui, label: &str, column: CpuSortColumn| {
                            let mut text = label.to_string();
                            if sort_column == column {
                                text.push_str(if ascending { " ↑" } else { " ↓" });
                            }
                            if ui.button(text).clicked() {
                                Self::set_cpu_sort(column);
                                self.rebuild_cpu_visible_rows();
                            }
                        };
                        header.col(|ui| header_button(ui, "PID", CpuSortColumn::Pid));
                        header.col(|ui| header_button(ui, "Name", CpuSortColumn::Name));
                        header.col(|ui| header_button(ui, "CPU %", CpuSortColumn::Cpu));
                        header.col(|ui| header_button(ui, "Priority", CpuSortColumn::Priority));
                        header.col(|ui| header_button(ui, "Affinity", CpuSortColumn::Affinity));
                        header.col(|ui| header_button(ui, "Status", CpuSortColumn::Status));
                    })
                    .body(|body| {
                        body.rows(26.0, self.state.cpu.cpu_processes.len(), |mut row| {
                            let idx = row.index();
                            let proc_row = self.state.cpu.cpu_processes[idx].clone();
                            let selected = self.state.cpu.cpu_selected_pid == proc_row.pid;
                            let pending_action: std::cell::RefCell<Option<CpuAction>> =
                                std::cell::RefCell::new(None);
                            let mut name_response: Option<egui::Response> = None;
                            let row_menu = |ui: &mut egui::Ui| {
                                if proc_row.has_children
                                    && ui
                                        .button(if proc_row.expanded {
                                            "Collapse tree"
                                        } else {
                                            "Expand tree"
                                        })
                                        .clicked()
                                {
                                    *pending_action.borrow_mut() =
                                        Some(CpuAction::ToggleTree(proc_row.pid));
                                    ui.close();
                                }
                                if proc_row.has_children {
                                    ui.separator();
                                }
                                ui.menu_button("CPU Priority", |ui| {
                                    ui.menu_button("Current", |ui| {
                                        for (level, label) in [
                                            (4, "High: 13"),
                                            (3, "Above Normal: 10"),
                                            (2, "Normal: 8"),
                                            (1, "Below Normal: 6"),
                                            (6, "Background: 4 (Low I/O and CPU)"),
                                            (0, "Idle: 4"),
                                        ] {
                                            if ui.button(label).clicked() {
                                                *pending_action.borrow_mut() =
                                                    Some(CpuAction::SetCpuCurrent(
                                                        proc_row.pid,
                                                        level,
                                                    ));
                                                ui.close();
                                            }
                                        }
                                    });
                                    ui.menu_button("Always", |ui| {
                                        for (value, label) in [
                                            (1, "Idle"),
                                            (5, "Below"),
                                            (2, "Normal"),
                                            (6, "Above"),
                                            (3, "High"),
                                            (4, "Realtime"),
                                        ] {
                                            if ui.button(label).clicked() {
                                                *pending_action.borrow_mut() =
                                                    Some(CpuAction::SetCpuAlways(
                                                        proc_row.name.clone(),
                                                        value,
                                                    ));
                                                ui.close();
                                            }
                                        }
                                    });
                                });
                                ui.menu_button("I/O Priority", |ui| {
                                    ui.menu_button("Current", |ui| {
                                        if ui.button("Low").clicked() {
                                            *pending_action.borrow_mut() =
                                                Some(CpuAction::SetIoCurrent(proc_row.pid, 0));
                                            ui.close();
                                        }
                                        if ui.button("Normal").clicked() {
                                            *pending_action.borrow_mut() =
                                                Some(CpuAction::SetIoCurrent(proc_row.pid, 1));
                                            ui.close();
                                        }
                                    });
                                    ui.menu_button("Always", |ui| {
                                        for (idx, label) in [(0, "Low"), (1, "Normal")] {
                                            if ui.button(label).clicked() {
                                                *pending_action.borrow_mut() =
                                                    Some(CpuAction::SetIoAlways(
                                                        proc_row.name.clone(),
                                                        idx,
                                                    ));
                                                ui.close();
                                            }
                                        }
                                    });
                                });
                                ui.menu_button("Affinity", |ui| {
                                    ui.menu_button("Current", |ui| {
                                        if ui.button("Open editor").clicked() {
                                            *pending_action.borrow_mut() =
                                                Some(CpuAction::OpenAffinity(
                                                    proc_row.pid,
                                                    proc_row.name.clone(),
                                                ));
                                            ui.close();
                                        }
                                        if ui.button("All cores").clicked() {
                                            *pending_action.borrow_mut() =
                                                Some(CpuAction::SetAffinityCurrent(
                                                    proc_row.pid,
                                                    0,
                                                ));
                                            ui.close();
                                        }
                                    });
                                });
                            };

                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    if proc_row.has_children {
                                        let icon = if proc_row.expanded { ">" } else { "v" };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(icon).size(11.0),
                                                )
                                                .frame(false),
                                            )
                                            .clicked()
                                        {
                                            *pending_action.borrow_mut() =
                                                Some(CpuAction::ToggleTree(proc_row.pid));
                                        }
                                    } else {
                                        ui.add_space(18.0);
                                    }
                                    ui.label(proc_row.pid.to_string());
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space((proc_row.depth as f32) * 14.0);
                                    let response = ui.selectable_label(selected, &proc_row.name);
                                    if response.clicked() {
                                        self.state.cpu.cpu_selected_pid = proc_row.pid;
                                        self.state.cpu.cpu_selected_name = proc_row.name.clone();
                                    }
                                    if response.secondary_clicked() {
                                        self.state.cpu.cpu_selected_pid = proc_row.pid;
                                        self.state.cpu.cpu_selected_name = proc_row.name.clone();
                                    }
                                    name_response = Some(response);
                                });
                            });
                            row.col(|ui| {
                                ui.add_sized(
                                    [ui.available_width(), 24.0],
                                    egui::Label::new(proc_row.cpu.clone())
                                        .truncate()
                                        .halign(egui::Align::Center),
                                );
                            });
                            row.col(|ui| {
                                ui.add_sized(
                                    [ui.available_width(), 24.0],
                                    egui::Label::new(proc_row.priority.clone())
                                        .truncate()
                                        .halign(egui::Align::Center),
                                );
                            });
                            row.col(|ui| {
                                ui.add_sized(
                                    [ui.available_width(), 24.0],
                                    egui::Label::new(proc_row.affinity.clone())
                                        .truncate()
                                        .halign(egui::Align::Center),
                                );
                            });
                            row.col(|ui| {
                                ui.add_sized(
                                    [ui.available_width(), 24.0],
                                    egui::Label::new(proc_row.status.clone())
                                        .truncate()
                                        .halign(egui::Align::Center),
                                );
                            });
                            if let Some(name_response) = name_response.as_ref() {
                                name_response.context_menu(|ui| row_menu(ui));
                            }
                            let row_response = row.response();
                            row_response.context_menu(|ui| row_menu(ui));
                            if let Some(action) = pending_action.borrow_mut().take() {
                                self.queue_cpu_action(action);
                            }
                        });
                    });
                self.process_cpu_actions();
                if self.state.cpu.cpu_selected_pid > 0 {
                    ui.add_space(12.0);
                    Self::card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.heading("Selected Process");
                                ui.label(format!(
                                    "{} (PID {})",
                                    self.state.cpu.cpu_selected_name,
                                    self.state.cpu.cpu_selected_pid
                                ));
                            });
                        });
                        ui.add_space(8.0);
                    });
                }
        if self.state.cpu.cpu_realtime_confirm_visible {
            egui::Window::new("Set Realtime Priority?")
                .collapsible(false)
                .resizable(false)
                .default_width(460.0)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ui.ctx(), |ui| {
                    ui.label(
                        "Realtime can freeze Windows responsiveness. Continue only if you understand the risk.",
                    );
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.state.cpu.cpu_realtime_confirm_visible = false;
                            self.state.cpu.cpu_realtime_pending_pid = -1;
                            self.state.cpu.cpu_realtime_pending_name.clear();
                        }
                        if ui.button("Confirm").clicked() {
                            let pid = self.state.cpu.cpu_realtime_pending_pid;
                            if pid > 0 {
                                let _ = cpu_set_process_priority_class(pid, 5);
                                self.request_cpu_reload();
                            }
                            self.state.cpu.cpu_realtime_confirm_visible = false;
                            self.state.cpu.cpu_realtime_pending_pid = -1;
                            self.state.cpu.cpu_realtime_pending_name.clear();
                        }
                    });
                });
        }

        if self.state.cpu.cpu_affinity_dialog_visible {
            let core_count = self.state.cpu.cpu_affinity_cores.len().max(1);
            let grid_cols = cpu_affinity_grid_cols(core_count);
            let grid_rows = core_count.div_ceil(grid_cols);
            let desired_width = 160.0 + (grid_cols as f32 * 132.0);
            let desired_height = 170.0 + (grid_rows as f32 * 34.0) + 56.0;
            let max_width = ui.ctx().content_rect().width() * 0.95;
            let max_height = ui.ctx().content_rect().height() * 0.90;
            egui::Window::new(format!(
                "CPU Affinity - {} (PID {})",
                self.state.cpu.cpu_affinity_dialog_name, self.state.cpu.cpu_affinity_dialog_pid
            ))
            .collapsible(false)
            .resizable(false)
            .default_width(desired_width.min(max_width).max(420.0))
            .default_height(desired_height.min(max_height).max(260.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.label(format!(
                    "Affinity bitmask (hex): {}",
                    self.state.cpu.cpu_affinity_dialog_mask
                ));
                ui.add_space(10.0);
                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        egui::Grid::new("cpu_affinity_grid")
                            .num_columns(grid_cols)
                            .spacing([16.0, 8.0])
                            .show(ui, |ui| {
                                let last_col = grid_cols.saturating_sub(1) as i32;
                                for core in self.state.cpu.cpu_affinity_cores.clone() {
                                    let mut checked = core.checked;
                                    let resp = ui.add_enabled(
                                        core.enabled,
                                        egui::Checkbox::new(&mut checked, &core.label),
                                    );
                                    if resp.changed() {
                                        self.toggle_cpu_affinity_core(core.idx, checked);
                                    }
                                    if core.col == last_col {
                                        ui.end_row();
                                    }
                                }
                            });
                    });
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Invert").clicked() {
                        self.invert_cpu_affinity_selection();
                    }
                    if ui.button("Clear").clicked() {
                        self.clear_cpu_affinity_selection();
                    }
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.state.cpu.cpu_affinity_dialog_visible = false;
                    }
                    if ui.button("Apply").clicked() {
                        self.apply_cpu_affinity_selection();
                    }
                });
            });
        }
    }
}

fn cpu_get_process_priority_label(pid: i32) -> String {
    use windows::Win32::System::Threading::{
        ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, GetPriorityClass,
        HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, OpenProcess,
        PROCESS_QUERY_INFORMATION, REALTIME_PRIORITY_CLASS,
    };
    unsafe {
        let handle = match OpenProcess(PROCESS_QUERY_INFORMATION, false, pid as u32) {
            Ok(h) => h,
            Err(_) => return "Unknown".to_string(),
        };
        let cls = GetPriorityClass(handle);
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        match cls {
            c if c == IDLE_PRIORITY_CLASS.0 => "Idle".to_string(),
            c if c == BELOW_NORMAL_PRIORITY_CLASS.0 => "Below Normal".to_string(),
            c if c == NORMAL_PRIORITY_CLASS.0 => "Normal".to_string(),
            c if c == ABOVE_NORMAL_PRIORITY_CLASS.0 => "Above Normal".to_string(),
            c if c == HIGH_PRIORITY_CLASS.0 => "High".to_string(),
            c if c == REALTIME_PRIORITY_CLASS.0 => "Realtime".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

fn cpu_get_process_affinity_label(pid: i32) -> String {
    use windows::Win32::System::Threading::{
        GetProcessAffinityMask, OpenProcess, PROCESS_QUERY_INFORMATION,
    };
    unsafe {
        let handle = match OpenProcess(PROCESS_QUERY_INFORMATION, false, pid as u32) {
            Ok(h) => h,
            Err(_) => return "-".to_string(),
        };
        let mut process_mask: usize = 0;
        let mut system_mask: usize = 0;
        let ok = GetProcessAffinityMask(handle, &mut process_mask, &mut system_mask).is_ok();
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        if !ok || system_mask == 0 {
            return "-".to_string();
        }
        if process_mask == system_mask {
            return "All cores".to_string();
        }
        if process_mask == 0 {
            return "Unknown".to_string();
        }

        let mut parts: Vec<String> = Vec::new();
        let mut start: Option<usize> = None;
        let mut prev: Option<usize> = None;
        for i in 0..(usize::BITS as usize) {
            let bit_set = (process_mask & (1usize << i)) != 0;
            match (start, prev, bit_set) {
                (None, _, true) => {
                    start = Some(i);
                    prev = Some(i);
                }
                (Some(_), Some(p), true) if i == p + 1 => {
                    prev = Some(i);
                }
                (Some(s), Some(p), false) => {
                    if s == p {
                        parts.push(format!("{}", s));
                    } else {
                        parts.push(format!("{}-{}", s, p));
                    }
                    start = None;
                    prev = None;
                }
                _ => {}
            }
        }
        if let (Some(s), Some(p)) = (start, prev) {
            if s == p {
                parts.push(format!("{}", s));
            } else {
                parts.push(format!("{}-{}", s, p));
            }
        }

        format!("CPU {}", parts.join(","))
    }
}

fn cpu_read_process_affinity_masks(pid: i32) -> Result<(usize, usize), String> {
    use windows::Win32::System::Threading::{
        GetProcessAffinityMask, OpenProcess, PROCESS_QUERY_INFORMATION,
    };
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid as u32)
            .map_err(|e| format!("OpenProcess failed for PID {}: {}", pid, e))?;
        let mut process_mask: usize = 0;
        let mut system_mask: usize = 0;
        let res = GetProcessAffinityMask(handle, &mut process_mask, &mut system_mask);
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        res.map_err(|e| format!("GetProcessAffinityMask failed for PID {}: {}", pid, e))?;
        if system_mask == 0 {
            return Err(format!("Invalid system affinity mask for PID {}", pid));
        }
        Ok((process_mask & system_mask, system_mask))
    }
}

fn cpu_build_affinity_core_rows(
    process_mask: usize,
    system_mask: usize,
) -> Vec<CpuAffinityCoreRow> {
    let mut rows = Vec::new();
    let mut n = 0i32;
    let total_cores = system_mask.count_ones() as usize;
    let cols = cpu_affinity_grid_cols(total_cores.max(1)) as i32;
    let efficiency_classes = cpu_logical_processor_efficiency_classes();
    for i in 0..(usize::BITS as usize) {
        let bit = 1usize << i;
        if (system_mask & bit) == 0 {
            continue;
        }
        let efficiency = efficiency_classes.get(&i).copied().unwrap_or(0);
        let label = if efficiency > 0 {
            format!("CPU {} (E)", i)
        } else {
            format!("CPU {}", i)
        };
        rows.push(CpuAffinityCoreRow {
            idx: i as i32,
            col: n % cols,
            label,
            checked: (process_mask & bit) != 0,
            enabled: true,
        });
        n += 1;
    }
    rows
}

fn cpu_affinity_grid_cols(core_count: usize) -> usize {
    match core_count {
        0..=8 => 2,
        9..=16 => 4,
        17..=32 => 6,
        _ => 8,
    }
}

fn cpu_logical_processor_efficiency_classes() -> HashMap<usize, u8> {
    use windows::Win32::System::SystemInformation::{
        CpuSetInformation, GetSystemCpuSetInformation, SYSTEM_CPU_SET_INFORMATION,
    };

    unsafe {
        let mut required = 0u32;
        let _ = GetSystemCpuSetInformation(None, 0, &mut required, None, Some(0));
        if required == 0 {
            return HashMap::new();
        }

        let mut buffer = vec![0u8; required as usize];
        let ok = GetSystemCpuSetInformation(
            Some(buffer.as_mut_ptr() as *mut SYSTEM_CPU_SET_INFORMATION),
            required,
            &mut required,
            None,
            Some(0),
        );
        if !ok.as_bool() {
            return HashMap::new();
        }

        let mut map = HashMap::new();
        let mut offset = 0usize;
        while offset + core::mem::size_of::<SYSTEM_CPU_SET_INFORMATION>() <= required as usize {
            let info = &*(buffer.as_ptr().add(offset) as *const SYSTEM_CPU_SET_INFORMATION);
            if info.Type == CpuSetInformation {
                let cpu = info.Anonymous.CpuSet;
                map.insert(cpu.LogicalProcessorIndex as usize, cpu.EfficiencyClass);
            }
            if info.Size == 0 {
                break;
            }
            offset += info.Size as usize;
        }

        map
    }
}

fn cpu_set_cpu_always_registry(process_name: &str, value: u32) -> Result<(), String> {
    let exe_name = if process_name.to_ascii_lowercase().ends_with(".exe") {
        process_name.to_string()
    } else {
        format!("{}.exe", process_name)
    };
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let path = format!(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}\\PerfOptions",
        exe_name
    );
    let (key, _) = hklm
        .create_subkey(path)
        .map_err(|e| format!("Failed to create/open PerfOptions key: {}", e))?;
    if value == 7 {
        key.set_value("CpuPriorityClass", &4u32)
            .map_err(|e| format!("Failed to write CpuPriorityClass: {}", e))?;
        key.set_value("IoPriority", &1u32)
            .map_err(|e| format!("Failed to write IoPriority: {}", e))?;
    } else {
        key.set_value("CpuPriorityClass", &value)
            .map_err(|e| format!("Failed to write CpuPriorityClass: {}", e))?;
    }
    Ok(())
}

fn cpu_set_io_always_registry(process_name: &str, value: u32) -> Result<(), String> {
    let exe_name = if process_name.to_ascii_lowercase().ends_with(".exe") {
        process_name.to_string()
    } else {
        format!("{}.exe", process_name)
    };
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let path = format!(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}\\PerfOptions",
        exe_name
    );
    let (key, _) = hklm
        .create_subkey(path)
        .map_err(|e| format!("Failed to create/open PerfOptions key: {}", e))?;
    key.set_value("IoPriority", &value)
        .map_err(|e| format!("Failed to write IoPriority: {}", e))?;
    Ok(())
}

fn cpu_set_process_priority_class(pid: i32, level: i32) -> Result<(), String> {
    use windows::Win32::System::Threading::{
        ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, GetPriorityClass,
        HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, OpenProcess,
        PROCESS_MODE_BACKGROUND_BEGIN, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION,
        REALTIME_PRIORITY_CLASS, SetPriorityClass,
    };
    let class = match level {
        0 => IDLE_PRIORITY_CLASS,
        1 => BELOW_NORMAL_PRIORITY_CLASS,
        2 => NORMAL_PRIORITY_CLASS,
        3 => ABOVE_NORMAL_PRIORITY_CLASS,
        4 => HIGH_PRIORITY_CLASS,
        5 => REALTIME_PRIORITY_CLASS,
        6 => PROCESS_MODE_BACKGROUND_BEGIN,
        _ => return Err("Invalid priority level".to_string()),
    };
    unsafe {
        let handle = OpenProcess(
            PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
            false,
            pid as u32,
        )
        .map_err(|e| format!("OpenProcess failed for PID {}: {}", pid, e))?;
        let _ = GetPriorityClass(handle);
        let ok = SetPriorityClass(handle, class).is_ok();
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        if ok {
            Ok(())
        } else {
            Err(format!("SetPriorityClass failed for PID {}", pid))
        }
    }
}

fn cpu_set_process_io_priority(pid: i32, idx: i32) -> Result<(), String> {
    use windows::Win32::System::Threading::{
        OpenProcess, PROCESS_INFORMATION_CLASS, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION,
        SetProcessInformation,
    };
    let io_priority: u32 = if idx == 0 { 1 } else { 2 };
    unsafe {
        let handle = OpenProcess(
            PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
            false,
            pid as u32,
        )
        .map_err(|e| format!("OpenProcess failed for PID {}: {}", pid, e))?;
        let cls = PROCESS_INFORMATION_CLASS(33);
        let ok = SetProcessInformation(
            handle,
            cls,
            &io_priority as *const u32 as *const core::ffi::c_void,
            core::mem::size_of::<u32>() as u32,
        )
        .is_ok();
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        if ok {
            Ok(())
        } else {
            Err(format!("SetProcessInformation(I/O) failed for PID {}", pid))
        }
    }
}
