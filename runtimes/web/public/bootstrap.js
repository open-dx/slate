// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
// let slate = await import("./pkg/slate.js")
//   .catch(e => console.error("Error importing `index.js`:", e));

// This is gross but it works for now.
// TODO: Find a real load pattern for wasm.
import slate_module from "./pkg/slate.js";
let slate = await slate_module();

slate.greet();
