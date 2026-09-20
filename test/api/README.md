# Ryu Test API

Deterministic local REST API for testing Ryu.

## Setup

```bash
cd test/api
npm install    # or: bun install
npm start      # or: bun start
```

Server starts at `http://localhost:3000`.

Use `npm run start:with-instructions` for import help on startup.

## Import Test Collections

A ready-to-import file is available: `test-collections.json`

1. Open Ryu
2. Click the ⋮ menu in the Collections sidebar
3. Select **Import Collections**
4. Choose `test-collections.json`

Collections included:
- **Basic API Tests** — GET/POST users, echo, error
- **API Hardening Tests** — query params, headers, body, URL params
- **Authentication Tests** — Bearer, Basic, API Key (header + query)
- **Advanced Tests** — mixed request, invalid JSON

## Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/api/users` | List users (200) |
| POST | `/api/users` | Create user (201) |
| GET | `/api/error` | Forced 500 error |
| ANY | `/api/echo` | Echo headers + body |
| GET | `/api/params` | Echo query params |
| GET | `/api/headers` | Echo request headers |
| POST | `/api/body` | Echo parsed body |
| GET | `/api/env/:value` | Echo URL param |
| GET | `/api/auth/bearer` | Requires Bearer token |
| GET | `/api/auth/basic` | Requires Basic auth |
| GET | `/api/auth/apikey` | Requires API key (header or query) |
| POST | `/api/mixed` | Echo everything |
