#include "rclcpp/rclcpp.hpp"
#include "nav2_msgs/action/compute_path_to_pose.hpp"
#include "nav2_msgs/srv/load_map.hpp"
#include "rclcpp_action/rclcpp_action.hpp"
#include "nav_msgs/msg/path.hpp"
#include "nav_msgs/msg/occupancy_grid.hpp"
#include "std_msgs/msg/string.hpp"
#include "tf2_ros/transform_broadcaster.h"
#include "geometry_msgs/msg/transform_stamped.hpp"
#include "sensor_msgs/msg/point_cloud2.hpp"
#include "sensor_msgs/point_cloud2_iterator.hpp"
#include <nlohmann/json.hpp> 
#include <cmath>
#include <vector>

using json = nlohmann::json;

class TestNode : public rclcpp::Node {
public:
    TestNode() : Node("backend_test_node"), current_x_(-4.0), current_y_(-4.0) {
        this->client_ = rclcpp_action::create_client<nav2_msgs::action::ComputePathToPose>(
            this, "/compute_path_to_pose");
            
        this->load_map_client_ = this->create_client<nav2_msgs::srv::LoadMap>("/map_server/load_map");

        this->plan_pub_ = this->create_publisher<nav_msgs::msg::Path>("/plan", 10);
        this->pc_pub_ = this->create_publisher<sensor_msgs::msg::PointCloud2>("/frontend/pointcloud", 10);
        this->ui_log_pub_ = this->create_publisher<std_msgs::msg::String>("/planner_log", 10);
        
        this->obs_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/obstacles", 10,
            std::bind(&TestNode::obstacle_callback, this, std::placeholders::_1));

        this->init_pose_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/initialpose", 10,
            std::bind(&TestNode::initialpose_callback, this, std::placeholders::_1));

        this->setup_sub_ = this->create_subscription<std_msgs::msg::String>(
            "/frontend/setup", 10,
            std::bind(&TestNode::setup_simulation_callback, this, std::placeholders::_1));

        this->map_sub_ = this->create_subscription<nav_msgs::msg::OccupancyGrid>(
            "/map", rclcpp::QoS(rclcpp::KeepLast(1)).transient_local(),
            [this](const nav_msgs::msg::OccupancyGrid::SharedPtr msg) {
                this->current_map_ = msg;
            });

        this->tf_broadcaster_ = std::make_unique<tf2_ros::TransformBroadcaster>(*this);
        this->timer_ = this->create_wall_timer(
            std::chrono::milliseconds(50), std::bind(&TestNode::broadcast_tf, this));
            
        RCLCPP_INFO(this->get_logger(), "Backend Test Node Online. Menunggu trigger dari UI...");
    }

    void send_goal(double goal_x, double goal_y, const std::string& planner_id) {
        if (is_in_obstacle(current_x_, current_y_)) {
            RCLCPP_ERROR(this->get_logger(), "ABORT: Titik START berada di dalam rintangan atau di luar batas!");
            publish_ui_error("[ERROR] ABORT: Titik START berada di dalam rintangan!");
            return;
        }
        if (is_in_obstacle(goal_x, goal_y)) {
            RCLCPP_ERROR(this->get_logger(), "ABORT: Titik GOAL berada di dalam rintangan atau di luar batas!");
            publish_ui_error("[ERROR] ABORT: Titik GOAL berada di dalam rintangan!");
            return;
        }

        if (!this->client_->wait_for_action_server(std::chrono::seconds(2))) {
            RCLCPP_ERROR(this->get_logger(), "Planner Server Nav2 tidak merespons!");
            publish_ui_error("[ERROR] ABORT: Planner Server Nav2 tidak merespons!");
            return;
        }

        auto goal_msg = nav2_msgs::action::ComputePathToPose::Goal();
        goal_msg.goal.header.frame_id = "map";
        goal_msg.goal.header.stamp = this->now();
        goal_msg.goal.pose.position.x = goal_x; 
        goal_msg.goal.pose.position.y = goal_y;
        
        if (planner_id == "AStar") goal_msg.planner_id = "AStar";
        else if (planner_id == "Dijkstra") goal_msg.planner_id = "Dijkstra";
        else goal_msg.planner_id = "GBFS";
        
        goal_msg.use_start = true;
        goal_msg.start.header.frame_id = "map";
        goal_msg.start.header.stamp = this->now();
        goal_msg.start.pose.position.x = current_x_;
        goal_msg.start.pose.position.y = current_y_;
        goal_msg.start.pose.orientation.w = 1.0;

        auto send_goal_options = rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SendGoalOptions();
        send_goal_options.goal_response_callback = std::bind(&TestNode::goal_response_callback, this, std::placeholders::_1);
        send_goal_options.result_callback = std::bind(&TestNode::result_callback, this, std::placeholders::_1);
        
        RCLCPP_INFO(this->get_logger(), "Mengirim Request Kalkulasi ke Nav2...");
        this->client_->async_send_goal(goal_msg, send_goal_options);
    }

private:
    double current_x_;
    double current_y_;
    nav_msgs::msg::OccupancyGrid::SharedPtr current_map_;

    bool is_in_obstacle(double x, double y) {
        if (!current_map_) return false;

        double res = current_map_->info.resolution;
        double origin_x = current_map_->info.origin.position.x;
        double origin_y = current_map_->info.origin.position.y;

        int grid_x = static_cast<int>((x - origin_x) / res);
        int grid_y = static_cast<int>((y - origin_y) / res);

        if (grid_x < 0 || grid_x >= static_cast<int>(current_map_->info.width) || 
            grid_y < 0 || grid_y >= static_cast<int>(current_map_->info.height)) {
            return true; 
        }

        int index = grid_y * current_map_->info.width + grid_x;
        int cost = current_map_->data[index];

        return cost >= 50;
    }

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
    
    void publish_ui_error(const std::string& msg) {
        std_msgs::msg::String log_msg;
        log_msg.data = msg;
        this->ui_log_pub_->publish(log_msg);
    }

    void initialpose_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json pose = json::parse(msg->data);
            current_x_ = pose["x"];
            current_y_ = pose["y"];
        } catch (...) {}
    }

    void obstacle_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json payload = json::parse(msg->data);
        
            if (payload.contains("start")) {
                current_x_ = payload["start"]["x"];
                current_y_ = payload["start"]["y"];
            }
            
            if (payload.contains("obstacles") && payload["obstacles"].is_array()) {
                auto pc_msg = std::make_unique<sensor_msgs::msg::PointCloud2>();
                pc_msg->header.frame_id = "map";
                pc_msg->header.stamp = this->now();
                pc_msg->height = 1;
                pc_msg->width = payload["obstacles"].size();
                pc_msg->is_dense = true;
                pc_msg->is_bigendian = false;
                
                sensor_msgs::PointCloud2Modifier modifier(*pc_msg);
                modifier.setPointCloud2FieldsByString(1, "xyz");
                modifier.resize(pc_msg->width);
                
                sensor_msgs::PointCloud2Iterator<float> iter_x(*pc_msg, "x");
                sensor_msgs::PointCloud2Iterator<float> iter_y(*pc_msg, "y");
                sensor_msgs::PointCloud2Iterator<float> iter_z(*pc_msg, "z");
                
                for (const auto& obs : payload["obstacles"]) {
                    *iter_x = obs["x"].get<float>();
                    *iter_y = obs["y"].get<float>();
                    *iter_z = 0.0f;
                    ++iter_x; ++iter_y; ++iter_z;
                }
                
                this->pc_pub_->publish(std::move(pc_msg));
            }

            std::string algo = payload.value("algorithm", "AStar");
            this->send_goal(payload["goal"]["x"], payload["goal"]["y"], algo);

        } catch (const std::exception& e) {
            RCLCPP_ERROR(this->get_logger(), "Error JSON Parsing: %s", e.what());
        }
    }

    void setup_simulation_callback(const std_msgs::msg::String::SharedPtr msg) {
        try {
            json payload = json::parse(msg->data);
            std::string selected_map = payload["map"]; 
            
            if (!this->load_map_client_->wait_for_service(std::chrono::seconds(2))) return;

            auto request = std::make_shared<nav2_msgs::srv::LoadMap::Request>();
            request->map_url = "/workspace/install/navigation/share/navigation/maps/" + selected_map;
            this->load_map_client_->async_send_request(request);
        } catch (...) {}
    }

    void goal_response_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::SharedPtr & gh) {
        if (!gh) {
            RCLCPP_ERROR(this->get_logger(), "Goal DITOLAK oleh Nav2 Server!");
            publish_ui_error("[ERROR] ABORT: Goal DITOLAK oleh Nav2 Server!");
            return;
        }
    }

    void result_callback(const rclcpp_action::ClientGoalHandle<nav2_msgs::action::ComputePathToPose>::WrappedResult & res) {
        if (res.code != rclcpp_action::ResultCode::SUCCEEDED) {
            RCLCPP_ERROR(this->get_logger(), "Kalkulasi GAGAL! Nav2 tidak menemukan jalan.");
            publish_ui_error("[ERROR] ABORT: Kalkulasi GAGAL! Nav2 tidak menemukan jalan.");
            return;
        }
        nav_msgs::msg::Path path_msg = res.result->path;
        path_msg.header.frame_id = "map";
        path_msg.header.stamp = this->now();
        
        this->plan_pub_->publish(path_msg);
        RCLCPP_INFO(this->get_logger(), "Rute SUKSES dikalkulasi! Jumlah Node: %zu", path_msg.poses.size());
    }

    rclcpp_action::Client<nav2_msgs::action::ComputePathToPose>::SharedPtr client_;
    rclcpp::Client<nav2_msgs::srv::LoadMap>::SharedPtr load_map_client_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr obs_sub_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr init_pose_sub_;
    rclcpp::Subscription<std_msgs::msg::String>::SharedPtr setup_sub_;
    rclcpp::Subscription<nav_msgs::msg::OccupancyGrid>::SharedPtr map_sub_;
    rclcpp::Publisher<nav_msgs::msg::Path>::SharedPtr plan_pub_; 
    rclcpp::Publisher<sensor_msgs::msg::PointCloud2>::SharedPtr pc_pub_;
    rclcpp::Publisher<std_msgs::msg::String>::SharedPtr ui_log_pub_;
    std::unique_ptr<tf2_ros::TransformBroadcaster> tf_broadcaster_;
    rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char * argv[]) {
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<TestNode>());
  rclcpp::shutdown();
  return 0;
}