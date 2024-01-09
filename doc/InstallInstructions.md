# Installation instructions

Compile the plugin (todo: release binaries) and place in `MU2/plugins/MU2Tweaks/64/`

Back up the MU-2 `.acf` files

**IMPORTANT:** do not forget to back up the .acf files. Also do not expect any support from the developers or publishers if you make any changes to the supplied software.

## Radio altimeter

The real unit is a Collins ALT-55B radio altimeter driving a DRI-55 display.

The tweaks here:
- show 00 on the ground
- update twice a second, not continuously
- show height to nearest 10ft and 20ft while close to the ground
- show height to nearest 50ft up to 2500ft
- blank above 2500ft

Without access to a manual or clearer in-flight footage I've guessed at how the precision should vary with height.

### Steps

Edit the `.acf` with Notepad++ or similar

Remove the existing radio altimeter:
```
gen_LED gen_LED.png
  POS 1774.000000 1600.500000
  IMAGE LEDs/led_DH_font_big
  DATAREF sim/cockpit2/gauges/indicators/radio_altimeter_height_ft_pilot
  SHOW_LESS 2500.000000 sim/cockpit2/gauges/indicators/radio_altimeter_height_ft_pilot
  LIGHT_MODE GLASS_AUTO
  LIGHT_RHEOSTAT 2
  BUS_SRC 1
  KEY_FRAME 0.000000 0.000000 1.000000
  KEY_FRAME 1.000000 1.000000
  DIGITS 4
  DECIMALS 0
  PERIOD_WIDTH 0
  LED_ROWS 0
  ROUNDING ROUND_ROUND
```

Insert in its place:
```
gen_LED RadAlt12
  POS 1762.000000 1602.500000
  IMAGE LEDs/led_DH_font_big
  DATAREF com/jdeeth/mu2tweaks/radio_altimeter_height_ft_pilot_12
  LIGHT_MODE GLASS_AUTO
  LIGHT_RHEOSTAT 2
  BUS_SRC 1
  KEY_FRAME 0.000000 0.000000 1.000000
  KEY_FRAME 1.000000 1.000000
  DIGITS 2
  DECIMALS 0
  PERIOD_WIDTH 0
  LED_ROWS 0
  ROUNDING ROUND_ROUND

gen_LED RadAlt34
  POS 1786.000000 1602.500000
  IMAGE LEDs/led_DH_font_big
  DATAREF com/jdeeth/mu2tweaks/radio_altimeter_height_ft_pilot_34
  LIGHT_MODE GLASS_AUTO
  LIGHT_RHEOSTAT 2
  BUS_SRC 1
  KEY_FRAME 0.000000 0.000000 1.000000
  KEY_FRAME 1.000000 1.000000
  DIGITS 2
  DECIMALS 0
  PERIOD_WIDTH 0
  LED_ROWS 0
  ROUNDING ROUND_ROUND
```

## HSI Course and Dist readouts

This can be fixed entirely in the .acf file.

There's two shortcomings to gen_LED instruments:
1. leading zeros are omitted
2. decimal places have the same width as a digit

The workaround is to change the keyframes to increase the value being displayed - so 4 becomes 1004 but you just see 004

The MU-2 already draws the decimal point on the distance displays separately, so we can also multiply the distance value by 10 and eliminate a different workaround that causes the 0.1s digit to have an extra glow.

Edit the .acf file in Notepad++ or similar. There are four instruments to update. In each section replace the lines that contain contradicting values e.g. `PERIOD_WIDTH -1`

### `gen_LED copilot_course` and `gen_LED pilot_course`:

    KEY_FRAME 0.000000 1000.000000 1.000000
    KEY_FRAME 360.000000 1360.000000

### `gen_LED copilot_distance` and `gen_LED pilot_distance`:

    KEY_FRAME 0.000000 10000.000000 1.000000
    KEY_FRAME 200.000000 12000.000000
    DIGITS 4
    DECIMALS 0
    PERIOD_WIDTH 0


## GPS power

In your GPS config, use `com/jdeeth/mu2tweaks/gps_power` to indicate if power is available.

It will be 1 if:
- electrical power is available
- the left radio main switch is turned on (by annunciator panel on left console)
- the `GPS 15` circuit breaker is in

otherwise it will be 0.

## Condition levers

The MU-2 ships with commands to move the levers to EMERG STOP and TAXI, but they
don't always move the levers, possibly depending on joystick setup. They might
simulate the "lift lever to move lower than Idle" behaviour in some way.

This plugin provides commands for all four labelled positions for each lever,
which just moves the lever directly without any simulation of a gate. (It
sets the `xscenery/mu2b60/manips/[LR]_condition_lever_rotate` datarefs directly.)

To simulate the "lift lever to move lower than Idle" behaviour, use an extra
modifier key when mapping the EmergStop command.

e.g. if Numpad 1, 4 and 7 are Left Idle, MinCruise, and TakeOffLanding, then
bind Ctrl+Numpad 1 to EmergStop. Pressing the extra modifier key simulates
the check provided by the real-life gate.