SECTIONS {
  . = ALIGN(4);
  .metadata : {
    KEEP(*(.metadata))
  }
  . = ALIGN(4);
}
INSERT AFTER .text;
