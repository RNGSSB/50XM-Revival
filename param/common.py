from pyprc import *

common = param("common_vanilla.prc")

#Buffer
common[hash("precede")].value = 6
common[hash("precede_extension")].value = 0

#Dash shit
common[hash("turn_dash_frame")].value = -1
common[hash("dash_escape_frame")].value = 1
common[hash("dash_enable_attack_frame")].value = 1
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
common[hash("tread_jump_speed_limit")].value = 999

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

#Shield Size
common[hash("shield_scale_min")].value = 0.15 #It can't be that easy! It was that easy. (0.15 default, 1 to turn off shield poking!)
common[hash("shield_dec1")].value = 0.15 #Shield depletion. Default is 0.15

#Re grab timer, og is 60
common[hash("invalid_capture_frame")].value = 35

#Shield Drop Lag
common[hash("guard_off_cancel_frame")].value = 11

#Knockback Bounce Min Speed
common[hash("damage_fly_reflect_d_speed")].value = 0.8 #Default 0.8 #5.0 works fine for something like Melee Fox Down Throw

#Zair Landing Motion Rate
common[hash("landing_air_lasso_motion_rate")].value = 8 #8 default

#Zair Landing Frames
common[hash("air_lasso_landing_frame")].value = 30 # 30 default

#Ledge Regrab Amounts
common[hash("air_lasso_catch_num")].value = 999
common[hash("cliff_max_count")].value = 999

#Ledge Invul
#common[hash("cliff_wait_air_xlu_max_frame")].value = -41 #Maximum ledge invul frames added from being airborne. Default 60
#common[hash("cliff_wait_damage_xlu_max_damage")].value = 999 #Ledge Invul % Treshold. Default 120
#common[hash("cliff_wait_damage_xlu_max_frame")].value = 41 #Maximum ledge invul added from damage. Default 44
#common[hash("cliff_wait_xlu_min_frame")].value = 0 #Minimum lede invul frames. Does not count the initial 19 frame anim. Default is 4

#Ledge Jump Lag Frames
common[hash("cliff_jump_disable_attack_frame")].value = 4 #2 Default

#Platform Drop Speed???
common[hash("pass_speed_y")].value = -0.5 #default -0.5

#Crouch Cancel
common[hash("squat_damage_reaction_mul")].value = 0.67 #default 0.85

common.save("common.prc")

