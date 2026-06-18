FROM osrf/ros:humble-desktop

RUN apt-get update && apt-get install -y \
    ros-humble-navigation2 \
    ros-humble-nav2-bringup \
    ros-humble-rosbridge-server \
    nlohmann-json3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# COPY . /workspace

# RUN . /opt/ros/humble/setup.sh && colcon build

COPY bash/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]