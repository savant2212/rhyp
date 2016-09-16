OUTPUT_ARCH(arm)
ENTRY(_entry)

SECTIONS {
	.text : {
		*.(text)
	}
	.rodata : { *(.rodata .rodata.*) }
}

