import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.substitutions import LaunchConfiguration, Command
from launch.actions import DeclareLaunchArgument
from launch_ros.actions import Node
from launch_ros.parameter_descriptions import ParameterValue

def generate_launch_description():
    pkg_name = 'description' 
    urdf_file_name = 'generated_robot.urdf' 

    pkg_path = os.path.join(get_package_share_directory(pkg_name))
    urdf_path = os.path.join(pkg_path, 'urdf', urdf_file_name)

    use_sim_time = LaunchConfiguration('use_sim_time')

    declare_use_sim_time_cmd = DeclareLaunchArgument(
        name='use_sim_time',
        default_value='false',
        description='Use simulation (Gazebo) clock if true'
    )

    robot_description_content = ParameterValue(
        Command(['xacro ', urdf_path]), 
        value_type=str
    )

    robot_state_publisher_node = Node(
     
        package='robot_state_publisher',
  
        executable='robot_state_publisher',
        name='robot_state_publisher',
        output='screen',
        parameters=[{
            'robot_description': robot_description_content,
            'use_sim_time': use_sim_time
        }]
    )

    # Node 2: Joint State Publisher (CRITICAL FOR RVIZ WHEEL RNDERING)
    joint_state_publisher_node = Node(
        package='joint_state_publisher',
        executable='joint_state_publisher',
        name='joint_state_publisher',
        parameters=[{'use_sim_time': use_sim_time}]
    )

    return LaunchDescription([
        declare_use_sim_time_cmd,
        robot_state_publisher_node,
        joint_state_publisher_node
    ])