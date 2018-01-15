CC = rustc
CFLAGS = --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C opt-level=2 \
         -C no-prepopulate-passes -C no-stack-check -C relocation-model=static -Z verbose -Z no-landing-pads
CFLAGS2 = --extern core=../lib/liblib.rlib

LD = i686-unknown-linux-gnu-ld
LFLAGS = -v -nostdlib -Tdata=0x00310000
LFLAGS2 = -T kernel.ld

INCLUDE = -I../include
SRC = $(wildcard *.rs)
OBJ = $(addprefix $(OBJDIR)/,$(SRC:.rs=.o))
#OBJ = $(OBJDIR)/osfunc.o
BINDIR = ../bin
OBJDIR = ../obj

TARGET = init_os

all :	$(TARGET)

$(TARGET) : $(OBJ) Makefile
	$(LD) $(LFLAGS) $(OBJ) $(OBJDIR)/osfunc.o $(LFLAGS2) -o $(BINDIR)/$(TARGET).bin
$(OBJDIR)/%.o : %.rs Makefile
	$(CC) $(CFLAGS) -o $(OBJDIR)/$*.o $*.rs $(CFLAGS2)

clean :
	rm $(OBJDIR)/*.o
