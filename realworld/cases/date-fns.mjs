// date-fns: formatting, parsing and arithmetic on UTC dates (the runner
// sets TZ=UTC).
import { addDays, addMonths, differenceInCalendarDays, eachDayOfInterval, format, formatDistanceStrict, formatISO, isLeapYear, parse, parseISO, startOfWeek, endOfMonth, intervalToDuration, formatDuration, getISOWeek, isValid, compareAsc } from "date-fns";
import { show } from "./lib/util.mjs";

const d = new Date(Date.UTC(2024, 1, 28, 13, 45, 7, 89));
show("format", format(d, "yyyy-MM-dd'T'HH:mm:ss.SSS EEEE do MMMM, QQQ, 'week' I, h:mm a"));
show("addDays", formatISO(addDays(d, 2)));
show("addMonths", formatISO(addMonths(new Date(Date.UTC(2023, 0, 31)), 1)));
show("leap", [2000, 1900, 2024, 2023].map((y) => isLeapYear(new Date(Date.UTC(y, 0, 1)))));
show("diff", differenceInCalendarDays(new Date(Date.UTC(2025, 0, 1)), d));
show("interval", eachDayOfInterval({ start: d, end: addDays(d, 3) }).map((x) => format(x, "EEE d")));
show("startOfWeek", formatISO(startOfWeek(d, { weekStartsOn: 1 })));
show("endOfMonth", formatISO(endOfMonth(d)));
show("parseISO", parseISO("2021-03-04T05:06:07.890Z").getTime());
const p = parse("12/31/1999 11:59 PM", "MM/dd/yyyy hh:mm a", new Date(0));
show("parse", [isValid(p), formatISO(p)]);
show("invalid parse", isValid(parse("13/45/1999", "MM/dd/yyyy", new Date(0))));
show("distance", formatDistanceStrict(d, addDays(d, 45)));
show("duration", formatDuration(intervalToDuration({ start: d, end: new Date(Date.UTC(2026, 6, 4, 0, 0, 0)) })));
show("isoWeek", [new Date(Date.UTC(2021, 0, 3)), new Date(Date.UTC(2020, 11, 31))].map(getISOWeek));
show("sorted", [addDays(d, 5), d, addDays(d, -5)].sort(compareAsc).map((x) => format(x, "MM-dd")));
