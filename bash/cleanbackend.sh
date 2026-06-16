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
pkill -f rosbridge

sleep 2

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$WORKSPACE_ROOT/ROS_workspace" || exit 1

if [ "$1" == "--clean" ]; then
    echo "=== Cleaning local build artifacts ==="
    rm -rf build/ install/ log/
else
    echo "=== Skipping clean: Using incremental build ==="
fi