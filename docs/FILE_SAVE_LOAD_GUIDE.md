# File Save & Load — Ryu

## Overview

Save any request as a portable JSON file and load it back later. Great for version control, team sharing, and request libraries.

## Save a Request

- **Button**: Click 💾 in the request bar
- **Keyboard**: `Ctrl+S` / `Cmd+S`

A native file dialog opens. Choose a location and filename.

## Load a Request

- **Button**: Click 📂 in the request bar
- **Keyboard**: `Ctrl+O` / `Cmd+O`

Select a `.json` file. The request is loaded into the UI but **not sent automatically**.

## File Format

```json
{
  "method": "GET",
  "url": "{{BASE_URL}}/users/1",
  "params": [{ "enabled": true, "key": "page", "value": "1" }],
  "headers": [{ "enabled": true, "key": "Authorization", "value": "Bearer {{TOKEN}}" }],
  "body": "",
  "auth": { "type": "bearer", "token": "{{TOKEN}}" }
}
```

- Variables like `{{BASE_URL}}` are **preserved as-is** — each user sets their own env vars
- Human-readable, Git-friendly

## Version Control Workflow

```bash
# Save request as api-requests/get-user.json
git add api-requests/get-user.json
git commit -m "feat: add get-user request"
git push

# Teammate pulls and loads
git pull
# Open Ryu → Ctrl+O → select get-user.json
```

## Keyboard Shortcuts

| Action | Windows/Linux | Mac |
|---|---|---|
| Save | `Ctrl+S` | `Cmd+S` |
| Load | `Ctrl+O` | `Cmd+O` |
| Send | `Ctrl+Enter` | `Cmd+Enter` |

---

**Last Updated**: September 19, 2026
