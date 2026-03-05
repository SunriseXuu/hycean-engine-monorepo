import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// set the base to the monorepo root (2 levels up from configs/tailwind)
const repoRoot = path.resolve(__dirname, "../../");

export const postcssConfig = {
  plugins: {
    "@tailwindcss/postcss": {
      base: repoRoot,
    },
  },
};
