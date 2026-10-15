// A small ray tracer: classes, Float64Array, recursion and a lot of
// floating-point arithmetic. Only +, -, *, / and Math.sqrt are used, which
// are correctly rounded everywhere, so the output is exact across engines.
import { fnv1a, show } from "./lib/util.mjs";

class Vec {
  constructor(x, y, z) { this.x = x; this.y = y; this.z = z; }
  add(o) { return new Vec(this.x + o.x, this.y + o.y, this.z + o.z); }
  sub(o) { return new Vec(this.x - o.x, this.y - o.y, this.z - o.z); }
  mul(k) { return new Vec(this.x * k, this.y * k, this.z * k); }
  dot(o) { return this.x * o.x + this.y * o.y + this.z * o.z; }
  get length() { return Math.sqrt(this.dot(this)); }
  norm() { return this.mul(1 / this.length); }
}

class Sphere {
  constructor(center, radius, color, reflect) { Object.assign(this, { center, radius, color, reflect }); }
  intersect(origin, dir) {
    const oc = origin.sub(this.center);
    const b = oc.dot(dir);
    const c = oc.dot(oc) - this.radius * this.radius;
    const disc = b * b - c;
    if (disc < 0) return Infinity;
    const s = Math.sqrt(disc);
    const t = -b - s > 1e-6 ? -b - s : -b + s;
    return t > 1e-6 ? t : Infinity;
  }
  normal(p) { return p.sub(this.center).norm(); }
}

class Plane {
  constructor(y, color) { this.y = y; this.color = color; this.reflect = 0.3; }
  intersect(origin, dir) {
    if (Math.abs(dir.y) < 1e-9) return Infinity;
    const t = (this.y - origin.y) / dir.y;
    return t > 1e-6 ? t : Infinity;
  }
  normal() { return new Vec(0, 1, 0); }
  colorAt(p) { return (Math.floor(p.x) + Math.floor(p.z)) & 1 ? this.color : new Vec(0.9, 0.9, 0.9); }
}

const scene = [
  new Sphere(new Vec(0, 1, 5), 1, new Vec(0.9, 0.2, 0.2), 0.4),
  new Sphere(new Vec(-2.2, 0.7, 6), 0.7, new Vec(0.2, 0.9, 0.3), 0.2),
  new Sphere(new Vec(2, 1.5, 7), 1.5, new Vec(0.3, 0.4, 0.95), 0.6),
  new Plane(0, new Vec(0.1, 0.1, 0.1)),
];
const light = new Vec(-4, 8, 0);

function trace(origin, dir, depth) {
  let best = Infinity, hit = null;
  for (const obj of scene) {
    const t = obj.intersect(origin, dir);
    if (t < best) { best = t; hit = obj; }
  }
  if (!hit) return new Vec(0.5, 0.7, 1).mul(0.5 + 0.5 * dir.y);
  const p = origin.add(dir.mul(best));
  const n = hit.normal(p);
  const toLight = light.sub(p).norm();
  const shadowed = scene.some((o) => o !== hit && o.intersect(p, toLight) < light.sub(p).length);
  const diffuse = shadowed ? 0.1 : Math.max(0.1, n.dot(toLight));
  let color = (hit.colorAt ? hit.colorAt(p) : hit.color).mul(diffuse);
  if (depth < 3 && hit.reflect > 0) {
    const r = dir.sub(n.mul(2 * dir.dot(n))).norm();
    color = color.mul(1 - hit.reflect).add(trace(p, r, depth + 1).mul(hit.reflect));
  }
  return color;
}

const W = 96, H = 64;
const pixels = new Float64Array(W * H * 3);
const eye = new Vec(0, 1.5, 0);
for (let y = 0; y < H; y++) {
  for (let x = 0; x < W; x++) {
    const dir = new Vec((x - W / 2) / H, (H / 2 - y) / H, 1).norm();
    const c = trace(eye, dir, 0);
    pixels.set([c.x, c.y, c.z], (y * W + x) * 3);
  }
}
const bytes = Array.from(pixels, (v) => Math.min(255, Math.max(0, Math.round(v * 255))));
show("pixels", bytes.length / 3);
show("image hash", fnv1a(String.fromCharCode(...bytes.slice(0, 30000)) + String.fromCharCode(...bytes.slice(30000))));
const ramp = " .:-=+*#%@";
for (let y = 0; y < H; y += 4) {
  let row = "";
  for (let x = 0; x < W; x += 2) {
    const i = (y * W + x) * 3;
    const lum = 0.299 * pixels[i] + 0.587 * pixels[i + 1] + 0.114 * pixels[i + 2];
    row += ramp[Math.min(9, Math.floor(lum * 10))];
  }
  console.log(row);
}
