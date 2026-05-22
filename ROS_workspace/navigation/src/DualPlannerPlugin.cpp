#include <DualPlannerPlugin.hpp>
#include "pluginlib/class_list_macros.hpp"
#include "rclcpp/rclcpp.hpp"
#include "nav2_costmap_2d/cost_values.hpp"

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

  RCLCPP_INFO(rclcpp::get_logger("DualPlannerPlugin"), "DualPlannerPlugin diinisialisasi.");
}

void DualPlannerPlugin::cleanup() {}
void DualPlannerPlugin::activate() {}
void DualPlannerPlugin::deactivate() {}

nav_msgs::msg::Path DualPlannerPlugin::createPlan(
  const geometry_msgs::msg::PoseStamped & start,
  const geometry_msgs::msg::PoseStamped & goal)
{
  rclcpp::Logger logger = rclcpp::get_logger("DualPlannerPlugin");

  nav_msgs::msg::Path path;
  path.poses.clear();
  path.header.stamp = rclcpp::Clock().now();
  path.header.frame_id = global_frame_;

  unsigned int start_mx, start_my, goal_mx, goal_my;
  
  // 1. Validasi Transformasi World ke Map
  if (!costmap_->worldToMap(start.pose.position.x, start.pose.position.y, start_mx, start_my)) {
    RCLCPP_ERROR(logger, "Start point (%.2f, %.2f) di luar batas costmap.", start.pose.position.x, start.pose.position.y);
    return path; 
  }
  if (!costmap_->worldToMap(goal.pose.position.x, goal.pose.position.y, goal_mx, goal_my)) {
    RCLCPP_ERROR(logger, "Goal point (%.2f, %.2f) di luar batas costmap.", goal.pose.position.x, goal.pose.position.y);
    return path; 
  }

  // 2. Validasi Nilai Costmap (Lethal Space / Rintangan)
  unsigned char start_cost = costmap_->getCost(start_mx, start_my);
  unsigned char goal_cost = costmap_->getCost(goal_mx, goal_my);

  if (start_cost == nav2_costmap_2d::LETHAL_OBSTACLE || start_cost == nav2_costmap_2d::NO_INFORMATION) {
    RCLCPP_ERROR(logger, "Start position berada di rintangan atau area unknown (Cost: %d).", start_cost);
    return path;
  }
  if (goal_cost == nav2_costmap_2d::LETHAL_OBSTACLE || goal_cost == nav2_costmap_2d::NO_INFORMATION) {
    RCLCPP_ERROR(logger, "Goal position berada di rintangan atau area unknown (Cost: %d).", goal_cost);
    return path;
  }

  unsigned int start_idx = costmap_->getIndex(start_mx, start_my);
  unsigned int goal_idx = costmap_->getIndex(goal_mx, goal_my);

  RCLCPP_INFO(logger, "Kalkulasi path dari index %u ke %u menggunakan A*...", start_idx, goal_idx);

  // 3. Eksekusi Algoritma
  A_Star_Solver astar(costmap_); 
  std::vector<unsigned int> raw_path = astar.createPath(start_idx, goal_idx);

  if (raw_path.empty()) {
      RCLCPP_WARN(logger, "A* gagal menemukan path. Fallback ke Dijkstra.");
      Djikstra_Solver dijkstra(costmap_);
      raw_path = dijkstra.createPath(start_idx, goal_idx);
  }

  if (raw_path.empty()) {
      RCLCPP_ERROR(logger, "Dijkstra gagal. Tidak ada path yang tersedia dari start ke goal.");
      return path;
  }

  // 4. Konstruksi Path
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

  RCLCPP_INFO(logger, "Path sukses dibuat dengan panjang %zu poses.", path.poses.size());
  return path;
}

} // namespace nav2planner

PLUGINLIB_EXPORT_CLASS(nav2planner::DualPlannerPlugin, nav2_core::GlobalPlanner)