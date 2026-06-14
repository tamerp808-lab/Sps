// src/world_model/environment.rs
// Phase 4 — World Model
// Zone B — Cognitive
//
// Purpose:
//   Environment models the external execution context of SPS:
//   the operating system, hardware resources, and network state.
//   It bridges the platform layer (Zone C) with the world model
//   (Zone B) by producing Events that describe environment changes.
//   It never mutates state directly.
//
// Constitution Compliance:
//   - المادة الخامسة عشرة (World Model Constitution)
//   - المادة الحادية عشرة (Platform Law): kernel unaware of platform,
//     but world model CAN represent it abstractly.
//   - Zone B: reads State, produces Events

use crate::kernel_core::event::EventPayload;

/// Types of environments SPS can operate in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentType {
    Linux,
    Android,
    Windows,
    BareMetal,
    Unknown,
}

/// A snapshot of resource availability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceSnapshot {
    pub cpu_available_mhz: u64,
    pub memory_available_bytes: u64,
    pub storage_available_bytes: u64,
    pub network_available: bool,
    pub battery_pct: Option<u8>,
}

/// The current environment state as known to the world model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    pub env_type: EnvironmentType,
    pub hostname: String,
    pub resources: ResourceSnapshot,
    pub uptime_ticks: u64,
}

pub struct EnvironmentModel;

impl EnvironmentModel {
    /// Proposes recording the initial environment state at boot.
    pub fn propose_boot(
        env_type: EnvironmentType,
        hostname: String,
        resources: ResourceSnapshot,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "EnvironmentBoot".into(),
            data: format!(
                "{:?}|{}|{}|{}|{}|{}|{:?}",
                env_type,
                hostname,
                resources.cpu_available_mhz,
                resources.memory_available_bytes,
                resources.storage_available_bytes,
                resources.network_available,
                resources.battery_pct,
            )
            .into_bytes(),
        }
    }

    /// Proposes updating resource availability.
    pub fn propose_resource_update(resources: ResourceSnapshot) -> EventPayload {
        EventPayload::Custom {
            event_type: "EnvironmentResourceUpdate".into(),
            data: format!(
                "{}|{}|{}|{}|{:?}",
                resources.cpu_available_mhz,
                resources.memory_available_bytes,
                resources.storage_available_bytes,
                resources.network_available,
                resources.battery_pct,
            )
            .into_bytes(),
        }
    }

    /// Proposes recording a platform change (e.g., migration).
    pub fn propose_platform_change(
        new_env_type: EnvironmentType,
        reason: String,
    ) -> EventPayload {
        EventPayload::Custom {
            event_type: "EnvironmentPlatformChange".into(),
            data: format!("{:?}|{}", new_env_type, reason).into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_boot_creates_event() {
        let payload = EnvironmentModel::propose_boot(
            EnvironmentType::Linux,
            "termux".into(),
            ResourceSnapshot {
                cpu_available_mhz: 2400,
                memory_available_bytes: 8_000_000_000,
                storage_available_bytes: 64_000_000_000,
                network_available: true,
                battery_pct: Some(85),
            },
        );
        match payload {
            EventPayload::Custom { event_type, data } => {
                assert!(event_type.contains("EnvironmentBoot"));
                let s = String::from_utf8(data).unwrap();
                assert!(s.contains("Linux"));
            }
            _ => panic!("Wrong payload"),
        }
    }

    #[test]
    fn propose_resource_update_creates_event() {
        let payload = EnvironmentModel::propose_resource_update(ResourceSnapshot {
            cpu_available_mhz: 1200,
            memory_available_bytes: 4_000_000_000,
            storage_available_bytes: 32_000_000_000,
            network_available: false,
            battery_pct: Some(10),
        });
        match payload {
            EventPayload::Custom { event_type, .. } => {
                assert!(event_type.contains("EnvironmentResourceUpdate"));
            }
            _ => panic!("Wrong payload"),
        }
    }
}
