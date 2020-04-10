//! Python bindings for Brave's adblocking library, which is written in Rust.
#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications
)]

use adblock::blocker::BlockerError as RustBlockerError;
use adblock::blocker::BlockerResult as RustBlockerResult;
use adblock::cosmetic_filter_cache::HostnameSpecificResources as RustHostnameSpecificResources;
use adblock::engine::Engine as RustEngine;
use failure::Fail;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::ValueError as PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::iter::FromIterator;

/// Brave's adblocking library in Python!
#[pymodule]
fn adblock(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Engine>()?;
    Ok(())
}

#[pyclass]
pub struct BlockerResult {
    #[pyo3(get)]
    pub matched: bool,
    #[pyo3(get)]
    pub explicit_cancel: bool,
    #[pyo3(get)]
    pub important: bool,
    #[pyo3(get)]
    pub redirect: Option<String>,
    #[pyo3(get)]
    pub exception: Option<String>,
    #[pyo3(get)]
    pub filter: Option<String>,
    #[pyo3(get)]
    pub error: Option<String>,
}

impl Into<BlockerResult> for RustBlockerResult {
    fn into(self) -> BlockerResult {
        BlockerResult {
            matched: self.matched,
            explicit_cancel: self.explicit_cancel,
            important: self.important,
            redirect: self.redirect,
            exception: self.exception,
            filter: self.filter,
            error: self.error,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for BlockerResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BlockerResult({}, {}, {}, {:?}, {:?}, {:?}, {:?})",
            self.matched,
            self.explicit_cancel,
            self.important,
            self.redirect,
            self.exception,
            self.filter,
            self.error
        ))
    }
}

#[derive(Fail, Debug, PartialEq, Copy, Clone)]
pub enum BlockerError {
    #[fail(display = "Serialization error")]
    SerializationError,
    #[fail(display = "Deserialization error")]
    DeserializationError,
    #[fail(display = "Optimized filter exists")]
    OptimizedFilterExistence,
    #[fail(display = "Bad filter add unsupported")]
    BadFilterAddUnsupported,
    #[fail(display = "Filter exists")]
    FilterExists,
}

impl Into<PyErr> for BlockerError {
    fn into(self) -> PyErr {
        PyErr::new::<PyValueError, _>(format!("{:?}", self))
    }
}

impl Into<BlockerError> for RustBlockerError {
    fn into(self) -> BlockerError {
        match self {
            Self::SerializationError => BlockerError::SerializationError,
            Self::DeserializationError => BlockerError::DeserializationError,
            Self::OptimizedFilterExistence => BlockerError::OptimizedFilterExistence,
            Self::BadFilterAddUnsupported => BlockerError::BadFilterAddUnsupported,
            Self::FilterExists => BlockerError::FilterExists,
        }
    }
}

/// Contains cosmetic filter information intended to be injected into a
/// particular hostname.
#[pyclass]
pub struct HostnameSpecificResources {
    /// A set of any CSS selector on the page that should be hidden, i.e.
    /// styled as `{ display: none !important; }`.
    #[pyo3(get)]
    pub hide_selectors: Vec<String>,
    /// A map of CSS selectors on the page to respective non-hide style rules,
    /// i.e. any required styles other than `display: none`.
    #[pyo3(get)]
    pub style_selectors: HashMap<String, Vec<String>>,
    /// A set of any class or id CSS selectors that should not have generic
    /// rules applied.
    // In practice, these should be passed to `class_id_stylesheet` and not
    // used otherwise.
    #[pyo3(get)]
    pub exceptions: Vec<String>,
    /// Javascript code for any scriptlets that should be injected into the
    /// page.
    #[pyo3(get)]
    pub injected_script: String,
}

impl Into<HostnameSpecificResources> for RustHostnameSpecificResources {
    fn into(self) -> HostnameSpecificResources {
        let hide_selectors = Vec::from_iter(self.hide_selectors.into_iter());
        let exceptions = Vec::from_iter(self.exceptions.into_iter());
        HostnameSpecificResources {
            hide_selectors,
            style_selectors: self.style_selectors,
            exceptions,
            injected_script: self.injected_script,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for HostnameSpecificResources {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "HostnameSpecificResources<{} hide selectors, {} style selectors, {} exceptions, injected_javascript={:?}>",
            self.hide_selectors.len(),
            self.style_selectors.len(),
            self.exceptions.len(),
            self.injected_script,
        ))
    }
}

#[pyclass]
pub struct Engine {
    engine: RustEngine,
}

#[pymethods]
impl Engine {
    #[new]
    pub fn from_rules(network_filters: Vec<String>) -> Self {
        let engine = RustEngine::from_rules(&network_filters);
        Self { engine }
    }

    /// ## Request types
    /// Examples of valid `request_type` parameters include:
    /// * `beacon`
    /// * `csp_report`
    /// * `document`
    /// * `font`
    /// * `media`
    /// * `object`
    /// * `script`
    /// * `stylesheet`
    /// * and et cetera...
    pub fn check_network_urls(
        &self,
        url: &str,
        source_url: &str,
        request_type: &str,
    ) -> BlockerResult {
        let blocker_result = self
            .engine
            .check_network_urls(url, source_url, request_type);
        blocker_result.into()
    }

    pub fn check_network_urls_with_hostnames(
        &self,
        url: &str,
        hostname: &str,
        source_hostname: &str,
        request_type: &str,
        third_party_request: Option<bool>,
    ) -> BlockerResult {
        let blocker_result = self.engine.check_network_urls_with_hostnames(
            url,
            hostname,
            source_hostname,
            request_type,
            third_party_request,
        );
        blocker_result.into()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn check_network_urls_with_hostnames_subset(
        &self,
        url: &str,
        hostname: &str,
        source_hostname: &str,
        request_type: &str,
        third_party_request: Option<bool>,
        previously_matched_rule: bool,
        force_check_exceptions: bool,
    ) -> BlockerResult {
        let blocker_result = self.engine.check_network_urls_with_hostnames_subset(
            url,
            hostname,
            source_hostname,
            request_type,
            third_party_request,
            previously_matched_rule,
            force_check_exceptions,
        );
        blocker_result.into()
    }

    pub fn serialize(&mut self) -> PyResult<Vec<u8>> {
        let result = self.engine.serialize();
        match result {
            Ok(x) => Ok(x),
            Err(error) => {
                let my_blocker_error: BlockerError = error.into();
                Err(my_blocker_error.into())
            }
        }
    }

    pub fn serialize_to_file(&mut self, file: &str) -> PyResult<()> {
        let data = self.serialize()?;
        let mut fd = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file)?;
        fd.write_all(&data)?;
        Ok(())
    }

    pub fn deserialize(&mut self, serialized: &[u8]) -> PyResult<()> {
        let result = self.engine.deserialize(serialized);
        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                let my_blocker_error: BlockerError = error.into();
                Err(my_blocker_error.into())
            }
        }
    }

    pub fn deserialize_from_file(&mut self, file: &str) -> PyResult<()> {
        let mut fd = fs::File::open(file)?;
        let mut data: Vec<u8> = Vec::new();
        fd.read_to_end(&mut data)?;
        self.deserialize(&data)
    }

    pub fn add_filter_list(&mut self, filter_list: &str) {
        self.engine.add_filter_list(filter_list);
    }

    pub fn filter_exists(&self, filter: &str) -> bool {
        self.engine.filter_exists(filter)
    }

    pub fn tags_enable(&mut self, tags: Vec<&str>) {
        self.engine.tags_enable(&tags);
    }

    pub fn tags_disable(&mut self, tags: Vec<&str>) {
        self.engine.tags_disable(&tags);
    }

    pub fn tag_exists(&self, tag: &str) -> bool {
        self.engine.tag_exists(tag)
    }

    /// Returns a set of cosmetic filter resources required for a particular
    /// hostname. Once this has been called, all CSS ids and classes on a
    /// page should be passed to hidden_class_id_selectors to obtain any
    /// stylesheets consisting of generic rules.
    pub fn hostname_cosmetic_resources(&self, hostname: &str) -> HostnameSpecificResources {
        self.engine.hostname_cosmetic_resources(hostname).into()
    }

    /// If any of the provided CSS classes or ids could cause a certain generic
    /// CSS hide rule (i.e. `{ display: none !important; }`) to be required, this
    /// method will return a list of CSS selectors corresponding to rules
    /// referencing those classes or ids, provided that the corresponding rules
    /// are not excepted.
    ///
    /// Exceptions should be passed directly from HostnameSpecificResources.
    ///
    /// ## Note
    /// The `exceptions` field will be changed to a set, once a new version of
    /// PyO3 is released.
    pub fn hidden_class_id_selectors(
        &self,
        classes: Vec<String>,
        ids: Vec<String>,
        exceptions: Vec<String>,
    ) -> PyResult<Vec<String>> {
        let exception_hashset: HashSet<String> = HashSet::from_iter(exceptions);
        Ok(self
            .engine
            .hidden_class_id_selectors(&classes, &ids, &exception_hashset))
    }
}
