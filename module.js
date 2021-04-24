import init, {greet} from "./rust-lib/pkg/rust_lib.js";

init().then(() => {
  greet("Electron");
});