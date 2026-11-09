set -g fish_greeting

starship init fish | source

# fastfetch
function fastfetch
    command fastfetch --kitty-direct /home/jotacastaman/Pictures/fastfetchicons/03 --logo-width 25 --logo-height 14 $argv
end

# tty-clock
function clock
    command tty-clock -c -t
end

# Poweroff and Restart
function poweroff
    command sudo poweroff
end

function reboot
    command sudo reboot
end
