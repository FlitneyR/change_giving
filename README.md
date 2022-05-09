# Change Giving

This is a command line tool written in Rust to determine whether
change can be given for a certain amount given certain coins.

It uses Dynamic Programming, i.e. it solves the problem by solving
smaller sub problems which together constitute a solution.

Specifically it works by determining all possible amounts of change
less than or equal to the target which can be given and how.

