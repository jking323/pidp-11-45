const GPIO_BASE: u64 = 0xFE20_0000;

//FSELx - Function Select x
//000 = GPIO Pin x is an input
//001 = GPIO Pin x is an output
//100 = GPIO Pin x takes alternate function 0
//101 = GPIO Pin x takes alternate function 1
//110 = GPIO Pin x takes alternate function 2
//111 = GPIO Pin x takes alternate function 3
//011 = GPIO Pin x takes alternate function 4
//010 = GPIO Pin x takes alternate function 5
//Set GPIO 0-9 Function, each function is 3 bits
pub const GPIO_GPFSEL0: u64 = GPIO_BASE + 0x00;
//Set GPIO 10-19 Function, each function is 3 bits
pub const GPIO_GPFSEL1: u64 = GPIO_BASE + 0x04;
//Set GPIO 20-29 Function, each function is 3 bits
pub const GPIO_GPFSEL2: u64 = GPIO_BASE + 0x08;
//Set GPIO 30-39 Function, each function is 3 bits
pub const GPIO_GPFSEL3: u64 = GPIO_BASE + 0x0C;
//Set GPIO 40-49 Function, each function is 3 bits
pub const GPIO_GPFSEL4: u64 = GPIO_BASE + 0x10;

//Set GPIO 0-31
pub const GPIO_GPSET0: u64 = GPIO_BASE + 0x1C;
pub const GPIO_GPSET1: u64 = GPIO_BASE + 0x20;

// Set GPIO 0-31 LOW
pub const GPIO_GPCLR0: u64 = GPIO_BASE + 0x28;
pub const GPIO_GPCLR1: u64 = GPIO_BASE + 0x2C;

//Set GPIO 0-31 High or Low
pub const GPIO_GPLEV0: u64 = GPIO_BASE + 0x34;
pub const GPIO_GPLEV1: u64 = GPIO_BASE + 0x38;
