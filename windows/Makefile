.PHONY: all clean

CANDLE=candle
LIGHT=light
VERSION=1.0

%.wixobj: %.wxs
	# Convert to Tab delimited file
	$(CANDLE) $< -dVERSION=$(VERSION)


%.msi: %.wixobj
	# Convert to Tab delimited file
	$(LIGHT) -ext WixUIExtension  $<

Odel.msi: Odel.wixobj

all: Odel.msi

debug: Odel.msi
	msiexec /i Odel.msi /L*v  Odel_debug.log

clean:
	-rm -f Odel.msi Odel.wixobjs

