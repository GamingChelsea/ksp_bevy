## 2025-02-04 - Optimization of orbital math and distance checks
**Learning:** In simulations with many orbiting bodies and proximity checks (like SOI transitions), trigonometric functions and distance calculations are frequent bottlenecks. Using `sin_cos()` for simultaneous sine/cosine calculation and `distance_squared()` for proximity checks (avoiding `sqrt`) are highly effective, low-risk optimizations.
**Action:** Always prefer `distance_squared()` for threshold comparisons and `sin_cos()` when both values are needed for the same angle.

## 2025-02-04 - Leveraging Change Detection for Simulation Pausing
**Learning:** In Bevy-based simulations, expensive update systems (like orbital mechanics or kinematic movement) can be skipped when the simulation is paused by gating them behind change detection on the simulation's clock resource. Mutating the resource only when time actually advances (delta > 0) is key to making this effective.
**Action:** Use `if !ut.is_changed() { return; }` in heavy simulation systems and ensure the time resource is only mutated on non-zero updates.

## 2025-02-12 - Fixing Broken Change Detection Gating
**Learning:** Bevy's `ResMut<T>` triggers change detection as soon as it is accessed mutably (e.g., `res.0 = val`), even if the value remains the same. In simulation loops, always check if a change is actually necessary before mutating to allow downstream `is_changed()` gates to function correctly when the simulation is paused.
**Action:** Wrap resource mutations in condition checks (e.g., `if delta > 0.0 { ... }`) to preserve change detection integrity.
