## 2025-02-04 - Optimization of orbital math and distance checks
**Learning:** In simulations with many orbiting bodies and proximity checks (like SOI transitions), trigonometric functions and distance calculations are frequent bottlenecks. Using `sin_cos()` for simultaneous sine/cosine calculation and `distance_squared()` for proximity checks (avoiding `sqrt`) are highly effective, low-risk optimizations.
**Action:** Always prefer `distance_squared()` for threshold comparisons and `sin_cos()` when both values are needed for the same angle.
