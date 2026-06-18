#pragma once
#include "PathFinder.hpp"

class Dijkstra_Solver : public PathFinder {
public:
    Dijkstra_Solver(nav2_costmap_2d::Costmap2D* costmap,rclcpp::Publisher<std_msgs::msg::String>::SharedPtr pub);
    std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) override;
};