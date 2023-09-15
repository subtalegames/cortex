#include <windows.h>
#include <cstring>

typedef void (*RustCallback)();

extern "C" {
  struct ExceptionInfo {
    DWORD code;
    PVOID exception_address;
  };
  
  __declspec(dllexport) ExceptionInfo run_with_seh(RustCallback callback) {
    ExceptionInfo exception_info = {0, nullptr};
    EXCEPTION_POINTERS* pException = nullptr;
  
    __try {
      callback();
    } __except(pException = GetExceptionInformation(), EXCEPTION_EXECUTE_HANDLER) {
      exception_info.code = pException->ExceptionRecord->ExceptionCode;
      exception_info.exception_address = pException->ExceptionRecord->ExceptionAddress;
    }
  
    return exception_info;
  }
}