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

"""High-level engine factory mirroring the Node.js SDK surface."""

from __future__ import annotations

from typing import Any

from telepathic.native import NativeBindingEngine
from telepathic.native import setup as _setup
from telepathic.types.engine import Engine, EngineImpl

# Side-effect import: init tracing + tokio runtime once per process.
_ = _setup


async def create_engine(user_config: dict[str, Any] | None = None) -> Engine:
    """Create a Telepathic engine backed by the native PyO3 bindings.

    Parameters
    ----------
    user_config :
        Optional configuration. Recognized keys mirror the Node.js SDK:
        ``cwd``, ``repository_root``, and nested ``settings`` (e.g.
        ``settings.log_level``).
    """
    bindings = NativeBindingEngine(user_config or {})
    session = await bindings.get_session()
    return EngineImpl(bindings=bindings, session=session)
