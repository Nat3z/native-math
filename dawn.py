from websocket import create_connection

ws = create_connection("ws://localhost:5432")


# def arm_movement():
#     # moves arm up for 1 second and then moves it down for 1 second
#     # assumes arm is attached to motor A of MC "ARM_MOTOR"
#     while True:
#         if Gamepad.get_value("a_button"):
#             Robot.set_value(ARM_MOTOR, "velocity_a", 0.5)
#             Robot.sleep(1)
#             Robot.set_value(ARM_MOTOR, "velocity_a", -0.5)
#             Robot.sleep(1)
# ​
# def teleop_setup():
#     # starts the arm_movement subroutine in parallel to the main thread
#     Robot.run(arm_movement)
#     pass
# ​
# def teleop_main():
#     # normal tank drive student code
#     left_stick_val = Gamepad.get_value("joystick_left_y")
#     right_stick_val = Gamepad.get_value("joystick_right_y")
#     
#     if abs(left_stick_val) > 0.1:
#         Robot.set_value(DRIVE_MOTOR, "velocity_a", left_stick_val)
#     else:
#         Robot.set_value(DRIVE_MOTOR, "velocity_a", 0)
#     
#     if abs(right_stick_val) > 0.1:
#         Robot.set_value(DRIVE_MOTOR, "velocity_b", right_stick_val)
#     else:
#         Robot.set_value(DRIVE_MOTOR, "velocity_b", 0)
