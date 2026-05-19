#include "rclcpp/rclcpp.hpp"

class LocalizationNode : public rclcpp::Node
{
public:
    LocalizationNode() : Node("ekf_filter_node")
    {
        RCLCPP_INFO(this->get_logger(), "Localization EKF node has started running.");
    }
};

int main(int argc, char * argv[])
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<LocalizationNode>());
    rclcpp::shutdown();
    return 0;
}