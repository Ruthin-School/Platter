# API Documentation

This document provides comprehensive information about the Platter API, a RESTful interface for managing dining operations.

## 📋 Table of Contents

- [Base URL](#base-url)
- [Authentication](#authentication)
- [Response Format](#response-format)
- [Error Handling](#error-handling)
- [Endpoints](#endpoints)
  - [Authentication](#authentication-1)
  - [Menu Items](#menu-items)
  - [Notices](#notices)
  - [Menu Presets](#menu-presets)
  - [Schedules](#schedules)
- [Examples](#examples)

## 🌐 Base URL

All API endpoints are relative to: `http://localhost:8080` (or your deployed domain)

## 🔐 Authentication

Most API endpoints require authentication. The application uses session-based authentication:

1. First, authenticate via the `/admin/login` endpoint
2. The API will set a session cookie
3. Include this cookie in subsequent requests

Admin credentials are required for most operations.

## 📤 Response Format

Successful responses return JSON with the following structure:

```json
{
  "status": "success",
  "data": { ... }
}
```

### Response Statuses

- `200 OK` - Request successful
- `201 Created` - Resource created successfully
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Authentication required
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## ⚠️ Error Handling

The API provides standardized error responses:

```json
{
  "status": "error",
  "error": "Error message",
  "error_type": "ERROR_TYPE",
  "message": "Human-readable error description"
}
```

### Common Error Types

- `VALIDATION_ERROR` - Request validation failed
- `AUTHENTICATION_ERROR` - Authentication required or failed
- `AUTHORIZATION_ERROR` - Insufficient permissions
- `NOT_FOUND` - Requested resource does not exist
- `SERVER_ERROR` - Internal server error

## 🛠️ Endpoints

### Authentication

#### Get login page
```
GET /admin/login
```
Returns the login page HTML.

#### Login
```
POST /admin/login
```
Authenticate user. Expects form data with `username` and `password`.

#### Logout
```
POST /admin/logout
```
End user session.

### Menu Items

#### List all menu items
```
GET /api/items
```

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": "string",
      "name": "string",
      "description": "string",
      "category": "string",
      "price": "string",
      "available": true,
      "tags": ["string"]
    }
  ]
}
```

#### Create menu item
```
POST /api/items
```
Create a new menu item.

**Request Body:**
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

#### Get specific menu item
```
GET /api/items/{id}
```

#### Update menu item
```
PUT /api/items/{id}
```
Update an existing menu item (partial updates allowed).

#### Delete menu item
```
DELETE /api/items/{id}
```
Delete a menu item permanently.

#### Reload menu items
```
POST /api/items/reload
```
Reload menu items from the data source.

### Notices

#### List all notices
```
GET /api/notices
```

#### Create notice
```
POST /api/notices
```
Create a new notice.

**Request Body:**
```json
{
  "title": "Notice Title",
  "content": "Notice content",
  "priority": "normal", // normal | high | urgent
  "startDate": "2025-01-01T00:00:00Z",
  "endDate": "2025-01-31T23:59:59Z"
}
```

#### Update notice
```
PUT /api/notices/{id}
```

#### Delete notice
```
DELETE /api/notices/{id}
```

#### Reload notices
```
POST /api/notices/reload
```

### Menu Presets

#### List all menu presets
```
GET /api/presets
```

#### Create menu preset
```
POST /api/presets
```

**Request Body:**
```json
{
  "name": "Preset Name",
  "description": "Preset description",
  "items": ["item_id_1", "item_id_2"]
}
```

#### Get specific menu preset
```
GET /api/presets/{id}
```

#### Update menu preset
```
PUT /api/presets/{id}
```

#### Delete menu preset
```
DELETE /api/presets/{id}
```

#### Reload menu presets
```
POST /api/presets/reload
```

### Schedules

#### List all menu schedules
```
GET /api/schedules
```

#### Create menu schedule
```
POST /api/schedules
```

**Request Body:**
```json
{
  "presetId": "preset_id",
  "date": "2025-01-01",
  "mealType": "breakfast", // breakfast | lunch | dinner
  "notes": "Optional notes"
}
```

#### Get specific menu schedule
```
GET /api/schedules/{id}
```

#### Update menu schedule
```
PUT /api/schedules/{id}
```

#### Delete menu schedule
```
DELETE /api/schedules/{id}
```

#### Get upcoming schedules
```
GET /api/schedules/upcoming
```

#### Validate schedule
```
POST /api/schedules/validate
```

#### Reload menu schedules
```
POST /api/schedules/reload
```

## 📚 Examples

### Example: Creating a new menu item

```http
POST /api/items
Content-Type: application/json

{
  "name": "Veggie Burger",
  "description": "Delicious plant-based burger",
  "category": "Main Course",
  "price": "12.99",
  "available": true,
  "tags": ["vegetarian", "gluten-free"]
}
```

**Response:**
```json
{
  "status": "success",
  "data": {
    "id": "abc123",
    "name": "Veggie Burger",
    "description": "Delicious plant-based burger",
    "category": "Main Course",
    "price": "12.99",
    "available": true,
    "tags": ["vegetarian", "gluten-free"]
  }
}
```

### Example: Getting all notices

```http
GET /api/notices
```

**Response:**
```json
{
  "status": "success",
  "data": [
    {
      "id": "notice_1",
      "title": "New Menu Items",
      "content": "Check out our new seasonal menu!",
      "priority": "normal",
      "startDate": "2025-01-01T00:00:00Z",
      "endDate": "2025-01-31T23:59:59Z"
    }
  ]
}
```

### Example: Error Response

```http
GET /api/items/nonexistent-id
```

**Response:**
```json
{
  "status": "error",
  "error": "Not Found",
  "error_type": "NOT_FOUND",
  "message": "Menu item with id nonexistent-id not found"
}
```