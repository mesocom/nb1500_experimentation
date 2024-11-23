MEMORY
{
  /* FLASH (rx) : ORIGIN = 0x2000, LENGTH = 256K - 8K /* First 8KB used by bootloader */
  FLASH (rx) : ORIGIN = 0x0000, LENGTH = 256K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
}