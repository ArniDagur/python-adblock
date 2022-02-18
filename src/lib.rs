//! Python wrapper for Brave's adblocking library, which is written in Rust.
#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications,
    deprecated
)]

use adblock::blocker::BlockerResult as RustBlockerResult;
use adblock::blocker::{BlockerError as RustBlockerError, Redirection};
use adblock::cosmetic_filter_cache::UrlSpecificResources as RustUrlSpecificResources;
use adblock::engine::Engine as RustEngine;
use adblock::lists::FilterSet as RustFilterSet;
use adblock::lists::{FilterFormat, ParseOptions};
use pyo3::class::PyObjectProtocol;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::PyErr;

use adblock::resources::{
    AddResourceError as RustAddResourceError, MimeType, Resource, ResourceType,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs;
use std::io::{Read, Write};

/// Brave's adblocking library in Python!
#[pymodule]
fn adblock(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Engine>()?;
    m.add_class::<FilterSet>()?;
    m.add_class::<BlockerResult>()?;
    m.add_class::<UrlSpecificResources>()?;
    m.add("AdblockException", py.get_type::<AdblockException>())?;
    m.add("BlockerException", py.get_type::<BlockerException>())?;
    m.add("SerializationError", py.get_type::<SerializationError>())?;
    m.add(
        "DeserializationError",
        py.get_type::<DeserializationError>(),
    )?;
    m.add(
        "OptimizedFilterExistence",
        py.get_type::<OptimizedFilterExistence>(),
    )?;
    m.add(
        "BadFilterAddUnsupported",
        py.get_type::<BadFilterAddUnsupported>(),
    )?;
    m.add("FilterExists", py.get_type::<FilterExists>())?;
    m.add(
        "AddResourceException",
        py.get_type::<AddResourceException>(),
    )?;
    m.add(
        "InvalidBase64ContentError",
        py.get_type::<InvalidBase64ContentError>(),
    )?;
    m.add(
        "InvalidUtf8ContentError",
        py.get_type::<InvalidUtf8ContentError>(),
    )?;
    Ok(())
}

/// The result of an ad-blocking check.
#[pyclass]
pub struct BlockerResult {
    #[pyo3(get)]
    pub matched: bool,
    /// Important is used to signal that a rule with the `important` option
    /// matched. An `important` match means that exceptions should not apply
    /// and no further checking is neccesary--the request should be blocked
    /// (empty body or cancelled).
    ///
    /// Brave Browser keeps seperate instances of Blocker for default lists
    /// and regional ones, so `important` here is used to correct behaviour
    /// between them: checking should stop instead of moving to the next
    /// instance iff an `important` rule matched.
    #[pyo3(get)]
    pub important: bool,
    /// Iff the blocker matches a rule which has the `redirect` option, as per
    /// [uBlock Origin's redirect syntax][1], the `redirect` is not `None`.
    /// The `redirect` field contains the body of the redirect to be injected.
    ///
    /// [1]: https://github.com/gorhill/uBlock/wiki/Static-filter-syntax#redirect
    #[pyo3(get)]
    pub redirect_type: Option<String>,
    /// Exception is not `None` when the blocker matched on an exception rule.
    /// Effectively this means that there was a match, but the request should
    /// not be blocked. It is a non-empty string if the blocker was initialized
    /// from a list of rules with debugging enabled, otherwise the original
    /// string representation is discarded to reduce memory use.
    #[pyo3(get)]
    pub redirect: Option<String>,
    /// Exception is not `None` when the blocker matched on an exception rule.
    /// Effectively this means that there was a match, but the request should
    /// not be blocked. It is a non-empty string if the blocker was initialized
    /// from a list of rules with debugging enabled, otherwise the original
    /// string representation is discarded to reduce memory use.
    #[pyo3(get)]
    pub exception: Option<String>,
    /// Filter--similarly to exception--includes the string representation of
    /// the rule when there is a match and debugging is enabled. Otherwise, on
    /// a match, it is not `None`.
    #[pyo3(get)]
    pub filter: Option<String>,
    /// The `error` field is only used to signal that there was an error in
    /// parsing the provided URLs when using the simpler
    /// `check_network_urls` method.
    #[pyo3(get)]
    pub error: Option<String>,
}

impl From<RustBlockerResult> for BlockerResult {
    fn from(br: RustBlockerResult) -> Self {
        let (redirect, redirect_type) = if let Some(resource) = br.redirect {
            match resource {
                Redirection::Resource(resource) => (Some(resource), Some("resource".to_string())),
                Redirection::Url(url) => (Some(url), Some("url".to_string())),
            }
        } else {
            (None, None)
        };

        Self {
            matched: br.matched,
            important: br.important,
            exception: br.exception,
            filter: br.filter,
            error: br.error,
            redirect_type,
            redirect,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for BlockerResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BlockerResult(matched={}, important={}, redirect={}, exception={}, filter={}, error={})",
            self.matched.diy_python_repr(),
            self.important.diy_python_repr(),
            self.redirect.diy_python_repr(),
            self.exception.diy_python_repr(),
            self.filter.diy_python_repr(),
            self.error.diy_python_repr(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BlockerError {
    SerializationError,
    DeserializationError,
    OptimizedFilterExistence,
    BadFilterAddUnsupported,
    FilterExists,
}

impl Error for BlockerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for BlockerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SerializationError => "Serialization error",
                Self::DeserializationError => "Deserialization error",
                Self::OptimizedFilterExistence => "Optimized filter exists",
                Self::BadFilterAddUnsupported => "Bad filter add unsupported",
                Self::FilterExists => "Filter exists",
            }
        )
    }
}

create_exception!(adblock, AdblockException, PyException);
create_exception!(adblock, BlockerException, AdblockException);
create_exception!(adblock, AddResourceException, AdblockException);
create_exception!(adblock, InvalidBase64ContentError, AddResourceException);
create_exception!(adblock, InvalidUtf8ContentError, AddResourceException);
create_exception!(adblock, SerializationError, BlockerException);
create_exception!(adblock, DeserializationError, BlockerException);
create_exception!(adblock, OptimizedFilterExistence, BlockerException);
create_exception!(adblock, BadFilterAddUnsupported, BlockerException);
create_exception!(adblock, FilterExists, BlockerException);

impl From<BlockerError> for PyErr {
    fn from(err: BlockerError) -> Self {
        let msg = format!("{:?}", err);
        match err {
            BlockerError::SerializationError => Self::new::<SerializationError, _>(msg),
            BlockerError::DeserializationError => Self::new::<DeserializationError, _>(msg),
            BlockerError::OptimizedFilterExistence => Self::new::<OptimizedFilterExistence, _>(msg),
            BlockerError::BadFilterAddUnsupported => Self::new::<BadFilterAddUnsupported, _>(msg),
            BlockerError::FilterExists => Self::new::<FilterExists, _>(msg),
        }
    }
}

impl From<RustBlockerError> for BlockerError {
    fn from(err: RustBlockerError) -> Self {
        match err {
            RustBlockerError::SerializationError => Self::SerializationError,
            RustBlockerError::DeserializationError => Self::DeserializationError,
            RustBlockerError::OptimizedFilterExistence => Self::OptimizedFilterExistence,
            RustBlockerError::BadFilterAddUnsupported => Self::BadFilterAddUnsupported,
            RustBlockerError::FilterExists => Self::FilterExists,
        }
    }
}

fn filter_format_from_string(filter_format: &str) -> PyResult<FilterFormat> {
    match filter_format {
        "standard" => Ok(FilterFormat::Standard),
        "hosts" => Ok(FilterFormat::Hosts),
        _ => Err(PyErr::new::<AdblockException, _>("Invalid format value")),
    }
}

/// Manages a set of rules to be added to an Engine.
///
/// To be able to efficiently handle special options like $badfilter, and to
/// allow optimizations, all rules must be available when the Engine is first
/// created. FilterSet allows assembling a compound list from multiple
/// different sources before compiling the rules into an Engine.
#[pyclass]
#[pyo3(text_signature = "($self, debug)")]
#[derive(Clone)]
pub struct FilterSet {
    filter_set: RustFilterSet,
    debug: bool,
}

#[pymethods]
impl FilterSet {
    /// Creates a new `FilterSet`. The `debug` argument specifies whether or
    /// not to save information about the original raw filter rules alongside
    /// the more compact internal representation. If enabled, this information
    /// will be passed to the corresponding Engine.
    #[new]
    #[args(debug = false)]
    pub fn new(debug: bool) -> Self {
        Self {
            filter_set: RustFilterSet::new(debug),
            debug,
        }
    }

    /// Adds the contents of an entire filter list to this FilterSet. Filters
    /// that cannot be parsed successfully are ignored.
    ///
    /// The format is a string containing either "standard" (ABP/uBO-style)
    /// or "hosts".
    #[pyo3(text_signature = "($self, filter_list, format, include_redirect_urls)")]
    #[args(filter_list, format = "\"standard\"", include_redirect_urls = "false")]
    pub fn add_filter_list(
        &mut self,
        filter_list: &str,
        format: &str,
        include_redirect_urls: bool,
    ) -> PyResult<()> {
        let filter_format = filter_format_from_string(format)?;
        self.filter_set.add_filter_list(
            filter_list,
            ParseOptions {
                format: filter_format,
                include_redirect_urls,
            },
        );
        Ok(())
    }

    /// Adds a collection of filter rules to this FilterSet. Filters that
    /// cannot be parsed successfully are ignored.
    ///
    /// The format is a string containing either "standard" (ABP/uBO-style)
    /// or "hosts".
    #[pyo3(text_signature = "($self, filters, format, include_redirect_urls)")]
    #[args(filters, format = "\"standard\"", include_redirect_urls = "false")]
    pub fn add_filters(
        &mut self,
        filters: Vec<String>,
        format: &str,
        include_redirect_urls: bool,
    ) -> PyResult<()> {
        let filter_format = filter_format_from_string(format)?;
        self.filter_set.add_filters(
            &filters,
            ParseOptions {
                format: filter_format,
                include_redirect_urls,
            },
        );
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for FilterSet {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("FilterSet(debug={})", self.debug.diy_python_repr()))
    }
}

/// Contains cosmetic filter information intended to be injected into a
/// particular hostname.
#[pyclass]
pub struct UrlSpecificResources {
    /// A set of any CSS selector on the page that should be hidden, i.e.
    /// styled as `{ display: none !important; }`.
    #[pyo3(get)]
    pub hide_selectors: HashSet<String>,
    /// A map of CSS selectors on the page to respective non-hide style rules,
    /// i.e. any required styles other than `display: none`.
    #[pyo3(get)]
    pub style_selectors: HashMap<String, Vec<String>>,
    /// A set of any class or id CSS selectors that should not have generic
    /// rules applied.
    // In practice, these should be passed to `class_id_stylesheet` and not
    // used otherwise.
    #[pyo3(get)]
    pub exceptions: HashSet<String>,
    /// Javascript code for any scriptlets that should be injected into the
    /// page.
    #[pyo3(get)]
    pub injected_script: String,
    /// `generichide` is set to `True` if there is a corresponding
    /// `$generichide` exception network filter. If so, the page should not
    /// query for additional generic rules using hidden_class_id_selectors.
    #[pyo3(get)]
    pub generichide: bool,
}

impl From<RustUrlSpecificResources> for UrlSpecificResources {
    fn from(r: RustUrlSpecificResources) -> Self {
        Self {
            hide_selectors: r.hide_selectors,
            style_selectors: r.style_selectors,
            exceptions: r.exceptions,
            injected_script: r.injected_script,
            generichide: r.generichide,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for UrlSpecificResources {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "UrlSpecificResources<{} hide selectors, {} style selectors, {} exceptions, injected_javascript={}, generichide={}>",
            self.hide_selectors.len(),
            self.style_selectors.len(),
            self.exceptions.len(),
            self.injected_script.diy_python_repr(),
            self.generichide.diy_python_repr(),
        ))
    }
}

/// The main object featured in this library. This object holds the adblocker's
/// state, and can be queried to see if a given request should be blocked or
/// not.
///
/// # Request types
/// A few of `Engine`'s methods have a field specifying a "resource type",
/// valid examples are:
/// * `beacon`
/// * `csp_report`
/// * `document`
/// * `font`
/// * `media`
/// * `object`
/// * `script`
/// * `stylesheet`
/// * and et cetera...
/// See the [Mozilla Web Documentation][1] for more info.
///
/// [1]: https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest/ResourceType
#[pyclass]
#[pyo3(text_signature = "($self, filter_set, optimize)")]
pub struct Engine {
    engine: RustEngine,
    optimize: bool,
}

#[pymethods]
impl Engine {
    /// Create a new adblocking engine
    #[new]
    #[args(filter_set, optimize = true)]
    pub fn new(filter_set: FilterSet, optimize: bool) -> Self {
        let engine = RustEngine::from_filter_set(filter_set.filter_set, optimize);
        Self { engine, optimize }
    }

    /// Check if the given `url`—pointing to a resource of type `request_type`—
    /// is blocked, assuming the request is made from the given `source_url`.
    /// Returns an object of type `BlockerResult`.
    ///
    /// # Arguments
    /// * `url` - The URL of the request to check
    /// * `source_url` - The URL from where the request is made
    /// * `request_type` - The resource type that the request points to
    #[pyo3(text_signature = "($self, url, source_url, request_type)")]
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

    /// Check if a request should be blocked based on the given parameters.
    ///
    /// # Arguments
    /// * `url` - The URL of the request to check
    /// * `hostname` - The given `url`'s hostname
    /// * `source_hostname` - The hostname of the source URL.
    /// * `request_type` - The resource type that the request points to
    /// * `third_party_request` - Is the given request to a third-party? Here,
    ///   `None` can be given and the engine will figure it out based on the
    ///   `hostname` and `source_hostname`.
    #[pyo3(
        text_signature = "($self, url, hostname, source_hostname, requsest_type, third_party_request)"
    )]
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

    /// Check if a request should be blocked based on the given parameters.
    ///
    /// # Arguments
    /// * `url` - The URL of the request to check
    /// * `hostname` - The given `url`'s hostname
    /// * `source_hostname` - The hostname of the source URL.
    /// * `request_type` - The resource type that the request points to
    /// * `third_party_request` - Is the given request to a third-party? Here,
    ///   `None` can be given and the engine will figure it out based on the
    ///   `hostname` and `source_hostname`.
    /// * `previously_matched_rule` - Return a match as long as there are no
    ///    exceptions
    /// * `force_check_exceptions` - Check exceptions even if no other rule matches
    #[pyo3(
        text_signature = "($self, url, hostname, source_hostname, request_type, \
        third_party_request, previously_matched_rule, force_check_exceptions)"
    )]
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

    /// Sets this engine's resources to additionally include `resource`.
    ///
    /// # Arguments
    /// * `name`: Represents the primary name of the resource, often a filename
    /// * `content_type`: How to interpret the resource data within `content`
    /// * `content`: The resource data, encoded using standard base64 configuration
    #[pyo3(text_signature = "($self, name, content_type, content)")]
    pub fn add_resource(&mut self, name: &str, content_type: &str, content: &str) -> PyResult<()> {
        let result = self.engine.add_resource(Resource {
            name: name.to_string(),
            aliases: vec![],
            kind: ResourceType::Mime(MimeType::from(std::borrow::Cow::from(
                content_type.to_string(),
            ))),
            content: content.to_string(),
        });

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                RustAddResourceError::InvalidBase64Content => Err(
                    InvalidBase64ContentError::new_err("invalid base64 content".to_string()),
                ),
                RustAddResourceError::InvalidUtf8Content => Err(InvalidUtf8ContentError::new_err(
                    "invalid utf content".to_string(),
                )),
            },
        }
    }

    /// Serialize this blocking engine to bytes. They can then be deserialized
    /// using `deserialize()` to get the same engine again.
    #[pyo3(text_signature = "($self)")]
    pub fn serialize<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        let bytes = self.serialize_inner()?;
        let py_bytes = PyBytes::new(py, &bytes);
        Ok(py_bytes)
    }

    fn serialize_inner(&mut self) -> PyResult<Vec<u8>> {
        let result = self.engine.serialize_raw();
        match result {
            Ok(x) => Ok(x),
            Err(error) => {
                let my_blocker_error: BlockerError = error.into();
                Err(my_blocker_error.into())
            }
        }
    }

    /// Serialize this blocking engine to a file. The file can then be
    /// deserialized using `deserialize_from_file()` to get the same engine
    /// again.
    #[pyo3(text_signature = "($self, file)")]
    pub fn serialize_to_file(&mut self, file: &str) -> PyResult<()> {
        let data = self.serialize_inner()?;
        let mut fd = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file)?;
        fd.write_all(&data)?;
        Ok(())
    }

    /// Deserialize a blocking engine from bytes produced with `serialize()`.
    #[pyo3(text_signature = "($self, serialized)")]
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

    /// Deserialize a blocking engine from file produced with
    /// `serialize_to_file()`.
    #[pyo3(text_signature = "($self, file)")]
    pub fn deserialize_from_file(&mut self, file: &str) -> PyResult<()> {
        let mut fd = fs::File::open(file)?;
        let mut data: Vec<u8> = Vec::new();
        fd.read_to_end(&mut data)?;
        self.deserialize(&data)
    }

    /// Checks if the given filter exists in the blocking engine.
    #[pyo3(text_signature = "($self, filter)")]
    pub fn filter_exists(&self, filter: &str) -> bool {
        self.engine.filter_exists(filter)
    }

    /// Sets this engine's tags to be _only_ the ones provided in tags.
    ///
    /// Tags can be used to cheaply enable or disable network rules with a
    /// corresponding $tag option.
    #[pyo3(text_signature = "($self, tags)")]
    pub fn use_tags(&mut self, tags: Vec<&str>) {
        self.engine.use_tags(&tags);
    }

    /// Sets this engine's tags to additionally include the ones provided in
    /// tags.
    ///
    /// Tags can be used to cheaply enable or disable network rules with a
    /// corresponding $tag option.
    #[pyo3(text_signature = "($self, tags)")]
    pub fn enable_tags(&mut self, tags: Vec<&str>) {
        self.engine.enable_tags(&tags);
    }

    /// Sets this engine's tags to no longer include the ones provided in
    /// tags.
    ///
    /// Tags can be used to cheaply enable or disable network rules with a
    /// corresponding $tag option.
    #[pyo3(text_signature = "($self, tags)")]
    pub fn disable_tags(&mut self, tags: Vec<&str>) {
        self.engine.disable_tags(&tags);
    }

    /// Checks if a given tag exists in this engine.
    ///
    /// Tags can be used to cheaply enable or disable network rules with a
    /// corresponding $tag option.
    #[pyo3(text_signature = "($self, tag)")]
    pub fn tag_exists(&self, tag: &str) -> bool {
        self.engine.tag_exists(tag)
    }

    /// Returns a set of cosmetic filter resources required for a particular
    /// url. Once this has been called, all CSS ids and classes on a
    /// page should be passed to hidden_class_id_selectors to obtain any
    /// stylesheets consisting of generic rules.
    #[pyo3(text_signature = "($self, url)")]
    pub fn url_cosmetic_resources(&self, url: &str) -> UrlSpecificResources {
        self.engine.url_cosmetic_resources(url).into()
    }

    /// If any of the provided CSS classes or ids could cause a certain generic
    /// CSS hide rule (i.e. `{ display: none !important; }`) to be required, this
    /// method will return a list of CSS selectors corresponding to rules
    /// referencing those classes or ids, provided that the corresponding rules
    /// are not excepted.
    ///
    /// Exceptions should be passed directly from UrlSpecificResources.
    #[pyo3(text_signature = "($self, classes, ids, exceptions)")]
    pub fn hidden_class_id_selectors(
        &self,
        classes: Vec<String>,
        ids: Vec<String>,
        exceptions: HashSet<String>,
    ) -> PyResult<Vec<String>> {
        Ok(self
            .engine
            .hidden_class_id_selectors(&classes, &ids, &exceptions))
    }
}

#[pyproto]
impl PyObjectProtocol for Engine {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Engine<optimize={}>",
            self.optimize.diy_python_repr()
        ))
    }
}

/// PyO3 doesn't offer the ability to get the Python representation of a Rust
/// object, so we make our own trait.
trait DiyPythonRepr {
    fn diy_python_repr(&self) -> String;
}

impl<T> DiyPythonRepr for Option<T>
where
    T: DiyPythonRepr,
{
    fn diy_python_repr(&self) -> String {
        match self {
            None => "None".to_owned(),
            Some(x) => x.diy_python_repr(),
        }
    }
}

impl DiyPythonRepr for String {
    fn diy_python_repr(&self) -> String {
        let mut res = format!("{:?}", self);
        // This is safe to do since we know that `res` will always be of
        // length >= 2.
        res.replace_range(0..1, "'");
        res.replace_range(res.len() - 1..res.len(), "'");
        res
    }
}

impl DiyPythonRepr for bool {
    fn diy_python_repr(&self) -> String {
        if *self {
            "True".to_owned()
        } else {
            "False".to_owned()
        }
    }
}
