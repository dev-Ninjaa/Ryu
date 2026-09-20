# Environment Variables — Ryu

## Overview

Use the `{{VARIABLE}}` syntax anywhere in a request — URL, headers, body, auth fields. Variables are stored in SQLite and resolved before the request is sent.

## Managing Variables

Go to the **Env** tab in the request editor:

- **Add**: Click "+ Add Variable"
- **Edit**: Modify key or value inline (auto-saves after 500 ms)
- **Delete**: Click ×
- **Mask**: Click ●● to hide the value
- **Copy**: Click ⧉ to copy the value to clipboard

## Syntax

```
{{VARIABLE_NAME}}
```

Works in: URL, query params, headers, request body, auth fields (token, API key, username, password).

## Example

```
Variables:
  BASE_URL  = https://api.example.com
  TOKEN     = secret-123

URL:   {{BASE_URL}}/users
Auth:  Bearer {{TOKEN}}

Resolved URL:   https://api.example.com/users
Resolved Auth:  Bearer secret-123
```

## Rules

- **Case-sensitive** — `BASE_URL` ≠ `base_url`
- **No nesting** — `{{VAR_{{OTHER}}}}` is not supported
- **Single environment** — no multi-env switching yet
- Unresolved variables are left as-is (e.g. `{{UNKNOWN}}` stays literal)

## Storage

Variables are stored in SQLite:
- Linux/macOS: `~/.local/share/ryu/env.db`
- Windows: `%APPDATA%/ryu/env.db`

They persist across restarts.

## Export / Import

Use the **Export Variables** and **Import Variables** buttons in the Env tab.

Format:
```json
{
  "version": "1.0",
  "exportedAt": "2026-09-19T00:00:00.000Z",
  "env": [
    { "key": "BASE_URL", "value": "https://api.example.com", "enabled": true }
  ]
}
```

---

**Last Updated**: September 19, 2026
