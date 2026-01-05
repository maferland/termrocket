function git --wraps git --description "Git wrapper that launches rocket on push"
    command git $argv
    set -l git_status $status

    if test $git_status -eq 0
        if contains push $argv
            if type -q termrocket
                termrocket launch &
                disown
            else
                # Trigger background download for next time
                _termrocket_download &
                disown
            end
        end
    end

    return $git_status
end
