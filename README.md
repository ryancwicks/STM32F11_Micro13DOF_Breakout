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

## Micro 13DOF Click board

This breakout board contains the following sensors:
BMM150 - geomagnetic sensor from Bosch
BME680 – Low power gas, pressure, temperature and humidity sensor from Bosch 
BMI088 – small, versatile 6Dof sensor module from Bosch

## Rust Crates

embedded_hal - Rust embedded system HAL
        https://crates.io/crates/embedded-hal
stm32f4 - Low level register definitions for STM32F4 libraries.
        https://crates.io/crates/stm32f4
stm32f4xx-hal - HAL abstractions for STM32F4 uC
        https://crates.io/crates/stm32f4xx-hal
BME680 - library for accessing the BME680 environmental sensor
        https://github.com/marcelbuesing/bme680
BMI160 - driver for BMI160 IMU, might be adaptable for the BMI088
        https://crates.io/crates/bmi160
BMI088 - Driver for BMI088, needs I2C support added.
        https://crates.io/crates/bmi088
usbd-serial - USB CDC Serial driver build around embedded_hal
        https://crates.io/crates/usbd-serial

