#!/bin/bash

# kill zombie process
pkill -f ros2
pkill -f planner_server
pkill -f controller_server
pkill -f behavior_server
pkill -f bt_navigator
pkill -f smoother_server
pkill -f waypoint_follower
pkill -f velocity_smoother
pkill -f tf2_ros
pkill -f component_container
sleep 2

# setup
WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$WORKSPACE_ROOT/ROS_workspace" || exit 1

# clean
rm -rf build/ install/ log/

# Rebuild workspace
colcon build

# Source env
source install/setup.bash
PARAMS_FILE="$WORKSPACE_ROOT/ROS_workspace/navigation/config/nav2_params.yaml"

# run navi
ros2 launch nav2_bringup navigation_launch.py params_file:="$PARAMS_FILE" &

# tf tree
ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 map odom &
ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 odom base_footprint &

# wait initialize
sleep 15

# backend test
ros2 run navigation backend_test_node