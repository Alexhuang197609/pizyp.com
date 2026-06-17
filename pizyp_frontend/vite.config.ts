import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const isStudio = env.VITE_BUILD_TYPE === "pistudio";

  return {
    plugins: [vue()],
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
      },
    },
    base: isStudio ? "./" : "/",
    server: {
      proxy: {
        "/api/v1": {
          // 修正：线上仅开放443 HTTPS，无外网3000
          target: "https://www.pizyp.com",
          changeOrigin: true,
          timeout: 60000,
          proxyTimeout: 600000,
          secure: false,
        },
      },
    },
  };
});
