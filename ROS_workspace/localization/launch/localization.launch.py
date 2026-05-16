### ROS_workspace/localization/launch/localization.launch.py
import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    # Package configuration setup
    # Replace 'localization' with your actual package name if different in package.xml
    pkg_name = 'localization' 
    
    # Path to your EKF parameters file
    ekf_config_path = os.path.join(
        get_package_share_directory(pkg_name),
        'config',
        'ekf.yaml'
    )

    # Declare the robot_localization node
    robot_localization_node = Node(
        package='robot_localization',
        executable='ekf_node',
        name='ekf_filter_node',
        output='screen',
        parameters=[ekf_config_path]
    )

    return LaunchDescription([
        robot_localization_node
    ])