#include "rclcpp/rclcpp.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "nav2_msgs/action/compute_path_to_pose.hpp"
#include "nav2_msgs/action/follow_path.hpp"
#include "nav2_msgs/srv/load_map.hpp"
#include "nav_msgs/msg/path.hpp"
#include "std_msgs/msg/string.hpp"
#include <nlohmann/json.hpp> 

using json = nlohmann::json;

class TestNode : public rclcpp::Node {
public:
    TestNode() : Node("backend_test_node") {
        compute_path_client_ = rclcpp_action::create_client<nav2_msgs::action::ComputePathToPose>(this, "compute_path_to_pose");
        follow_path_client_ = rclcpp_action::create_client<nav2_msgs::action::FollowPath>(this, "follow_path");
        
        ui_log_pub_ = this->create_publisher<std_msgs::msg::String>("/planner_log", 10);
        
        obs_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/obstacles", 10, std::bind(&TestNode::obstacle_callback, this, std::placeholders::_1));
            
        RCLCPP_INFO(this->get_logger(), "=== Backend Siap. Mode: Diagnostic ===");
    }

private:
    rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SharedPtr compute_path_client_;
    rclcpp_action::Client<nav2_msgs::action::FollowPath>::SharedPtr follow_path_client_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr obs_sub_;
    rclcpp::Publisher<std_msgs::msg::String>::SharedPtr ui_log_pub_;

    void obstacle_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json payload = json::parse(msg->data);
            
            // 1. Validasi Input
            if (!payload.contains("goal") || !payload.contains("algorithm")) {
                RCLCPP_ERROR(this->get_logger(), "JSON Format Salah!");
                return;
            }

            auto goal_msg = nav2_msgs::action::ComputePathToPose::Goal();
            
            // Setup Titik Goal
            goal_msg.goal.header.frame_id = "map";
            goal_msg.goal.header.stamp = this->now();
            goal_msg.goal.pose.position.x = payload["goal"]["x"].get<double>();
            goal_msg.goal.pose.position.y = payload["goal"]["y"].get<double>();
            goal_msg.goal.pose.orientation.w = 1.0;

            // Setup Titik Start
            if (payload.contains("start")) {
                goal_msg.use_start = true;
                goal_msg.start.header.frame_id = "map";
                goal_msg.start.header.stamp = this->now();
                goal_msg.start.pose.position.x = payload["start"]["x"].get<double>();
                goal_msg.start.pose.position.y = payload["start"]["y"].get<double>();
                goal_msg.start.pose.orientation.w = 1.0;
            }

            goal_msg.planner_id = payload["algorithm"].get<std::string>(); 

            RCLCPP_INFO(this->get_logger(), "REQUEST: Path dari (%f, %f) ke (%f, %f) pakai %s", 
                goal_msg.start.pose.position.x, goal_msg.start.pose.position.y,
                goal_msg.goal.pose.position.x, goal_msg.goal.pose.position.y, 
                goal_msg.planner_id.c_str());

            auto send_options = rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SendGoalOptions();
            
            // 2. Cek apakah Goal DITERIMA
            send_options.goal_response_callback = [this](auto handle) {
                if (!handle) publish_ui_error("[CRITICAL] Planner menolak goal!");
            };

            // 3. Cek hasil planning
            send_options.result_callback = [this](const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::WrappedResult & res) {
                if (res.code == rclcpp_action::ResultCode::SUCCEEDED) {
                    if (res.result->path.poses.empty()) {
                        publish_ui_error("[ERROR] Path kosong (Planner Gagal)!");
                    } else {
                        publish_ui_error("[INFO] Path OK. Eksekusi!");
                        execute_path(res.result->path);
                    }
                } else {
                    publish_ui_error("[ERROR] Planning Gagal (Code: " + std::to_string((int)res.code) + ")");
                }
            };
            
            compute_path_client_->async_send_goal(goal_msg, send_options);
        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error Parsing: %s", e.what());
        }
    }

    void execute_path(const nav_msgs::msg::Path& path) {
        auto follow_goal = nav2_msgs::action::FollowPath::Goal();
        follow_goal.path = path;
        follow_goal.controller_id = "FollowPath";
        follow_path_client_->async_send_goal(follow_goal, {});
    }

    void publish_ui_error(const std::string& msg) {
        RCLCPP_INFO(this->get_logger(), "LOG: %s", msg.c_str());
        std_msgs::msg::String log_msg; log_msg.data = msg; ui_log_pub_->publish(log_msg);
    }
};

int main(int argc, char * argv[]) {
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<TestNode>());
    rclcpp::shutdown();
    return 0;
}