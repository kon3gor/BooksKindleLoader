result=${PWD##*/}          # to assign to a variable
result=${result:-/}        # to correct for the case where PWD=/

printf '%s\n' "${PWD##*/}"
