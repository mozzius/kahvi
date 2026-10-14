import { a } from './cycle-a.mjs';
export function b() {
  return 'b';
}
export const fromA = () => a();
