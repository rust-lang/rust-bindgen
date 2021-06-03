for name in (rg -lF addr_of! tests/expectations | sd '.*/(.*).rs' '$1')
    set path (fd --glob "$name.*" tests/headers)
    if test -n "$path"

        set flags (rg -F "// bindgen-flags" $path)
        if test -n "$flags"
            set minor (rg ".*\-\-rust\-target[ =]1.(\d+).*" $path -r '$1')
            if test -n "$minor"
                if test $minor -gt 47
                    echo  $path needs to change the version from 1.$minor to 1.47
                    sd -s "1.$minor" "1.47" $path
                else
                    echo $path uses version 1.$minor and that is fine
                end
            else
                echo $path does not have the `--rust-target` flag
                sd "// bindgen-flags: (.*)" '// bindgen-flags: --rust-target 1.47 $1' $path
            end
        else
            echo $path does not have the flags at all
            set contents (echo -e "// bindgen-flags: --rust-target 1.47\n"; cat $path)
            rm $path
            touch $path
            for line in $contents
                echo $line >> $path
            end
        end
    else
        echo $name headers not found
    end
end
