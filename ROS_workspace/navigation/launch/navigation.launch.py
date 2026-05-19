import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node

def generate_launch_description():
    custom_pkg_dir = get_package_share_directory('navigation')
    nav2_bringup_dir = get_package_share_directory('nav2_bringup')

    use_sim_time = LaunchConfiguration('use_sim_time')
    params_file = LaunchConfiguration('params_file')
    map_yaml_file = LaunchConfiguration('map')

    declare_use_sim_time_cmd = DeclareLaunchArgument(
        'use_sim_time',
        default_value='false',
        description='Use simulation clock if true')

    declare_params_file_cmd = DeclareLaunchArgument(
        'params_file',
        default_value=os.path.join(custom_pkg_dir, 'config', 'nav2_params.yaml'),
        description='Full path to the ROS2 parameters file to use for all instantiated nodes')

    declare_map_yaml_cmd = DeclareLaunchArgument(
        'map',
        default_value=os.path.join(custom_pkg_dir, 'maps', 'map.yaml'), 
        description='Full path to the map yaml file to load')

    # Link Segment 1: Global Map to Odometry Frame
    fake_map_bridge = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='fake_map_publisher',
        output='screen',
        arguments=['0', '0', '0', '0', '0', '0', 'map', 'odom']
    )
    # Bikin jembatan dari odom ke base_footprint
    fake_odom_publisher = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='fake_odom_publisher',
        output='screen',
        arguments=['0', '0', '0', '0', '0', '0', 'odom', 'base_footprint']
    )
    

    # Link Segment 2: Local Odometry to Base Footprint Frame
    # fake_odometry_bridge = Node(
    #     package='tf2_ros',
    #     executable='static_transform_publisher',
    #     name='fake_odom_publisher',
    #     output='screen',
    #     arguments=['0', '0', '0', '0', '0', '0', 'odom', 'base_footprint']
    # )

    # Link Segment 3: Base Footprint to Base Link (Resolves Controller Timeout)
    fake_base_link_bridge = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='fake_base_link_publisher',
        output='screen',
        arguments=['0', '0', '0', '0', '0', '0', 'base_footprint', 'base_link']
    )

    nav2_launch_cmd = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(nav2_bringup_dir, 'launch', 'bringup_launch.py')
        ),
        launch_arguments={
            'use_sim_time': use_sim_time,
            'params_file': params_file,
            'map': map_yaml_file,
            'slam': 'False',          
            'autostart': 'true',
            'use_composition': 'False'
        }.items()
    )

    ld = LaunchDescription()
    ld.add_action(declare_use_sim_time_cmd)
    ld.add_action(declare_params_file_cmd)
    ld.add_action(declare_map_yaml_cmd)
    
    # Inject all three static transform broadcasters into the execution loop
    ld.add_action(fake_map_bridge)
    # ld.add_action(fake_odometry_bridge)
    ld.add_action(fake_base_link_bridge)
    ld.add_action(fake_odom_publisher)
    
    ld.add_action(nav2_launch_cmd)

    return ld