#!/bin/bash
set -e

source /opt/ros/humble/setup.bash

echo "=== Building ROS 2 Workspace ==="
cd /workspace
rm -rf build/ install/ log/
colcon build

source install/setup.bash

echo "=== Launching Nav2 & TF ==="
ros2 launch navigation navigation.launch.py &

sleep 5

echo "=== Running Backend Test Node ==="
ros2 run navigation backend_test_node &

echo "=== Starting ROSBridge Server (Foreground) ==="
exec ros2 run rosbridge_server rosbridge_websocket