# Panduan Penggunaan SeaORM di Rust

SeaORM adalah ORM (*Object-Relational Mapping*) asinkron yang kuat untuk Rust. Panduan ini menjelaskan langkah-langkah dasar untuk mengintegrasikannya ke dalam proyek Axum Anda.

## 1. Persiapan
Tambahkan dependensi berikut ke file `Cargo.toml`:

```toml
[dependencies]
# SeaORM dengan runtime tokio dan driver postgres
sea-orm = { version = "1.1", features = [ "runtime-tokio-rustls", "sqlx-postgres", "macros" ] }
```

## 2. Koneksi Database
Ganti koneksi `sqlx` Anda dengan `Database::connect`:

```rust
use sea_orm::{Database, DatabaseConnection};

pub async fn connect_db() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url)
        .await
        .expect("Failed to connect to database")
}
```

## 3. Membuat Entity (Model)
Entity adalah representasi tabel database dalam kode Rust. Anda bisa membuatnya manual atau menggunakan `sea-orm-cli`.

### Contoh Entity `Task`:
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

## 4. Operasi CRUD di Handler

### Membaca Data (SELECT)
```rust
// Mengambil semua task
let tasks: Vec<task::Model> = Task::find().all(db).await?;

// Mengambil satu task berdasarkan ID
let task: Option<task::Model> = Task::find_by_id(id).one(db).await?;
```

### Menambah Data (INSERT)
```rust
use sea_orm::ActiveValue::Set;

let new_task = task::ActiveModel {
    title: Set("Belajar SeaORM".to_owned()),
    completed: Set(false),
    ..Default::default() // Menangani id (auto-increment)
};

let result = new_task.insert(db).await?;
```

### Mengubah Data (UPDATE)
```rust
let task: task::ActiveModel = Task::find_by_id(id)
    .one(db)
    .await?
    .ok_or(AppError::NotFound)?
    .into(); // Ubah Model menjadi ActiveModel

task.completed = Set(true);
let updated_task = task.update(db).await?;
```

### Menghapus Data (DELETE)
```rust
let result = Task::delete_by_id(id).exec(db).await?;
```

## 5. Keuntungan SeaORM dibanding SQLx mentah
1. **Dynamic Queries**: Tidak perlu koneksi DB saat compile (menghindari error `DATABASE_URL` saat `cargo check`).
2. **Relationship**: Mudah menangani Join dan relasi antar tabel.
3. **Pagination**: Fitur pagination sudah bawaan dan sangat mudah digunakan.
4. **ActiveRecord Pattern**: Manipulasi data terasa lebih natural bagi pengembang yang datang dari bahasa lain.

---

## 6. Menggunakan Sea-ORM CLI
`sea-orm-cli` adalah alat baris perintah yang sangat membantu untuk manajemen migrasi dan pembuatan kode entity otomatis.

### Instalasi CLI
```bash
cargo install sea-orm-cli
```

### Inisialisasi Migrasi
Jika Anda ingin SeaORM mengelola struktur tabel Anda:
```bash
sea-orm-cli migrate init
```
Ini akan membuat folder `migration/` di proyek Anda.

### Membuat Entity Otomatis (Database First)
Jika tabel sudah ada di database, Anda bisa men-generate file Entity secara otomatis tanpa menulis kode manual:
```bash
sea-orm-cli generate entity \
    -u postgres://user:password@localhost:5432/todo_db \
    -o src/entities
```
- `-u`: URL Database Anda.
- `-o`: Folder tujuan file entity (biasanya `src/entities`).

### Menjalankan Migrasi
```bash
sea-orm-cli migrate up
```
Berfungsi untuk memperbarui skema database sesuai dengan file migrasi yang ada.

---

## 7. Menambah Kolom atau Tabel Baru (Migrasi)
SeaORM menggunakan sistem migrasi berbasis kode Rust (bukan SQL mentah), sehingga riwayat perubahan skema Anda tercatat dengan aman.

### Langkah 1: Generate File Migrasi Baru
Gunakan CLI untuk membuat file migrasi baru:
```bash
sea-orm-cli migrate generate nama_perubahan_anda
```
Contoh: `sea-orm-cli migrate generate add_priority_to_tasks`

### Langkah 2: Edit File Migrasi
Buka file baru yang terbuat di folder `migration/src/m202xxxx_xxxxxx_...rs`.

**Untuk Menambah Tabel Baru:**
```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.create_table(
        Table::create()
            .table(Post::Table)
            .if_not_exists()
            .col(ColumnDef::new(Post::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Post::Title).string().not_null())
            .to_owned()
    ).await
}
```

**Untuk Menambah Kolom ke Tabel yang Sudah Ada:**
```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.alter_table(
        Table::alter()
            .table(Tasks::Table)
            .add_column(ColumnDef::new(Alias::new("priority")).integer().not_null().default(1))
            .to_owned()
    ).await
}
```

### Langkah 3: Jalankan Migrasi
```bash
sea-orm-cli migrate up
```

### Langkah 4: Update Entity
Penting! Setelah database berubah, Anda harus memperbarui struct di `src/entities/` agar sesuai. Anda bisa edit manual atau jalankan kembali perintah:
```bash
sea-orm-cli generate entity -u $DATABASE_URL -o src/entities
```
