#include "rclcpp/rclcpp.hpp"
#include "nav2_msgs/action/compute_path_to_pose.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "sensor_msgs/msg/laser_scan.hpp" 
#include "std_msgs/msg/string.hpp"
#include "tf2_ros/transform_broadcaster.h"
#include "geometry_msgs/msg/transform_stamped.hpp"
#include <nlohmann/json.hpp> 
#include <cstdlib>
#include <cmath>

using json = nlohmann::json;

class TestNode : public rclcpp::Node {
public:
    TestNode() : Node("backend_test_node"), current_x_(0.0), current_y_(0.0) {
        this->client_ = rclcpp_action::create_client<nav2_msgs::action::ComputePathToPose>(
            this, "/compute_path_to_pose");
            
        this->scan_pub_ = this->create_publisher<sensor_msgs::msg::LaserScan>("/scan", 10);
        
        // Subscriber untuk menerima rintangan peta
        this->obs_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/obstacles", 10,
            std::bind(&TestNode::obstacle_callback, this, std::placeholders::_1));

        // Subscriber untuk menerima inisialisasi posisi awal dari Bevy
        this->init_pose_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/initialpose", 10,
            std::bind(&TestNode::initialpose_callback, this, std::placeholders::_1));

        // Inisialisasi TF Broadcaster
        this->tf_broadcaster_ = std::make_unique<tf2_ros::TransformBroadcaster>(*this);
        
        // Timer untuk mempublikasikan TF odom -> base_footprint secara terus-menerus pada 20Hz (50ms)
        this->timer_ = this->create_wall_timer(
            std::chrono::milliseconds(50), std::bind(&TestNode::broadcast_tf, this));
            
        RCLCPP_INFO(this->get_logger(), "Backend Test Node aktif. Menunggu inisialisasi posisi dan peta dari Bevy...");
    }

    void send_goal(double goal_x, double goal_y) {
        if (!this->client_->wait_for_action_server(std::chrono::seconds(5))) {
            RCLCPP_ERROR(this->get_logger(), "Action server tidak ditemukan!");
            return;
        }

        auto goal_msg = nav2_msgs::action::ComputePathToPose::Goal();
        goal_msg.goal.header.frame_id = "map";
        goal_msg.goal.pose.position.x = goal_x; 
        goal_msg.goal.pose.position.y = goal_y;
        goal_msg.planner_id = "GridBased";
        
        // Gunakan current_x_ dan current_y_ yang disetel dari UI sebagai titik mulai kalkulasi
        goal_msg.use_start = true;
        goal_msg.start.header.frame_id = "map";
        goal_msg.start.pose.position.x = current_x_;
        goal_msg.start.pose.position.y = current_y_;
        goal_msg.start.pose.orientation.w = 1.0;

        auto send_goal_options = rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SendGoalOptions();
        send_goal_options.goal_response_callback = std::bind(&TestNode::goal_response_callback, this, std::placeholders::_1);
        send_goal_options.result_callback = std::bind(&TestNode::result_callback, this, std::placeholders::_1);
        
        this->client_->async_send_goal(goal_msg, send_goal_options);
    }

private:
    double current_x_;
    double current_y_;

    void broadcast_tf() {
        geometry_msgs::msg::TransformStamped t;
        t.header.stamp = this->now();
        t.header.frame_id = "odom";
        t.child_frame_id = "base_footprint";
        t.transform.translation.x = current_x_;
        t.transform.translation.y = current_y_;
        t.transform.translation.z = 0.0;
        
        t.transform.rotation.x = 0.0;
        t.transform.rotation.y = 0.0;
        t.transform.rotation.z = 0.0;
        t.transform.rotation.w = 1.0;

        tf_broadcaster_->sendTransform(t);
    }

    void initialpose_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json pose = json::parse(msg->data);
            current_x_ = pose["x"];
            current_y_ = pose["y"];
            RCLCPP_INFO(this->get_logger(), "Inisialisasi posisi diperbarui ke x: %.2f, y: %.2f", current_x_, current_y_);
        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error parsing JSON initialpose: %s", e.what());
        }
    }

    void obstacle_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json payload = json::parse(msg->data);
            
            // Asumsi struktur JSON sekarang mengirim obstacles dan goal point secara terpisah dalam satu request
            json obstacles = payload["obstacles"];
            double goal_x = payload["goal"]["x"];
            double goal_y = payload["goal"]["y"];
            
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
            
            // Pemicu komputasi dengan goal baru
            this->send_goal(goal_x, goal_y);

        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error parsing JSON obstacle/goal: %s", e.what());
        }
    }

    void goal_response_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::SharedPtr & goal_handle) {
        if (!goal_handle) RCLCPP_ERROR(this->get_logger(), "Goal ditolak!");
    }

    void result_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::WrappedResult & result) {
        if (result.code != rclcpp_action::ResultCode::SUCCEEDED) {
            RCLCPP_ERROR(this->get_logger(), "Gagal kalkulasi rute!");
            return;
        }
        RCLCPP_INFO(this->get_logger(), "Rute dikalkulasi.");
    }

    rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SharedPtr client_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr obs_sub_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr init_pose_sub_;
    rclcpp::Publisher<sensor_msgs::msg::LaserScan>::SharedPtr scan_pub_;
    std::unique_ptr<tf2_ros::TransformBroadcaster> tf_broadcaster_;
    rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<TestNode>();
  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}