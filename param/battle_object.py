from pyprc import *

battleobject = param("battle_object_vanilla.prc")

#Shield Reflect
battleobject[hash("just_shield_reflect_attack_mul")].value = 0.6
battleobject[hash("just_shield_reflect_speed_mul")].value = 0.9

#Hitstun
battleobject[hash("damage_frame_mul")].value = 0.4 #Value 0.4

#Hitstun Thresholds
battleobject[hash("damage_level1")].value = 15 #Value 15
battleobject[hash("damage_level2")].value = 21 #Value 21
battleobject[hash("damage_level3")].value = 32 #Value 32

#Balloon Knockback
battleobject[hash("damage_fly_speed_up_reaction_frame_min")].value = 50 #30
battleobject[hash("damage_fly_speed_up_reaction_frame_max")].value = 100 #80

#Sakurai Angle
battleobject[hash("damage_angle_air")].value = 0.785398
battleobject[hash("damage_angle_ground_max")].value = 44
battleobject[hash("damage_angle_ground_reaction_min")].value = 32
battleobject[hash("damage_angle_ground_reaction_max")].value = 32.1

#Hitlag
battleobject[hash("hitstop_frame_max")].value = 30 #Hitlag Max Frames 30
battleobject[hash("hitstop_frame_add")].value = 6 #Hitlag Base Frames 6
battleobject[hash("hitstop_frame_mul")].value = 0.65 #Hitlag Multiplier 0.65
battleobject[hash("hitstop_elec_mul")].value = 1.5 #Hitlag Multiplier Electric 1.5

#Random Trip Minimum Knockback
battleobject[hash("slip_damage_reaction")].value = 999 #55

#1v1 multiplier is removed, original value is 1.2


battleobject.save("battle_object.prc")

