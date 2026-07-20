# -------------------------------------------------------------------
#
#                   🗲 Storm Software - Telepathic
#
# This code was released as part of the Telepathic project. Telepathic
# is maintained by Storm Software under the Apache-2.0 license, and is
# free for commercial and private use. For more information, please visit
# our licensing page at https://stormsoftware.com/licenses/projects/telepathic.
#
# Website:                  https://stormsoftware.com
# Repository:               https://github.com/storm-software/telepathic
# Documentation:            https://docs.telepathic.sh
# Contact:                  https://stormsoftware.com/contact
#
# SPDX-License-Identifier:  Apache-2.0
#
# -------------------------------------------------------------------

"""TypedDict shapes matching the PyO3 binding input/output surface."""

from __future__ import annotations

from typing import Any, TypedDict


class BindingUser(TypedDict):
    name: str
    display_name: str
    language_preferences: list[str]


class BindingDevice(TypedDict):
    name: str
    display_name: str
    platform: str
    distro: str
    desktop_env: str
    cpu_arch: str


class BindingSession(TypedDict):
    session_id: str
    started_at: int
    user: BindingUser
    device: BindingDevice


class BindingEnvPaths(TypedDict):
    cache: str
    config: str
    data: str
    logs: str
    temp: str
    downloads: str
    executable: str


class BindingDefinition(TypedDict):
    name: str
    qualified_name: str
    label: str
    start_line: int
    end_line: int
    decorators: list[str]
    base_classes: list[str]
    param_names: list[str]
    param_types: list[str]
    return_types: list[str]
    complexity: int
    lines: int
    is_exported: bool
    is_test: bool
    is_entry_point: bool
    file_path: str | None
    signature: str | None
    return_type: str | None
    parent_class: str | None


class BindingGetSchemaOutput(TypedDict):
    schema: str


class BindingListRepositoriesOutput(TypedDict):
    repositories: list[str]


class BindingIndexRepositoryOutput(TypedDict):
    success: bool
    errors: list[str]


class BindingListProjectsInput(TypedDict, total=False):
    repository_id: str | None
    depends_on: str | None


class BindingListProjectsOutput(TypedDict):
    projects: list[str]


class BindingWriteGraphInput(TypedDict):
    node: BindingDefinition
    properties: dict[str, Any] | None


class BindingWriteGraphOutput(TypedDict):
    success: bool
    errors: list[str]


class BindingReadGraphInput(TypedDict):
    name: str


class BindingReadGraphOutput(TypedDict):
    node: str


class BindingQueryGraphInput(TypedDict):
    query: str
    params: dict[str, Any] | None


class BindingQueryGraphOutput(TypedDict):
    results: list[str]


class BindingExecutionSearchHit(TypedDict):
    execution_id: str
    score: float | None
    snippet: str | None


class BindingSearchGraphInput(TypedDict):
    name: str
    qualified_name: str
    label: str
    query: str | None
    last_user_id: str | None
    file_path: str | None
    labels: list[str] | None
    embedding: list[float] | None
    limit: int | None


class BindingSearchGraphOutput(TypedDict):
    results: list[BindingExecutionSearchHit]


class BindingTraceGraphInput(TypedDict):
    call_site_name: str
    qualified_name: str
    strategy: str
    confidence: float


class BindingTraceGraphOutput(TypedDict):
    results: list[str]


class BindingExportOkfInput(TypedDict):
    output_path: str


class BindingExportOkfOutput(TypedDict):
    success: bool
    errors: list[str]
