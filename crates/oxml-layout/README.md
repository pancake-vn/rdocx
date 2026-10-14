# oxml-layout

Represent fully laid-out pages in a backend-neutral form ready for PDF or
raster rendering.

## Capabilities

- Page frames with positioned text, images, lines, rectangles, and paths.
- Transforms, paints, effects, links, outlines, and logical structure.
- Font discovery, shaping, metrics, and deterministic bundled fonts.
- Multilingual line breaking, bidirectional text, tabs, and inline items.
- Recursive groups, marked content, transforms, source ranges, and logical
  structure give downstream backends one complete positioned-page contract.

## Measured footprint and speed

The archive row is regenerated from the complete published package that carries this README.

| Measurement | Value | Version | Platform | Build mode | Input | Command | Statistic | Measured on |
|---|---|---|---|---|---|---|---|---|
| Crates.io archive: oxml-layout | 4,635,449 compressed bytes, 9,275,311 member bytes, 51 members | 0.14.0 | macOS 26.4.1, arm64 | `cargo package --locked --no-verify` | Tracked `oxml-layout` package inventory | `python3 scripts/readme_doctests.py --record-measurements` | gzip archive bytes, tar member bytes, tar member count | 2026-10-13 |

## Use it when

Use this crate as the interchange layer between a format-specific layout engine
and an output backend. It does not parse DOCX or PPTX and does not perform
their format-specific pagination.

## Relationship

`rdocx-layout` and `rpptx-render` produce this model. `oxml-pdf` consumes it.

Consumers that walk `PageFrame::elements` must recurse through
`PositionedElement::MarkedContent` and visit its `MarkedContent::children`, or
use `oxml_layout::walk` to perform recursive traversal. Because
`PositionedElement` is non-exhaustive, a wildcard arm that ignores a new
container can otherwise omit visible page content.

## Example

```rust,no_run
use oxml_layout::Color;

let accent = Color::from_hex("3366CC");
assert_eq!((accent.r, accent.g, accent.b), (0.2, 0.4, 0.8));
```

Add `oxml-layout = { version = "0.14.0", default-features = false }` to select the small configuration.

The default `full-fonts` feature embeds all 27 font files. Disabling it embeds
only the four Liberation Sans styles outside macOS and Windows. On macOS and
Windows, a small build uses system or caller-supplied fonts.

Use `default-features = false, features = ["full-fonts"]` for deterministic
rendering with the complete font set and no system font discovery. Use
`default-features = false, features = ["system-fonts"]` for a small native build
with system font discovery. Cargo combines features across dependencies, so
every dependency path must leave `full-fonts` disabled for a small build.
