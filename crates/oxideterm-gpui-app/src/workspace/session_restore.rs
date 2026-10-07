use std::{
    fs, io,
    path::{Path, PathBuf},
};

use super::*;

use serde::{Deserialize, Serialize};

const VERSION: u32 = 1;
const MAX_BYTES: usize = 1024 * 1024;
const MAX_TABS: usize = 128;
const MAX_PANES: usize = 512;
const MAX_DEPTH: usize = 16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct Snapshot {
    version: u32,
    pub(super) tabs: Vec<SavedTab>,
    pub(super) active_tab: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct SavedTab {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) root: SavedLayout,
    pub(super) active_pane: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum SavedLayout {
    Terminal {
        pane_id: String,
        target: RestoreTarget,
    },
    Split {
        direction: SplitDirection,
        children: Vec<SavedChild>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct SavedChild {
    pub(super) ratio: f32,
    pub(super) layout: SavedLayout,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(super) enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum RestoreTarget {
    SavedLocal {
        profile_id: String,
        shell_id: String,
        cwd: Option<String>,
    },
    Local {
        shell_id: String,
        cwd: Option<String>,
    },
    SavedSsh {
        profile_id: String,
        node_id: String,
    },
    TemporarySsh {
        node_id: String,
        host: String,
        port: u16,
        username: String,
    },
}

impl Snapshot {
    pub(super) fn new(tabs: Vec<SavedTab>, active_tab: Option<String>) -> io::Result<Self> {
        let snapshot = Self {
            version: VERSION,
            tabs,
            active_tab,
        };
        snapshot.validate()?;
        Ok(snapshot)
    }

    fn validate(&self) -> io::Result<()> {
        if self.version != VERSION || self.tabs.len() > MAX_TABS {
            return Err(invalid(
                "unsupported or oversized terminal workspace snapshot",
            ));
        }
        if self
            .active_tab
            .as_ref()
            .is_some_and(|id| !self.tabs.iter().any(|tab| &tab.id == id))
        {
            return Err(invalid("active terminal tab is not present in snapshot"));
        }
        for tab in &self.tabs {
            let mut tab_panes = 0;
            validate_layout(&tab.root, 0, &mut tab_panes)?;
            if tab_panes > oxideterm_workspace::MAX_PANES_PER_TAB {
                return Err(invalid("terminal tab exceeds the pane limit"));
            }
            if !layout_has_pane(&tab.root, &tab.active_pane) {
                return Err(invalid("active terminal pane is not present in tab"));
            }
        }
        Ok(())
    }
}

fn validate_layout(layout: &SavedLayout, depth: usize, panes: &mut usize) -> io::Result<()> {
    if depth > MAX_DEPTH {
        return Err(invalid("terminal layout nesting exceeds limit"));
    }
    match layout {
        SavedLayout::Terminal { pane_id, target } => {
            *panes += 1;
            if *panes > MAX_PANES || pane_id.is_empty() {
                return Err(invalid("invalid terminal pane count or identity"));
            }
            match target {
                RestoreTarget::SavedLocal {
                    profile_id,
                    shell_id,
                    ..
                } if profile_id.is_empty() || shell_id.is_empty() => {
                    Err(invalid("invalid saved local terminal identity"))
                }
                RestoreTarget::Local { shell_id, .. } if shell_id.is_empty() => {
                    Err(invalid("invalid local shell identity"))
                }
                RestoreTarget::SavedSsh {
                    profile_id,
                    node_id,
                } if profile_id.is_empty() || node_id.is_empty() => {
                    Err(invalid("invalid saved SSH terminal identity"))
                }
                RestoreTarget::TemporarySsh {
                    node_id,
                    host,
                    username,
                    port,
                } if node_id.is_empty() || host.is_empty() || username.is_empty() || *port == 0 => {
                    Err(invalid("invalid temporary SSH terminal identity"))
                }
                _ => Ok(()),
            }
        }
        SavedLayout::Split { children, .. } => {
            if children.len() < 2 {
                return Err(invalid("terminal split requires at least two children"));
            }
            let sum: f32 = children.iter().map(|child| child.ratio).sum();
            if children
                .iter()
                .any(|child| !child.ratio.is_finite() || child.ratio <= 0.0)
                || !sum.is_finite()
                || sum <= 0.0
            {
                return Err(invalid("invalid terminal split proportions"));
            }
            for child in children {
                validate_layout(&child.layout, depth + 1, panes)?;
            }
            Ok(())
        }
    }
}

fn layout_has_pane(layout: &SavedLayout, id: &str) -> bool {
    match layout {
        SavedLayout::Terminal { pane_id, .. } => pane_id == id,
        SavedLayout::Split { children, .. } => children
            .iter()
            .any(|child| layout_has_pane(&child.layout, id)),
    }
}

fn layout_first_pane(layout: &SavedLayout) -> Option<String> {
    match layout {
        SavedLayout::Terminal { pane_id, .. } => Some(pane_id.clone()),
        SavedLayout::Split { children, .. } => children
            .first()
            .and_then(|child| layout_first_pane(&child.layout)),
    }
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

pub(super) fn snapshot_path(settings_path: &Path) -> PathBuf {
    settings_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("terminal_workspace.json")
}

pub(super) fn load(path: &Path) -> io::Result<Option<Snapshot>> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if bytes.len() > MAX_BYTES {
        return Err(invalid("terminal workspace snapshot exceeds size limit"));
    }
    let snapshot: Snapshot = serde_json::from_slice(&bytes)
        .map_err(|_| invalid("terminal workspace snapshot is invalid"))?;
    snapshot.validate()?;
    Ok(Some(snapshot))
}

pub(super) fn save(path: &Path, snapshot: &Snapshot) -> io::Result<()> {
    snapshot.validate()?;
    let bytes = serde_json::to_vec(snapshot)
        .map_err(|_| invalid("terminal workspace snapshot cannot be serialized"))?;
    if bytes.len() > MAX_BYTES {
        return Err(invalid("terminal workspace snapshot exceeds size limit"));
    }
    oxideterm_atomic_file::durable_write(path, &bytes)
}

#[derive(Clone)]
struct RestoredLocalLayout {
    tab_id: TabId,
    active_pane: PaneId,
    panes: Vec<(String, PaneId)>,
}

pub(super) struct PendingRestore {
    snapshot: Snapshot,
    next_tab: usize,
    restored_tab_ids: Vec<(String, TabId)>,
    ready_ssh_nodes: std::collections::HashMap<String, NodeId>,
    unavailable_ssh_profiles: std::collections::HashSet<String>,
}

impl PendingRestore {
    fn new(snapshot: Snapshot) -> Self {
        Self {
            snapshot,
            next_tab: 0,
            restored_tab_ids: Vec::new(),
            ready_ssh_nodes: std::collections::HashMap::new(),
            unavailable_ssh_profiles: std::collections::HashSet::new(),
        }
    }

    fn mark_unavailable(&mut self, profile_id: String) {
        self.unavailable_ssh_profiles.insert(profile_id);
    }

    pub(super) fn wants_node(&self, _node_id: &NodeId, profile_id: Option<&str>) -> bool {
        profile_id.is_some_and(|profile_id| {
            self.snapshot
                .tabs
                .iter()
                .skip(self.next_tab)
                .any(|tab| saved_layout_contains_ssh_profile(&tab.root, Some(profile_id)))
        })
    }
}

fn saved_layout_contains_ssh_profile(layout: &SavedLayout, profile_id: Option<&str>) -> bool {
    match layout {
        SavedLayout::Terminal {
            target: RestoreTarget::SavedSsh {
                profile_id: saved, ..
            },
            ..
        } => profile_id == Some(saved.as_str()),
        SavedLayout::Terminal { .. } => false,
        SavedLayout::Split { children, .. } => children
            .iter()
            .any(|child| saved_layout_contains_ssh_profile(&child.layout, profile_id)),
    }
}

fn saved_layout_ssh_profiles(layout: &SavedLayout) -> std::collections::HashSet<String> {
    let mut profiles = std::collections::HashSet::new();
    collect_saved_ssh_profiles(layout, &mut profiles);
    profiles
}

fn collect_saved_ssh_profiles(
    layout: &SavedLayout,
    profiles: &mut std::collections::HashSet<String>,
) {
    match layout {
        SavedLayout::Terminal {
            target: RestoreTarget::SavedSsh { profile_id, .. },
            ..
        } => {
            profiles.insert(profile_id.clone());
        }
        SavedLayout::Terminal { .. } => {}
        SavedLayout::Split { children, .. } => {
            for child in children {
                collect_saved_ssh_profiles(&child.layout, profiles);
            }
        }
    }
}

fn snapshot_saved_ssh_profiles(snapshot: &Snapshot) -> Vec<String> {
    let mut profiles = std::collections::HashSet::new();
    for tab in &snapshot.tabs {
        profiles.extend(saved_layout_ssh_profiles(&tab.root));
    }
    profiles.into_iter().collect()
}

fn layout_contains_temporary_ssh(layout: &SavedLayout) -> bool {
    match layout {
        SavedLayout::Terminal {
            target: RestoreTarget::TemporarySsh { .. },
            ..
        } => true,
        SavedLayout::Terminal { .. } => false,
        SavedLayout::Split { children, .. } => children
            .iter()
            .any(|child| layout_contains_temporary_ssh(&child.layout)),
    }
}

impl WorkspaceApp {
    pub(super) fn save_terminal_workspace_snapshot(&self, cx: &App) -> io::Result<()> {
        let snapshot = self.capture_terminal_workspace_snapshot(cx)?;
        save(&snapshot_path(self.settings_store.path()), &snapshot)
    }

    fn capture_terminal_workspace_snapshot(&self, cx: &App) -> io::Result<Snapshot> {
        let host = self.tab_host.read(cx);
        let nodes: Vec<_> = self
            .ssh_nodes
            .iter()
            .map(|(id, node)| {
                (
                    id.0.clone(),
                    node.saved_connection_id.clone(),
                    node.terminal_ids.clone(),
                    node.endpoint.host.clone(),
                    node.endpoint.port,
                    node.endpoint.username.clone(),
                )
            })
            .collect();
        let mut tabs = Vec::new();
        for tab in host.tabs() {
            let Some(root) = tab.root_pane.as_ref() else {
                continue;
            };
            let mut pane_sessions = Vec::new();
            root.collect_session_ids(&mut pane_sessions);
            if pane_sessions.is_empty() {
                continue;
            }
            let root = capture_layout(root, &host, &nodes, &mut 0usize, cx)?;
            let active_pane = tab
                .active_pane_id
                .map(|id| id.0.to_string())
                .filter(|id| layout_has_pane(&root, id))
                .or_else(|| layout_first_pane(&root));
            let Some(active_pane) = active_pane else {
                continue;
            };
            tabs.push(SavedTab {
                id: tab.id.0.to_string(),
                title: tab.title.clone(),
                root,
                active_pane,
            });
        }
        let active_tab = host
            .active_tab_id()
            .map(|id| id.0.to_string())
            .filter(|id| tabs.iter().any(|tab| &tab.id == id));
        Snapshot::new(tabs, active_tab)
    }
    pub(super) fn restore_terminal_workspace(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self
            .settings_store
            .settings()
            .general
            .restore_terminal_workspace
        {
            return;
        }
        let snapshot = match load(&snapshot_path(self.settings_store.path())) {
            Ok(Some(snapshot)) => snapshot,
            Ok(None) => return,
            Err(error) => {
                self.report_restore_error(
                    format!("Terminal workspace restore failed: {error}"),
                    cx,
                );
                return;
            }
        };
        let profile_ids = snapshot_saved_ssh_profiles(&snapshot);
        let missing_profiles = profile_ids
            .iter()
            .filter(|profile_id| self.connection_store.get(profile_id).is_none())
            .cloned()
            .collect::<Vec<_>>();
        self.pending_terminal_workspace_restore = Some(PendingRestore::new(snapshot));
        if let Some(pending) = self.pending_terminal_workspace_restore.as_mut() {
            for profile_id in &missing_profiles {
                pending.mark_unavailable(profile_id.clone());
            }
        }
        self.finish_pending_terminal_restore(window, cx);
        for profile_id in profile_ids {
            if !missing_profiles.contains(&profile_id) {
                self.open_saved_connection(&profile_id, window, cx);
            } else {
                self.report_restore_error(
                    format!("Saved SSH profile '{profile_id}' no longer exists"),
                    cx,
                );
            }
        }
        self.finish_pending_terminal_restore(window, cx);
    }

    pub(super) fn restore_ready_ssh_node(
        &mut self,
        profile_id: Option<String>,
        node_id: NodeId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(mut pending) = self.pending_terminal_workspace_restore.take() else {
            return;
        };
        let profile_id = profile_id.or_else(|| {
            self.ssh_nodes
                .get(&node_id)
                .and_then(|node| node.saved_connection_id.clone())
        });
        if let Some(profile_id) = profile_id {
            pending.ready_ssh_nodes.insert(profile_id, node_id);
        }
        self.pending_terminal_workspace_restore = Some(pending);
        self.finish_pending_terminal_restore(window, cx);
    }

    fn finish_pending_terminal_restore(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(mut pending) = self.pending_terminal_workspace_restore.take() else {
            return;
        };
        while pending.next_tab < pending.snapshot.tabs.len() {
            let tab = pending.snapshot.tabs[pending.next_tab].clone();
            if layout_contains_temporary_ssh(&tab.root) {
                self.report_restore_error(
                    "Temporary SSH terminal cannot be restored without its saved authentication configuration".into(),
                    cx,
                );
                pending.next_tab += 1;
                continue;
            }
            let required_profiles = saved_layout_ssh_profiles(&tab.root);
            if required_profiles
                .iter()
                .any(|profile_id| pending.unavailable_ssh_profiles.contains(profile_id))
            {
                pending.next_tab += 1;
                continue;
            }
            if required_profiles
                .iter()
                .any(|profile_id| !pending.ready_ssh_nodes.contains_key(profile_id))
            {
                break;
            }
            match self.restore_saved_layout(
                &tab.root,
                &tab.title,
                &pending.ready_ssh_nodes,
                window,
                cx,
            ) {
                Ok(restored) => {
                    let active_pane = restored
                        .panes
                        .iter()
                        .find(|(saved, _)| saved == &tab.active_pane)
                        .map(|(_, pane)| *pane)
                        .unwrap_or(restored.active_pane);
                    self.tab_host.update(cx, |host, _| {
                        host.set_active_pane(Some(restored.tab_id), active_pane);
                        host.rename_terminal_tab(restored.tab_id, &tab.title);
                    });
                    pending.restored_tab_ids.push((tab.id, restored.tab_id));
                }
                Err(error) => self.report_restore_error(error.to_string(), cx),
            }
            pending.next_tab += 1;
        }
        if pending.next_tab == pending.snapshot.tabs.len() {
            if let Some((_, active_tab_id)) = pending
                .snapshot
                .active_tab
                .as_ref()
                .and_then(|active| pending.restored_tab_ids.iter().find(|(id, _)| id == active))
            {
                self.set_main_window_active_tab(Some(*active_tab_id), cx);
            }
        } else {
            self.pending_terminal_workspace_restore = Some(pending);
        }
    }

    fn restore_saved_layout(
        &mut self,
        layout: &SavedLayout,
        title: &str,
        ready_ssh_nodes: &std::collections::HashMap<String, NodeId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<RestoredLocalLayout> {
        match layout {
            SavedLayout::Terminal {
                pane_id: saved_pane_id,
                target,
            } => {
                let (tab_id, runtime_pane_id) = match target {
                    RestoreTarget::Local { shell_id, cwd } => self.restore_local_terminal(
                        Some(shell_id),
                        cwd.as_deref(),
                        title,
                        None,
                        window,
                        cx,
                    )?,
                    RestoreTarget::SavedLocal {
                        profile_id, cwd, ..
                    } => {
                        let profile = self
                            .connection_store
                            .local_terminal_profiles()
                            .iter()
                            .find(|profile| profile.id.as_str() == profile_id)
                            .cloned()
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "saved local terminal profile '{profile_id}' no longer exists"
                                )
                            })?;
                        self.restore_local_terminal(
                            profile.shell_id.as_deref(),
                            cwd.as_deref().or(profile.cwd.as_deref()),
                            &profile.name,
                            Some(profile.id),
                            window,
                            cx,
                        )?
                    }
                    RestoreTarget::SavedSsh { profile_id, .. } => {
                        let node_id = ready_ssh_nodes
                            .get(profile_id)
                            .ok_or_else(|| {
                                anyhow::anyhow!("saved SSH profile '{profile_id}' is not connected")
                            })?
                            .clone();
                        self.create_ssh_terminal_tab_for_existing_node_with_policy(
                            &node_id,
                            None,
                            title.to_owned(),
                            false,
                            window,
                            cx,
                        )?;
                        let tab_id = self
                            .active_tab_id(cx)
                            .ok_or_else(|| anyhow::anyhow!("restored SSH terminal has no tab"))?;
                        let pane_id = self
                            .tab_by_id(tab_id, cx)
                            .and_then(|tab| tab.root_pane.as_ref())
                            .map(oxideterm_workspace::PaneNode::first_pane_id)
                            .ok_or_else(|| anyhow::anyhow!("restored SSH terminal has no pane"))?;
                        (tab_id, pane_id)
                    }
                    RestoreTarget::TemporarySsh { .. } => {
                        return Err(anyhow::anyhow!("temporary SSH terminal cannot be restored"));
                    }
                };
                Ok(RestoredLocalLayout {
                    tab_id,
                    active_pane: runtime_pane_id,
                    panes: vec![(saved_pane_id.clone(), runtime_pane_id)],
                })
            }
            SavedLayout::Split {
                direction,
                children,
            } => {
                let first = children
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("terminal split has no children"))?;
                let mut combined =
                    self.restore_saved_layout(&first.layout, title, ready_ssh_nodes, window, cx)?;
                let split_direction = match direction {
                    SplitDirection::Horizontal => oxideterm_workspace::SplitDirection::Horizontal,
                    SplitDirection::Vertical => oxideterm_workspace::SplitDirection::Vertical,
                };
                let mut sizes = vec![first.ratio];
                for child in children.iter().skip(1) {
                    let next = self.restore_saved_layout(
                        &child.layout,
                        title,
                        ready_ssh_nodes,
                        window,
                        cx,
                    )?;
                    if !self.combine_tabs_at(
                        next.tab_id,
                        combined.tab_id,
                        None,
                        split_direction,
                        false,
                        window,
                        cx,
                    ) {
                        return Err(anyhow::anyhow!("failed to rebuild terminal split"));
                    }
                    combined.tab_id = self
                        .active_tab_id(cx)
                        .ok_or_else(|| anyhow::anyhow!("combined terminal split has no tab"))?;
                    combined.panes.extend(next.panes);
                    sizes.push(child.ratio);
                    self.tab_host.update(cx, |host, _| {
                        host.update_tab_root_sizes(combined.tab_id, &sizes);
                    });
                }
                Ok(combined)
            }
        }
    }

    fn restore_local_terminal(
        &mut self,
        shell_id: Option<&str>,
        cwd: Option<&str>,
        title: &str,
        profile_id: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(TabId, PaneId)> {
        let config = self.local_profile_config(shell_id, cwd, None)?;
        let (session_id, _) = self.create_local_terminal_tab_with_owned_session(
            config,
            title.to_owned(),
            window,
            cx,
        )?;
        if let Some(profile_id) = profile_id {
            self.tab_host.update(cx, |host, _| {
                if let Some(instance) = host.local_sessions.get_mut(&session_id) {
                    instance.profile_id = Some(profile_id);
                }
            });
        }
        let tab_id = self
            .active_tab_id(cx)
            .ok_or_else(|| anyhow::anyhow!("restored local terminal has no tab"))?;
        let pane_id = self
            .tab_by_id(tab_id, cx)
            .and_then(|tab| tab.root_pane.as_ref())
            .map(oxideterm_workspace::PaneNode::first_pane_id)
            .ok_or_else(|| anyhow::anyhow!("restored local terminal has no pane"))?;
        Ok((tab_id, pane_id))
    }

    fn report_restore_error(&self, message: String, cx: &mut Context<Self>) {
        self.session_manager
            .update(cx, |manager, cx| manager.set_status(Some(message), cx));
    }
}

fn capture_layout(
    node: &oxideterm_workspace::PaneNode,
    host: &tabs::WorkspaceTabHostEntity,
    nodes: &[(
        String,
        Option<String>,
        Vec<TerminalSessionId>,
        String,
        u16,
        String,
    )],
    pane_count: &mut usize,
    cx: &App,
) -> io::Result<SavedLayout> {
    match node {
        oxideterm_workspace::PaneNode::Page { .. } => {
            Err(invalid("tool pages cannot be persisted as terminal panes"))
        }
        oxideterm_workspace::PaneNode::Leaf {
            pane_id,
            session_id,
        } => {
            *pane_count += 1;
            let target = if let Some(instance) = host.local_sessions.get(session_id) {
                let shell_id = instance
                    .shell
                    .as_ref()
                    .map(|shell| shell.id.clone())
                    .ok_or_else(|| invalid("local terminal shell identity is unavailable"))?;
                let cwd = host.panes().get(pane_id).and_then(|pane| {
                    terminal_cwd::terminal_cwd_snapshot_from_pane(
                        oxideterm_environment::CurrentDirectoryScope::Local,
                        pane.read(cx),
                    )
                });
                if let Some(profile_id) = &instance.profile_id {
                    RestoreTarget::SavedLocal {
                        profile_id: profile_id.clone(),
                        shell_id,
                        cwd: cwd.map(|value| value.path().to_owned()).or_else(|| {
                            instance
                                .cwd
                                .as_ref()
                                .map(|value| value.to_string_lossy().into_owned())
                        }),
                    }
                } else {
                    RestoreTarget::Local {
                        shell_id,
                        cwd: cwd.map(|value| value.path().to_owned()).or_else(|| {
                            instance
                                .cwd
                                .as_ref()
                                .map(|value| value.to_string_lossy().into_owned())
                        }),
                    }
                }
            } else {
                let (node_id, profile_id, _terminals, host_name, port, username) = nodes
                    .iter()
                    .find(|(_, _, terminals, _, _, _)| terminals.contains(session_id))
                    .ok_or_else(|| invalid("SSH terminal has no workspace-owned node"))?;
                if let Some(profile_id) = profile_id {
                    RestoreTarget::SavedSsh {
                        profile_id: profile_id.clone(),
                        node_id: node_id.clone(),
                    }
                } else {
                    RestoreTarget::TemporarySsh {
                        node_id: node_id.clone(),
                        host: host_name.clone(),
                        port: *port,
                        username: username.clone(),
                    }
                }
            };
            Ok(SavedLayout::Terminal {
                pane_id: pane_id.0.to_string(),
                target,
            })
        }
        oxideterm_workspace::PaneNode::Group {
            direction,
            children,
            ..
        } => {
            let direction = match direction {
                oxideterm_workspace::SplitDirection::Horizontal => SplitDirection::Horizontal,
                oxideterm_workspace::SplitDirection::Vertical => SplitDirection::Vertical,
            };
            let mut saved = Vec::new();
            for child in children {
                let mut sessions = Vec::new();
                child.node.collect_session_ids(&mut sessions);
                // Tool pages share split trees but have no terminal restore target.
                if sessions.is_empty() {
                    continue;
                }
                let layout = capture_layout(&child.node, host, nodes, pane_count, cx)?;
                saved.push(SavedChild {
                    ratio: child.size,
                    layout,
                });
            }
            match saved.len() {
                0 => Err(invalid("layout contains no restorable terminals")),
                1 => Ok(saved.remove(0).layout),
                _ => Ok(SavedLayout::Split {
                    direction,
                    children: saved,
                }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Snapshot {
        Snapshot::new(
            vec![SavedTab {
                id: "tab-1".into(),
                title: "shells".into(),
                active_pane: "pane-b".into(),
                root: SavedLayout::Split {
                    direction: SplitDirection::Vertical,
                    children: vec![
                        SavedChild {
                            ratio: 0.35,
                            layout: SavedLayout::Terminal {
                                pane_id: "pane-a".into(),
                                target: RestoreTarget::SavedLocal {
                                    profile_id: "profile-1".into(),
                                    shell_id: "pwsh".into(),
                                    cwd: Some("C:/work".into()),
                                },
                            },
                        },
                        SavedChild {
                            ratio: 0.65,
                            layout: SavedLayout::Terminal {
                                pane_id: "pane-b".into(),
                                target: RestoreTarget::SavedSsh {
                                    profile_id: "ssh-2".into(),
                                    node_id: "node-2".into(),
                                },
                            },
                        },
                    ],
                },
            }],
            Some("tab-1".into()),
        )
        .unwrap()
    }

    #[gpui::test]
    fn mixed_page_groups_preserve_terminal_snapshot_layout(cx: &mut gpui::TestAppContext) {
        use oxideterm_workspace::{PaneNode, PaneSplitChild};

        let page = |id| PaneNode::Page {
            pane_id: PaneId(id),
            tab_id: TabId(id),
        };
        let root = PaneNode::Group {
            id: PaneId(10),
            direction: oxideterm_workspace::SplitDirection::Horizontal,
            children: vec![
                PaneSplitChild::new(
                    PaneNode::Group {
                        id: PaneId(11),
                        direction: oxideterm_workspace::SplitDirection::Vertical,
                        children: vec![
                            PaneSplitChild::new(page(3), 0.5),
                            PaneSplitChild::new(page(4), 0.5),
                        ],
                    },
                    0.2,
                ),
                PaneSplitChild::new(
                    PaneNode::Group {
                        id: PaneId(12),
                        direction: oxideterm_workspace::SplitDirection::Vertical,
                        children: vec![
                            PaneSplitChild::new(page(5), 0.5),
                            PaneSplitChild::new(
                                PaneNode::leaf(PaneId(1), TerminalSessionId(1)),
                                0.5,
                            ),
                        ],
                    },
                    0.3,
                ),
                PaneSplitChild::new(PaneNode::leaf(PaneId(2), TerminalSessionId(2)), 0.5),
            ],
        };
        let host = tabs::WorkspaceTabHostEntity::new();
        let nodes = vec![(
            "node-1".into(),
            Some("profile-1".into()),
            vec![TerminalSessionId(1), TerminalSessionId(2)],
            "example.test".into(),
            22,
            "user".into(),
        )];
        let layout = cx.update(|cx| capture_layout(&root, &host, &nodes, &mut 0, cx).unwrap());
        assert_eq!(
            layout,
            SavedLayout::Split {
                direction: SplitDirection::Horizontal,
                children: [("1", 0.3), ("2", 0.5)]
                    .into_iter()
                    .map(|(pane_id, ratio)| SavedChild {
                        ratio,
                        layout: SavedLayout::Terminal {
                            pane_id: pane_id.into(),
                            target: RestoreTarget::SavedSsh {
                                profile_id: "profile-1".into(),
                                node_id: "node-1".into(),
                            },
                        },
                    })
                    .collect(),
            }
        );
    }

    #[test]
    fn explicit_id_order_and_split_layout_survive_storage() {
        let snapshot = fixture();
        assert_eq!(snapshot.tabs[0].id, "tab-1");
        assert_eq!(snapshot.tabs[0].active_pane, "pane-b");
        let SavedLayout::Split {
            direction,
            children,
        } = &snapshot.tabs[0].root
        else {
            panic!("split expected")
        };
        assert_eq!(*direction, SplitDirection::Vertical);
        assert_eq!(
            children.iter().map(|child| child.ratio).collect::<Vec<_>>(),
            vec![0.35, 0.65]
        );
        assert!(
            matches!(&children[0].layout, SavedLayout::Terminal { pane_id, target: RestoreTarget::SavedLocal { profile_id, .. } } if pane_id == "pane-a" && profile_id == "profile-1")
        );
        let dir = std::env::temp_dir().join(format!(
            "oxideterm-restore-roundtrip-{}",
            uuid::Uuid::new_v4()
        ));
        let path = dir.join("terminal_workspace.json");
        save(&path, &snapshot).unwrap();
        assert_eq!(load(&path).unwrap(), Some(snapshot));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn serialization_omits_runtime_secrets_and_command_environment() {
        let mut value = serde_json::to_value(fixture()).unwrap();
        let text = value.to_string();
        for secret in ["password", "postcmd", "environment", "command"] {
            assert!(!text.contains(secret));
        }
        assert!(text.contains("profile-1"));
        value
            .as_object_mut()
            .unwrap()
            .insert("version".into(), serde_json::json!(99));
        assert!(
            serde_json::from_value::<Snapshot>(value)
                .unwrap()
                .validate()
                .is_err()
        );
    }

    #[test]
    fn failed_atomic_replace_preserves_previous_complete_snapshot() {
        let dir = std::env::temp_dir().join(format!("oxideterm-restore-{}", uuid::Uuid::new_v4()));
        let path = dir.join("terminal_workspace.json");
        save(&path, &fixture()).unwrap();
        let previous = fs::read(&path).unwrap();
        assert!(
            oxideterm_atomic_file::durable_write_with_before_replace(&path, b"incomplete", || Err(
                io::Error::other("injected replace failure")
            ))
            .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), previous);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn rejects_invalid_proportions_and_excessive_nesting() {
        let mut snapshot = fixture();
        if let SavedLayout::Split { children, .. } = &mut snapshot.tabs[0].root {
            children[0].ratio = f32::NAN;
        }
        assert!(snapshot.validate().is_err());
    }
}
