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

"""Thin async wrapper around the PyO3 ``BindingEngine`` class."""

from __future__ import annotations

from typing import Any

from telepathic._telepathic_pyo3 import BindingEngine
from telepathic.native.config import binding_options_from_config
from telepathic.native.utils import get_binding_field, unwrap_binding_result
from telepathic.types.bindings import (
    BindingExportOkfInput,
    BindingExportOkfOutput,
    BindingGetSchemaOutput,
    BindingIndexRepositoryOutput,
    BindingListProjectsInput,
    BindingListProjectsOutput,
    BindingListRepositoriesOutput,
    BindingQueryGraphInput,
    BindingQueryGraphOutput,
    BindingReadGraphInput,
    BindingReadGraphOutput,
    BindingSearchGraphInput,
    BindingSearchGraphOutput,
    BindingSession,
    BindingTraceGraphInput,
    BindingTraceGraphOutput,
    BindingWriteGraphInput,
    BindingWriteGraphOutput,
)


class NativeBindingEngine:
    """Python facade over the native ``BindingEngine`` PyO3 class."""

    def __init__(self, config: dict[str, Any] | None = None) -> None:
        self._binding = BindingEngine(binding_options_from_config(config or {}))
        self._is_closed = False

    @property
    def is_closed(self) -> bool:
        return self._is_closed or bool(self._binding.is_closed)

    async def get_session(self) -> BindingSession:
        result = await self._binding.get_session()
        output = unwrap_binding_result(result, "Get Session")
        return get_binding_field(output, "session")

    async def get_schema(self) -> BindingGetSchemaOutput:
        result = await self._binding.get_schema()
        return unwrap_binding_result(result, "Get Schema")

    async def list_repositories(self) -> BindingListRepositoriesOutput:
        result = await self._binding.list_repositories()
        return unwrap_binding_result(result, "List Repositories")

    async def index_repository(self) -> BindingIndexRepositoryOutput:
        result = await self._binding.index_repository({})
        return unwrap_binding_result(result, "Index Repository")

    async def list_projects(
        self,
        input: BindingListProjectsInput,
    ) -> BindingListProjectsOutput:
        result = await self._binding.list_projects(dict(input))
        return unwrap_binding_result(result, "List Projects")

    async def write_graph(
        self,
        input: BindingWriteGraphInput,
    ) -> BindingWriteGraphOutput:
        result = await self._binding.write_graph(dict(input))
        return unwrap_binding_result(result, "Write Graph")

    async def read_graph(
        self,
        input: BindingReadGraphInput,
    ) -> BindingReadGraphOutput:
        result = await self._binding.read_graph(dict(input))
        return unwrap_binding_result(result, "Read Graph")

    async def query_graph(
        self,
        input: BindingQueryGraphInput,
    ) -> BindingQueryGraphOutput:
        result = await self._binding.query_graph(dict(input))
        return unwrap_binding_result(result, "Query Graph")

    async def search_graph(
        self,
        input: BindingSearchGraphInput,
    ) -> BindingSearchGraphOutput:
        result = await self._binding.search_graph(dict(input))
        return unwrap_binding_result(result, "Search Graph")

    async def trace_graph(
        self,
        input: BindingTraceGraphInput,
    ) -> BindingTraceGraphOutput:
        result = await self._binding.trace_graph(dict(input))
        return unwrap_binding_result(result, "Trace Graph")

    async def export_okf(
        self,
        input: BindingExportOkfInput,
    ) -> BindingExportOkfOutput:
        result = await self._binding.export_okf(dict(input))
        return unwrap_binding_result(result, "Export OKF")
    async def close(self) -> None:
        await self._binding.close()
        self._is_closed = True

    async def __aenter__(self) -> NativeBindingEngine:
        return self

    async def __aexit__(self, *_exc: object) -> None:
        await self.close()
