# BawalPathFinder

Sistem simulasi robotika pathfinding menggunakan **ROS 2 Humble (Nav2)** sebagai backend pemrosesan jalur di dalam lingkungan terisolasi Docker, dan **Bevy Engine (Rust)** sebagai frontend visualisasi grafis 2D/3D secara native di host OS.

## System Requirements

- **OS**: Linux (Direkomendasikan)
- **Rust / Bevy**: Cargo & Rustc (Edition 2021), Bevy Engine v0.18
- **C++**: C++17 (Standar minimum untuk ROS 2 Humble)
- **Docker**: Diperlukan untuk menjalankan backend secara terisolasi
- **Python 3**: Diperlukan untuk men-generate peta test (Pillow, faker)

## Docker Image

Sistem ini menggunakan Docker untuk membungkus environment backend secara aman.
- **Base Image**: `osrf/ros:humble-desktop`
- **Output Image**: `nav2_backend:latest`

## Cara Menjalankan & Setup

Langkah paling mudah untuk menjalankan proyek ini adalah menggunakan perintah `make`:

```bash
# 1. Men-generate dan menyiapkan peta simulasi
make map

# 2. Menjalankan sistem keseluruhan (akan otomatis mengompilasi Frontend & Backend)
make run
```

Atau bisa juga menggunakan bash script secara manual:

```bash
# Memberikan izin akses eksekusi ke semua script bash
chmod +x bash/*.sh

# Menjalankan simulasi keseluruhan
./bash/run_all.sh
```

## Command List

### Makefile Commands

| Command | Deskripsi |
| --- | --- |
| `make all` | Mengompilasi frontend (Rust) dan mem-build image backend (Docker). |
| `make build_frontend`| Mengompilasi UI Rust dalam mode rilis (`--release`) ke direktori `bin/`. |
| `make build_backend` | Membangun image Docker `nav2_backend:latest` menggunakan `Dockerfile`. |
| `make run` | Menjalankan sistem keseluruhan (menjalankan frontend UI dan backend container). |
| `make stop` | Menghentikan container ROS dan mematikan proses frontend. |
| `make clean` | Menghapus binary frontend, cache Cargo, dan membersihkan peta. |
| `make map` | Men-generate peta uji coba sintetik menggunakan Python dan memindahkannya ke workspace ROS. |
| `make map_clean` | Menghapus aset file peta dari direktori lokal dan workspace ROS. |

### Bash Scripts (`bash/`)

| Script | Deskripsi |
| --- | --- |
| `run_all.sh` | Membersihkan container lama, membangun image docker baru, menjalankan container backend, dan meluncurkan *executable* interface Bevy. |
| `run_backend.sh` | Dieksekusi untuk membersihkan & membangun ulang workspace ROS, lalu meluncurkan tumpukan navigasi Nav2 beserta *node* komunikasi tambahan. |
| `cleanbackend.sh`| Membersihkan sisa proses ROS (zombie process) dan menghapus folder build artefak (build, install, log) secara lokal di workspace. |
| `entrypoint.sh` | Skrip konfigurasi awal (*entrypoint*) bawaan saat container Docker dijalankan. |
