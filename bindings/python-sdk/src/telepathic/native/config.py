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

"""Pure-Python config mapping for native ``BindingOptions``."""

from __future__ import annotations

from typing import Any


def binding_options_from_config(config: dict[str, Any]) -> dict[str, Any]:
    """Map user-facing config (incl. Node-style aliases) to binding options."""
    settings = config.get("settings")
    options: dict[str, Any] = {}

    if isinstance(settings, dict):
        if "log_level" in settings:
            options["log_level"] = settings["log_level"]
        elif "logLevel" in settings:
            options["log_level"] = settings["logLevel"]
        if "mode" in settings:
            options["mode"] = settings["mode"]

    for key in ("mode", "username", "log_level", "cwd", "repository_root", "custom_logger"):
        if key in config:
            options[key] = config[key]

    if "repositoryRoot" in config and "repository_root" not in options:
        options["repository_root"] = config["repositoryRoot"]

    return options
