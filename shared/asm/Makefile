NASM = nasm
BFLAGS = -f bin
OFLAGS = -f elf32
# OFLAGS = -f win32

BINDIR = ../bin
# LISTDIR = ../list
OBJDIR = ../obj

ASM_FUNC = osfunc

all : secondboot.asm ipl.asm func

%.asm :
	$(NASM) $(BFLAGS) $*.asm -o $(BINDIR)/$*.bin

func :
	$(NASM) $(OFLAGS) $(ASM_FUNC).asm -o $(OBJDIR)/$(ASM_FUNC).o

clean :
	rm $(BINDIR)/*.bin
#	rm $(LISTDIR)/*.lst
