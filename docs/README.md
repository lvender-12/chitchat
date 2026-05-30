# Chat App Documentation

## Features
- Authentication (login & register)
- Friend Management
  - Add friend
  - List friends
  - Pending sent requests (menunggu diterima)
  - Incoming friend requests
  - Accept / reject friend request
- Realtime messaging (WebSocket)
- User profile

## API Reference
- [API Docs](./api/chitchat-documentation.html)

## ERD
![ERD](./erd_chat.png)

## Tech Stack

### Backend
- Rust + Axum
- PostgreSQL
- Redis
- WebSocket (broadcast channel)

### Frontend
- React + Vite
- WebSocket client

## Configuration

### Backend (`config.yaml`)
| Key | Description | Default |
|---|---|---|
| `app.name` | Nama aplikasi | Chat-App |
| `app.host` | Host server | localhost |
| `app.port` | Port server | 3000 |
| `db.host` | PostgreSQL host | localhost |
| `db.port` | PostgreSQL port | 3306 |
| `db.username` | PostgreSQL username | - |
| `db.password` | PostgreSQL password | - |
| `db.name` | Nama database | - |
| `redis.host` | Redis host | localhost |
| `redis.port` | Redis port | 3306 |
| `redis.username` | Redis username | - |
| `redis.password` | Redis password | - |
| `jwt.secret` | JWT signing secret | - |
| `jwt.expiry` | JWT expiry (seconds) | 86400 |
| `api.secret` | API secret key | - |

### Frontend (`.env`)
| Variable | Description | Dev |
|---|---|---|
| `VITE_API_URL` | Backend URL | kosong (pakai proxy) |
| `VITE_WS_URL` | WebSocket URL | kosong (pakai proxy) |
| `VITE_API_SECRET` | API secret key | - |

## Quick Start

### Backend
```bash
cd backend
cp config.example.yaml config.yaml
cargo run
```

### Frontend
```bash
cd frontend
cp .env.example .env
npm install
npm run dev
```
