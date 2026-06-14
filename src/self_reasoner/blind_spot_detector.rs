use crate::kernel_core::event::EventPayload;

pub struct BlindSpotDetector;

impl BlindSpotDetector {
    /// Detects if a knowledge domain is completely absent.
    pub fn detect(domains_covered: &[String], all_domains: &[String]) -> Vec<String> {
        all_domains.iter()
            .filter(|d| !domains_covered.contains(d))
            .cloned()
            .collect()
    }

    pub fn propose_blind_spot(domains: Vec<String>) -> EventPayload {
        let domains_str = domains.join(",");
        EventPayload::Custom {
            event_type: "BlindSpotDetected".into(),
            data: domains_str.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_missing_domains() {
        let covered = vec!["memory".into()];
        let all = vec!["memory".into(), "world_model".into()];
        let spots = BlindSpotDetector::detect(&covered, &all);
        assert_eq!(spots, vec!["world_model"]);
    }
}
