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

from __future__ import annotations

from types import SimpleNamespace

import pytest

from telepathic.native.utils import (
    get_binding_field,
    is_binding_failure_result,
    unwrap_binding_result,
)


def test_is_binding_failure_result_true() -> None:
    failure = SimpleNamespace(
        is_binding_errors=True,
        errors=[SimpleNamespace(kind="NativeError", message="boom")],
    )
    assert is_binding_failure_result(failure) is True


def test_is_binding_failure_result_false_for_success_payload() -> None:
    success = {"session": {"session_id": "abc"}}
    assert is_binding_failure_result(success) is False


def test_unwrap_binding_result_returns_success() -> None:
    payload = {"schema": "graph"}
    assert unwrap_binding_result(payload, "Get Schema") == payload


def test_unwrap_binding_result_raises_on_failure() -> None:
    failure = SimpleNamespace(
        is_binding_errors=True,
        errors=[SimpleNamespace(kind="NativeError", message="nope")],
    )
    with pytest.raises(RuntimeError, match="Get Schema failed"):
        unwrap_binding_result(failure, "Get Schema")


def test_get_binding_field_from_mapping_and_object() -> None:
    assert get_binding_field({"session": "s1"}, "session") == "s1"
    assert get_binding_field(SimpleNamespace(session="s2"), "session") == "s2"
