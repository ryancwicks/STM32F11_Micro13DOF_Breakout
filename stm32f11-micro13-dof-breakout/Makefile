build_hex: src/main.rs 
    cargo objcopy --bin stm32f11-micro13-dof-breakout --release -- -O binary imu_reader_hex

upload: build_hex
    st-flash --reset write imu_reader_hex 0x8000000 
