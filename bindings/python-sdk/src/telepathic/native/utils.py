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

"""Helpers for native binding results."""

from __future__ import annotations

from collections.abc import Mapping
from typing import Any, TypeGuard, TypeVar

T = TypeVar("T")


def is_binding_failure_result(value: object) -> TypeGuard[Any]:
    """Return True when ``value`` is a native ``BindingErrors`` failure payload."""
    return (
        value is not None
        and hasattr(value, "is_binding_errors")
        and bool(getattr(value, "is_binding_errors", False))
        and hasattr(value, "errors")
        and isinstance(getattr(value, "errors"), list)
    )


def unwrap_binding_result(result: T | Any, operation: str) -> T:
    """Unwrap a native binding result or raise with aggregated error messages."""
    if is_binding_failure_result(result):
        messages: list[str] = []
        for error in result.errors:
            message = getattr(error, "message", None)
            if isinstance(message, str):
                messages.append(message)
            else:
                messages.append(str(error))
        joined = "\n".join(messages) if messages else "unknown binding error"
        msg = f"Telepathic - {operation} failed with errors: {joined}"
        raise RuntimeError(msg)
    return result  # type: ignore[return-value]


def get_binding_field(value: Any, name: str) -> Any:
    """Read a field from a mapping or attribute-bearing native object."""
    if isinstance(value, Mapping):
        return value[name]
    return getattr(value, name)
