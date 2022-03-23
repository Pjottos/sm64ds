#include "primitives.h"

extern void FUN_020577fc();
extern void FUN_02057c34();
extern void FUN_020575ec(void (*)(), void *);
extern void FUN_02019d4c();
extern void FUN_0203a5a4(u32, u32);
extern void FUN_02019d74();
extern void FUN_020195f0();

extern const u32 DAT_02098f9c;

extern "C" void os_main() {
    FUN_020577fc();
    FUN_02057c34();
    // TODO: resolve hardcoded address pointing outside the arm9 binary
    FUN_020575ec(&FUN_02019d4c, reinterpret_cast<void *>(0x0209e2dc));
    FUN_0203a5a4(DAT_02098f9c, 2);
    FUN_02019d74();
    FUN_0203a5a4(DAT_02098f9c, 3);
    FUN_020195f0();
}
