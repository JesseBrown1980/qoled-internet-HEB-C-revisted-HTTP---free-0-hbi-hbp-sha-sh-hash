import fs from 'fs';
import crypto from 'crypto';

const K = 27;
const L = 24;
const V = L * L * L; // 13,824
const N = V * K;     // 373,248
const PER_CELL = 32;

// 1. Velocities
const velocities = [];
for (let x = -1; x <= 1; x++) {
  for (let y = -1; y <= 1; y++) {
    for (let z = -1; z <= 1; z++) {
      velocities.push({ x, y, z, e: x * x + y * y + z * z });
    }
  }
}

function lex(a, b) {
  for (let i = 0; i < Math.min(a.length, b.length); i++) {
    if (a[i] !== b[i]) return a[i] - b[i];
  }
  return a.length - b.length;
}

// 2. Collisions (identical to index.html and C++)
function buildCollisions() {
  const groups = new Map();
  for (let a = 0; a < K; a++) {
    for (let b = a + 1; b < K; b++) {
      let va = velocities[a], vb = velocities[b];
      let key = [va.x + vb.x, va.y + vb.y, va.z + vb.z, va.e + vb.e].join(',');
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key).push([a, b]);
    }
  }
  const uniques = new Map();
  for (let pairs of groups.values()) {
    for (let i = 0; i < pairs.length; i++) {
      for (let j = i + 1; j < pairs.length; j++) {
        let q = [...pairs[i], ...pairs[j]].sort((a, b) => a - b);
        if (new Set(q).size === 4) uniques.set(q.join(','), q);
      }
    }
  }
  const quads = [...uniques.values()].sort(lex), collisions = [];
  for (const q of quads) {
    const lut = new Uint8Array(81);
    for (let code = 0; code < 81; code++) lut[code] = code;
    const cls = new Map();
    for (let code = 0; code < 81; code++) {
      let d0 = code % 3, rem1 = Math.floor(code / 3);
      let d1 = rem1 % 3, rem2 = Math.floor(rem1 / 3);
      let d2 = rem2 % 3, d3 = Math.floor(rem2 / 3);
      let d = [d0, d1, d2, d3];
      let m = 0, px = 0, py = 0, pz = 0, e = 0;
      for (let j = 0; j < 4; j++) {
        m += d[j];
        px += d[j] * velocities[q[j]].x;
        py += d[j] * velocities[q[j]].y;
        pz += d[j] * velocities[q[j]].z;
        e  += d[j] * velocities[q[j]].e;
      }
      let key = [m, px, py, pz, e].join(',');
      if (!cls.has(key)) cls.set(key, []);
      cls.get(key).push(code);
    }
    for (let codes of cls.values()) {
      codes = codes.filter(c => c !== 40);
      for (let j = 0; j + 1 < codes.length; j += 2) {
        lut[codes[j]] = codes[j + 1];
        lut[codes[j + 1]] = codes[j];
      }
    }
    let active = false;
    for (let code = 0; code < 81; code++) {
      if (lut[code] !== code) { active = true; break; }
    }
    if (active) collisions.push({ a: q[0], b: q[1], c: q[2], d: q[3], lut });
  }
  return collisions;
}

const collisions = buildCollisions();
const G = collisions.length; // 288

// 3. Streaming destination table
const streamDest = new Int32Array(N);
for (let x = 0; x < L; x++) {
  for (let y = 0; y < L; y++) {
    for (let z = 0; z < L; z++) {
      const srcCell = (x * L + y) * L + z;
      for (let k = 0; k < K; k++) {
        const nx = (x + velocities[k].x + L) % L;
        const ny = (y + velocities[k].y + L) % L;
        const nz = (z + velocities[k].z + L) % L;
        const dstCell = (nx * L + ny) * L + nz;
        streamDest[srcCell * K + k] = dstCell * K + k;
      }
    }
  }
}

// 4. Involutive Collision Stage
function collide(a, t) {
  for (let cell = 0, base = 0; cell < V; cell++, base += K) {
    for (let j = 0; j < PER_CELL; j++) {
      const gate = collisions[(t * PER_CELL + cell * 17 + j * 37) % G];
      const A = base + gate.a, B = base + gate.b, C = base + gate.c, D = base + gate.d;
      let code = a[A] + 3 * a[B] + 9 * a[C] + 27 * a[D];
      code = gate.lut[code];
      a[A] = code % 3; code = (code / 3) | 0;
      a[B] = code % 3; code = (code / 3) | 0;
      a[C] = code % 3; code = (code / 3) | 0;
      a[D] = code % 3;
    }
  }
}

// 5. Load physical 110,592-byte block from sector-000.rooms
const raw = fs.readFileSync('D:/ASOLARIA-HOST8-ROOMS-300G/sector-000.rooms').subarray(0, 110592);
const blockSha = crypto.createHash('sha256').update(raw).digest('hex');
console.log('JS: Loaded physical block SHA-256:', blockSha);

const view = new DataView(raw.buffer, raw.byteOffset, raw.byteLength);
let state = new Uint8Array(N);
let next = new Uint8Array(N);

for (let cell = 0; cell < V; cell++) {
  let val = view.getBigUint64(cell * 8, true);
  const base = cell * K;
  for (let k = 0; k < K; k++) {
    state[base + k] = Number(val % 3n);
    val /= 3n;
  }
}

const initSha = crypto.createHash('sha256').update(state).digest('hex');
console.log('JS: Initial state SHA-256 (tick 0):', initSha);

const jsCsvRows = ['tick,state_sha256\n', `0,${initSha}\n`];

const t0 = performance.now();
for (let t = 0; t < 243; t++) {
  // Collide
  collide(state, t);
  // Spatial stream
  for (let i = 0; i < N; i++) {
    next[streamDest[i]] = state[i];
  }
  // Swap
  const tmp = state;
  state = next;
  next = tmp;

  const tickSha = crypto.createHash('sha256').update(state).digest('hex');
  jsCsvRows.push(`${t + 1},${tickSha}\n`);
}
const elapsed = (performance.now() - t0) / 1000.0;
const finalSha = crypto.createHash('sha256').update(state).digest('hex');

console.log('JS: Final state SHA-256 (tick 243):', finalSha);
console.log(`JS: Kernel 243-tick execution time: ${elapsed.toFixed(3)} s (${(243 / elapsed).toFixed(1)} ticks/s, ${(243 * V / elapsed).toFixed(0)} rooms/s)`);

fs.writeFileSync('hebc_js_physical_243_ticks.csv', jsCsvRows.join(''));
