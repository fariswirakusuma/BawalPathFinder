#pragma once
#include "PathFinder.hpp"

class Djikstra_Solver : public PathFinder {
public:
    Djikstra_Solver(nav2_costmap_2d::Costmap2D* costmap);
    std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) override;
};