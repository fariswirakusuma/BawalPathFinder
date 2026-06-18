#pragma once

#include <vector>
#include <queue>
#include "nav2_costmap_2d/costmap_2d.hpp"

struct StepLog {
    unsigned int index;
    float f;
    float g;
    float h;
    StepLog(unsigned int idx, float f_val, float g_val, float h_val) 
        : index(idx), f(f_val), g(g_val), h(h_val) {}
};

struct GridNode {
    unsigned int index;
    float f_cost;
    float g_cost;
    float h_cost;

    GridNode(unsigned int idx, float f, float g = 0.0, float h = 0.0) 
        : index(idx), f_cost(f), g_cost(g), h_cost(h) {}

    float get_f() const { return f_cost; }
    float get_g() const { return g_cost; }
    float get_h() const { return h_cost; }
    unsigned int get_index() const { return index; }

    bool operator>(const GridNode& other) const {
        if (f_cost == other.f_cost) {
            return h_cost > other.h_cost; 
        }
        return f_cost > other.f_cost;
    }
};

class PathFinder {
protected:
    nav2_costmap_2d::Costmap2D* costmap_;
    unsigned int nx_, ny_;
    
    std::vector<StepLog> calculation_history_;

    std::vector<unsigned int> get_neighbors(unsigned int current_idx);

public:
    PathFinder(nav2_costmap_2d::Costmap2D* costmap);
    virtual ~PathFinder() = default;
    
    virtual std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) = 0;
    
    const std::vector<StepLog>& get_calculation_history() const {
        return calculation_history_;
    }
};