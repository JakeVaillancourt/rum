# Universal State Machine

by Jake Vaillancourt and Christian Collins

# Acknowledges help you may have received from or collaborative work you may have undertaken with others

We recieved help on some ideas for the design of our project and some debugging from TA Isaac.

# Identifies what has been correctly implemented and what has not

We believe that our program has been implemented completely correctly besides our tests.
Unfortunately, we ran out of time to test some outlier testcases as this was a lesser priority for us.

# Briefly enumerates any significant departures from your design

Their were many departures from our original design. As we gain a better understanding of how this
program should work, we updated our plans for how we would create our registers and memory segments. Additionally, we added a vector to track our empty memory segments. Most importantly, rather than having everything in separate modules, we decided everything would be put together under one machine struct.

# Succinctly describes the architecture of your system. Identify the
# modules used, what abstractions they implement, what secrets they
# know, and how they relate to one another. Avoid narrative descriptions
# of the behavior of particular modules.

Our program uses a Um struct within um.rs that holds the field of our registers, memory segment, empty keys,
and program counter. It implements 2 functions: initialize_program(), which sets up our machine, and
run_program(), which runs the program. Instructions.rs contains our 13 opcode functions. Umload.rs holds our 
function that allows us to read in our instructions from the input file.

# Explains how long it takes your UM to execute 50 million instructions,
# and how you know

I tested our program against midmark using the command "Measure-Command{cargo run --release midmark.um}"
on powershell. It ran in 9.56 seconds.

# Says approximately how many hours you have spent analyzing the assignment

Approximately 4 hours.

# Says approximately how many hours you have spent preparing your design

Approximately 6 hours

# Says approximately how many hours you have spent solving the problems after your analysis

Approximately 4 hours.
