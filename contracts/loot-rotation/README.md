# Loot Rotation Contract

Read model for the currently configured loot pool and its rollover timing.

## Public reads

- `active_pool_snapshot()` returns the configured pool, pause flag, current timestamp, and seconds until rollover. Missing pools return `has_active_pool = false` with zeroed pool values.
- `rollover_delay()` returns only the timing and readiness fields needed by clients deciding whether rotation work is due.

Rollover delay is saturated at zero when no pool exists or when `now >= ends_at`.
