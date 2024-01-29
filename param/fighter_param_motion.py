from pyprc import *

fighter_param = param("fighter_param_motion_vanilla.prc")
table = fighter_param[hash("fighter_param_motion_table")]

# actual param traversal and editing
for fighter in table:
    # the returned value is a hash, not a string
    fighter_name = fighter[hash("fighter_kind")].value
    
    fighter[hash("escape_n_hit_xlu_frame")].value = 3
    fighter[hash("escape_n_hit_normal_frame")].value = 16
    fighter[hash("escape_n_penalty_hit_xlu_frame")].value = 6
    fighter[hash("escape_n_penalty_hit_normal_frame")].value = 13
    fighter[hash("escape_n_cancel_frame")].value = 26
    fighter[hash("escape_attack_frame")].value = fighter[hash("escape_n_cancel_frame")].value 
    fighter[hash("escape_air_slide_hit_xlu_frame")].value = 3
    fighter[hash("escape_air_slide_hit_normal_frame")].value = 24
    fighter[hash("escape_air_slide_penalty_hit_xlu_frame")].value = 6
    fighter[hash("escape_air_slide_penalty_hit_normal_frame")].value = 24
    fighter[hash("escape_air_slide_back_distance")].value = 0
    fighter[hash("escape_air_slide_back_end_frame")].value = 0
    fighter[hash("escape_air_slide_speed")].value = 2.4
    fighter[hash("escape_air_slide_distance")].value = 25
    fighter[hash("escape_air_slide_penalty_speed")].value = 2.4
    fighter[hash("escape_air_slide_penalty_distance")].value = 25
    fighter[hash("escape_air_slide_end_speed")].value = 0
    fighter[hash("escape_air_slide_cancel_frame")].value = 40
    fighter[hash("landing_frame_escape_air_slide_max")].value = 13

fighter_param.save("fighter_param_motion.prc")

