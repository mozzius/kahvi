// zod: schema validation, error reporting, transforms and JSON Schema.
import { z } from "zod";
import { show } from "./lib/util.mjs";

const User = z.object({
  id: z.number().int().positive(),
  email: z.string().email(),
  name: z.string().min(2).max(20).trim(),
  tags: z.array(z.enum(["admin", "dev", "ops"])).default([]),
  address: z.object({ city: z.string(), zip: z.string().regex(/^\d{5}$/) }).optional(),
  createdAt: z.coerce.date(),
}).strict();

show("valid", User.parse({ id: 1, email: "a@b.co", name: "  Ada  ", createdAt: "2020-01-02T03:04:05Z" }));
const bad = User.safeParse({ id: -1.5, email: "nope", name: "A", tags: ["root"], address: { city: 3, zip: "1234" }, extra: true });
show("invalid", bad.success);
for (const issue of bad.error.issues) show("issue", [issue.code, issue.path.join("."), issue.message]);
show("flatten", z.flattenError(bad.error));

const Shape = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("circle"), r: z.number() }),
  z.object({ kind: z.literal("rect"), w: z.number(), h: z.number() }),
]);
const area = Shape.transform((s) => (s.kind === "circle" ? Math.PI * s.r ** 2 : s.w * s.h));
show("areas", [area.parse({ kind: "circle", r: 2 }), area.parse({ kind: "rect", w: 3, h: 4 })]);
show("union error", Shape.safeParse({ kind: "triangle" }).error.issues.map((i) => i.code));
const Password = z.string().refine((s) => /[A-Z]/.test(s), { message: "needs uppercase" }).pipe(z.string().min(8));
show("refine", [Password.safeParse("lowercase1").error?.issues[0].message, Password.safeParse("Upper").error?.issues[0].message, Password.safeParse("UpperCase").success]);
const Rec = z.record(z.string(), z.tuple([z.number(), z.boolean()]));
show("record", Rec.safeParse({ a: [1, true], b: [2, "no"] }).error.issues.map((i) => i.path.join("/")));
const Tree = z.lazy(() => z.object({ value: z.number(), children: z.array(Tree) }));
show("lazy", Tree.safeParse({ value: 1, children: [{ value: 2, children: [{ value: "x", children: [] }] }] }).error.issues[0].path);
show("jsonSchema", z.toJSONSchema(User.omit({ createdAt: true })));
