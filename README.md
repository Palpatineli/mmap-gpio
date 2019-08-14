Exposes GPIOController which has on() and off().
Can be used with both beagle bone and pocket beagle. Tested to be able to achieve > 20kHz flipping.
Both beagles have four banks of GPIOs, and it is occupied if you create a controller. Different controllers can be created for different banks.
on() and off() each take a pin number on the bank.
```rust
const PIN_NO: u32 = 23;
let controller = GPIOController::new(GPIOAddrs::GPIO0).expect("failed to mmap to gpio");
controller.on(PIN_NO);
controller.off(PIN_NO);
```
The correspondence between real pin number and the silk mask pin number can be looked up at:
![Pocket Beagle](https://github.com/beagleboard/pocketbeagle/wiki/System-Reference-Manual)
Table 7 and Table 8 (mode 7)

or 

![Beagle Bone](https://components101.com/microcontrollers/beaglebone-black-pinout-datasheet)
Table 1 and Table 2 (mode 7)
