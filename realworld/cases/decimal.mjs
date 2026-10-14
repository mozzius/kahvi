// decimal.js: arbitrary-precision decimal arithmetic.
import Decimal from "decimal.js";
import { show } from "./lib/util.mjs";

Decimal.set({ precision: 60, rounding: Decimal.ROUND_HALF_EVEN });
show("0.1+0.2", new Decimal("0.1").plus("0.2").toString());
show("sqrt2", Decimal.sqrt(2).toString());
show("pi", Decimal.acos(-1).toString());
show("e", Decimal.exp(1).toString());
show("ln10", Decimal.ln(10).toString());
show("pow", new Decimal(2).pow(200).toString());
show("big div", new Decimal("123456789012345678901234567890").div("987654321").toFixed(30));
show("fraction", new Decimal("0.142857142857142857").toFraction(1000).map(String));
show("rounding", [2.5, 3.5, -2.5].map((n) => new Decimal(n).toDecimalPlaces(0).toString()));
show("hex", [new Decimal(255.5).toHex(), new Decimal("0x1f.8").toString(), new Decimal(10).toBinary(8)]);
show("exp notation", [new Decimal("1e-20").toString(), new Decimal("123.456e30").toExponential(5), new Decimal(-0).isNegative()]);
show("compare", [new Decimal("1.000").eq(1), Decimal.max("3.3", "10", "-1").toString(), new Decimal(NaN).isNaN()]);
let sum = new Decimal(0);
for (let i = 1; i <= 1000; i++) sum = sum.plus(new Decimal(1).div(i * i));
show("basel(1000)", sum.toSignificantDigits(40).toString());
