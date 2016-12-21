#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <time.h>
#include "m68k.h"
#include "m68kcpu.h"

/* Read/write macros */
#define READ_BYTE(BASE, ADDR) (BASE)[ADDR]
#define READ_WORD(BASE, ADDR) (((BASE)[ADDR]<<8) |			\
							  (BASE)[(ADDR)+1])
#define READ_LONG(BASE, ADDR) (((BASE)[ADDR]<<24) |			\
							  ((BASE)[(ADDR)+1]<<16) |		\
							  ((BASE)[(ADDR)+2]<<8) |		\
							  (BASE)[(ADDR)+3])

#define WRITE_BYTE(BASE, ADDR, VAL) (BASE)[ADDR] = (VAL)&0xff
#define WRITE_WORD(BASE, ADDR, VAL) (BASE)[ADDR] = ((VAL)>>8) & 0xff;		\
									(BASE)[(ADDR)+1] = (VAL)&0xff
#define WRITE_LONG(BASE, ADDR, VAL) (BASE)[ADDR] = ((VAL)>>24) & 0xff;		\
									(BASE)[(ADDR)+1] = ((VAL)>>16)&0xff;	\
									(BASE)[(ADDR)+2] = ((VAL)>>8)&0xff;		\
									(BASE)[(ADDR)+3] = (VAL)&0xff


unsigned int cpu_read_byte(unsigned int address);
unsigned int cpu_read_word(unsigned int address);
unsigned int cpu_read_long(unsigned int address);
void cpu_write_byte(unsigned int address, unsigned int value);
void cpu_write_word(unsigned int address, unsigned int value);
void cpu_write_long(unsigned int address, unsigned int value);

#define MAX_ROM 0xfff
unsigned char* g_rom;

void exit_error(char* fmt, ...)
{
	static int guard_val = 0;
	char buff[100];
	unsigned int pc;
	va_list args;

	if(guard_val)
		return;
	else
		guard_val = 1;

	va_start(args, fmt);
	vfprintf(stderr, fmt, args);
	va_end(args);
	fprintf(stderr, "\n");
	pc = m68k_get_reg(NULL, M68K_REG_PPC);
	m68k_disassemble(buff, pc, M68K_CPU_TYPE_68000);
	fprintf(stderr, "At %04x: %s\n", pc, buff);

	exit(EXIT_FAILURE);
}


/* Read data from RAM, ROM, or a device */
unsigned int m68k_read_memory_8(unsigned int address)
{
	//printf("m68k_read_memory_8: %08x\n", address);
	if (address > MAX_ROM)
		exit_error("Attempted to read byte from ROM address %08x", address);

	return READ_BYTE(g_rom, address);
}

unsigned int m68k_read_memory_16(unsigned int address)
{
	//printf("m68k_read_memory_16: %08x\n", address);
	if (address > MAX_ROM) {
		printf("A0 %08x\n", REG_A[0]);
		exit_error("Attempted to read word from ROM address %08x", address);
	}

	return READ_WORD(g_rom, address);
}

unsigned int m68k_read_memory_32(unsigned int address)
{
	//printf("m68k_read_memory_32: %08x\n", address);
	if(address > MAX_ROM)
		exit_error("Attempted to read long from ROM address %08x", address);

	return READ_LONG(g_rom, address);
}


unsigned int m68k_read_disassembler_16(unsigned int address)
{
	//printf("%08x\n", address);
	if(address > MAX_ROM)
		exit_error("Disassembler attempted to read word from ROM address %08x", address);

	return READ_WORD(g_rom, address);
}

unsigned int m68k_read_disassembler_32(unsigned int address)
{
	//printf("%08x\n", address);
	if(address > MAX_ROM)
		exit_error("Dasm attempted to read long from ROM address %08x", address);

	return READ_LONG(g_rom, address);
}


/* Write data to RAM or a device */
void m68k_write_memory_8(unsigned int address, unsigned int value)
{
	(void)address;
	(void)value;
}

void m68k_write_memory_16(unsigned int address, unsigned int value)
{
	(void)address;
	(void)value;
}

void m68k_write_memory_32(unsigned int address, unsigned int value)
{
	(void)address;
	(void)value;
}


void m68k_wrapper_init() {
	m68k_init();
	m68k_set_cpu_type(M68K_CPU_TYPE_68000);
	m68k_pulse_reset();
	printf("emulator init\n");
}

void m68k_run_instructions(void* data, int inst_count, int* cycle_result)
{
	g_rom = data;
	m68k_pulse_reset();
	m68k_execute_inst_count(inst_count, cycle_result);
	/*
	int pc = 0;

	for (int i = 0; i < inst_count; ++i) {
		char buff[100];
		pc += m68k_disassemble(buff, pc, M68K_CPU_TYPE_68000);
		printf("inst %04d - %s - %d\n", i, buff, pc);
	}
	*/
}
