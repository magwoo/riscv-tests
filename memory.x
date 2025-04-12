MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 32M
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

_heap_size = 64K;
_stack_per_hart = 16K;
_max_hart_id = 127;
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
