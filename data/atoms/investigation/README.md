# Investigation Atoms — Working Directory

Atoms are extracted entities from `gardens/amicus/publicRecord/site/static/graph.json` (45MB).

## Current Files

| File | Size | Contents |
|------|------|----------|
| `bingo-index.tsv` | 461KB | 12,817 entity IDs with type and ZK hash |
| `false-faces.json` | 19KB | 19 false face nodes with edge counts |
| `corporations.json` | 7KB | 19 corporation atoms |
| `top-institutions.json` | 18KB | Top 30 institutions by edge count |

## Wave History

| Wave | Action | Result |
|------|--------|--------|
| 233 | Initial extraction | 12,817 bingo stamps, 19 false faces atomized, corporations + top institutions extracted |

## Next Wave Targets

- [ ] Extract all 5,907 persons with connection counts
- [ ] Extract all 235 legal cases with jurisdiction data
- [ ] Extract all 915 events with date/location
- [ ] OSINT scan of 19 false faces (Wikipedia, old web)
- [ ] Geographic coordinates for all institutions (geocoding)
- [ ] Sourdough mastication batches per entity cluster

## Schema

Each atom file is a self-contained JSON array. Each entry has at minimum:
- `id` — graph node ID
- `type` — node type from graph.json
- `zk` — BingoCube identity hash (8 hex chars)

Atoms are WORKING files. When processing is complete, results are fossilized
to `data/fossils/` and the atoms can be regenerated from the graph.
