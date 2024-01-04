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

## GPS power

In your GPS config, use `com/jdeeth/mu2tweaks/gps_power` to indicate if power is available.

It will be 1 if:
- electrical power is available
- the left radio main switch is turned on (by annunciator panel on left console)
- the `GPS 15` circuit breaker is in

otherwise it will be 0.