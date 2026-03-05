import path from "node:path";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // 设置 turbopack 根目录
  turbopack: {
    root: path.join(__dirname, "../.."),
  },
  // 解析 monorepo 中的包
  transpilePackages: [
    "@hycean-engine/engine",
    // "@hycean-engine/ui",
    // "@hycean-engine/api",
    // "@hycean-engine/store",
    // "@hycean-engine/tools",
    // "@hycean-engine/constants",
    // "@hycean-engine/types",
    // "@hycean-engine/i18n",
  ],
};

export default nextConfig;
