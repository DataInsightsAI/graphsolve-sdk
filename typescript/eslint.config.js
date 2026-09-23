import js from "@eslint/js";
import tseslint from "typescript-eslint";

export default tseslint.config(
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    rules: {
      // The generated surface and the wire types are snake_case on purpose:
      // they are the API's names, not ours.
      "@typescript-eslint/naming-convention": "off",
    },
  },
  { ignores: ["dist/"] },
);
