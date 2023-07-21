#include <stdio.h>
#include <stdint.h>
#include "bindings.h"

int main() {
    auto tire = create_random_tire();
    const char *materials[] = { "Plastic", "Rubber" };

    printf("Tire pressure: %.2f\n", tire.pressure);
    printf("Tire material: %s\n", materials[(unsigned int)tire.material]);
    printf("Tire size: %.2fw, %.2fh\n", tire.size.width, tire.size.height);
    printf("The tire have an aspect ratio of %.2f\n",wheel_size_aspect(tire.size.width, tire.size.height));
    uint8_t values[3] = {77, 42, 69};
    format_wheel_identifier(&values);

    func_with_no_return();

    return 0;
}
