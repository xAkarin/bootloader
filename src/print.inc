_print:

	mov $0x0e, %ah

_print_loop:
	lodsb
	cmp $0, %al
	je  _print_done
	int $0x10
	jmp _print_loop

_print_done:
	ret
