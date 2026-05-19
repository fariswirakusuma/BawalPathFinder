### ROS_workspace/localization/launch/localization.launch.py
import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    pkg_name = 'localization' 
    
    #  EKF parameters path
    ekf_config_path = os.path.join(
        get_package_share_directory(pkg_name),
        'config',
        'ekf.yml'
    )

    # robot_localization node
    localization= Node(
        package='localization',
        executable='ekf_node',
        name='ekf_filter_node',
        output='screen',
        parameters=[ekf_config_path]
    )

    return LaunchDescription([
        localization
    ])