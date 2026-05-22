#include "rclcpp/rclcpp.hpp"
#include "nav2_msgs/action/compute_path_to_pose.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include <fstream>
#include <nlohmann/json.hpp> 

using json = nlohmann::json;

class TestNode : public rclcpp::Node {
public:
    TestNode() : Node("backend_test_node") {
        this->client_ = rclcpp_action::create_client<nav2_msgs::action::ComputePathToPose>(
            this, "/compute_path_to_pose");
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
        
        // Memaksa titik awal (start) secara eksplisit agar tidak bergantung pada amcl/transformasi lokalisasi
        goal_msg.use_start = true;
        goal_msg.start.header.frame_id = "map";
        goal_msg.start.pose.position.x = 0.0;
        goal_msg.start.pose.position.y = 0.0;
        goal_msg.start.pose.orientation.w = 1.0;

        RCLCPP_INFO(this->get_logger(), "Mengirim Goal ke (%.2f, %.2f) dari (%.2f, %.2f) di frame %s", 
            goal_msg.goal.pose.position.x, 
            goal_msg.goal.pose.position.y,
            goal_msg.start.pose.position.x,
            goal_msg.start.pose.position.y,
            goal_msg.goal.header.frame_id.c_str());

        auto send_goal_options = rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SendGoalOptions();
        send_goal_options.result_callback = std::bind(&TestNode::result_callback, this, std::placeholders::_1);
        
        this->client_->async_send_goal(goal_msg, send_goal_options);
    }

private:
   void result_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::WrappedResult & result) {
        RCLCPP_INFO(this->get_logger(), "Menerima result dari planner...");
        
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

        json j;
        for (const auto & pose : result.result->path.poses) {
            j.push_back({{"x", pose.pose.position.x}, {"y", pose.pose.position.y}});
        }

        std::string path = "../Test/backendTest/path_result.json";
        std::ofstream file(path);
        
        if (file.is_open()) {
            file << j.dump(4);
            file.close();
            RCLCPP_INFO(this->get_logger(), "Berhasil menyimpan file ke: %s", path.c_str());
        } else {
            RCLCPP_ERROR(this->get_logger(), "Gagal membuka file! Apakah folder Test/backendTest ada?");
        }
    }
    rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SharedPtr client_;
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<TestNode>();
  node->send_goal(); 
  
  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}