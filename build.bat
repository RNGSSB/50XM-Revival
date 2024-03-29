
cd param

python fighter_param.py
python fighter_param_motion.py
python common.py
python battle_object.py

copy /y battle_object.prc "build"
copy /y common.prc "build"
copy /y fighter_param.prc "build"
copy /y fighter_param_motion.prc "build"

cd..

xcopy /y "param/build" "50XM/fighter/common/param"

cd skyline-rs-template

cargo skyline build -- --offline

cd target

cd aarch64-skyline-switch

cd debug

del plugin.nro

ren libplugin.nro plugin.nro

cd..

cd..

cd..

cd..

copy /y "skyline-rs-template\target\aarch64-skyline-switch\debug\plugin.nro" "C:\Users\RNG\Documents\GitHub\50XM-Revival\50XM"

xcopy /y /e "50XM" "C:\Users\RNG\AppData\Roaming\suyu\sdmc\ultimate\mods\50XM"

cmd /k


