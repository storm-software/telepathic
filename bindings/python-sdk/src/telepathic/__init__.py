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

"""Telepathic Python SDK — PyO3 bindings for the Telepathic runtime."""

from __future__ import annotations

from importlib import import_module
from importlib.metadata import PackageNotFoundError, version
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from telepathic.engine import create_engine
    from telepathic.types import Engine

try:
    __version__ = version("telepathic")
except PackageNotFoundError:
    __version__ = "0.0.0-dev"

__all__ = [
    "Engine",
    "__version__",
    "create_engine",
]


def __getattr__(name: str) -> Any:
    if name == "create_engine":
        return import_module("telepathic.engine").create_engine
    if name == "Engine":
        return import_module("telepathic.types").Engine
    msg = f"module {__name__!r} has no attribute {name!r}"
    raise AttributeError(msg)
