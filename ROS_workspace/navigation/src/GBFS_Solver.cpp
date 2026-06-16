#include <GBFS_Solver.hpp>
#include <queue>
#include <unordered_map>
#include <cmath>
#include <algorithm>
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
    std::vector<unsigned int> total_path;

    using Pair = std::pair<float, unsigned int>;
    std::priority_queue<Pair, std::vector<Pair>, std::greater<Pair>> open_set;
    
    std::unordered_map<unsigned int, unsigned int> came_from;
    std::unordered_map<unsigned int, bool> closed_set;

    open_set.push({calculate_h(start_idx, goal_idx), start_idx});

    bool path_found = false;

    while (!open_set.empty())
    {
        unsigned int current = open_set.top().second;
        open_set.pop();

        if (current == goal_idx)
        {
            path_found = true;
            break;
        }

        if (closed_set.find(current) != closed_set.end())
        {
            continue;
        }
        closed_set[current] = true;

        unsigned int cx, cy;
        costmap_->indexToCells(current, cx, cy);

        for (int dx = -1; dx <= 1; ++dx)
        {
            for (int dy = -1; dy <= 1; ++dy)
            {
                if (dx == 0 && dy == 0)
                {
                    continue;
                }

                unsigned int nx = cx + dx;
                unsigned int ny = cy + dy;

                if (nx >= costmap_->getSizeInCellsX() || ny >= costmap_->getSizeInCellsY())
                {
                    continue;
                }

                unsigned int neighbor = costmap_->getIndex(nx, ny);

                if (costmap_->getCost(neighbor) >= nav2_costmap_2d::LETHAL_OBSTACLE)
                {
                    continue;
                }

                if (closed_set.find(neighbor) != closed_set.end())
                {
                    continue;
                }

                if (came_from.find(neighbor) == came_from.end())
                {
                    came_from[neighbor] = current;
                    float h_score = calculate_h(neighbor, goal_idx);
                    open_set.push({h_score, neighbor});
                }
            }
        }
    }

    if (path_found)
    {
        unsigned int curr = goal_idx;
        while (curr != start_idx)
        {
            total_path.push_back(curr);
            curr = came_from[curr];
        }
        total_path.push_back(start_idx);
        std::reverse(total_path.begin(), total_path.end());
    }

    return total_path;
}