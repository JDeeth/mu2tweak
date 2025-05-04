# Installation instructions

With Rust installed, `cargo build --release`. Copy the resulting binary to the aircraft plugins folder.

Alternatively (Windows only for now) find the `mu2tweaks` subfolder in the `compiled` folder, copy it into `MU2/plugins`, where `MU2` is your X-Plane MU2 aircraft folder

You should have something like `X-Plane/Aircraft/Mu2/plugins/mu2tweaks/win_x64/mu2tweaks.xpl`

To check the plugin version, view its description in the plugin manager in X-Plane.

**IMPORTANT:** do not forget to back up any .acf and .obj files you modify. Also do not expect any support from the developers or publishers if you make any changes to the supplied software.

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

After making a backup copy, edit the `.acf` with Notepad++ or similar

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

After backing up the original, edit the .acf file in Notepad++ or similar. There are four instruments to update. In each section replace the lines that contain contradicting values e.g. `PERIOD_WIDTH -1`

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

No changes are needed to use these commands.

## OEM Radio stack

The OEM MU-2 overrides the default X-Plane radio tuner commands, and currently does it in a way that tunes the radios in incorrect/incomplete steps. e.g. NAV radios tune in increments of 0.01 MHz rather than 0.05, COM radios tune in increments of 0.01 MHz also rather than 0.025 or 0.00833. It also does not update the frequency on the roller drum displays if something other than the user has changed the frequency.

This plugin provides new commands that increment the frequencies correctly, and replacement datarefs for animating the roller drums.

To implement:

1. After saving a backup copy, edit `MU2/objects/VAR_OEM_Radio Stack/VAR_OEM_Radio_Stack.obj`
2. Search for `frequency`. There are 20 lines that start `ANIM_rotate_begin` and mention a dataref - one for each drum (5 drums per radio, 4 radios). Each dataref needs to be replaced:

old|new
-|-
xscenery/mu2b60/radios/com1_mhz_frequency_100s | com/jdeeth/mu2tweaks/com1_mhz_100
xscenery/mu2b60/radios/com1_mhz_frequency_10s | com/jdeeth/mu2tweaks/com1_mhz_010
xscenery/mu2b60/radios/com1_mhz_frequency_1s | com/jdeeth/mu2tweaks/com1_mhz_001
xscenery/mu2b60/radios/com1_khz_frequency_10s | com/jdeeth/mu2tweaks/com1_khz_10
xscenery/mu2b60/radios/com1_khz_frequency_1s | com/jdeeth/mu2tweaks/com1_khz_01

And so on for com2, nav1, and nav2.

Also replace the `ATTR_manip_command_knob` entries to use `mu2tweaks/com1_fine_up`, `mu2tweaks/com1_fine_down`, etc for all four radios.

To display 8.33 kHz spacing on a 5-digit drum, the last digit is partially rotated if the next digit is 5. So `130.105` is displayed like `130.1½` with the last digit halfway between 0 and 1.

## Rotating anti-collision beacons

As of 2.1 the anti-collision beacons don't rotate, but oscillate back and forth.

The plugin provides a new dataref. Edit `objects/Exterior_Lights.obj` and replace the **two** references to:
```
xscenery/mu2b60/lights/beacon_oscillator
```
with
```
jdeeth/mu2tweaks/beacon_oscillator
```

## Transponder Reply light

This is default X-Plane behaviour, where the Reply light flashes even with the
transponder in standby mode. The plugin provides an alternative dataref that
doesn't do this.

To use it, edit `objects/LIT_items_OEM.obj` (and for other versions too if
needed) to change:
```
sim/cockpit2/radios/indicators/transponder_brightness
```
with
```
jdeeth/mu2tweaks/transponder_brightness
```
