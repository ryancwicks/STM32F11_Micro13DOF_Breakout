# STM32F411 Micro 13DOF IMU Breakout board

Code and board files for an STM32F411 breakout board and the associated code. My plan is to code in Rust.

## Board connections

The Black Pill board I'm using has an STM32F411CEU6 (LQFP64 package).

Initial development has been done using a Black Pill board and a Micro 13 DOF IMU. The only connection is through I2C. The black pill board will supply power to the Micro board, with a 10uF Cap for power supply filtering.

The I2C pins used are:

| Function | Chip ID | Pin # | Black Pill Pin |
| -------- | ------- | ----- | -------------- |
| I2C1_SCL | PB6     | 58    | Header 2, pin 14, B6 |
| I2C1_SDA | PB7     | 59    | Header 2, Pin 15, B7 |

Pullups are on the Micro board.

