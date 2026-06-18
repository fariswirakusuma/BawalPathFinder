#include <PathFinder.hpp>
#include "nav2_costmap_2d/cost_values.hpp"

PathFinder::PathFinder(nav2_costmap_2d::Costmap2D* costmap, rclcpp::Publisher<std_msgs::msg::String>::SharedPtr pub) 
    : costmap_(costmap), log_pub_(pub) { 
    nx_ = costmap_->getSizeInCellsX();
    ny_ = costmap_->getSizeInCellsY();
}

std::vector<unsigned int> PathFinder::get_neighbors(unsigned int current_idx) {
    std::vector<unsigned int> neighbors;
    unsigned int mx, my;
    costmap_->indexToCells(current_idx, mx, my);

    for (int dx = -1; dx <= 1; ++dx) {
        for (int dy = -1; dy <= 1; ++dy) {
            if (dx == 0 && dy == 0) continue;

            int nx = mx + dx;
            int ny = my + dy;

            if (nx >= 0 && nx < (int)nx_ && ny >= 0 && ny < (int)ny_) {
                unsigned char cost = costmap_->getCost(nx, ny);
                if (cost != nav2_costmap_2d::LETHAL_OBSTACLE && 
                    cost != nav2_costmap_2d::NO_INFORMATION &&
                    cost != nav2_costmap_2d::INSCRIBED_INFLATED_OBSTACLE) 
                {
                    neighbors.push_back(costmap_->getIndex(nx, ny));
                }
            }
        }
    }
    return neighbors;
}