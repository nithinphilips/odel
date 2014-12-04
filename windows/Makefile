.PHONY: all clean

PROGRAM=Odel

CANDLE=candle
LIGHT=light
VERSION=0.2.0

%.wixobj: %.wxs
	# Convert to Tab delimited file
	$(CANDLE) $< -dVERSION=$(VERSION)


%.msi: %.wixobj
	# Convert to Tab delimited file
	$(LIGHT) -ext WixUIExtension  $<

$(PROGRAM).msi: $(PROGRAM).wixobj

all: $(PROGRAM).msi

debug: $(PROGRAM).msi
	msiexec /i $(PROGRAM).msi /L*v  $(PROGRAM)_debug.log

clean:
	-rm -f $(PROGRAM).msi $(PROGRAM).wixobj $(PROGRAM).wixpdb

