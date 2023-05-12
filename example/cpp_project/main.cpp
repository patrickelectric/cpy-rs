#include <stdio.h>
#include <stdint.h>
#include "bindings.h"

int main() {
    auto tire = create_random_tire();
    const char *materials[] = { "Plastic", "Rubber" };

    printf("Tire pressure: %f\n", tire.pressure);
    printf("Tire material: %s\n", materials[(unsigned int)tire.material]);
    printf("Tire size: %fw, %fh\n", tire.size.width, tire.size.height);

    return 0;
}