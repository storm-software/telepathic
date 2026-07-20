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

"""Native PyO3 binding wrappers."""

from __future__ import annotations

from importlib import import_module
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from telepathic.native.engine import NativeBindingEngine

__all__ = ["NativeBindingEngine"]


def __getattr__(name: str) -> Any:
    if name == "NativeBindingEngine":
        return import_module("telepathic.native.engine").NativeBindingEngine
    msg = f"module {__name__!r} has no attribute {name!r}"
    raise AttributeError(msg)
