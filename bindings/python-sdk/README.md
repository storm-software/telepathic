<!-- START header -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->


<div align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://public.storm-cdn.com/telepathic/media/banner-1280x640-dark.gif">
  <source media="(prefers-color-scheme: light)" srcset="https://public.storm-cdn.com/telepathic/media/banner-1280x640-light.gif">
<img src="https://public.storm-cdn.com/telepathic/media/banner-1280x640-dark.gif" width="100%" alt="Telepathic" />
</picture>
</div>
<br />

<div align="center">
<b>
<a href="https://stormsoftware.com" target="_blank">Website</a>  •
<a href="https://github.com/storm-software/telepathic" target="_blank">GitHub</a>  •
<a href="https://discord.gg/MQ6YVzakM5">Discord</a>  •  <a href="https://docs.stormsoftware.com/projects/telepathic/" target="_blank">Docs</a>  •  <a href="https://stormsoftware.com/contact" target="_blank">Contact</a>  •
<a href="https://github.com/storm-software/telepathic/issues/new?assignees=&labels=bug&template=bug-report.yml&title=Bug Report%3A+">Report a Bug</a>
</b>
</div>

<br />

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- END header -->

# Telepathic - Python Bindings

**Telepathic - Python Bindings** binds the Telepathic runtime to [Python](https://www.python.org/) using [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/).

## Installing

```bash
pip install telepathic
```

For local development from this monorepo:

```bash
cd bindings/python-sdk
maturin develop
```

## Usage

```python
import asyncio
from telepathic import create_engine

async def main() -> None:
    engine = await create_engine({"cwd": "."})
    schema = await engine.get_schema()
    print(schema)

asyncio.run(main())
```

## Building

Run `nx build bindings-python-sdk` (or `maturin build --release` in this directory).

## Running unit tests

Run `nx test bindings-python-sdk` (or `pytest` in this directory).

## License

Apache-2.0. See [LICENSE](../../LICENSE).
