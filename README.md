# Weser Wind Park 🌬️

A real-time data platform for a **fictional** offshore wind park near Bremerhaven, built by [DoppleJa](https://doppleja.de).
All data is simulated. This is a demo and learning project.

```
Simulator (Rust) → MQTT → Ingest & Clean (Python) → TimescaleDB → API/WebSocket → Dashboard
```

## Structure
| Folder | Purpose | Module |
|---|---|---|
| `simulator/` | Physics-based wind & turbine simulator, publishes to MQTT (Rust) | M1 |
| `ingest/` | Validation, cleaning, 10-min aggregation (Python/FastAPI) | M2 |
| `api/` | WebSocket + REST for the dashboard | M3 |
| `dashboard/` | Frontend (currently self-simulating prototype) | M3 |
| `ml/` | Anomaly detection, forecasting | M4–M5 |
| `infra/` | Docker Compose, Mosquitto, later Kubernetes | M0, M9 |
| `docs/` | Architecture and learning notes | all |

## Quick start (local)
```bash
cp .env.example .env            # then edit the password
docker compose -f infra/docker-compose.yml --env-file .env up -d
open dashboard/index.html
```

## Roadmap
- [x] Dashboard prototype
- [ ] **M0** Foundation: repo, Docker Compose, CI, deploy to Hetzner
- [ ] **M1** Rust simulator → MQTT
- [ ] **M2** Ingest & clean → TimescaleDB
- [ ] **M3** Live dashboard via WebSocket
- [ ] M4 Anomaly detection & alerts · M5 Forecast · M6 Energy prices
- [ ] M7 Wake model · M8 Predictive maintenance · M9 Kubernetes · M10 AWS edition
