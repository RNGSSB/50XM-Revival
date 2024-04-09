yamlist.exe asm "yaml\lucario.yaml" -o fighter/lucario/motion_list.bin
yamlist.exe asm "yaml\ness.yaml" -o fighter/ness/motion_list.bin
yamlist.exe asm "yaml\luigi.yaml" -o fighter/luigi/motion_list.bin

cd..

copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c00"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c01"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c02"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c03"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c04"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c05"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c06"
copy /y "motion_list\fighter\lucario\motion_list.bin" "50XM\fighter\lucario\motion\body\c07"

copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c00"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c01"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c02"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c03"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c04"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c05"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c06"
copy /y "motion_list\fighter\ness\motion_list.bin" "50XM\fighter\ness\motion\body\c07"

copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c00"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c01"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c02"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c03"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c04"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c05"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c06"
copy /y "motion_list\fighter\luigi\motion_list.bin" "50XM\fighter\luigi\motion\body\c07"


cmd /k