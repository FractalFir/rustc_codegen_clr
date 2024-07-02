var globalMemoryArray = new ArrayBuffer(4096 + 4096 * 8 + 4096 * 64);
var globalMemory = new DataView(globalMemoryArray);
var stackPointer = 4096n;
var heapPointer = 4096n + 4096n * 8n;
function System_Runtime_InteropServices_NativeMemory_AlignedAlloc_usus(size, align) {
    // Round up
    if (heapPointer % align != 0) {
        const freeBlockSize = heapPointer % align;
        const freeBlockStart = heapPointer;
        heapPointer += freeBlockSize;
    }
    const newBuffer = heapPointer;
    heapPointer += size;
    console.log(`allocating a buffer of size ${size} align ${align} at address ${newBuffer}`);
    return newBuffer;
}
function System_Environment_GetCommandLineArgs_() {
    if (process) {
        return process.argv;
    } else {
        return [""];
    }
}
function i32_to_u32(i32) {
    return (i32 & 2147483647) | Math.sign(i32) << 32;
}