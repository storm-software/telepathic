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

from telepathic.native.config import binding_options_from_config


def test_binding_options_from_nested_settings() -> None:
    options = binding_options_from_config(
        {
            "cwd": "/tmp/repo",
            "settings": {"log_level": "debug", "mode": "development"},
        },
    )
    assert options == {
        "log_level": "debug",
        "mode": "development",
        "cwd": "/tmp/repo",
    }


def test_binding_options_accepts_camel_case_aliases() -> None:
    options = binding_options_from_config(
        {
            "repositoryRoot": "/tmp/root",
            "settings": {"logLevel": "info"},
        },
    )
    assert options["repository_root"] == "/tmp/root"
    assert options["log_level"] == "info"
