from typing import Optional, Dict, List, Set

class BlockerResult:
    matched: bool
    explicit_cancel: bool
    important: bool
    redirect: Optional[str]
    exception: Optional[str]
    filter: Optional[str]
    error: Optional[str]
    def __repr__(self) -> str:
        pass

class HostnameSpecificResources:
    hide_selectors: Set[str]
    style_selectors: Dict[str, List[str]]
    exceptions: Set[str]
    injected_script: str
    def __repr__(self) -> str:
        pass

class Engine:
    def __init__(
        self,
        network_filters: Optional[List[str]] = None,
        load_network: bool = True,
        load_cosmetic: bool = False,
        debug: bool = False,
    ) -> None:
        pass
    def check_network_urls(
        self, url: str, source_url: str, request_type: str
    ) -> BlockerResult:
        pass
    def check_network_urls_with_hostnames(
        self,
        url: str,
        hostname: str,
        source_hostname: str,
        request_type: str,
        third_party_request: Optional[bool],
    ) -> BlockerResult:
        pass
    def check_network_urls_with_hostnames_subset(
        self,
        url: str,
        hostname: str,
        source_hostname: str,
        request_type: str,
        third_party_request: Optional[bool],
        previously_matched_rule: bool,
        force_check_exceptions: bool,
    ) -> BlockerResult:
        pass
    def serialize(self) -> bytes:
        pass
    def serialize_to_file(self, file: str) -> None:
        pass
    def deserialize(self, serialized: bytes) -> None:
        pass
    def deserialize_from_file(self, file: str) -> None:
        pass
    def add_filter_list(self, filter_list: str) -> None:
        pass
    def filter_exists(self, filter: str) -> bool:
        pass
    def tags_enable(self, tags: List[str]) -> None:
        pass
    def tags_disable(self, tags: List[str]) -> None:
        pass
    def tag_exists(self, tag: str) -> bool:
        pass
    def hostname_cosmetic_resources(self, hostname: str) -> HostnameSpecificResources:
        pass
    def hidden_class_id_selectors(
        self, classes: List[str], ids: List[str], exceptions: Set[str]
    ) -> List[str]:
        pass
