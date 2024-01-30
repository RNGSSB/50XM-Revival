from pyprc import *

common = param("common_vanilla.prc")

#Buffer
common[hash("precede")].value = 6
common[hash("precede_extension")].value = 0

#Dash shit
common[hash("turn_dash_frame")].value = -1
common[hash("dash_escape_frame")].value = 1
common[hash("dash_enable_attack_frame")].value = 3
common[hash("dash_speed_keep_frame")].value = 1

#Wavedash speed
common[hash("escape_air_slide_landing_speed_max")].value = 4

#Neutral Air Dodge Landing Lag
common[hash("landing_frame_escape_air")].value = 15

#Teching
common[hash("invalid_passive_speed")].value = 999  
common[hash("air_escape_passive_trigger_frame")].value = 1
common[hash("passive_trigger_frame")].value = 20
common[hash("no_rapid_frame_value")].value = 40

#Footstool invul
common[hash("tread_jump_after_xlu_frame")].value = 0

#Ledge
common[hash("cliff_no_catch_frame")].value = 0

#Rage
common[hash("damage_reaction_mul_max")].value = 1

#Shorthop Attack Multiplier
common[hash("mini_jump_attack_mul")].value = 1

#Gravity Boost
common[hash("damage_fly_speed_y_mul_base_accel")].value = 0
common[hash("damage_fly_speed_y_mul")].value = 0

#DI and LSI
common[hash("damage_fly_correction_max")].value = 18
common[hash("damage_fly_length_mul_max")].value = 1
common[hash("damage_fly_length_mul_min")].value = 1

#Dodge Staling
common[hash("escape_penalty_frame")].value = 1
common[hash("escape_penalty_max_count")].value = 1
common[hash("escape_penalty_recovry_frame")].value = 1
common[hash("escape_penalty_motion_rate")].value = 0
common[hash("escape_f_penalty_motion_rate")].value = 0
common[hash("escape_b_penalty_motion_rate")].value = 0

#Hitstun Cancels
common[hash("damage_fly_attack_frame")].value = 999
common[hash("damage_fly_escape_frame")].value = 999

#Grounded Hitstun
common[hash("damage_fly_reflect_reaction_frame_mul")].value = 0.8 #0.8
common[hash("damage_fly_reflect_d_speed_mul")].value = 0.9 # 0.9

#Stupid Blast zone mechanic
common[hash("dead_down_damage_check_bottom")].value = float('nan') 

#Shieldstun stuff lmao
common[hash("shield_setoff_add")].value = 6 #Normal value is 3
common[hash("shield_setoff_mul")].value = 0.8 #Normal value is 0.8
common[hash("shield_setoff_mul_fighter_shot")].value = 0.29 #Normal value is 0.29
common[hash("shield_stiff_mul_attack_4")].value = 0.725 #Normal value is 0.725 
common[hash("shield_stiff_mul_attack_air")].value = 0.33 #Normal value is 0.33

#Parry Advantage is 7 frames now, original is 3, param has no name so i edited with prcEditor

#Parry Window, og is 5 frames
common[hash("shield_just_frame")].value = 5 

#Shield Grab Nerf
common[hash("shield_setoff_catch_frame")].value = 0

#Shield Minimum Hold Frames
common[hash("shield_min_frame")].value = 1

#Re grab timer, og is 60
common[hash("invalid_capture_frame")].value = 40

common.save("common.prc")

