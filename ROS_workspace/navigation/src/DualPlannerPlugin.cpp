#include <DualPlannerPlugin.hpp>
#include "pluginlib/class_list_macros.hpp"

namespace nav2planner
{

void DualPlannerPlugin::configure(
  const rclcpp_lifecycle::LifecycleNode::WeakPtr & parent,
  std::string name, std::shared_ptr<tf2_ros::Buffer> tf,
  std::shared_ptr<nav2_costmap_2d::Costmap2DROS> costmap_ros)
{
  (void)parent;
  (void)tf;
  name_ = name;
  costmap_ros_ = costmap_ros;
  costmap_ = costmap_ros_->getCostmap();
  global_frame_ = costmap_ros_->getGlobalFrameID();
}

void DualPlannerPlugin::cleanup() {}
void DualPlannerPlugin::activate() {}
void DualPlannerPlugin::deactivate() {}

nav_msgs::msg::Path DualPlannerPlugin::createPlan(
  const geometry_msgs::msg::PoseStamped & start,
  const geometry_msgs::msg::PoseStamped & goal)
{
  nav_msgs::msg::Path path;
  path.poses.clear();
  path.header.stamp = rclcpp::Clock().now();
  path.header.frame_id = global_frame_;

  unsigned int start_mx, start_my, goal_mx, goal_my;
  if (!costmap_->worldToMap(start.pose.position.x, start.pose.position.y, start_mx, start_my) ||
      !costmap_->worldToMap(goal.pose.position.x, goal.pose.position.y, goal_mx, goal_my))
  {
    return path; // Di luar map
  }

  unsigned int start_idx = costmap_->getIndex(start_mx, start_my);
  unsigned int goal_idx = costmap_->getIndex(goal_mx, goal_my);

  A_Star_Solver astar(costmap_); 
  std::vector<unsigned int> raw_path = astar.createPath(start_idx, goal_idx);

  if (raw_path.empty()) {
      Djikstra_Solver dijkstra(costmap_);
      raw_path = dijkstra.createPath(start_idx, goal_idx);
  }

  for (unsigned int idx : raw_path) {
      unsigned int mx, my;
      costmap_->indexToCells(idx, mx, my);
      
      double wx, wy;
      costmap_->mapToWorld(mx, my, wx, wy);
      
      geometry_msgs::msg::PoseStamped pose;
      pose.header.frame_id = global_frame_;
      pose.header.stamp = path.header.stamp;
      pose.pose.position.x = wx;
      pose.pose.position.y = wy;
      pose.pose.orientation.w = 1.0; 
      
      path.poses.push_back(pose);
  }

  return path;
}

} // namespace nav2planner

PLUGINLIB_EXPORT_CLASS(nav2planner::DualPlannerPlugin, nav2_core::GlobalPlanner)