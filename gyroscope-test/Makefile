
compile:
	arduino-cli compile --build-path build/ --verbose --warnings all

upload:
	arduino-cli compile --build-path build/ --verbose --warnings all --upload

monitor:
	arduino-cli monitor -p /dev/ttyACM0

graph:
	arduino-cli monitor -p /dev/ttyACM0 | graphstream "255,255,0;0,0,255;0,255,0;0,255,255;255,0,255;255,0,0"

clean:
	rm -rf build/
