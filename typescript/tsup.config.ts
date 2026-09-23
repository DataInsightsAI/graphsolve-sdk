import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  // Dual output: plenty of the people who will use this are in a CommonJS
  // toolchain they do not control.
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
  sourcemap: true,
  target: "es2022",
  // Errors are matched with `instanceof`, but the class name is what shows up
  // in a log line, and a minifier renames classes unless told not to.
  keepNames: true,
});
