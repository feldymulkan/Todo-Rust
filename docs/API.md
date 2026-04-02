# API Documentation

The Todo-Rust API provides standard CRUD operations for managing tasks. All endpoints are prefixed with `/api`.

## 🌐 Base URL

`http://localhost:8080/api`

## 📋 Endpoints

### 1. List All Tasks
- **URL**: `/tasks`
- **Method**: `GET`
- **Response**: `200 OK`
  ```json
  [
    {
      "id": 1,
      "title": "Learn Rust",
      "completed": false
    }
  ]
  ```

### 2. Create Task
- **URL**: `/tasks`
- **Method**: `POST`
- **Body**:
  ```json
  {
    "title": "Buy groceries"
  }
  ```
- **Response**: `200 OK`
  ```json
  {
    "id": 2,
    "title": "Buy groceries",
    "completed": false
  }
  ```

### 3. Get Task Detail
- **URL**: `/tasks/{id}`
- **Method**: `GET`
- **Parameter**: `id` (integer)
- **Response**: `200 OK` OR `404 Not Found`
  ```json
  {
    "id": 1,
    "title": "Learn Rust",
    "completed": false
  }
  ```

### 4. Update Task (Partial)
- **URL**: `/tasks/{id}`
- **Method**: `PATCH`
- **Parameter**: `id` (integer)
- **Body** (All fields optional):
  ```json
  {
    "title": "Updated Title",
    "completed": true
  }
  ```
- **Response**: `200 OK` OR `404 Not Found`

### 5. Delete Task
- **URL**: `/tasks/{id}`
- **Method**: `DELETE`
- **Parameter**: `id` (integer)
- **Response**: `200 OK` OR `404 Not Found`

## ⚠️ Error Responses

| Code | Message | Description |
| ---- | ------- | ----------- |
| 404 | `Not found: Task with ID {id} not found` | The requested task ID does not exist. |
| 500 | `Database error` | Internal server or database connection error. |
