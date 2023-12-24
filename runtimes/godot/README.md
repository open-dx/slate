# Godot Slate

Experimental support for Slate in Godot.

Reference: https://godot-rust.github.io/book

## Notes:

-   The project currently references cargo's `target` folder for build artifacts, which introduces some configuration limitations (location of target dir, for example).
    -   **TODO:** Write a build script to move generated binaries to a dist folder (or something) so godot can get to them easier.

-   Godot editor can't be open while making changes to Rust code. This is a huge pain for the workflow.
