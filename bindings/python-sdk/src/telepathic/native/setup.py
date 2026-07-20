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

"""Initialize tracing and the Tokio runtime used by PyO3 async methods."""

from __future__ import annotations

import atexit

from telepathic._telepathic_pyo3 import create_tokio_runtime, init_trace_subscriber

_subscriber_guard = init_trace_subscriber()
create_tokio_runtime()


def _close_trace_subscriber() -> None:
    if _subscriber_guard is not None:
        _subscriber_guard.close()


atexit.register(_close_trace_subscriber)
