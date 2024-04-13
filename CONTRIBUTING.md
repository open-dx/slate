# Contributing

1. Check existing issues and projects for ongoing work.
2. Fork the repository and create your feature branch.
3. Write code and tests in line with the project's standards.
4. Ensure your changes do not break existing functionality.
5. Submit a pull request with a clear description of your changes.

## Roadmap

### Goals:

TODO

#### Future Considerations:

1.  We intend to support ui and scene composition for Unreal, Unity, and Godot. Please read the respective README files (when they exist).

### Work Breakdown:

TODO: Move all of this to Github issues/project.

-   [x] Parsing JSX-like Blocks
-   [x] Scaffold: UI Composition
-   [ ] Nested layouts.
-   [ ] Scaffold: Layout Solver (Cassowary)
-   [ ] Styles: TBD
-   [ ] Events: Routing
-   [ ] Terminal Surface (Crossterm)
-   [ ] UI Surface (Bevy)
-   [ ] Web Surface (___?)
-   [ ] Open-source Staging

## Benchmarks

TODO

### Notes:

-   Install `gnuplot` for better Criterion reports.

## Profiling

TODO

### Notes

-   Run examples and binaries with `--feature profiling` to enable profiling features.

-   Install the Tracy GUI to view tracy output.

## Architecture

Slate's architecture is adaptable to various rendering systems. Key components include:

### Surface

A Surface can be used to build a Scaffold and register additional styles for an instance of an Element. An Element represents a visual/spatial element in a graph of Elements. The graph structure represents a hybrid set of Elements indexed in various ways (parent/child, roots, etc).

### Scaffold

A Surface can be used to build a Scaffold and register additional styles for an instance of an Element. An Element represents a visual/spatial element in a graph of Elements. The graph structure represents a hybrid set of Elements indexed in various ways (parent/child, roots, etc).

### Style

A Styleguide represents a collection of style decisions to use when composing the output for a render target. While building the surface, targetted style overrides can be added on top of the Stylesheet generated for each element by the base Styleguide by saving a tuple of the Node UUID to a `Vec<Rule>` from the Styleguide. Each Rule is either global or is matched to the Element by UUID.

### Stages

0.  Compilation:
    -   A JSX-like DSL is decomposed into a Scaffold, which is then "drawn" to a surface: `surface.draw(|scaffold| { .. })`.
    -   

1.  Runtime:
    a.  A `Surface` is built up during application startup phases.
    b.  Extnernal `Runtime` systems (Bevy, Ratatui, the web, etc) coordinate events and updates for a Surface, usually through plugins.
        -   Ratatui: TODO
        -   Bevy: TODO
