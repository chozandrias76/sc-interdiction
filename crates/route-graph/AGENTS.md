# route-graph

Graph construction, spatial indexing, and fuel math for quantum travel routes.

## Modules

### graph.rs
Builds a directed graph of stations and terminals using `petgraph`. Key types:

- `RouteGraph` -- wraps a `DiGraph<Node, Edge>` with a code-to-index map
- `Node` -- a location (station, outpost, city, orbital marker) with optional 3D coords
- `Edge` -- a quantum travel path with distance (km) and estimated travel time

Core operations:
- `add_station` / `add_terminal` -- idempotent node insertion
- `connect(from, to, distance)` -- bidirectional edge with travel time = `(dist / 60_000) + 10s`
- `connect_system(system)` -- full mesh within a system, using Euclidean distance when coords exist
- `find_path(from, to)` -- A* (zero heuristic = Dijkstra) returning node IDs

### spatial.rs
3D spatial search over interdiction hotspots. Key types:

- `Point3D` -- (x, y, z) in Mkm from system center
- `SpatialIndex` -- flat list with linear scan; use `from_chokepoints` to build from live data
- `RouteSegment` -- a trade route as a 3D line segment with cargo/threat metadata
- `RouteIntersection` -- a zone where multiple routes converge, with interdiction value scoring

Chokepoint algorithm (`find_route_intersections`):
1. For every route pair, compute closest approach via parametric line-segment math
2. Cluster nearby approach points within `proximity_threshold`
3. Filter clusters by `min_routes`, then score by `cargo_value / threat_level`
4. Attach jump instructions (QT destination + exit distance) via `calculate_jump_instruction`

Position fallback: `estimate_position` maps known location names to approximate Mkm coords when
real coordinates are absent.

### fuel.rs
Quantum fuel math. Key types:

- `QtDriveEfficiency` -- `fuel_per_mkm` by drive size (S1=40, S2=80, S3=160; estimated values)
- `FuelStationIndex` -- filters terminals by `is_refuel`, resolves positions, supports nearest-neighbor
- `Waypoint` -- a hop in a multi-stop route

Key functions:
- `calculate_qt_fuel_consumption(distance_mkm, efficiency)` -- linear: `dist * fuel_per_mkm`
- `can_complete_route` -- returns `(bool, required, remaining)`
- `max_range_mkm` -- `capacity / fuel_per_mkm`
- `find_route_with_refueling` -- greedy waypoint planner; inserts refuel stops along route
- `find_nearest_on_route` -- perpendicular distance filter to find on-path fuel stations

Fuel prices (`HYDROGEN_FUEL_PRICE_PER_UNIT`, `QUANTUM_FUEL_PRICE_PER_UNIT`) are placeholder
estimates. See `docs/DATA_SOURCES.md` before treating them as accurate.

## Tests

```bash
cargo test -p route-graph
```

Tests live in `#[cfg(test)]` blocks at the bottom of each file. They cover:
- Graph construction, idempotency, pathfinding, travel time math
- Spatial distance, nearest-neighbor sorting, radius filtering, system lookup
- Fuel consumption, range limits, refuel cost, boundary conditions

## Gotchas

- `connect_system` does a full O(n^2) mesh -- fine for dozens of nodes, not hundreds
- `find_route_intersections` is also O(n^2) on route pairs; keep input lists bounded
- Fuel efficiency values are estimated. Tests assert relative ordering (S1 < S2 < S3), not
  absolute correctness
- `estimate_position` in `spatial.rs` uses hardcoded Stanton/Pyro coords as a fallback;
  prefer real coords from the API when available
