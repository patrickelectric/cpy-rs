#include <stdio.h>
#include <stdint.h>
#include <vector>
#include "bindings.h"

int main() {
    auto tire = create_random_tire();
    const char *materials[] = { "Plastic", "Rubber" };

    printf("Tire pressure: %.2f\n", tire.pressure);
    printf("Tire material: %s\n", materials[(unsigned int)tire.material]);
    printf("Tire size: %.2fw, %.2fh\n", tire.size.width, tire.size.height);
    printf("The tire have an aspect ratio of %.2f\n",wheel_size_aspect(tire.size.width, tire.size.height));
    std::vector<uint8_t> values = {77, 42, 69};
    format_wheel_identifier(reinterpret_cast<const uint8_t (*)[3]>(&values[0]));
    format_size_of_wheels(reinterpret_cast<const uint8_t*>(&values[0]), values.size());
    values.push_back(30);
    format_size_of_wheels(reinterpret_cast<const uint8_t*>(&values[0]), values.size());

    func_with_no_return();

    return 0;
}
