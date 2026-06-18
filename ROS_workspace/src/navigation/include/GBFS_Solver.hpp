#pragma once
#include "PathFinder.hpp"

class GBFS_Solver : public PathFinder {
private:
    float calculate_h(unsigned int current_idx, unsigned int goal_idx);

public:
    explicit GBFS_Solver(nav2_costmap_2d::Costmap2D* costmap, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr pub);
    std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) override;
};