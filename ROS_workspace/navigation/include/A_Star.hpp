#pragma once
#include "PathFinder.hpp"

class A_Star_Solver : public PathFinder {
private:
    float calculate_h(unsigned int current_idx, unsigned int goal_idx);

public:
    A_Star_Solver(nav2_costmap_2d::Costmap2D* costmap);
    std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) override;
};