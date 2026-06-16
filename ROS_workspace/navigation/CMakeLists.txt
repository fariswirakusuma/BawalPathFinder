cmake_minimum_required(VERSION 3.8)
project(navigation)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(rclcpp_action REQUIRED)
find_package(std_msgs REQUIRED)
find_package(nav2_core REQUIRED)
find_package(nav2_costmap_2d REQUIRED)
find_package(nav2_util REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(nav_msgs REQUIRED)
find_package(nav2_msgs REQUIRED)
find_package(sensor_msgs REQUIRED)
find_package(std_srvs REQUIRED)
find_package(pluginlib REQUIRED)
find_package(nlohmann_json REQUIRED)
find_package(tf2_ros REQUIRED)

# Nama target diubah menjadi selectable_planner
add_library(selectable_planner SHARED
  src/SelectablePlanner.cpp
  src/PathFinder.cpp
  src/A_Star_Solver.cpp
  src/Dijkstra_Solver.cpp
  src/GBFS_Solver.cpp
)

target_include_directories(selectable_planner
  PUBLIC
  $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
  $<INSTALL_INTERFACE:include>
)

ament_target_dependencies(selectable_planner
  rclcpp
  nav2_core
  nav2_costmap_2d
  nav2_util
  geometry_msgs
  nav_msgs
  rclcpp_action
  pluginlib
)

add_executable(backend_test_node src/TestNode.cpp)

ament_target_dependencies(backend_test_node
  rclcpp
  rclcpp_action
  nav2_msgs
  nlohmann_json
  nav_msgs
  sensor_msgs
  std_srvs
  tf2_ros
  geometry_msgs
)

install(DIRECTORY
  launch
  config
  maps
  DESTINATION share/${PROJECT_NAME}
)

# Memastikan XML diinstal
install(FILES plugin_description.xml
  DESTINATION share/${PROJECT_NAME}
)

# Instalasi target dengan nama yang benar
install(TARGETS selectable_planner
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION bin
)

install(TARGETS backend_test_node
  DESTINATION lib/${PROJECT_NAME}
)

pluginlib_export_plugin_description_file(nav2_core plugin_description.xml)
ament_package()