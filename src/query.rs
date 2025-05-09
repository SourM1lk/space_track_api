//! Fluent builder used to construct and execute Space-Track queries.

use crate::{client::BASE_URL, error::Error, format::Format};
use serde::de::DeserializeOwned;
use std::fmt::Display;

/// Generic query builder parameterised on the expected output type `T`.
#[derive(Debug)]
pub struct QueryBuilder<'a, T> {
    client: &'a crate::client::SpaceTrackClient,
    controller: &'static str,
    class: &'static str,
    segments: Vec<String>,
    format: Format,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> QueryBuilder<'a, T> {
    pub(crate) fn new(
        client: &'a crate::client::SpaceTrackClient,
        controller: &'static str,
        class: &'static str,
    ) -> Self {
        Self {
            client,
            controller,
            class,
            segments: Vec::new(),
            format: Format::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    // ---------------------------------------------------------------------
    // Builder modifiers
    // ---------------------------------------------------------------------

    /// Push raw `/FIELD/VALUE/` segment.
    pub fn filter<V: Display>(mut self, field: &str, value: V) -> Self {
        self.segments.push(format!("{}/{}", field, value));
        self
    }

    /// Limit returned rows.
    pub fn limit(mut self, n: u32) -> Self {
        self.segments.push(format!("limit/{}/", n));
        self
    }

    /// Ordering (e.g. `"EPOCH desc"`).
    pub fn order_by(mut self, clause: &str) -> Self {
        self.segments.push(format!("orderby/{}/", clause));
        self
    }

    /// Choose response format.
    pub fn format(mut self, fmt: Format) -> Self {
        self.format = fmt;
        self
    }
}

impl<'a, T> QueryBuilder<'a, T>
where
    T: DeserializeOwned,
{
    // ---------------------------------------------------------------------
    // Execute & parse
    // ---------------------------------------------------------------------

    /// Execute and deserialize into `Vec<T>` (JSON or CSV).
    pub fn execute(self) -> Result<Vec<T>, Error> {
        let url = self.build_url();
        let resp = self.client.agent.get(&url).call()?;

        if resp.status() != 200 {
            return Err(Error::Http(resp.status()));
        }

        match self.format {
            Format::Json => {
                let data: Vec<T> = resp.into_json()?;
                if data.is_empty() {
                    Err(Error::Empty)
                } else {
                    Ok(data)
                }
            }
            Format::Csv => {
                #[cfg(not(feature = "csv"))]
                {
                    Err(Error::Other(
                        "compile with `csv` feature to enable CSV parsing".into(),
                    ))
                }
                #[cfg(feature = "csv")]
                {
                    use csv::ReaderBuilder;
                    let text = resp.into_string()?;
                    let mut rdr = ReaderBuilder::new().from_reader(text.as_bytes());
                    let mut out = Vec::new();
                    for rec in rdr.deserialize::<T>() {
                        out.push(rec?);
                    }
                    if out.is_empty() {
                        Err(Error::Empty)
                    } else {
                        Ok(out)
                    }
                }
            }
            _ => Err(Error::Other(
                "use `execute_raw()` for TLE, XML, or other text formats".into(),
            )),
        }
    }

    /// Convenience when you expect exactly one record.
    pub fn execute_one(self) -> Result<T, Error> {
        let mut v = self.limit(1).execute()?;
        v.pop().ok_or(Error::Empty)
    }
}

impl<'a, T> QueryBuilder<'a, T> {
    /// Execute and return raw string (TLE, XML, HTML, etc.).
    pub fn execute_raw(self) -> Result<String, Error> {
        let url = self.build_url();
        let resp = self.client.agent.get(&url).call()?;

        if resp.status() != 200 {
            return Err(Error::Http(resp.status()));
        }
        Ok(resp.into_string()?)
    }

    // ---------------------------------------------------------------------
    // Internal – build final REST URL
    // ---------------------------------------------------------------------
    fn build_url(&self) -> String {
        // Most classes live under /basicspacedata/…
        let mut url = if self.controller == "basicspacedata" {
            format!("{BASE_URL}/basicspacedata/query/class/{}/", self.class)
        } else {
            format!("{BASE_URL}/{}/query/class/{}/", self.controller, self.class)
        };

        // additional path segments (filters, order, limit…)
        for seg in &self.segments {
            url.push_str(seg);
        }

        // ensure format segment
        if self.format != Format::Json {
            url.push_str(&format!("format/{}/", self.format.as_str()));
        } else if !self.segments.iter().any(|s| s.starts_with("format/")) {
            url.push_str("format/json/");
        }
        url
    }
}
