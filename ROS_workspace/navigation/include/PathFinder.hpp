#pragma once

#include <vector>
#include <queue>
#include "nav2_costmap_2d/costmap_2d.hpp"
struct GridNode {
    unsigned int index;
    float f_cost;

    bool operator>(const GridNode& other) const {
        return f_cost > other.f_cost;
    }
};

class PathFinder {
protected:
    nav2_costmap_2d::Costmap2D* costmap_;
    unsigned int nx_, ny_;

    std::vector<unsigned int> get_neighbors(unsigned int current_idx);

public:
    PathFinder(nav2_costmap_2d::Costmap2D* costmap);
    virtual ~PathFinder() = default;
    virtual std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) = 0;
};