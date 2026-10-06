# Raymatic v0.2.0

Raymatic v0.2.0 adds a bounded, safe path for moving a conventional Pelican publication into a native Raymatic project.

## Highlights

- `ray migrate inspect <source>` inventories a Pelican project without modifying it.
- `ray migrate import <source> <destination>` creates and validates a native project before publishing the destination.
- The supported subset covers conventional Markdown, Pelican header metadata or YAML front matter, pages, portable assets, drafts, aliases, selected literal site settings, and common `{static}` and `{filename}` links.
- `--generate-alt-text` is an explicit opt-in for deterministic filename-based labels when imported images have empty alternative text; strict accessibility validation remains the default.
- Inspection and asset validation now use portable paths across macOS, Linux, and Windows.

## Supported platform

The published artifact targets macOS Apple Silicon (`aarch64-apple-darwin`). The source and continuous integration are exercised on macOS, Linux, and Windows.

## Migration boundary

Raymatic does not execute Pelican configuration, plugins, themes, templates, or hooks. Those features, visual parity, arbitrary static-path selection, and non-slash legacy URLs remain explicit review items. See [the migration guide](docs/MIGRATION.md) for the supported contract.

## Upgrade

Replace an existing `ray` executable with the verified v0.2.0 archive. Existing Raymatic projects remain compatible; migration commands create a new destination and do not modify the Pelican source.
