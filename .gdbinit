dir src
file target/debug/cim

tui enable

# note: rust uses an internal main function, to view source use cim::main
b cim::main
