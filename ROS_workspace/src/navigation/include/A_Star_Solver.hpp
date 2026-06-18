#pragma once
#include "PathFinder.hpp"

class A_Star_Solver : public PathFinder {
private:
    float calculate_h(unsigned int current_idx, unsigned int goal_idx);
    std::vector<float> g_score_;
    std::vector<float> f_score_;

public:
    A_Star_Solver(nav2_costmap_2d::Costmap2D* costmap);
    std::vector<unsigned int> createPath(unsigned int start_idx, unsigned int goal_idx) override;
    float get_g(unsigned int idx) const { return g_score_[idx]; }
    float get_f(unsigned int idx) const { return f_score_[idx]; }
    float get_h(unsigned int idx) const { return f_score_[idx] - g_score_[idx]; }
};