// BingoCube — Browser-side visual identity engine
// Port of primals/bingoCube/crates/core BLAKE3-based identity system
// Fog of war: subcube(x) reveals progressively as you zoom in

(function(window) {
  'use strict';

  const PALETTE_16 = [
    [100,149,237], [60,179,113], [255,215,0], [220,20,60],
    [138,43,226], [255,140,0], [46,139,87], [199,21,133],
    [0,191,255], [154,205,50], [255,105,180], [64,224,208],
    [255,69,0], [186,85,211], [50,205,50], [255,20,147]
  ];

  // FNV-1a 64-bit hash (deterministic, fast, no WASM needed)
  // For browser we use FNV as BLAKE3 proxy — same seed = same grid
  function fnv64(str) {
    let h1 = 0x811c9dc5, h2 = 0x811c9dc5;
    for (let i = 0; i < str.length; i++) {
      let c = str.charCodeAt(i);
      h1 = Math.imul(h1 ^ c, 0x01000193);
      h2 = Math.imul(h2 ^ (c + 0x9e), 0x01000193);
    }
    return [h1 >>> 0, h2 >>> 0];
  }

  // Seeded PRNG (xorshift32)
  function xorshift(seed) {
    let s = seed | 1;
    return function() {
      s ^= s << 13; s ^= s >> 17; s ^= s << 5;
      return (s >>> 0) / 4294967296;
    };
  }

  // Generate a BingoCube color grid from entity ID
  function generate(entityId, gridSize) {
    gridSize = gridSize || 5;
    let [seedA, seedB] = fnv64(entityId + '_BINGOCUBE_V1');

    let rngA = xorshift(seedA);
    let rngB = xorshift(seedB);

    // Generate two boards
    let boardA = genBoard(gridSize, rngA);
    let boardB = genBoard(gridSize, rngB);

    // Cross-hash to color grid
    let grid = [];
    for (let i = 0; i < gridSize; i++) {
      grid[i] = [];
      for (let j = 0; j < gridSize; j++) {
        let [ha, hb] = fnv64('CELL_' + i + '_' + j + '_' + boardA[i][j] + '_' + boardB[i][j]);
        grid[i][j] = (ha ^ hb) % 16;
      }
    }
    return grid;
  }

  function genBoard(size, rng) {
    let board = [];
    for (let i = 0; i < size; i++) {
      board[i] = [];
      for (let j = 0; j < size; j++) {
        board[i][j] = Math.floor(rng() * 100);
      }
    }
    return board;
  }

  // Subcube reveal — returns set of revealed cells at level x ∈ (0,1]
  function subcube(grid, x) {
    let size = grid.length;
    let total = size * size;
    let revealCount = Math.ceil(x * total);
    
    // Compute scalar priority per cell (deterministic from grid values)
    let cells = [];
    for (let i = 0; i < size; i++) {
      for (let j = 0; j < size; j++) {
        let [s] = fnv64('SCALAR_' + i + '_' + j + '_' + grid[i][j]);
        cells.push({ i, j, scalar: s, color: grid[i][j] });
      }
    }
    cells.sort((a, b) => b.scalar - a.scalar);
    
    let revealed = new Set();
    for (let k = 0; k < revealCount && k < cells.length; k++) {
      revealed.add(cells[k].i + ',' + cells[k].j);
    }
    return { revealed, cells: cells.slice(0, revealCount) };
  }

  // Render a bingo card to a canvas element
  function renderToCanvas(canvas, entityId, opts) {
    opts = opts || {};
    let gridSize = opts.gridSize || 5;
    let cellPx = opts.cellPx || 8;
    let revealX = opts.revealX != null ? opts.revealX : 1.0;
    let entityType = opts.entityType || 'entity';
    let fogColor = opts.fogColor || '#1e1e23';

    let grid = generate(entityId, gridSize);
    let sub = subcube(grid, revealX);
    let size = gridSize * cellPx;

    canvas.width = size;
    canvas.height = size;
    let ctx = canvas.getContext('2d');

    // Type-specific border
    let borderColors = {
      false_face: '#FF0000',
      true_face: '#00FF88',
      person: '#6495ED',
      institution: '#FFD700',
      corporation: '#e040fb',
      legal_case: '#FF9100',
      article: '#888888',
      event: '#00BFFF',
      concept: '#9ACD32',
      infrastructure: '#2E8B57',
      evidence: '#FF4500'
    };

    for (let i = 0; i < gridSize; i++) {
      for (let j = 0; j < gridSize; j++) {
        let key = i + ',' + j;
        if (sub.revealed.has(key)) {
          let c = PALETTE_16[grid[i][j]];
          ctx.fillStyle = 'rgb(' + c[0] + ',' + c[1] + ',' + c[2] + ')';
        } else {
          ctx.fillStyle = fogColor;
        }
        ctx.fillRect(j * cellPx, i * cellPx, cellPx, cellPx);
      }
    }

    // Border glow based on entity type
    let bc = borderColors[entityType] || '#444';
    ctx.strokeStyle = bc;
    ctx.lineWidth = 2;
    ctx.strokeRect(0, 0, size, size);

    return canvas;
  }

  // Generate a data URL for a bingo card (for Leaflet markers)
  function toDataURL(entityId, opts) {
    let canvas = document.createElement('canvas');
    renderToCanvas(canvas, entityId, opts);
    return canvas.toDataURL();
  }

  // Generate a CSS gradient string (lightweight, no canvas needed)
  function toGradient(entityId, gridSize) {
    gridSize = gridSize || 3;
    let grid = generate(entityId, gridSize);
    let colors = [];
    for (let i = 0; i < gridSize; i++) {
      for (let j = 0; j < gridSize; j++) {
        let c = PALETTE_16[grid[i][j]];
        colors.push('rgb(' + c[0] + ',' + c[1] + ',' + c[2] + ')');
      }
    }
    let step = 100 / colors.length;
    let stops = colors.map((c, i) => c + ' ' + (i * step) + '% ' + ((i + 1) * step) + '%');
    return 'linear-gradient(135deg, ' + stops.join(', ') + ')';
  }

  // Compute a hex hash for entity (6 chars, deterministic)
  function entityHash(entityId) {
    let [h1, h2] = fnv64(entityId + '_ZK_IDENTITY');
    return ((h1 ^ h2) >>> 0).toString(16).padStart(8, '0').slice(0, 8);
  }

  // Fog of war level based on map zoom
  function fogFromZoom(zoom, minZoom, maxZoom) {
    minZoom = minZoom || 4;
    maxZoom = maxZoom || 18;
    let t = (zoom - minZoom) / (maxZoom - minZoom);
    return Math.max(0.08, Math.min(1.0, t));
  }

  window.BingoCube = {
    generate: generate,
    subcube: subcube,
    renderToCanvas: renderToCanvas,
    toDataURL: toDataURL,
    toGradient: toGradient,
    entityHash: entityHash,
    fogFromZoom: fogFromZoom,
    PALETTE_16: PALETTE_16
  };

})(window);
