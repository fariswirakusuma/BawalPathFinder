#include "rclcpp/rclcpp.hpp"
#include "nav2_msgs/action/compute_path_to_pose.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "nav_msgs/msg/occupancy_grid.hpp"
#include "std_msgs/msg/string.hpp"
#include "tf2_ros/transform_broadcaster.h"
#include "geometry_msgs/msg/transform_stamped.hpp"
#include <nlohmann/json.hpp> 
#include <cmath>
#include <vector>

using json = nlohmann::json;

class TestNode : public rclcpp::Node {
public:
    TestNode() : Node("backend_test_node"), current_x_(0.0), current_y_(0.0) {
        this->client_ = rclcpp_action::create_client<nav2_msgs::action::ComputePathToPose>(
            this, "/compute_path_to_pose");
            
        rclcpp::QoS map_qos(rclcpp::KeepLast(1));
        map_qos.transient_local();
        this->map_pub_ = this->create_publisher<nav_msgs::msg::OccupancyGrid>("/map", map_qos);
        
        this->obs_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/obstacles", 10,
            std::bind(&TestNode::obstacle_callback, this, std::placeholders::_1));

        this->init_pose_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/initialpose", 10,
            std::bind(&TestNode::initialpose_callback, this, std::placeholders::_1));

        this->tf_broadcaster_ = std::make_unique<tf2_ros::TransformBroadcaster>(*this);
        this->timer_ = this->create_wall_timer(
            std::chrono::milliseconds(50), std::bind(&TestNode::broadcast_tf, this));
            
        RCLCPP_INFO(this->get_logger(), "Backend Test Node aktif. Menggunakan OccupancyGrid & Dynamic Planner.");
    }

    void send_goal(double goal_x, double goal_y, const std::string& planner_id) {
        if (!this->client_->wait_for_action_server(std::chrono::seconds(5))) {
            RCLCPP_ERROR(this->get_logger(), "Action server tidak ditemukan!");
            return;
        }

        auto goal_msg = nav2_msgs::action::ComputePathToPose::Goal();
        goal_msg.goal.header.frame_id = "map";
        goal_msg.goal.pose.position.x = goal_x; 
        goal_msg.goal.pose.position.y = goal_y;
        goal_msg.planner_id = planner_id; 
        
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
        t.transform.rotation.w = 1.0;
        tf_broadcaster_->sendTransform(t);
    }

    void initialpose_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json pose = json::parse(msg->data);
            current_x_ = pose["x"];
            current_y_ = pose["y"];
        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error parsing JSON initialpose: %s", e.what());
        }
    }

    void obstacle_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json payload = json::parse(msg->data);
            auto map_size = payload["map_size"];
            
            // Konversi dimensi ke grid cell
            double res = map_size["resolution"];
            int width = static_cast<int>(map_size["width"].get<double>() / res);
            int height = static_cast<int>(map_size["height"].get<double>() / res);

            auto grid = nav_msgs::msg::OccupancyGrid();
            grid.header.stamp = this->now();
            grid.header.frame_id = "map";
            grid.info.resolution = res;
            grid.info.width = width;
            grid.info.height = height;
            
            // Centering map
            grid.info.origin.position.x = -(map_size["width"].get<double>() / 2.0);
            grid.info.origin.position.y = -(map_size["height"].get<double>() / 2.0);
            grid.info.origin.orientation.w = 1.0;

            grid.data.assign(width * height, 0); 

            // Plot rintangan
            for (const auto& obs : payload["obstacles"]) {
                double ox = obs["x"];
                double oy = obs["y"];
                int mx = static_cast<int>((ox - grid.info.origin.position.x) / res);
                int my = static_cast<int>((oy - grid.info.origin.position.y) / res);

                if (mx >= 0 && mx < width && my >= 0 && my < height) {
                    grid.data[my * width + mx] = 100; 
                }
            }

            this->map_pub_->publish(grid);
            
            std::string algo = payload["algorithm"];
            this->send_goal(payload["goal"]["x"], payload["goal"]["y"], algo);

        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error parsing payload: %s", e.what());
        }
    }

    void goal_response_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::SharedPtr & gh) {
        if (!gh) RCLCPP_ERROR(this->get_logger(), "Goal ditolak!");
    }

    void result_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::WrappedResult & res) {
        if (res.code != rclcpp_action::ResultCode::SUCCEEDED) return;
        RCLCPP_INFO(this->get_logger(), "Rute dikalkulasi.");
    }

    rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SharedPtr client_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr obs_sub_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr init_pose_sub_;
    rclcpp::Publisher<nav_msgs::msg::OccupancyGrid>::SharedPtr map_pub_;
    std::unique_ptr<tf2_ros::TransformBroadcaster> tf_broadcaster_;
    rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char * argv[]) {
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<TestNode>());
  rclcpp::shutdown();
  return 0;
}