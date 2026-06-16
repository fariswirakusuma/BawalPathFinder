#!/bin/bash

# docker image
echo "=== Building ROS 2 Backend Image ==="
docker build -t nav2_backend:latest .

# docker clean
docker rm -f nav2_sim_backend 2>/dev/null

#run backend (background)
echo "=== Starting ROS 2 Backend Container ==="
docker run -d \
    --name nav2_sim_backend \
    -p 9090:9090 \
    -v "$(pwd)/ROS_workspace:/workspace" \
    nav2_backend:latest

# 4. Beri waktu inisialisasi untuk Nav2 dan ROSBridge di dalam container
echo "=== Waiting for ROSBridge Server (15s) ==="
sleep 15

# 5. Jalankan Frontend Bevy secara native di OS Host (Foreground)
echo "=== Launching Bevy Frontend ==="
cargo run &
FRONTEND_PID=$!

# clean_up(cntr+c interrupt)
cleanup() {
    echo -e "\n=== Shutting down simulation ==="
    kill $FRONTEND_PID 2>/dev/null
    docker stop nav2_sim_backend >/dev/null
    docker rm nav2_sim_backend >/dev/null
    echo "=== Clean up complete ==="
    exit 0
}

# SIGINT (Ctrl+C) (cleanup)
trap cleanup SIGINT
wait $FRONTEND_PID