#include <Arduino_MKRIoTCarrier.h>

MKRIoTCarrier carrier;

[[noreturn]] void halt()
{
    while (true) {
        delay(1);
    }
}

void setup() {
    Serial.begin(115200);
    if (carrier.begin() != 0) {
        Serial.println("error: carrier.begin() failed");
    }
}

template <typename T>
String& operator<<(String& rhs, T&& lhs)
{
    rhs += lhs;
    return rhs;
}

void loop() {
    if (carrier.IMUmodule.accelerationAvailable()
        && carrier.IMUmodule.gyroscopeAvailable()) {

        float x, y, z;
        float a, b, c;
        carrier.IMUmodule.readAcceleration(x, y, z);
        carrier.IMUmodule.readGyroscope(a, b, c);

        String packet;
        packet
            << x * 200
            << ";" << y * 200
            << ";" << z * 200
            << ";" << a
            << ";" << b
            << ";" << c;
        Serial.println(packet);
    }
}
