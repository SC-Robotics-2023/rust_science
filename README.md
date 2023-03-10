# science_2022_23
Command and control software for SC Robotics' science subsystem.

Developed using ROS2 and implemented using Rust.

## Installation
***This section only needs to be done once per computer.***

In your workspace's root directory clone the repository using the following command: `git clone git@github.com:SC-Robotics-2021/science_2022_23.git src`.

There is a provided script that will provision your computer with the necessary source code needed to run ROS2 in rust. You can do this from within the directory of the repo by running one the following two commands depending on your ROS2 version:

`chmod +x ./src/science_2023_23/ros2_foxy_rust_setup.bash && ./src/science_2023_23/ros2_foxy_rust_setup.bash`

`chmod +x ./src/science_2023_23/ros2_humble_rust_setup.bash && ./src/science_2023_23/ros2_humble_rust_setup.bash`

depending on which version of ROS2 you are using.

To verify all the necessary things have been installed, please copy and paste the following command in your terminal:

`rustup --version && rustc --version && cargo --version && cargo search cargo-ament-build && python3 -m pip show colcon-cargo && python3 -m pip show colcon-ros-cargo`.

If any warnings or errors are displaying, then please rerun the bash script above.

## Buidling
Building ROS2 packages written in Rust can be done as with any other language that ROS2 supports. To build the package, run:

`colcon build --symlink-install; colcon build --symlink install --packages-skip-build-finished`

There is currently and issue were if you attempt to build all the packages at once, science_clients_rs and science_servers_rs will fail to build. If you build twice, the packages will finish cleanly. The command above builds in two steps, which seems to be an sufficient workaround to get everything to work for the time being.

## Running the code
These packages can be run just as if it were any other package. If you would like to test any of the nodes individually use the following commands:

### Client Nodes
`ros2 run science_clients_rs brush_client` \
`ros2 run science_clients_rs ir_camera_client` \
`ros2 run science_clients_rs microscope_client` \
`ros2 run science_clients_rs stepper_motor_client` \
`ros2 run science_clients_rs uv_camera_client` \
`ros2 run science_clients_rs uv_light_client` \
`ros2 run science_clients_rs vacuum_client` \
`ros2 run science_clients_rs water_pump_client`

At the moment there is no launch file for the client side nodes as there need's to be a Gui interface developed for them.

### Server Nodes
`ros2 run science_servers_rs brush_server` \
`ros2 run science_servers_rs ir_camera_server` \
`ros2 run science_servers_rs microscope_server` \
`ros2 run science_servers_rs stepper_motor_server` \
`ros2 run science_servers_rs uv_camera_server` \
`ros2 run science_servers_rs uv_light_server` \
`ros2 run science_servers_rs vacuum_server` \
`ros2 run science_servers_rs water_pump_server`

To run all the server-side node (those which would be executing on-board the rover) please run: \
`ros2 launch science_servers_rs science_servers.launch.xml`.

## Notes
There currently is not command and control for the turret implemented for the repository as the encoded motor chosen to drive it has been changed in our design. Once more information is available about this motor, additional source code will be included in this repo.
