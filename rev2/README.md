# (WIP) HWLife Rev2

**This revision is on indefinite hold; I am currently working on a design
with independent modules and no backplane (rev3).**

A redesign of the backplane system.

Major changes:

- Module hardware
  - CH32V003
  - Right-angle backplane connectors and LEDs (allowing single-sided SMT assembly)
  - RGB LEDs (WS2812)

- Comms
  - Analog neighbor communication
  - Serial communication to individual modules (for programming state or dimming)
