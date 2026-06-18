#include "SelectablePlanner.hpp"
#include "nav2_util/node_utils.hpp"
#include <pluginlib/class_list_macros.hpp>
#include "A_Star_Solver.hpp"
#include "Dijkstra_Solver.hpp"
#include "GBFS_Solver.hpp"

namespace nav2planner
{

void SelectablePlanner::configure(
  const rclcpp_lifecycle::LifecycleNode::WeakPtr & parent,
  std::string name, std::shared_ptr<tf2_ros::Buffer> tf,
  std::shared_ptr<nav2_costmap_2d::Costmap2DROS> costmap_ros)
{
  auto node = parent.lock();
  name_ = name;
  costmap_ros_ = costmap_ros;
  costmap_ = costmap_ros_->getCostmap();
  global_frame_ = costmap_ros_->getGlobalFrameID();

  (void)tf;

  nav2_util::declare_parameter_if_not_declared(
    node, name_ + ".algorithm", rclcpp::ParameterValue("A_STAR"));
  
  node->get_parameter(name_ + ".algorithm", selected_algorithm_);
  
  RCLCPP_INFO(node->get_logger(), "SelectablePlanner dikonfigurasi. Algoritma aktif: %s", selected_algorithm_.c_str());
}

void SelectablePlanner::cleanup() {}
void SelectablePlanner::activate() {}
void SelectablePlanner::deactivate() {}

nav_msgs::msg::Path SelectablePlanner::createPlan(
  const geometry_msgs::msg::PoseStamped & start,
  const geometry_msgs::msg::PoseStamped & goal)
{
  nav_msgs::msg::Path final_path;
  final_path.header.stamp = start.header.stamp;
  final_path.header.frame_id = global_frame_;

  unsigned int mx_start, my_start, mx_goal, my_goal;
  if (!costmap_->worldToMap(start.pose.position.x, start.pose.position.y, mx_start, my_start) ||
      !costmap_->worldToMap(goal.pose.position.x, goal.pose.position.y, mx_goal, my_goal)) {
      RCLCPP_ERROR(rclcpp::get_logger("SelectablePlanner"), "Koordinat Start atau Goal berada di luar batas Costmap!");
      return final_path;
  }

  unsigned int start_idx = costmap_->getIndex(mx_start, my_start);
  unsigned int goal_idx = costmap_->getIndex(mx_goal, my_goal);
  std::unique_ptr<PathFinder> solver;

  if (selected_algorithm_ == "A_STAR") {
      solver = std::make_unique<A_Star_Solver>(costmap_);
  } else if (selected_algorithm_ == "UCS" || selected_algorithm_ == "DP") {
      solver = std::make_unique<Dijkstra_Solver>(costmap_);
  } else if (selected_algorithm_ == "GBFS") {
      solver = std::make_unique<GBFS_Solver>(costmap_);
  } else {
      RCLCPP_WARN(rclcpp::get_logger("SelectablePlanner"), 
                  "Algoritma %s tidak dikenali. Jatuh kembali ke A_STAR.", selected_algorithm_.c_str());
      solver = std::make_unique<A_Star_Solver>(costmap_);
  }

  std::vector<unsigned int> path_indices = solver->createPath(start_idx, goal_idx);

  for (unsigned int idx : path_indices) {
      unsigned int mx, my;
      costmap_->indexToCells(idx, mx, my);
      
      double wx, wy;
      costmap_->mapToWorld(mx, my, wx, wy);
      
      geometry_msgs::msg::PoseStamped pose;
      pose.pose.position.x = wx;
      pose.pose.position.y = wy;
      pose.pose.position.z = 0.0;
      pose.pose.orientation.w = 1.0;
      final_path.poses.push_back(pose);
  }

  return final_path;
}

} 

#include "pluginlib/class_list_macros.hpp"
PLUGINLIB_EXPORT_CLASS(nav2planner::SelectablePlanner, nav2_core::GlobalPlanner)