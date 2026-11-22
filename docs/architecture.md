# Architecture

## Data flow
1. **Simulator** produces one message per turbine per second:
   `windpark/{turbine_id}/telemetry` → JSON `{ts, wind_ms, power_mw, rotor_rpm, gearbox_c, vibration_mms, status}`
2. **MQTT** (Mosquitto) transports messages.
3. **Ingest** subscribes, validates (range, gaps, spikes), cleans (Hampel filter, interpolation), writes raw + clean rows.
4. **TimescaleDB** stores a `telemetry` hypertable plus a continuous aggregate for 10-minute means.
5. **API** pushes live data to the dashboard via WebSocket.

## Physics (simulator)
- Wind: Ornstein–Uhlenbeck process around a Weibull-distributed mean, plus turbulence
- Power: P = ½·ρ·A·Cp·v³, capped at rated power; cut-in 3 m/s, cut-out 25 m/s (restart < 20 m/s)
- Turbine: 4.2 MW rated, 136 m rotor diameter

## Decisions log
- 2026-11-22: monorepo, Rust for simulator, Python for ingest, TimescaleDB, MQTT.
