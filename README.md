# BawalPathFinder 🐟

![Version](https://img.shields.io/badge/Release-v0.1.0--alpha-blue)
![Build](https://img.shields.io/badge/Build-Makefile_Ready-success)
![C++](https://img.shields.io/badge/C++-17-orange)
![Bevy](https://img.shields.io/badge/Bevy-0.18-red)
![ROS 2](https://img.shields.io/badge/ROS2-Humble-blue)
![Python](https://img.shields.io/badge/Python-3.8+-yellow)
![Docker](https://img.shields.io/badge/Docker-Containerized-blue)

[English](README_en.md) | [Indonesia](README.md)

## Tentang Proyek
**BawalPathFinder** adalah sistem simulasi navigasi robot berbasis ROS 2 Nav2.
- **Backend**: ROS 2 Humble (Nav2) berjalan dalam Docker Container untuk menjamin isolasi *environment*.
- **Frontend**: Bevy Engine (Rust) untuk visualisasi 2D/3D real-time.
- **Komunikasi**: WebSocket (Rosbridge) menghubungkan frontend Rust dengan backend ROS 2.

## Tech Stack
* **Bevy (Rust)**: Engine rendering untuk antarmuka pengguna (UI) dan visualisasi jalur.
* **ROS 2 (Humble)**: Middleware utama untuk navigasi, planning, dan *costmap*.
* **CMake 3.10+**: Digunakan untuk membangun plugin C++ Nav2.
* **Python 3**: Digunakan untuk script helper, pembuatan map testing, dan otomatisasi testing.

## Algoritma Pathfinding
Sistem ini mendukung beberapa algoritma pathfinding. Implementasi utama dilakukan di sisi C++ (Nav2 Plugins).

| Algoritma | Logika Kalkulasi ($f(n) = g(n) + h(n)$) | Karakteristik |
| :--- | :--- | :--- |
| **A\*** | $f(n) = g(n) + h(n)$ | Optimal & Cepat (Cost + Heuristic). |
| **GBFS** | $f(n) = h(n)$ | Sangat cepat, namun tidak selalu menjamin rute terpendek. |
| **Dijkstra / UCS** | $f(n) = g(n)$ | Menjamin rute terpendek, tapi eksplorasi luas (lambat). |

* **$g(n)$**: Biaya sebenarnya dari start ke node sekarang.
* **$h(n)$**: Estimasi biaya (Heuristic) dari node sekarang ke goal.

> **Di mana meletakkan implementasi C++?**
> Untuk menambah atau memodifikasi logika kalkulasi, masuk ke direktori:
> `src/navigation/plugins/` atau `src/navigation/src/`
> Pastikan Anda menyesuaikan `CMakeLists.txt` jika menambah file `.cpp` atau `.hpp` baru agar terkompilasi ke dalam *workspace* ROS.

## Cara Setup & Menjalankan

### 1. Kebutuhan Sistem
Pastikan lingkungan Anda memenuhi spesifikasi berikut:
- **OS**: Linux (Ubuntu 22.04 LTS sangat disarankan).
- **Toolchain**: Rust/Cargo (Edition 2021), C++17, Python 3.8+.
- **Docker**: Wajib untuk menjalankan backend.

### 2. Command Makefile (Penting!)
Gunakan `make rebuild_all` sebagai *command* utama untuk memastikan sinkronisasi antara *frontend* dan *backend* (ini akan melakukan build ulang UI dan re-sync Docker).

| Command | Deskripsi |
| --- | --- |
| `make rebuild_all` | Membersihkan cache, build ulang frontend & backend container. |
| `make run` | Menjalankan simulasi (Frontend + Backend). |
| `make map` | Membuat file peta baru dengan Python. |
| `make stop` | Menghentikan container ROS & proses Bevy. |
| `make clean` | Menghapus semua file build dan cache. |

### 3. Bash Scripts
Jika memerlukan kontrol manual:
- `./bash/run_all.sh`: Menjalankan seluruh pipeline simulasi.
- `./bash/run_backend.sh`: Menjalankan node navigasi secara terpisah.
- `./bash/cleanbackend.sh`: Membersihkan proses zombie ROS 2.

> [!NOTE]
> **Troubleshooting (Catatan v0.1.0-alpha)**
> * **Visual Path Menggantung**: Nav2 menggunakan *inflation layer* (radius rintangan). Path mungkin terlihat tidak menyentuh dinding atau goal secara presisi karena robot membutuhkan *clearance*.
> * **Code 6 (Planning Failed)**: Jika goal berada di dalam zona *lethal cost* (tabrakan), Nav2 akan menolak planning.
> * **Data "Zombie"**: Jika path masih muncul setelah reset, pastikan fungsi `cleanup_sim2d` memanggil `cancel_goal` ke `/compute_path_to_pose/_action/cancel_goal` untuk menghentikan kalkulasi backend.   
---