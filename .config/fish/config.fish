if status is-interactive
    # Commands to run in interactive sessions can go here
    alias ls='ls --color=auto'
    alias pipes.sh='pipes.sh -c 7'
    alias matrix='unimatrix -c white -s 96 -l o'
    alias clock='tty-clock -c 7'
    alias bonsai='cbonsai -l -s 2'
    alias fastfetch='fastfetch --kitty-direct /home/jotacastaman/fastfetchicon/03.png --logo-width "22" --logo-height "12"'

    set -gx PATH $HOME/.local/bin $PATH

    eval "$(starship init fish)"
end
