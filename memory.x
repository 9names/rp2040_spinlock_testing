MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM : ORIGIN    = 0x20000000, LENGTH = 63K
    PANDUMP: ORIGIN = 0x2000FC00, LENGTH = 1K
}

_panic_dump_start = ORIGIN(PANDUMP);
_panic_dump_end   = ORIGIN(PANDUMP) + LENGTH(PANDUMP);