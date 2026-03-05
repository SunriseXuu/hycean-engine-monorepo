"use client";

import { useEffect, useState } from "react";

export default function Engine() {
  const [isEngineReady, setIsEngineReady] = useState<boolean>(false);

  useEffect(() => {
    // 只有在浏览器挂载完成后，才开始动态加载 WASM 引擎
    const igniteEngine = async () => {
      try {
        // 动态导入你的 Rust 引擎包
        const engine = await import("@hycean-engine/engine");

        // 执行 wasm-pack 生成的默认初始化函数
        // 这行代码执行完，你的 Rust 里的 #[wasm_bindgen(start)] 就会被唤醒！
        await engine.default();

        setIsEngineReady(true);
        console.log("🌌 Hycean Engine Ignited Successfully!");
      } catch (error) {
        console.error("引擎点火失败，核心引擎未能加载:", error);
      }
    };

    // 延迟一小点时间，确保 DOM 里的 canvas 已经彻底画好了
    requestAnimationFrame(() => igniteEngine());
  }, []);

  return (
    <div className="flex min-h-screen w-full items-center justify-center overflow-hidden">
      {/* 极客风的加载动画，当引擎还没就绪时显示 */}
      {!isEngineReady && (
        <div className="absolute z-10 flex flex-col items-center gap-4">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-emerald-500 border-t-transparent" />
          <p className="font-mono text-emerald-500 animate-pulse tracking-widest">
            INITIALIZING CORE ENGINE...
          </p>
        </div>
      )}

      {/* 这就是我们在 Rust 里指定的物理学坐标原点！
        注意 id="engine" 必须和 Rust 里配置的画布 ID 一字不差 
      */}
      <canvas
        id="engine"
        className="w-full! min-h-screen! outline-none"
        // 屏蔽浏览器的右键默认菜单，因为 3D 编辑器通常需要右键拖拽视角
        onContextMenu={(e) => e.preventDefault()}
      />
    </div>
  );
}
