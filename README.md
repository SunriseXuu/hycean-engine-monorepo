# 🌌 Hycean Engine (氦闪引擎)

![License: AGPL 3.0](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)
![WebGPU Ready](https://img.shields.io/badge/Render-WebGPU-orange.svg)
![Rust & Bevy](https://img.shields.io/badge/Core-Rust%20%2F%20Bevy-black.svg)
![Next.js](https://img.shields.io/badge/UI-Next.js-black.svg)

**Hycean Engine** is a next-generation, highly performant 3D editor and rendering engine built for the modern web. 
Powered by **Rust**, **Bevy ECS**, and **WebGPU**, it delivers desktop-grade 3D performance directly in the browser with zero GC stutters.

## 🚀 Architecture

Hycean Engine uses a cutting-edge Monorepo architecture to cleanly separate the hardcore 3D rendering pipeline from the modern web UI:

- **Core Engine (`packages/engine`)**: A pure Rust/Bevy ECS core compiled to WebAssembly. Handles all matrix math, raycasting, and WebGPU rendering pipelines.
- **Editor UI (`apps/web-next`)**: A Next.js application that provides a buttery-smooth, Figma-like interface to interact with the 3D core.
- **Desktop Ready (`apps/desktop-tauri`)**: *(Coming soon)* Native desktop client wrapped with Tauri for zero-latency local asset management.

## 🛠️ Tech Stack

* **Graphics**: wgpu / WebGPU / WebGL2 fallback
* **Logic Core**: Bevy Engine (Entity-Component-System)
* **Web UI**: Next.js, React, TailwindCSS, Shadcn
* **Toolchain**: Turborepo, Biome, pnpm, wasm-pack

## ⚖️ License & Commercial Use

Hycean Engine uses a **Dual-Licensing** model to support both the open-source community and commercial enterprise needs.

### 1. Open Source & Personal Use
This project is licensed under the [GNU AGPL-3.0 License](./LICENSE). 
You are free to use, modify, and distribute this software for **personal learning, open-source projects, and non-commercial purposes**, provided you also open-source your entire derivative work under the same AGPL-3.0 license.

### 2. Commercial License
If you wish to use Hycean Engine in a **closed-source commercial product**, or provide it as a SaaS service without open-sourcing your own application code, you **must** obtain a Commercial License.

For commercial licensing, enterprise support, or custom WebGPU rendering pipelines, please contact:
📧 **[x2992155894@gmail.com]**
