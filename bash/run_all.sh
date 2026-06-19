#!/bin/bash
export PATH="$PATH:/usr/local/bin"
BASH_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT="$(cd "$BASH_DIR/.." && pwd)"

echo "=== Membersihkan Kontainer Lama ==="
docker rm -f nav2_sim_backend 2>/dev/null

echo "=== Building ROS 2 Backend Image ==="
docker build -t nav2_backend:latest -f "$WORKSPACE_ROOT/Dockerfile" "$WORKSPACE_ROOT"

echo "=== Starting ROS 2 Backend Container ==="
docker run -d \
    --name nav2_sim_backend \
    -p 9090:9090 \
    -v "$WORKSPACE_ROOT/ROS_workspace:/workspace" \
    nav2_backend:latest

echo "=== Menunggu ROS 2 Ready (Waiting for ROS Bridge)... ==="
sleep 5 

echo "=== Memindahkan dan Meluncurkan Executable (Release) ==="
cd "$WORKSPACE_ROOT" || exit 1

export ROSBRIDGE_URL="ws://127.0.0.1:9090"
export NAV2_PARAMS_PATH="$WORKSPACE_ROOT/ROS_workspace/src/navigation/config/nav2_params.yaml"

mkdir -p "$WORKSPACE_ROOT/bin"
cp "$WORKSPACE_ROOT/Interface/target/release/BawalPathFinder" "$WORKSPACE_ROOT/bin/"

./bin/BawalPathFinder &
FRONTEND_PID=$!

cleanup() {
    echo -e "\n=== Shutting down simulation ==="
    kill $FRONTEND_PID 2>/dev/null
    docker rm -f nav2_sim_backend >/dev/null
    exit 0
}

trap cleanup SIGINT
wait $FRONTEND_PID