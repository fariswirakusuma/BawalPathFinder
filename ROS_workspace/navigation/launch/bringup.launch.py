import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node

def generate_launch_description():
    custom_pkg_dir = get_package_share_directory('navigation')

    use_sim_time = LaunchConfiguration('use_sim_time')
    params_file = LaunchConfiguration('params_file')
    map_yaml_file = LaunchConfiguration('map')

    declare_use_sim_time_cmd = DeclareLaunchArgument(
        'use_sim_time', default_value='false')

    declare_params_file_cmd = DeclareLaunchArgument(
        'params_file', default_value=os.path.join(custom_pkg_dir, 'config', 'nav2_params.yaml'))

    declare_map_yaml_cmd = DeclareLaunchArgument(
        'map', default_value=os.path.join(custom_pkg_dir, 'maps', 'map.yaml'))

    fake_map_bridge = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='fake_map_publisher',
        output='screen',
        arguments=['--x', '0', '--y', '0', '--z', '0', '--yaw', '0', '--pitch', '0', '--roll', '0', '--frame-id', 'map', '--child-frame-id', 'odom']
    )

    fake_odom_publisher = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='fake_odom_publisher',
        output='screen',
        arguments=['--x', '0', '--y', '0', '--z', '0', '--yaw', '0', '--pitch', '0', '--roll', '0', '--frame-id', 'odom', '--child-frame-id', 'base_footprint']
    )

    fake_base_link_bridge = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='fake_base_link_publisher',
        output='screen',
        arguments=['--x', '0', '--y', '0', '--z', '0', '--yaw', '0', '--pitch', '0', '--roll', '0', '--frame-id', 'base_footprint', '--child-frame-id', 'base_link']
    )

    map_server_node = Node(
        package='nav2_map_server',
        executable='map_server',
        name='map_server',
        output='screen',
        parameters=[params_file, {
            'yaml_filename': map_yaml_file, 
            'use_sim_time': use_sim_time, 
            'use_bond': True, 
            'topic_name': 'map', 
            'frame_id': 'map'
        }]
    )

    planner_server_node = Node(
        package='nav2_planner',
        executable='planner_server',
        name='planner_server',
        output='screen',
        parameters=[params_file, {'use_sim_time': use_sim_time, 'use_bond': True}]
    )

    controller_server_node = Node(
        package='nav2_controller',
        executable='controller_server',
        name='controller_server',
        output='screen',
        parameters=[params_file, {'use_sim_time': use_sim_time, 'use_bond': True}]
    )

    lifecycle_manager_node = Node(
        package='nav2_lifecycle_manager',
        executable='lifecycle_manager',
        name='lifecycle_manager_navigation',
        output='screen',
        parameters=[{
            'use_sim_time': use_sim_time,
            'autostart': True,
            'node_names': ['map_server', 'planner_server', 'controller_server'],
            'use_bond': True,
            'bond_timeout': 10.0,
            'attempt_respawn_reconnection': True
        }]
    )

    ld = LaunchDescription()
    
    ld.add_action(declare_use_sim_time_cmd)
    ld.add_action(declare_params_file_cmd)
    ld.add_action(declare_map_yaml_cmd)
    
    ld.add_action(fake_map_bridge)
    ld.add_action(fake_odom_publisher)
    ld.add_action(fake_base_link_bridge)
    
    ld.add_action(map_server_node)
    ld.add_action(planner_server_node)
    ld.add_action(controller_server_node)
    ld.add_action(lifecycle_manager_node)

    return ld