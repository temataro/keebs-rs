# TODOs for ataro_v0.3

## Software
  - Implement a full working keyboard firmware with qmk/zmk 


## Hardware
  - Use RP Pico dev board footprint for microcontroller
    - Actually see if you can implement software definitions for UART TX/RX selection on the two hemispheres.
    -   => Let go of the 0 ohm resistors for manual switching.
  - Have any unused GPIO pins interface out cleanly to a corner of each hemisphere for future improvements
    - (maybe trackballs, side scrolling, idk...)  
  - Use footprints with stabliizers for keys bigger than 1.5u (?)
  - Use irreversible footprints for switches and include hot swap sockets (find/make switch footprints that are reversible *and* incorporate hotswap sockets) 
  - Add a small slot for a riser on the back of each keyboard case ([like one of these](https://vi.aliexpress.com/item/1005007286593544.html)).


## Placement, Layout
  - Move to asymmetric left and right sides; only minor differences but still.
    - Keep rotary encoder on both, have an extra row on the right side for extra keys... maybe can still keep things symmetric if I give the left side some unnecessary keys too!! **Just 3d print a different shaped top half for the different hemispheres instead of diferent PCB designs**
  - Make next revision PCBs white, leave silkscreen space for other radiant orders to put on there. V0.3 will be order Windrunner
  - Reposition mounting holes based on 3D printed case considerations
