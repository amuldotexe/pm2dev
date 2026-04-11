import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

let serverUrl = process.env["AQUASCOPE_SERVER"] ?? "http://127.0.0.1:8008";

export default defineConfig(({mode}) => ({
  base: "./",
  define: {
    SERVER_URL: JSON.stringify(serverUrl),
    "process.env.NODE_ENV": JSON.stringify(mode),
  },
  plugins: [react()]
}));
