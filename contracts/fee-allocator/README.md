# Fee Allocator Contract

Tracks configured allocation routes and exposes read-only drift and rebalance state.

## Public reads

- `allocation_drift_summary()` returns every route with its target bps, allocated amount, expected amount, and absolute drift.
- `rebalance_readiness()` returns whether the allocator is configured, unpaused, has a valid 10000 bps target, has funds, and has drift above the default threshold.

Expected amounts use floor division: `(total_allocated * target_bps) / 10000`. The default drift threshold is `1`, so one unit of rounding dust does not make the allocator rebalance-ready.
