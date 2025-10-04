# 📡 API Reference Guide

> **Navigation:** [Documentation Home](../README.md) → [API Documentation](README.md) → API Reference

**Reading Time:** Approximately 15 minutes  
**Complexity:** Intermediate  
**Prerequisites:** Understanding of RESTful APIs (Representational State Transfer Application Programming Interfaces) and JSON (JavaScript Object Notation) format

## 📖 What This Guide Covers

This guide provides complete technical reference for the Platter API (Application Programming Interface). By the end of this guide, you will understand:

1. How to authenticate with the API
2. Which endpoints are available for each resource
3. What request format each endpoint expects
4. What response format each endpoint returns
5. How to handle errors correctly

---

## 📋 Section 1: Base Configuration

### 1.1: Base URL (Uniform Resource Locator)

All API endpoints are relative to your deployment address:

**Development environment:**
```
http://localhost:8080
```

**Production environment:**
```
https://yourdomain.com
```

Replace `yourdomain.com` with your actual deployed domain name.

---

## 🔐 Section 2: Authentication

The API uses session-based authentication with secure cookies.

### 2.1: Authentication Process

Execute these steps in sequence to authenticate:

1. Send credentials to `/admin/login` endpoint
2. The server validates credentials and creates a session
3. The server sends a session cookie to your browser
4. Your browser includes this cookie automatically in subsequent requests

**Important:** Administrative credentials are required for most API operations.

### 2.2: Authentication Status

- **Authenticated requests:** Include the session cookie automatically set by the browser
- **Unauthenticated requests:** Return HTTP status code 401 with error details

---

## 📤 Section 3: Response Format

### 3.1: Successful Response Structure

All successful API responses return JSON with this structure:

```json
{
  "status": "success",
  "data": { ... }
}
```

**Field definitions:**
- `status`: Always contains the string `"success"` for successful operations
- `data`: Contains the requested resource or operation result

### 3.2: HTTP Status Codes

The API uses standard HTTP status codes:

| Status Code | Meaning | When This Occurs |
|-------------|---------|------------------|
| `200 OK` | Request successful | Data retrieved or operation completed successfully |
| `201 Created` | Resource created | New resource created successfully |
| `400 Bad Request` | Invalid request | Request parameters are invalid or missing |
| `401 Unauthorized` | Authentication required | Session cookie missing or expired |
| `403 Forbidden` | Insufficient permissions | Authenticated but lacking required permissions |
| `404 Not Found` | Resource not found | Requested resource does not exist |
| `500 Internal Server Error` | Server error | Unexpected server-side error occurred |

---

## ⚠️ Section 4: Error Handling

### 4.1: Error Response Structure

The API provides standardised error responses:

```json
{
  "status": "error",
  "error": "Error message",
  "error_type": "ERROR_TYPE",
  "message": "Human-readable error description"
}
```

**Field definitions:**
- `status`: Always contains `"error"` for failed operations
- `error`: Brief error title
- `error_type`: Machine-readable error category
- `message`: Detailed explanation of what went wrong

### 4.2: Common Error Types

| Error Type | Cause | Solution |
|------------|-------|----------|
| `VALIDATION_ERROR` | Request validation failed | Check request parameters match expected format |
| `AUTHENTICATION_ERROR` | Authentication required or failed | Log in and retry the request |
| `AUTHORIZATION_ERROR` | Insufficient permissions | Ensure you have administrative access |
| `NOT_FOUND` | Requested resource does not exist | Verify the resource ID is correct |
| `SERVER_ERROR` | Internal server error | Review server logs or contact administrator |

---

## 🛠️ Section 5: API Endpoints

### 5.1: Authentication Endpoints

#### Endpoint 5.1.1: Display Login Page

```
GET /admin/login
```

**Purpose:** Retrieves the HTML login page for browser-based authentication.

**Authentication required:** No

**Response:** HTML page content

---

#### Endpoint 5.1.2: Authenticate User

```
POST /admin/login
```

**Purpose:** Authenticates a user with username and password credentials.

**Authentication required:** No

**Request format:** Form data (Content-Type: application/x-www-form-urlencoded)

**Required parameters:**
- `username`: Administrator username
- `password`: Administrator password

**Success response:** Redirects to admin dashboard with session cookie set

**Error response:** Returns error message with HTTP 401 status

---

#### Endpoint 5.1.3: End User Session

```
POST /admin/logout
```

**Purpose:** Terminates the current user session and clears the session cookie.

**Authentication required:** Yes

**Success response:** Redirects to login page

---

### 5.2: Menu Item Endpoints

#### Endpoint 5.2.1: List All Menu Items

```
GET /api/items
```

**Purpose:** Retrieves all menu items currently stored in the system.

**Authentication required:** No

**Success response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": "unique-identifier-string",
      "name": "Item name",
      "description": "Item description text",
      "category": "Category name",
      "price": "10.99",
      "available": true,
      "tags": ["tag1", "tag2"]
    }
  ]
}
```

**Field definitions:**
- `id`: Unique identifier for the menu item
- `name`: Display name of the item
- `description`: Detailed description text
- `category`: Item category (e.g., "Main Course", "Dessert")
- `price`: Price as a string (e.g., "10.99")
- `available`: Boolean indicating if item is currently available
- `tags`: Array of string tags for filtering (e.g., ["vegetarian", "gluten-free"])

---

#### Endpoint 5.2.2: Create Menu Item

```
POST /api/items
```

**Purpose:** Creates a new menu item in the system.

**Authentication required:** Yes (administrator only)

**Request format:** JSON (Content-Type: application/json)

**Request body:**
```json
{
  "name": "Item Name",
  "description": "Item description",
  "category": "Category",
  "price": "10.99",
  "available": true,
  "tags": ["tag1", "tag2"]
}
```

**All fields are required.**

**Success response:** Returns created item with generated ID (HTTP 201)

---

#### Endpoint 5.2.3: Retrieve Specific Menu Item

```
GET /api/items/{id}
```

**Purpose:** Retrieves a single menu item by its unique identifier.

**Authentication required:** No

**URL parameters:**
- `{id}`: Replace with the menu item's unique identifier

**Success response:** Returns menu item object (HTTP 200)

**Error response:** Returns `NOT_FOUND` error if item does not exist (HTTP 404)

---

#### Endpoint 5.2.4: Update Menu Item

```
PUT /api/items/{id}
```

**Purpose:** Updates an existing menu item. Partial updates are allowed (you can update individual fields).

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the menu item's unique identifier

**Request format:** JSON (Content-Type: application/json)

**Request body:** Include only fields you wish to update

**Success response:** Returns updated item (HTTP 200)

---

#### Endpoint 5.2.5: Delete Menu Item

```
DELETE /api/items/{id}
```

**Purpose:** Permanently deletes a menu item from the system.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the menu item's unique identifier

**Success response:** Returns confirmation message (HTTP 200)

⚠️ **Warning:** This action is permanent and cannot be undone.

---

#### Endpoint 5.2.6: Reload Menu Items

```
POST /api/items/reload
```

**Purpose:** Reloads menu items from the data source file.

**Authentication required:** Yes (administrator only)

**Use case:** Use this after manually editing menu data files

**Success response:** Returns confirmation message (HTTP 200)

---

### 5.3: Notice Endpoints

#### Endpoint 5.3.1: List All Notices

```
GET /api/notices
```

**Purpose:** Retrieves all active notices in the system.

**Authentication required:** No

**Success response:** Returns array of notice objects

---

#### Endpoint 5.3.2: Create Notice

```
POST /api/notices
```

**Purpose:** Creates a new notice in the system.

**Authentication required:** Yes (administrator only)

**Request format:** JSON (Content-Type: application/json)

**Request body:**
```json
{
  "title": "Notice Title",
  "content": "Notice content text",
  "priority": "normal",
  "startDate": "2025-01-01T00:00:00Z",
  "endDate": "2025-01-31T23:59:59Z"
}
```

**Field definitions:**
- `title`: Notice headline (required)
- `content`: Notice message content (required)
- `priority`: Priority level - must be one of: `"normal"`, `"high"`, or `"urgent"` (required)
- `startDate`: ISO 8601 (International Organisation for Standardisation date format) date-time when notice becomes active (required)
- `endDate`: ISO 8601 date-time when notice expires (required)

**Success response:** Returns created notice with generated ID (HTTP 201)

---

#### Endpoint 5.3.3: Update Notice

```
PUT /api/notices/{id}
```

**Purpose:** Updates an existing notice.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the notice's unique identifier

**Request format:** JSON with fields to update

**Success response:** Returns updated notice (HTTP 200)

---

#### Endpoint 5.3.4: Delete Notice

```
DELETE /api/notices/{id}
```

**Purpose:** Permanently deletes a notice from the system.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the notice's unique identifier

**Success response:** Returns confirmation message (HTTP 200)

---

#### Endpoint 5.3.5: Reload Notices

```
POST /api/notices/reload
```

**Purpose:** Reloads notices from the data source file.

**Authentication required:** Yes (administrator only)

**Success response:** Returns confirmation message (HTTP 200)

---

### 5.4: Menu Preset Endpoints

#### Endpoint 5.4.1: List All Menu Presets

```
GET /api/presets
```

**Purpose:** Retrieves all menu presets (pre-configured menu combinations).

**Authentication required:** No

**Success response:** Returns array of preset objects

---

#### Endpoint 5.4.2: Create Menu Preset

```
POST /api/presets
```

**Purpose:** Creates a new menu preset (a reusable combination of menu items).

**Authentication required:** Yes (administrator only)

**Request format:** JSON (Content-Type: application/json)

**Request body:**
```json
{
  "name": "Preset Name",
  "description": "Preset description",
  "items": ["item_id_1", "item_id_2"]
}
```

**Field definitions:**
- `name`: Preset display name (required)
- `description`: Preset description (required)
- `items`: Array of menu item IDs included in this preset (required)

**Success response:** Returns created preset with generated ID (HTTP 201)

---

#### Endpoint 5.4.3: Retrieve Specific Menu Preset

```
GET /api/presets/{id}
```

**Purpose:** Retrieves a single menu preset by its unique identifier.

**Authentication required:** No

**URL parameters:**
- `{id}`: Replace with the preset's unique identifier

**Success response:** Returns preset object (HTTP 200)

---

#### Endpoint 5.4.4: Update Menu Preset

```
PUT /api/presets/{id}
```

**Purpose:** Updates an existing menu preset.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the preset's unique identifier

**Request format:** JSON with fields to update

**Success response:** Returns updated preset (HTTP 200)

---

#### Endpoint 5.4.5: Delete Menu Preset

```
DELETE /api/presets/{id}
```

**Purpose:** Permanently deletes a menu preset from the system.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the preset's unique identifier

**Success response:** Returns confirmation message (HTTP 200)

---

#### Endpoint 5.4.6: Reload Menu Presets

```
POST /api/presets/reload
```

**Purpose:** Reloads menu presets from the data source file.

**Authentication required:** Yes (administrator only)

**Success response:** Returns confirmation message (HTTP 200)

---

### 5.5: Schedule Endpoints

#### Endpoint 5.5.1: List All Menu Schedules

```
GET /api/schedules
```

**Purpose:** Retrieves all menu schedules (planned menu assignments for specific dates).

**Authentication required:** No

**Success response:** Returns array of schedule objects

---

#### Endpoint 5.5.2: Create Menu Schedule

```
POST /api/schedules
```

**Purpose:** Creates a new menu schedule entry.

**Authentication required:** Yes (administrator only)

**Request format:** JSON (Content-Type: application/json)

**Request body:**
```json
{
  "presetId": "preset_unique_id",
  "date": "2025-01-01",
  "mealType": "lunch",
  "notes": "Optional notes about this schedule"
}
```

**Field definitions:**
- `presetId`: ID of the menu preset to schedule (required)
- `date`: Date in YYYY-MM-DD format (required)
- `mealType`: Meal type - must be one of: `"breakfast"`, `"lunch"`, or `"dinner"` (required)
- `notes`: Optional notes about this schedule (optional)

**Success response:** Returns created schedule with generated ID (HTTP 201)

---

#### Endpoint 5.5.3: Retrieve Specific Menu Schedule

```
GET /api/schedules/{id}
```

**Purpose:** Retrieves a single menu schedule by its unique identifier.

**Authentication required:** No

**URL parameters:**
- `{id}`: Replace with the schedule's unique identifier

**Success response:** Returns schedule object (HTTP 200)

---

#### Endpoint 5.5.4: Update Menu Schedule

```
PUT /api/schedules/{id}
```

**Purpose:** Updates an existing menu schedule.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the schedule's unique identifier

**Request format:** JSON with fields to update

**Success response:** Returns updated schedule (HTTP 200)

---

#### Endpoint 5.5.5: Delete Menu Schedule

```
DELETE /api/schedules/{id}
```

**Purpose:** Permanently deletes a menu schedule from the system.

**Authentication required:** Yes (administrator only)

**URL parameters:**
- `{id}`: Replace with the schedule's unique identifier

**Success response:** Returns confirmation message (HTTP 200)

---

#### Endpoint 5.5.6: Retrieve Upcoming Schedules

```
GET /api/schedules/upcoming
```

**Purpose:** Retrieves menu schedules for upcoming dates.

**Authentication required:** No

**Success response:** Returns array of schedule objects sorted by date

---

#### Endpoint 5.5.7: Validate Schedule

```
POST /api/schedules/validate
```

**Purpose:** Validates a schedule configuration before creation.

**Authentication required:** Yes (administrator only)

**Request format:** JSON matching schedule creation format

**Success response:** Returns validation result (HTTP 200)

---

#### Endpoint 5.5.8: Reload Menu Schedules

```
POST /api/schedules/reload
```

**Purpose:** Reloads menu schedules from the data source file.

**Authentication required:** Yes (administrator only)

**Success response:** Returns confirmation message (HTTP 200)

---

## 📚 Section 6: Practical Examples

### Example 6.1: Creating a New Menu Item

**Request:**
```http
POST /api/items
Content-Type: application/json

{
  "name": "Veggie Burger",
  "description": "Plant-based burger with fresh vegetables",
  "category": "Main Course",
  "price": "12.99",
  "available": true,
  "tags": ["vegetarian", "gluten-free"]
}
```

**Success response:**
```json
{
  "status": "success",
  "data": {
    "id": "abc123",
    "name": "Veggie Burger",
    "description": "Plant-based burger with fresh vegetables",
    "category": "Main Course",
    "price": "12.99",
    "available": true,
    "tags": ["vegetarian", "gluten-free"]
  }
}
```

**What happened:**
1. Server received the request
2. Server validated all required fields
3. Server generated unique ID `"abc123"`
4. Server saved the item to storage
5. Server returned the complete item including the generated ID

---

### Example 6.2: Retrieving All Notices

**Request:**
```http
GET /api/notices
```

**Success response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": "notice_1",
      "title": "New Menu Items",
      "content": "Review our new seasonal menu items",
      "priority": "normal",
      "startDate": "2025-01-01T00:00:00Z",
      "endDate": "2025-01-31T23:59:59Z"
    }
  ]
}
```

**What happened:**
1. Server received the request
2. Server retrieved all active notices
3. Server returned notices in JSON array format

---

### Example 6.3: Error Response

**Request:**
```http
GET /api/items/nonexistent-id
```

**Error response:**
```json
{
  "status": "error",
  "error": "Not Found",
  "error_type": "NOT_FOUND",
  "message": "Menu item with id nonexistent-id not found"
}
```

**What happened:**
1. Server received request for item with ID `"nonexistent-id"`
2. Server searched storage and found no matching item
3. Server returned standardised error response with HTTP 404 status

**How to resolve:** Verify the item ID is correct and the item exists in the system.

---

## 🎯 Section 7: Next Steps

After understanding the API reference, proceed with these tasks:

### For Developers

Complete these tasks in sequence:

1. **Test API Locally** – Follow the [Development Setup Guide](../development/setup.md)
2. **Review Security Requirements** – Study the [Security Documentation](../architecture/security.md)

### For System Integrators

Complete these tasks in sequence:

1. **Review Configuration Options** – Study the [Configuration Guide](../guides/configuration.md)
2. **Plan API Integration** – Review your specific use case requirements
3. **Test in Development** – Verify API behaviour before production deployment

---

## 📖 Related Documentation

Access these resources for additional information:

- **[Getting Started Guide](../guides/getting-started.md)** – Quick start and installation instructions
- **[Configuration Guide](../guides/configuration.md)** – Environment variables and settings configuration
- **[Security Documentation](../architecture/security.md)** – API security considerations and best practices
- **[Development Setup Guide](../development/setup.md)** – Testing API endpoints in local development

---

[← Back to API Documentation](README.md) | [Documentation Home](../README.md)