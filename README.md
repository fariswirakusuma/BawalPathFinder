# BawalPathFinder 🐟

![Version](https://img.shields.io/badge/Release-v0.1.0-blue)
![Build](https://img.shields.io/badge/Build-Makefile_Ready-success)
![Language](https://img.shields.io/badge/Language-C++17-orange)

[English](README_en.md) | [Indonesia](README.md)

Halo! Selamat datang di **BawalPathFinder**. Intinya ini adalah sistem simulasi robot buat nyari jalan (pathfinding). Backend-nya kita pakai **ROS 2 Humble (Nav2)** yang jalan aman di dalam Docker, nah buat tampilan visualnya (frontend) kita pakai **Bevy Engine (Rust)** yang jalan langsung di OS kamu buat nampilin grafis 2D atau 3D.

## Kebutuhan Sistem (Biar Jalan Mulus)

Biar bisa jalan tanpa hambatan, pastikan kamu udah nyiapin ini:
- **OS**: Linux (Sangat disarankan ya!)
- **Rust / Bevy**: Cargo & Rustc (Edition 2021), dan Bevy Engine v0.18
- **C++**: Harus C++17 (Syarat mutlak buat ROS 2 Humble)
- **Docker**: Wajib ada buat ngerun backend-nya di dalam kontainer.
- **Python 3**: Kepake buat bikin peta-peta buat testing (butuh library Pillow sama faker).

## Tentang Docker Image

Buat ngejaga environment backend tetap rapi dan terisolasi, kita bungkus pakai Docker.
- **Base Image**: `osrf/ros:humble-desktop`
- **Output Image**: `nav2_backend:latest`

## Cara Setup & Menjalankan

Cara paling gampang dan sat-set tuh pakai perintah `make`. Gini urutannya:

```bash
# 1. Bikin dan nyiapin peta simulasinya dulu
make map

# 2. Jalanin deh semuanya (otomatis nge-build Frontend & Backend kok)
make run
```

Atau kalau kamu lebih suka jalanin script bash secara manual, bisa juga:

```bash
# Kasih izin jalan dulu ke semua file bash-nya
chmod +x bash/*.sh

# Terus tinggal jalanin deh simulasi utamanya
./bash/run_all.sh
```

## Daftar Command Lengkap

### Command Makefile

Kalau kamu tim `make`, ini daftarnya:

| Command | Ngapain aja tuh? |
| --- | --- |
| `make all` | Nge-build frontend (Rust) sama image backend (Docker). |
| `make build_frontend`| Nge-compile UI Rust dalam mode rilis (`--release`) ke folder `bin/`. |
| `make build_backend` | Bikin image Docker `nav2_backend:latest` dari `Dockerfile`. |
| `make run` | Jalanin semua sistemnya (frontend UI jalan, backend container juga jalan). |
| `make stop` | Berhentiin container ROS sama matiin proses frontend. |
| `make clean` | Bersihin binary frontend, cache Cargo, sama ngapus-ngapusin peta. |
| `make map` | Bikin peta testing pakai Python terus dipindahin ke workspace ROS. |
| `make map_clean` | Hapus semua file peta dari komputer dan workspace ROS. |

### Bash Scripts (`bash/`)

Kalau kamu lebih suka main di folder `bash/`:

| Script | Ngapain aja tuh? |
| --- | --- |
| `run_all.sh` | Bersihin container lama, bikin image docker baru, jalanin backend, sama buka UI Bevy-nya. Lengkap deh. |
| `run_backend.sh` | Bersihin & build ulang workspace ROS, terus jalanin navigasi Nav2 dan node pelengkapnya. |
| `cleanbackend.sh`| Basmi zombie process ROS sama bersihin file-file sisa build (kayak folder build, install, log). |
| `entrypoint.sh` | Script bawaan yang jalan pas container Docker pertama kali dihidupin. |
