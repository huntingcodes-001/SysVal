#include <iostream>
#include <cstdlib>

int main() {
    std::system("nvidia-smi --query-gpu=utilization.gpu,temperature.gpu --format=csv");
    return 0;
}
