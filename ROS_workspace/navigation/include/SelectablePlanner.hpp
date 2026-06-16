#ifndef SELECTABLE_PLANNER_HPP_
#define SELECTABLE_PLANNER_HPP_

#include <string>
#include <memory>
#include "rclcpp/rclcpp.hpp"
#include "geometry_msgs/msg/pose_stamped.hpp"
#include "nav2_core/global_planner.hpp"
#include "nav_msgs/msg/path.hpp"
#include "nav2_util/lifecycle_node.hpp"
#include "nav2_costmap_2d/costmap_2d_ros.hpp"

namespace navigation
{

class SelectablePlanner : public nav2_core::GlobalPlanner
{
public:
  SelectablePlanner() = default;
  ~SelectablePlanner() = default;

  void configure(
    const rclcpp_lifecycle::LifecycleNode::WeakPtr & parent,
    std::string name, std::shared_ptr<tf2_ros::Buffer> tf,
    std::shared_ptr<nav2_costmap_2d::Costmap2DROS> costmap_ros) override;

  void cleanup() override;
  void activate() override;
  void deactivate() override;

  nav_msgs::msg::Path createPlan(
    const geometry_msgs::msg::PoseStamped & start,
    const geometry_msgs::msg::PoseStamped & goal) override;

private:
  std::shared_ptr<nav2_costmap_2d::Costmap2DROS> costmap_ros_;
  
  nav2_costmap_2d::Costmap2D * costmap_;
  
  std::string global_frame_;
  std::string name_;
  
  std::string selected_algorithm_;
};

}  

#endif 