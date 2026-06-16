#!/bin/bash

# Deteksi lokasi absolut dari root workspace (satu tingkat di atas folder bash/)
BASH_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT="$(cd "$BASH_DIR/.." && pwd)"

echo "=== Building ROS 2 Backend Image ==="
# Jalankan build dari root workspace menggunakan file Dockerfile di root
docker build -t nav2_backend:latest -f "$WORKSPACE_ROOT/Dockerfile" "$WORKSPACE_ROOT"

# Bersihkan container lama jika ada
docker rm -f nav2_sim_backend 2>/dev/null

echo "=== Starting ROS 2 Backend Container ==="
# Mount folder ROS_workspace menggunakan path absolut yang dinamis
docker run -d \
    --name nav2_sim_backend \
    -p 9090:9090 \
    -v "$WORKSPACE_ROOT/ROS_workspace:/workspace" \
    nav2_backend:latest

echo "=== Waiting for ROSBridge Server (15s) ==="
sleep 15

echo "=== Launching Bevy Frontend ==="
# Pindah ke direktori Interface di root untuk mengeksekusi Bevy
cd "$WORKSPACE_ROOT/Interface" || exit 1
cargo run &
FRONTEND_PID=$!

cleanup() {
    echo -e "\n=== Shutting down simulation ==="
    kill $FRONTEND_PID 2>/dev/null
    docker stop nav2_sim_backend >/dev/null
    docker rm nav2_sim_backend >/dev/null
    echo "=== Clean up complete ==="
    exit 0
}

trap cleanup SIGINT

wait $FRONTEND_PID