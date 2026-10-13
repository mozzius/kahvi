"""Run state in SQLite (work/state.db). Ticket files say what to do; this
says what has happened."""

import sqlite3
import threading
import time

from common import WORK

SCHEMA = """
CREATE TABLE IF NOT EXISTS tickets (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'open',  -- open | claimed | merging | done | blocked | needs-architect
    tier TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    strikes INTEGER NOT NULL DEFAULT 0,
    reason TEXT NOT NULL DEFAULT '',
    salvage_attempt INTEGER,              -- attempt whose diff the next attempt starts from
    updated REAL NOT NULL
);
CREATE TABLE IF NOT EXISTS attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ticket TEXT NOT NULL,
    slot INTEGER NOT NULL,
    tier TEXT NOT NULL,
    model TEXT NOT NULL,
    base TEXT NOT NULL,
    head TEXT,
    started REAL NOT NULL,
    finished REAL,
    outcome TEXT,          -- no-diff | blocked | gate-reject | good | incomplete | salvageable | garbage | error
    merged INTEGER NOT NULL DEFAULT 0,
    new_passes INTEGER NOT NULL DEFAULT 0,
    worker_cost REAL NOT NULL DEFAULT 0,
    verifier_cost REAL NOT NULL DEFAULT 0,
    turns INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS notes (
    ticket TEXT NOT NULL,
    attempt INTEGER,
    ts REAL NOT NULL,
    text TEXT NOT NULL
);
"""


class DB:
    def __init__(self, path=WORK / "state.db"):
        path.parent.mkdir(parents=True, exist_ok=True)
        self.conn = sqlite3.connect(path, check_same_thread=False, isolation_level=None)
        self.conn.row_factory = sqlite3.Row
        self.lock = threading.Lock()
        with self.lock:
            self.conn.executescript(SCHEMA)

    def q(self, sql, *args):
        with self.lock:
            return self.conn.execute(sql, args).fetchall()

    def one(self, sql, *args):
        rows = self.q(sql, *args)
        return rows[0] if rows else None

    def sync_tickets(self, tickets):
        now = time.time()
        for t in tickets.values():
            self.q("INSERT OR IGNORE INTO tickets (id, tier, updated) VALUES (?, ?, ?)", t.id, t.tier, now)

    def ticket(self, tid):
        return self.one("SELECT * FROM tickets WHERE id = ?", tid)

    def set_ticket(self, tid, **fields):
        fields["updated"] = time.time()
        cols = ", ".join(f"{k} = ?" for k in fields)
        self.q(f"UPDATE tickets SET {cols} WHERE id = ?", *fields.values(), tid)

    def claim_next(self, tickets, only=None):
        """Atomically picks the best ready ticket and marks it claimed."""
        with self.lock:
            rows = self.conn.execute("SELECT id, status FROM tickets").fetchall()
            status = {r["id"]: r["status"] for r in rows}
            ready = [
                t for t in tickets.values()
                if status.get(t.id) == "open"
                and (only is None or t.id in only)
                and all(status.get(n) == "done" for n in t.needs)
            ]
            if not ready:
                return None
            ready.sort(key=lambda t: (-t.priority, t.id))
            pick = ready[0]
            self.conn.execute(
                "UPDATE tickets SET status = 'claimed', updated = ? WHERE id = ?", (time.time(), pick.id)
            )
            return pick

    def new_attempt(self, ticket, slot, tier, model, base):
        with self.lock:
            cur = self.conn.execute(
                "INSERT INTO attempts (ticket, slot, tier, model, base, started) VALUES (?, ?, ?, ?, ?, ?)",
                (ticket, slot, tier, model, base, time.time()),
            )
            self.conn.execute("UPDATE tickets SET attempts = attempts + 1 WHERE id = ?", (ticket,))
            return cur.lastrowid

    def set_attempt(self, aid, **fields):
        cols = ", ".join(f"{k} = ?" for k in fields)
        self.q(f"UPDATE attempts SET {cols} WHERE id = ?", *fields.values(), aid)

    def note(self, ticket, attempt, text):
        self.q("INSERT INTO notes (ticket, attempt, ts, text) VALUES (?, ?, ?, ?)", ticket, attempt, time.time(), text)

    def notes(self, ticket, limit=4):
        rows = self.q("SELECT * FROM notes WHERE ticket = ? ORDER BY ts DESC LIMIT ?", ticket, limit)
        return list(reversed(rows))

    def release_stale_claims(self):
        """After a crash, claimed/merging tickets go back to the pool."""
        self.q("UPDATE tickets SET status = 'open' WHERE status IN ('claimed', 'merging')")
