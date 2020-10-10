#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t doub(int32_t x);

char *inp(const char *prompt);

void free_inpu(char *s);

} // extern "C"
