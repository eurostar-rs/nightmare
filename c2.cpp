#include "includes.h"

int main(int argc, wchar_t* argv[])
{

    // USERNAME
    char username[UNLEN + 1];
    DWORD username_len = UNLEN + 1;
    GetUserName((TCHAR*)username, &username_len);
    
    // RAM
    
    memInfo.dwLength = sizeof(MEMORYSTATUSEX);
    GlobalMemoryStatusEx(&memInfo);
    DWORDLONG totalVirtualMem = memInfo.ullTotalPhys;

    // GPU

    wstring GPU = ReadRegValue(HKEY_LOCAL_MACHINE, L"HARDWARE\\DeviceMap\\Video", L"\\Device\\Video0");

    for (int i = 1; i <= 5; i++)
        GPU.pop_back();

    GPU.erase(0, 57);

    wstring location = L"SYSTEM\\CurrentControlSet\\Control\\Video\\";
    wstring full = location + GPU + L"\\0000";

    //OUTPUT IN .TXT

    g << "User: " << (TCHAR*)username << "\n";
    g << "Computer name: " << ReadRegValue(HKEY_LOCAL_MACHINE, L"SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName", L"ComputerName") << "\n";
    g << "Windows Version: " << ReadRegValue(HKEY_LOCAL_MACHINE, L"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", L"ProductName") << "\n";
    g << "Architecture: " << ReadRegValue(HKEY_LOCAL_MACHINE, L"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", L"PROCESSOR_ARCHITECTURE") << "\n";
   // g << "Windows Build Version: " << ReadRegValue(HKEY_LOCAL_MACHINE, L"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", L"DisplayVersion")  << "\n";
    g << "Processor: " << ReadRegValue(HKEY_LOCAL_MACHINE, L"HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", L"ProcessorNameString") << "\n";
    g << "GPU: " << ReadRegValue(HKEY_LOCAL_MACHINE, full, L"HardwareInformation.AdapterString") << "\n";
    g << "Physical memory (RAM): " << totalVirtualMem / 1000000 << " MB\n";
    g << "IP address: ";
    g.close();
    //system("curl api.ipify.org >> PC-Info.txt");

    system("pause");
    return 0;
}