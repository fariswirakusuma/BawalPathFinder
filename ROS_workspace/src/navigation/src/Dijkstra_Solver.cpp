#include <Dijkstra_Solver.hpp>
#include <limits>
#include <algorithm>
#include <nlohmann/json.hpp> 

Dijkstra_Solver::Dijkstra_Solver(nav2_costmap_2d::Costmap2D* costmap,rclcpp::Publisher<std_msgs::msg::String>::SharedPtr pub) : PathFinder(costmap,pub) {}

std::vector<unsigned int> Dijkstra_Solver::createPath(unsigned int start_idx, unsigned int goal_idx) {
    unsigned int map_size = nx_ * ny_;
    
    std::vector<float> g_costs(map_size, std::numeric_limits<float>::infinity());
    std::vector<int> parents(map_size, -1);
    std::vector<bool> closed_set(map_size, false);
    std::priority_queue<GridNode, std::vector<GridNode>, std::greater<GridNode>> open_set;

    g_costs[start_idx] = 0.0f;
    open_set.push({start_idx, 0.0f}); 

    bool found = false;

    calculation_history_.emplace_back(start_idx, 0.0f, 0.0f, 0.0f);

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

            unsigned int cx, cy, nx, ny;
            costmap_->indexToCells(current_idx, cx, cy);
            costmap_->indexToCells(neighbor_idx, nx, ny);
            
            float step_cost = (cx != nx && cy != ny) ? 1.414f : 1.0f;
            float map_penalty = static_cast<float>(costmap_->getCost(nx, ny)) / 50.0f; 
            float tentative_g = g_costs[current_idx] + step_cost + map_penalty;

            if (tentative_g < g_costs[neighbor_idx]) {
                parents[neighbor_idx] = current_idx;
                g_costs[neighbor_idx] = tentative_g;
                
                float f_cost = tentative_g; 

                nlohmann::json j;
                j["index"] = (unsigned int)neighbor_idx;
                j["f"] = (float)f_cost;
                j["g"] = (float)tentative_g;
                j["h"] = 0.0f;
                
                std_msgs::msg::String msg;
                msg.data = j.dump();
                if (log_pub_) log_pub_->publish(msg);
                
                open_set.push(GridNode(neighbor_idx, f_cost, tentative_g, 0.0f));
                calculation_history_.push_back({neighbor_idx, f_cost, tentative_g, 0.0f});
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