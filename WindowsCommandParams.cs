using PInvoke;
using System;
using System.Runtime.InteropServices;

namespace KKADBlow
{
    [StructLayout(LayoutKind.Sequential)]
    public struct WindowsCommandParams
    {
        public IntPtr MainHandle;
        public IntPtr HwndAdArea;
        public RECT RectAdArea;
        public bool MainHandleFound => !MainHandle.Equals(IntPtr.Zero);
        public bool AdHandleFound => !HwndAdArea.Equals(IntPtr.Zero);
    }
}
