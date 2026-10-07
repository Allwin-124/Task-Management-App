# Full-Stack Task Management System (Rust + React)

A full-stack task management application featuring a Rust (Axum) web backend and a modern React (Vite) frontend. The system implements email-based Two-Factor Authentication (2FA), Role-Based Access Control (RBAC), task creation & assignment, and per-user caching with automatic invalidation.

---

## 🛠️ Technology Stack

* **Backend**: Rust (2024 Edition), Axum `v0.8`, Tokio Async Runtime, Serde JSON, Bcrypt password hashing, Jsonwebtoken (JWT), `tower-http` CORS.
* **Frontend**: React 19, Vite 8, Vanilla CSS (Dark Mode Design System with Glassmorphism).
* **Caching**: In-Memory Thread-Safe Cache (`Arc<RwLock<HashMap<String, CachedUserTasks>>>`) with per-user caching and automatic cache invalidation upon task assignment/updates.
* **Authentication**: 2FA Challenge/Verification flow yielding JWT Bearer access tokens.

---

## 📁 Repository Structure

```
.
├── backend/                  # Rust Axum Web API
│   ├── src/
│   │   ├── auth.rs           # JWT & Role Middleware
│   │   ├── errors.rs         # HTTP AppError definitions
│   │   ├── handlers.rs       # Route handlers (2FA, tasks, cache)
│   │   ├── lib.rs            # App router & initialization
│   │   ├── main.rs           # Web server entrypoint
│   │   ├── model.rs          # Data models & Serde DTOs
│   │   └── state.rs          # Thread-safe application state
│   ├── tests/
│   │   └── integration_tests.rs # Automated integration tests
│   ├── Cargo.toml
│   └── .env.example
├── frontend/                 # React + Vite UI
│   ├── src/
│   │   ├── components/       # Header, AuthScreen, AdminDashboard, StaffDashboard
│   │   ├── services/api.js   # API service client
│   │   ├── App.jsx           # Main React App
│   │   ├── index.css         # Styling system
│   │   └── main.jsx
│   ├── package.json
│   └── .env.example
├── README.md                 # System Documentation & Validation Output
└── AI_USAGE.md               # AI Tools & Manual Implementation Report
```

---

## 🚀 Setup & Running Instructions

### 1. Backend Setup (Rust)
Navigate to `backend/`:
```bash
cd backend
cargo check
cargo test        # Run integration tests
cargo run         # Start Axum server on http://127.0.0.1:3000
```

### 2. Frontend Setup (React)
Navigate to `frontend/`:
```bash
cd frontend
npm install
npm run dev       # Start Vite dev server on http://localhost:5173
```

---

## 🔐 Required Validation Workflow

Follow these steps via the Frontend UI or `curl` / Postman:

1. **Seed Users**: Call `POST /seed/users`. Creates **Admin** (`admin@example.com`) and **James Bond** (`jamesbond@example.com`).
2. **Login as Admin**: Call `POST /auth/login` with Admin credentials (`Password123!`). Returns `login_challenge_id` (NO JWT returned).
3. **Retrieve 2FA Code**: Call `GET /dev/email-logs/latest` to get the latest 6-digit verification code.
4. **Verify Admin 2FA**: Call `POST /auth/verify-2fa` with `login_challenge_id` and code to receive Admin JWT token.
5. **Create 5 Tasks**: Call `POST /tasks` with Admin JWT to create exactly 5 tasks.
6. **Assign 3 Tasks**: Call `POST /tasks/assign` with Admin JWT to assign 3 selected tasks to `jamesbond@example.com`.
7. **Login & Verify James Bond**: Login as `jamesbond@example.com`, retrieve 2FA code, and verify to get James Bond JWT token.
8. **Test 403 Forbidden**: Call `POST /tasks` with James Bond JWT token. API returns `403 Forbidden`.
9. **View James Bond Tasks (Cache Miss)**: Call `GET /tasks/view-my-tasks` with James Bond JWT token. Returns 3 assigned tasks with `cache.hit = false`.
10. **View James Bond Tasks (Cache Hit)**: Call `GET /tasks/view-my-tasks` again. Returns cached response with `cache.hit = true`.

---

## 📋 Final API Validation Output (`GET /tasks/view-my-tasks`)

### Call #1: Initial Database Fetch (`cache.hit = false`)

```http
GET /tasks/view-my-tasks
Authorization: Bearer <JAMES_BOND_TOKEN>
```

**Response (200 OK):**
```json
{
  "user": {
    "email": "jamesbond@example.com",
    "role": "staff"
  },
  "tasks": [
    {
      "id": "87e21c12-504d-4618-b4d7-fa99703336e5",
      "title": "Infiltrate Enemy Compound",
      "description": "Gather intelligence on secret facility",
      "status": "todo",
      "priority": "high",
      "created_by": "admin@example.com",
      "assigned_to": "jamesbond@example.com",
      "created_at": "2026-10-07T08:48:08.519946600+00:00",
      "updated_at": "2026-10-07T08:48:08.526760400+00:00"
    },
    {
      "id": "22b370ea-b3e5-40d9-939c-a36c1df50b42",
      "title": "Disable Security Grid",
      "description": "Bypass laser perimeter and mainframe",
      "status": "todo",
      "priority": "medium",
      "created_by": "admin@example.com",
      "assigned_to": "jamesbond@example.com",
      "created_at": "2026-10-07T08:48:08.521293700+00:00",
      "updated_at": "2026-10-07T08:48:08.526760400+00:00"
    },
    {
      "id": "b3443534-a09a-46ab-8bba-a2c41b476246",
      "title": "Exfiltrate VIP Target",
      "description": "Escort agent safely to helicopter extraction",
      "status": "todo",
      "priority": "low",
      "created_by": "admin@example.com",
      "assigned_to": "jamesbond@example.com",
      "created_at": "2026-10-07T08:48:08.522334100+00:00",
      "updated_at": "2026-10-07T08:48:08.526760400+00:00"
    }
  ],
  "summary": {
    "total_assigned_tasks": 3
  },
  "cache": {
    "hit": false
  }
}
```

---

### Call #2: Second Fetch (`cache.hit = true`)

```http
GET /tasks/view-my-tasks
Authorization: Bearer <JAMES_BOND_TOKEN>
```

**Response (200 OK):**
```json
{
  "user": {
    "email": "jamesbond@example.com",
    "role": "staff"
  },
  "tasks": [
    {
      "id": "87e21c12-504d-4618-b4d7-fa99703336e5",
      "title": "Infiltrate Enemy Compound",
      "description": "Gather intelligence on secret facility",
      "status": "todo",
      "priority": "high",
      "created_by": "admin@example.com",
      "assigned_to": "jamesbond@example.com",
      "created_at": "2026-10-07T08:48:08.519946600+00:00",
      "updated_at": "2026-10-07T08:48:08.526760400+00:00"
    },
    {
      "id": "22b370ea-b3e5-40d9-939c-a36c1df50b42",
      "title": "Disable Security Grid",
      "description": "Bypass laser perimeter and mainframe",
      "status": "todo",
      "priority": "medium",
      "created_by": "admin@example.com",
      "assigned_to": "jamesbond@example.com",
      "created_at": "2026-10-07T08:48:08.521293700+00:00",
      "updated_at": "2026-10-07T08:48:08.526760400+00:00"
    },
    {
      "id": "b3443534-a09a-46ab-8bba-a2c41b476246",
      "title": "Exfiltrate VIP Target",
      "description": "Escort agent safely to helicopter extraction",
      "status": "todo",
      "priority": "low",
      "created_by": "admin@example.com",
      "assigned_to": "jamesbond@example.com",
      "created_at": "2026-10-07T08:48:08.522334100+00:00",
      "updated_at": "2026-10-07T08:48:08.526760400+00:00"
    }
  ],
  "summary": {
    "total_assigned_tasks": 3
  },
  "cache": {
    "hit": true
  }
}
```

---

## 🧪 Automated Testing

To execute the automated Rust integration test suite (covering 2FA challenge creation, single-use 2FA verification, code expiration, admin privileges, 403 forbidden checks, and cache hit/invalidation):

```bash
cd backend
cargo test
```

Expected output:
```text
running 2 tests
test test_invalid_and_reused_2fa_codes ... ok
test test_full_application_workflow ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.29s
```

---

## 📧 Submission Contact

Repository link for evaluation submission: **`unnati.pandey@alkye.com`**
