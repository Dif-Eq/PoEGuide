use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::Act;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SaveState {
    /// key = `"act_index:zone_index:step_index"` -> checked
    pub checks: HashMap<String, bool>,
}

impl SaveState {
    #[must_use]
    pub fn save_path() -> Option<std::path::PathBuf> {
        dirs::data_local_dir().map(|d| d.join("poe2_guide").join("progress.json"))
    }

    #[must_use]
    pub fn load() -> Self {
        if let Some(path) = Self::save_path() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str(&data) {
                    return state;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::save_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(data) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(path, data);
            }
        }
    }

    #[must_use]
    pub fn key(act: usize, zone: usize, step: usize) -> String {
        format!("{act}:{zone}:{step}")
    }

    #[must_use]
    pub fn is_checked(&self, act: usize, zone: usize, step: usize) -> bool {
        *self.checks.get(&Self::key(act, zone, step)).unwrap_or(&false)
    }

    pub fn toggle(&mut self, act: usize, zone: usize, step: usize) {
        let k = Self::key(act, zone, step);
        let v = self.checks.entry(k).or_insert(false);
        *v = !*v;
    }

    #[must_use]
    pub fn act_progress(&self, act_idx: usize, act: &Act) -> (usize, usize) {
        let mut done = 0;
        let mut total = 0;
        for (zi, zone) in act.zones.iter().enumerate() {
            if zone.name == "TIP" { continue; }
            for (si, _) in zone.steps.iter().enumerate() {
                total += 1;
                if self.is_checked(act_idx, zi, si) {
                    done += 1;
                }
            }
        }
        (done, total)
    }

    pub fn reset_act(&mut self, act_idx: usize, act: &Act) {
        for (zi, zone) in act.zones.iter().enumerate() {
            if zone.name == "TIP" { continue; }
            for (si, _) in zone.steps.iter().enumerate() {
                self.checks.remove(&Self::key(act_idx, zi, si));
            }
        }
    }

    /// Returns `(act_idx, zone_idx, step_idx)` of the first unchecked step globally
    #[must_use]
    pub fn first_unchecked(&self, acts: &[Act]) -> Option<(usize, usize, usize)> {
        for (ai, act) in acts.iter().enumerate() {
            for (zi, zone) in act.zones.iter().enumerate() {
                if zone.name == "TIP" { continue; }
                for (si, _) in zone.steps.iter().enumerate() {
                    if !self.is_checked(ai, zi, si) {
                        return Some((ai, zi, si));
                    }
                }
            }
        }
        None
    }

    /// Returns up to `count` unchecked steps starting from the first unchecked,
    /// as `(act_idx, zone_idx, step_idx)` tuples.
    #[must_use]
    pub fn next_unchecked(&self, acts: &[Act], count: usize) -> Vec<(usize, usize, usize)> {
        let mut results = Vec::new();
        let mut collecting = false;

        'outer: for (ai, act) in acts.iter().enumerate() {
            for (zi, zone) in act.zones.iter().enumerate() {
                if zone.name == "TIP" { continue; }
                for (si, _) in zone.steps.iter().enumerate() {
                    if !self.is_checked(ai, zi, si) {
                        collecting = true;
                    }
                    if collecting && !self.is_checked(ai, zi, si) {
                        results.push((ai, zi, si));
                        if results.len() >= count {
                            break 'outer;
                        }
                    }
                }
            }
        }
        results
    }
}
