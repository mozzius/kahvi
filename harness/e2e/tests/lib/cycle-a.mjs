import { b } from './cycle-b.mjs';
export function a() {
  return 'a';
}
export const fromB = () => b();
