#include "rclcpp/rclcpp.hpp"
#include "nav2_msgs/action/compute_path_to_pose.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "sensor_msgs/msg/laser_scan.hpp" 
#include "std_msgs/msg/string.hpp"
#include <nlohmann/json.hpp> 
#include <cstdlib>
#include <cmath>

using json = nlohmann::json;

class TestNode : public rclcpp::Node {
public:
    TestNode() : Node("backend_test_node") {
        this->client_ = rclcpp_action::create_client<nav2_msgs::action::ComputePathToPose>(
            this, "/compute_path_to_pose");
            
        this->scan_pub_ = this->create_publisher<sensor_msgs::msg::LaserScan>("/scan", 10);
        
        this->obs_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/obstacles", 10,
            std::bind(&TestNode::obstacle_callback, this, std::placeholders::_1));
            
        RCLCPP_INFO(this->get_logger(), "Backend Test Node aktif. Menunggu data dari frontend via ROSBridge...");
    }

    void send_goal() {
        if (!this->client_->wait_for_action_server(std::chrono::seconds(5))) {
            RCLCPP_ERROR(this->get_logger(), "Action server tidak ditemukan!");
            return;
        }

        auto goal_msg = nav2_msgs::action::ComputePathToPose::Goal();
        goal_msg.goal.header.frame_id = "map";
        goal_msg.goal.pose.position.x = 0.5; 
        goal_msg.goal.pose.position.y = 0.5;
        goal_msg.planner_id = "GridBased";
        
        goal_msg.use_start = true;
        goal_msg.start.header.frame_id = "map";
        goal_msg.start.pose.position.x = 0.0;
        goal_msg.start.pose.position.y = 0.0;
        goal_msg.start.pose.orientation.w = 1.0;

        auto send_goal_options = rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SendGoalOptions();
        send_goal_options.goal_response_callback = std::bind(&TestNode::goal_response_callback, this, std::placeholders::_1);
        send_goal_options.result_callback = std::bind(&TestNode::result_callback, this, std::placeholders::_1);
        
        this->client_->async_send_goal(goal_msg, send_goal_options);
    }

private:
    void obstacle_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json obstacles = json::parse(msg->data);
            
            auto scan = sensor_msgs::msg::LaserScan();
            scan.header.stamp = this->now();
            scan.header.frame_id = "map";
            scan.angle_min = -3.14;
            scan.angle_max = 3.14;
            scan.angle_increment = 0.01;
            scan.range_min = 0.0;
            scan.range_max = 10.0;

            std::vector<float> ranges(629, 10.0);
            for (const auto& obs : obstacles) {
                float x = obs["x"];
                float y = obs["y"];
                float dist = std::sqrt(x*x + y*y);
                if (dist < 10.0) {
                    int index = static_cast<int>((std::atan2(y, x) + 3.14) / 0.01);
                    if (index >= 0 && index < 629) ranges[index] = dist;
                }
            }
            scan.ranges = ranges;
            this->scan_pub_->publish(scan);
            RCLCPP_INFO(this->get_logger(), "Rintangan dari frontend disuntikkan ke /scan.");

            // Panggil send_goal secara dinamis setiap ada pembaruan rintangan
            this->send_goal();

        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error parsing JSON dari frontend: %s", e.what());
        }
    }

    void goal_response_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::SharedPtr & goal_handle) {
        if (!goal_handle) {
            RCLCPP_ERROR(this->get_logger(), "Goal ditolak oleh server!");
        } else {
            this->goal_handle_ = goal_handle;
        }
    }

    void result_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::WrappedResult & result) {
        if (result.code != rclcpp_action::ResultCode::SUCCEEDED) {
            std::string status_str;
            switch (result.code) {
                case rclcpp_action::ResultCode::ABORTED: status_str = "ABORTED"; break;
                case rclcpp_action::ResultCode::CANCELED: status_str = "CANCELED"; break;
                default: status_str = "UNKNOWN"; break;
            }
            RCLCPP_ERROR(this->get_logger(), "Gagal kalkulasi rute! Kode: %s", status_str.c_str());
            return;
        }

        RCLCPP_INFO(this->get_logger(), "Kalkulasi path selesai. Hasil dipublikasikan otomatis ke /plan oleh Nav2.");
    }

    rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SharedPtr client_;
    rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::SharedPtr goal_handle_; 
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr obs_sub_;
    rclcpp::Publisher<sensor_msgs::msg::LaserScan>::SharedPtr scan_pub_;
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<TestNode>();
  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}