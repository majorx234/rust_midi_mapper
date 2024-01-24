# Info
- WIP
- detect midi controls and their messages and map them to functions

# Todo
- implement Display trait for MidiFunctionFile
- add configurable list of midi_functions
- need new GUI elements to visualize different functionalities
- add assigning function (invert, log,lin,exp-scaling)
- add better json generation
- add drag&drop functionality
- add other output formates for MIXXX & Equis

# usage (WIP not usable yet!)
- ```target/release/midi_elements_gui -f test_midi_functions.json```
  - use config file with function names

# References
- taken util from:
  - https://github.com/samdoshi/midi-rs/tree/master
