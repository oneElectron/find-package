# This is taken directly from https://github.com/Homebrew/homebrew-command-not-found

function fish_command_not_found
    set -l cmd $argv[1]
    set -l txt

    if not contains -- "$cmd" "-h" "--help" "--usage" "-?"
        if which command-not-found > /dev/null
            set txt (command-not-found --explain $cmd 2> /dev/null)
        else
            set txt (command-not-found --explain $cmd 2> /dev/null)
        end
    end

    if test -z "$txt"
        __fish_default_command_not_found_handler $cmd
    else
        string collect $txt
    end
end

function __fish_command_not_found_handler --on-event fish_command_not_found
    fish_command_not_found $argv
end
