/*
 ██████╗ ██╗   ██╗████████╗██████╗  ██████╗  ██████╗  ██████╗     ██████╗  ██████╗  ██████╗ ██████╗  ██████╗ 
██╔═══██╗██║   ██║╚══██╔══╝██╔══██╗██╔═══██╗██╔════╝ ██╔═══██╗    ██╔══██╗██╔═══██╗██╔════╝██╔═══██╗██╔═══██╗
██║   ██║██║   ██║   ██║   ██████╔╝██║   ██║██║  ███╗██║   ██║    ██████╔╝██║   ██║██║     ██║   ██║██║   ██║
██║▄▄ ██║██║   ██║   ██║   ██╔═══╝ ██║   ██║██║   ██║██║   ██║    ██╔══██╗██║   ██║██║     ██║   ██║██║   ██║
╚██████╔╝╚██████╔╝   ██║   ██║     ╚██████╔╝╚██████╔╝╚██████╔╝    ██║  ██║╚██████╔╝╚██████╗╚██████╔╝╚██████╔╝
 ╚══▀▀═╝  ╚═════╝    ╚═╝   ╚═╝      ╚═════╝  ╚═════╝  ╚═════╝     ╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═════╝  ╚═════╝ 
                                                                                          app/static/bootstrap.js
*/

// Minimal progressive-enhancement bootstrap for worklets and workers

// CSS Paint Worklet (Houdini)
if (globalThis.CSS && 'paintWorklet' in CSS) {
  // Load a simple grid painter for future rulers/grids
  CSS.paintWorklet.addModule('static/worklets/paint-grid.js').catch(console.warn);
}

// Start a placeholder worker for OffscreenCanvas-based tasks
try {
  const worker = new Worker('static/workers/diff-worker.js', { type: 'module' });
  worker.postMessage({ type: 'init' });
} catch (e) {
  console.warn('Worker init failed:', e);
}
