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

"""Engine protocol and concrete implementation."""

from __future__ import annotations

from typing import TYPE_CHECKING, Protocol

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

if TYPE_CHECKING:
    from telepathic.native.engine import NativeBindingEngine


class Engine(Protocol):
    """Public Telepathic engine surface (mirrors the Node.js SDK)."""

    session: BindingSession

    async def get_schema(self) -> BindingGetSchemaOutput: ...

    async def list_repositories(self) -> BindingListRepositoriesOutput: ...

    async def index_repository(self) -> BindingIndexRepositoryOutput: ...

    async def list_projects(
        self,
        input: BindingListProjectsInput,
    ) -> BindingListProjectsOutput: ...

    async def write_graph(
        self,
        input: BindingWriteGraphInput,
    ) -> BindingWriteGraphOutput: ...

    async def read_graph(
        self,
        input: BindingReadGraphInput,
    ) -> BindingReadGraphOutput: ...

    async def query_graph(
        self,
        input: BindingQueryGraphInput,
    ) -> BindingQueryGraphOutput: ...

    async def search_graph(
        self,
        input: BindingSearchGraphInput,
    ) -> BindingSearchGraphOutput: ...

    async def trace_graph(
        self,
        input: BindingTraceGraphInput,
    ) -> BindingTraceGraphOutput: ...

    async def export_okf(
        self,
        input: BindingExportOkfInput,
    ) -> BindingExportOkfOutput: ...


class EngineImpl:
    """Concrete engine that delegates to ``NativeBindingEngine``."""

    def __init__(self, bindings: NativeBindingEngine, session: BindingSession) -> None:
        self._bindings = bindings
        self.session = session

    async def get_schema(self) -> BindingGetSchemaOutput:
        return await self._bindings.get_schema()

    async def list_repositories(self) -> BindingListRepositoriesOutput:
        return await self._bindings.list_repositories()

    async def index_repository(self) -> BindingIndexRepositoryOutput:
        return await self._bindings.index_repository()

    async def list_projects(
        self,
        input: BindingListProjectsInput,
    ) -> BindingListProjectsOutput:
        return await self._bindings.list_projects(input)

    async def write_graph(
        self,
        input: BindingWriteGraphInput,
    ) -> BindingWriteGraphOutput:
        return await self._bindings.write_graph(input)

    async def read_graph(
        self,
        input: BindingReadGraphInput,
    ) -> BindingReadGraphOutput:
        return await self._bindings.read_graph(input)

    async def query_graph(
        self,
        input: BindingQueryGraphInput,
    ) -> BindingQueryGraphOutput:
        return await self._bindings.query_graph(input)

    async def search_graph(
        self,
        input: BindingSearchGraphInput,
    ) -> BindingSearchGraphOutput:
        return await self._bindings.search_graph(input)

    async def trace_graph(
        self,
        input: BindingTraceGraphInput,
    ) -> BindingTraceGraphOutput:
        return await self._bindings.trace_graph(input)

    async def export_okf(
        self,
        input: BindingExportOkfInput,
    ) -> BindingExportOkfOutput:
        return await self._bindings.export_okf(input)
