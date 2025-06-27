# plllots

A plotting library for creating SVG charts in Rust with a clean, modular architecture.

## Project Structure

The project is organized into several modules, inspired by the structure of the [Charming](https://github.com/yuankunzhang/charming) visualization library:

```
src/
├── lib.rs              # Main library entry point and public API
├── chart/              # Chart building and configuration
│   ├── mod.rs         # Chart module exports
│   └── builder.rs     # Chart struct with builder pattern
├── component/          # Chart components (axes, grids, etc.)
│   ├── mod.rs         # Component module exports
│   └── axis.rs        # X and Y axis implementations
├── element/            # Basic chart elements and primitives
│   ├── mod.rs         # Element module exports
│   ├── margin.rs      # Margin types and calculations
│   └── size.rs        # Plot size definitions
├── renderer/           # Output format renderers
│   ├── mod.rs         # Renderer module exports
│   └── svg.rs         # SVG renderer implementation
├── series/             # Different chart series types
│   ├── mod.rs         # Series module exports
│   └── line.rs        # Line chart series implementation
└── utils/              # Utility functions
    └── mod.rs         # Axis scaling and calculation utilities
```

## Architecture Benefits

### Separation of Concerns
- **Elements**: Basic building blocks (sizes, margins, colors)
- **Components**: Chart components (axes, legends, grids)
- **Series**: Different chart types (line, bar, pie, etc.)
- **Renderers**: Output format handlers (SVG, PNG, etc.)
- **Chart**: Main orchestration and builder pattern

### Extensibility
- Easy to add new chart types by implementing the `RenderSeries` trait
- New renderers can be added by implementing output-specific logic
- Components can be extended with new axis types, legends, etc.

### Maintainability
- Each module has a single responsibility
- Clear interfaces between modules
- Easy to locate and modify specific functionality

## Quick Start

```rust
use plllots::{Chart, SvgRenderer};
use plllots::element::PlotSize;
use plllots::component::{XAxis, YAxis, AxisData};

let chart = Chart::builder()
    .size(PlotSize { width: 800.0, height: 600.0 })
    .x_axis(XAxis::builder()
        .data(AxisData::Category(vec![
            "Jan".to_string(), "Feb".to_string(), "Mar".to_string()
        ]))
        .build())
    .y_axis(YAxis::builder()
        .data(AxisData::Values(vec![100.0, 200.0, 150.0]))
        .build())
    .build();

let renderer = SvgRenderer::new();
renderer.save(&chart, "chart.svg").unwrap();
```

## Module Details

### `chart/`
Contains the main `Chart` struct with builder pattern support. Handles the overall chart configuration and coordinates between different components.

### `component/`
Chart components like axes, grids, and legends. Each component implements the `AppendSvg` trait to render itself to the SVG document.

### `element/`
Basic elements and primitives used throughout the library:
- `PlotSize`: Chart dimensions
- `Margins`: Chart margins with pixel or percentage values
- `Offsets`: Calculated positioning values

### `renderer/`
Different output format renderers. Currently supports SVG output, but designed to be extensible for other formats.

### `series/`
Different chart series types. Each series implements the `RenderSeries` trait to handle data visualization.

### `utils/`
Utility functions for mathematical calculations, axis scaling, and other common operations.

## Development

Run tests:
```bash
cargo test
```

Check compilation:
```bash
cargo check
```

Generate documentation:
```bash
cargo doc --open
```

## Contributing

When adding new functionality:

1. **New chart types**: Add to `series/` module and implement `RenderSeries`
2. **New components**: Add to `component/` module and implement `AppendSvg`
3. **New renderers**: Add to `renderer/` module
4. **New elements**: Add to `element/` module for reusable primitives

Follow the existing patterns and ensure all public APIs are well-documented.