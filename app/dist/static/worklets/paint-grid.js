/*
 ██████╗  █████╗ ██╗███╗   ██╗████████╗     ██████╗ ██████╗ ██╗██████╗     ██████╗ ██████╗ ██╗██████╗ 
██╔═══██╗██╔══██╗██║████╗  ██║╚══██╔══╝    ██╔════╝ ██╔══██╗██║██╔══██╗    ██╔══██╗██╔══██╗██║██╔══██╗
██║   ██║███████║██║██╔██╗ ██║   ██║       ██║  ███╗██████╔╝██║██████╔╝    ██████╔╝██████╔╝██║██████╔╝
██║   ██║██╔══██║██║██║╚██╗██║   ██║       ██║   ██║██╔═══╝ ██║██╔══██╗    ██╔═══╝ ██╔══██╗██║██╔═══╝ 
╚██████╔╝██║  ██║██║██║ ╚████║   ██║       ╚██████╔╝██║     ██║██║  ██║    ██║     ██║  ██║██║██║     
 ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝   ╚═╝        ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝    ╚═╝     ╚═╝  ╚═╝╚═╝╚═╝     
                                                                                 static/worklets/paint-grid.js
*/

// Minimal grid painter; used as progressive enhancement
class GridPainter {
  static get inputProperties() {
    return [
      '--grid-color',
      '--grid-size',
      '--grid-line-width',
      '--grid-opacity',
    ];
  }
  paint(ctx, geom, properties) {
    const w = geom.width;
    const h = geom.height;
    const size = parseFloat(properties.get('--grid-size')?.toString() || '8');
    const color = properties.get('--grid-color')?.toString().trim() || '#7e8aa0';
    const lw = parseFloat(properties.get('--grid-line-width')?.toString() || '1');
    const opacity = parseFloat(properties.get('--grid-opacity')?.toString() || '0.25');

    ctx.lineWidth = lw;
    ctx.strokeStyle = color;
    ctx.globalAlpha = opacity;

    for (let x = 0; x <= w; x += size) {
      ctx.beginPath();
      ctx.moveTo(x + 0.5, 0);
      ctx.lineTo(x + 0.5, h);
      ctx.stroke();
    }
    for (let y = 0; y <= h; y += size) {
      ctx.beginPath();
      ctx.moveTo(0, y + 0.5);
      ctx.lineTo(w, y + 0.5);
      ctx.stroke();
    }
  }
}

// Register under "grid". Use via: background: paint(grid);
registerPaint('grid', GridPainter);
