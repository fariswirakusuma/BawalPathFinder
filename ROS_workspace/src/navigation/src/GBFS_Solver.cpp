#include <GBFS_Solver.hpp>
#include <queue>
#include <unordered_map>
#include <cmath>
#include <algorithm>
#include <limits>
#include "nav2_costmap_2d/cost_values.hpp"

GBFS_Solver::GBFS_Solver(nav2_costmap_2d::Costmap2D* costmap)
    : PathFinder(costmap)
{
}

float GBFS_Solver::calculate_h(unsigned int current_idx, unsigned int goal_idx)
{
    unsigned int cx, cy, gx, gy;
    costmap_->indexToCells(current_idx, cx, cy);
    costmap_->indexToCells(goal_idx, gx, gy);

    int dx = static_cast<int>(gx) - static_cast<int>(cx);
    int dy = static_cast<int>(gy) - static_cast<int>(cy);
    
    return std::sqrt(dx * dx + dy * dy);
}

std::vector<unsigned int> GBFS_Solver::createPath(unsigned int start_idx, unsigned int goal_idx)
{
    unsigned int map_size = nx_ * ny_;
    std::vector<float> h_costs(map_size, std::numeric_limits<float>::infinity());
    std::vector<int> parents(map_size, -1);
    std::vector<bool> closed_set(map_size, false);
    std::priority_queue<GridNode, std::vector<GridNode>, std::greater<GridNode>> open_set;

    float start_h = calculate_h(start_idx, goal_idx);
    h_costs[start_idx] = start_h;
    open_set.push(GridNode(start_idx, start_h, 0.0f, start_h));

    bool found = false;

    calculation_history_.emplace_back(start_idx, start_h, 0.0f, start_h);

    while (!open_set.empty()) {
        unsigned int current_idx = open_set.top().index;
        open_set.pop();

        if (current_idx == goal_idx) {
            found = true;
            break;
        }

        if (closed_set[current_idx]) continue;
        closed_set[current_idx] = true;

        for (unsigned int neighbor_idx : get_neighbors(current_idx)) {
            if (closed_set[neighbor_idx]) continue;

            float h_score = calculate_h(neighbor_idx, goal_idx);
            float f_score = h_score;

            if (h_score < h_costs[neighbor_idx]) {
                parents[neighbor_idx] = current_idx;
                h_costs[neighbor_idx] = h_score;
                open_set.push(GridNode(neighbor_idx, f_score, 0.0f, h_score));
                calculation_history_.push_back({neighbor_idx, f_score, 0.0f, h_score});
            }
        }
    }

    std::vector<unsigned int> path;
    if (found) {
        int current = goal_idx;
        while (current != -1) {
            path.push_back(current);
            current = parents[current];
        }
        std::reverse(path.begin(), path.end());
    }
    return path;
}