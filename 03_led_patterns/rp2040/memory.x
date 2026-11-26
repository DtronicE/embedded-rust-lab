/* Raspberry Pi Pico (RP2040) with 2 MiB external QSPI flash, executed in place (XIP). */
MEMORY
{
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100          /* 256-byte second-stage bootloader */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* Place the boot2 blob at the very start of flash, before .vector_table / .text */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
