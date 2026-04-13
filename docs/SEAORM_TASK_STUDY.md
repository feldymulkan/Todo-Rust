# Implementasi SeaORM: Studi Kasus Todo-Rust

Dokumen ini berisi panduan teknis dan kode lengkap untuk mengubah proyek **Todo-Rust** dari `sqlx` menjadi **SeaORM**, khusus untuk tabel `tasks` Anda.

## 1. Perubahan `Cargo.toml`
Hapus `sqlx` (atau biarkan jika masih butuh) dan tambahkan SeaORM:

```toml
[dependencies]
# SeaORM
sea-orm = { version = "1.1", features = [ "runtime-tokio-rustls", "sqlx-postgres", "macros" ] }
# Axum & Serde tetap sama
```

## 2. Definisi Entity (Model Baru)
Buat file baru, misalnya `src/entities/task.rs`. Ini menggantikan model manual Anda.

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

## 3. Update AppState (`src/state.rs`)
Ganti `PgPool` menjadi `DatabaseConnection`.

```rust
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DatabaseConnection, // Menggunakan koneksi SeaORM
}
```

## 4. Handler Lengkap (`src/handlers/handler_tasks.rs`)
Berikut adalah perbandingan logika `sqlx` vs `SeaORM` untuk setiap endpoint:

```rust
use ax_extract::{Path, State};
use axum::{Json, http::StatusCode};
use crate::state::AppState;
use crate::entities::task::{self, Entity as Task}; // Import Model/Entity
use sea_orm::{ActiveModelTrait, EntityTrait, Set, ActiveValue};

// 1. CREATE TASK
pub async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<NewTask>
) -> Result<Json<task::Model>, AppError> {
    let new_task = task::ActiveModel {
        title: Set(payload.title),
        completed: Set(false),
        ..Default::default()
    };

    let result = new_task.insert(&app_state.db_pool).await?;
    Ok(Json(result))
}

// 2. LIST ALL TASKS
pub async fn list_task(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<task::Model>>, AppError> {
    let tasks = Task::find().all(&app_state.db_pool).await?;
    Ok(Json(tasks))
}

// 3. DETAIL TASK
pub async fn detail_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<i32>
) -> Result<Json<task::Model>, AppError> {
    let task = Task::find_by_id(task_id)
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task {} tidak ditemukan", task_id)))?;

    Ok(Json(task))
}

// 4. UPDATE TASK (COALESCE Logic)
pub async fn update_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<i32>,
    Json(payload): Json<UpdateTask>
) -> Result<StatusCode, AppError> {
    // Ambil data lama
    let task = Task::find_by_id(task_id)
        .one(&app_state.db_pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task {} tidak ditemukan", task_id)))?;

    // Ubah ke ActiveModel untuk update
    let mut task: task::ActiveModel = task.into();

    // Logika COALESCE (update hanya jika ada nilainya)
    if let Some(title) = payload.title {
        task.title = Set(title);
    }
    if let Some(completed) = payload.completed {
        task.completed = Set(completed);
    }

    task.update(&app_state.db_pool).await?;
    Ok(StatusCode::OK)
}

// 5. DELETE TASK
pub async fn delete_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<i32>
) -> Result<StatusCode, AppError> {
    let result = Task::delete_by_id(task_id)
        .exec(&app_state.db_pool)
        .await?;

    if result.rows_affected > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("Task {} tidak ada", task_id)))
    }
}
```

## 5. Mengapa Ini Lebih Baik untuk Anda?
1. **Tidak Ada Error Macro**: Anda bisa menjalankan `cargo check` atau `cargo run` tanpa harus memastikan PostgreSQL menyala (asalkan logic kodenya benar).
2. **Type Safety Tinggi**: `Set(payload.title)` memastikan tipe data yang masuk sesuai dengan kolom di DB.
3. **Pembacaan Kode**: Logika `update_task` jauh lebih manusiawi dibanding `COALESCE` di SQL mentah.
