## Comments start with "##"
## This is a template file for future references while constructing the syntax and functionalities of rush
## Contents of this file are solely for guidance purposes at the time of writing
## This file demonstrates some of the basic functions of rush which will later be implemented


## Define a regular function
func some_func[ (-? ?# help :: String) ] {
    if ($help | empty?) {
        exit 1;
    }
    else {
        let command_help::String = $"Help for ($name)";
        print $command_help;
        let y?#user_input::String = (read);
        switch (user_input):
            case ('y'|'n'): {
                print $"Input was ($y)";
            }
            default: {
                print "Input was something else";
            }
        exit 0;
    }
}

## Define a regular function alternate way
deffunc some_func_def => {
    args => {
        (-? ?# --help): String?
    }

    impl => {
        if (not ($$help | empty?)) {
            let command_help::String = $"Help for ($name)"
            print $command_help
            let y?#user_input::String = (read)
            switch (user_input):
                case ('y'|'n'): {
                    print $"Input was ($y)"
                }
                default: {
                    print "Input was something else";
                }
            exit 0
        }
        else {
            exit 1
        }
    }
}

## Define a function with custom configurations
## like a custom file to read for input and,
## custom output destination instead of the regular
## STDIN and STDOUT
deffunc custom_func => {
    configuration => {
        name: "custom_func",
        desc: "Function with a custom configuration",
        write: "~/output.txt",
        read:  "~/input.txt"
    }

    args => {
        (-f ?# --file): String?,
        (-h ?# --help): ()
    }

    impl => {
        if (not ($$file | empty?)) {
            open $$file
        }
        else {
            if ($help | empty?) {
                ## Print to stdout
                print "File name not given..." >> STDOUT
            }
            else {
                print $tok_details
                print "No help for this one sorry"
            }
        }
    }
}


## Define a custom Keyword
defkeyw a => {
    syn => "a strlist str anylist any expr block";

    args => {
        strlist: COLLECTION[String],
        str: TOKEN[String],
        anylist: COLLECTION[*],
        any: TOKEN[*],
        expr: EXPRESSION,
        block: CODEBLOCK,
    }

    impl => {
        for x in $$strlist {
            print $x
        }
    }
}

## Define a keyword that checks if a number is even
defkeyw even? => {
    configuration => {
        output: Boolean
    }

    syn => "even? numtoken"

    args => {
        numtoken: TOKEN[Number]
    }

    impl => {
        if (($$numtoken % 2) == 0) {
            return true
            exit 0
        }
        else if ($$numtoken == 0) {
            return false >> STDOUT ## Return and write to stdout
            ## there can be only one return sequence that returns a value
            exit 0
        }
        else {
            print "Not an even number" ## Wriiten to stdout but not stored in output
            return false ## Not written to stdout but stored in output
            ## there can be only one return sequence in a block
            exit 1
        }
    }
}

let var = (even? 5) ## var is false
let var = (even? 2) ## var is true
let var = (even? 0) ## var is false
let var::Number = (even? 5)? ## var is 1
let var::Number = (even? 2)? ## var is 0
let var::Number = (even? 0)? ## var is 0
