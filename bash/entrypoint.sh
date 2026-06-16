#!/bin/bash
set -e

# Source ROS 2 base environment
source /opt/ros/humble/setup.bash

# Clean & Build workspace
rm -rf build/ install/ log/
colcon build

# Source local workspace
source install/setup.bash

PARAMS_FILE="/workspace/navigation/config/nav2_params.yaml"

# # ros2 init pos (temp)
# ros2 launch nav2_bringup navigation_launch.py params_file:="$PARAMS_FILE" &
# ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 map odom &
# ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 odom base_footprint &
# ros2 run rosbridge_server rosbridge_websocket &

# def pos (background)
ros2 launch nav2_bringup navigation_launch.py params_file:="$PARAMS_FILE" &
ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 map odom &
ros2 run rosbridge_server rosbridge_websocket &

# Nav2 init
sleep 15

# TestNode
exec ros2 run navigation backend_test_node