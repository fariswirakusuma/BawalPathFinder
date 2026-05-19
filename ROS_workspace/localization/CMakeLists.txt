cmake_minimum_required(VERSION 3.8)
project(localization) 

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(nav_msgs REQUIRED)

# 1. Define the executable binary and its source files
add_executable(ekf_node 
  src/ekf_node.cpp
)

# 2. Link ROS 2 library tracking dependencies
ament_target_dependencies(ekf_node
  rclcpp
  geometry_msgs
  nav_msgs
)

# 3. Install the executable to the global lib/ directory tree
install(TARGETS ekf_node
  DESTINATION lib/${PROJECT_NAME}
)

# 4. Install setup launch patterns and runtime asset files
install(DIRECTORY
  launch
  config
  DESTINATION share/${PROJECT_NAME}/
)

ament_package()