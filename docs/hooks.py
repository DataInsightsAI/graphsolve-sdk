"""MkDocs hooks for the SDK documentation site."""

import shutil
from pathlib import Path


def on_post_build(config, **kwargs):
    """Publish the API specification beside the HTTP reference page."""
    spec = Path(config["config_file_path"]).parent / "spec" / "graphsolve-v1.json"
    shutil.copyfile(spec, Path(config["site_dir"]) / "openapi.json")
