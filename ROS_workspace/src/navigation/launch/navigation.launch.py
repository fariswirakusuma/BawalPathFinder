import os
import glob
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node

def generate_launch_description():
    custom_pkg_dir = get_package_share_directory('navigation')
    nav2_bringup_dir = get_package_share_directory('nav2_bringup')

    maps_dir = os.path.join(custom_pkg_dir, 'maps')
    map_files = glob.glob(os.path.join(maps_dir, '*.yaml'))
    
    if map_files:
        latest_map_yaml = max(map_files, key=os.path.getmtime)
    else:
        latest_map_yaml = os.path.join(maps_dir, 'map.yaml')

    params_file = LaunchConfiguration('params_file')
    map_yaml_file = LaunchConfiguration('map')

    declare_params_file_cmd = DeclareLaunchArgument(
        'params_file',
        default_value=os.path.join(custom_pkg_dir, 'config', 'nav2_params.yaml'))

    declare_map_yaml_cmd = DeclareLaunchArgument(
        'map',
        default_value=latest_map_yaml)

    # TF Bridges
    fake_map_bridge = Node(
        package='tf2_ros', executable='static_transform_publisher',
        arguments=['0', '0', '0', '0', '0', '0', 'map', 'odom'])

    fake_odom_publisher = Node(
        package='tf2_ros', executable='static_transform_publisher',
        arguments=['0', '0', '0', '0', '0', '0', 'odom', 'base_footprint'])

    fake_base_link_bridge = Node(
        package='tf2_ros', executable='static_transform_publisher',
        arguments=['0', '0', '0', '0', '0', '0', 'base_footprint', 'base_link'])

    # Eksekusi bawaan bringup_launch.py
    nav2_launch_cmd = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(nav2_bringup_dir, 'launch', 'bringup_launch.py')
        ),
        launch_arguments={
            'use_sim_time': 'false',
            'params_file': params_file,
            'map': map_yaml_file, 
            'slam': 'False',          
            'autostart': 'true',
            'use_composition': 'False'
        }.items()
    )

    return LaunchDescription([
        declare_params_file_cmd,
        declare_map_yaml_cmd,
        fake_map_bridge,
        fake_odom_publisher,
        fake_base_link_bridge,
        nav2_launch_cmd
    ])