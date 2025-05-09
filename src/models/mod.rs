//! Data structures corresponding to Space‑Track API classes.
//!
//! Only a subset of fields are enumerated for brevity; the rest are captured in
//! `extra` maps if necessary. PRs welcome for full schemas.

use serde::Deserialize;
use std::collections::HashMap;

/// Macro to capture unknown JSON keys into an `extra` map without manual `#[serde(flatten)]` each time.
macro_rules! common {
    ($name:ident { $($field:ident : $t:ty),* $(,)? }) => {
        #[derive(Debug, Deserialize)]
        pub struct $name {
            $(pub $field: $t,)*
            #[serde(flatten)]
            pub extra: HashMap<String, serde_json::Value>,
        }
    };
}

// ================= basicspacedata =================

common!(SatCatEntry {
    OBJECT_NAME: String,
    OBJECT_ID: String,
    NORAD_CAT_ID: u32,
    COUNTRY_CODE: String,
    LAUNCH_DATE: Option<chrono::NaiveDate>,
    DECAY_DATE: Option<chrono::NaiveDate>,
    OBJECT_TYPE: String,
});

common!(GpEntry {
    NORAD_CAT_ID: u32,
    EPOCH: chrono::DateTime<chrono::Utc>,
    MEAN_MOTION: f64,
    ECCENTRICITY: f64,
    INCLINATION: f64,
});

common!(GpHistoryEntry {
    NORAD_CAT_ID: u32,
    EPOCH: chrono::DateTime<chrono::Utc>,
});

common!(BoxScore {
    COUNTRY: String,
    COUNTRY_TOTAL: u32,
});

common!(DecayEntry {
    NORAD_CAT_ID: u32,
    DECAY_EPOCH: chrono::DateTime<chrono::Utc>,
    PRECEDENCE: i8,
});
