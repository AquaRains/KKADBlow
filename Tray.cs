using System;
using System.Runtime.CompilerServices;
using System.Text;
using System.Windows.Forms;
using static PInvoke.User32;

namespace KKADBlow
{
    public partial class Tray : Form
    {
        public const int ErrorLimit = 5;
        public delegate void TimerInvoker();

        private System.Timers.Timer _timer;
        private int _errorCount;

        WindowsCommandParams kkParams;

        public Tray()
        {
            InitializeComponent();
            Init();
        }

        private void Init()
        {
            WindowState = FormWindowState.Minimized;
            ShowInTaskbar = false;
            Visible = false;

            notifyIcon1.Visible = true;
            notifyIcon1.ContextMenuStrip = contextMenuStrip1;
            notifyIcon1.ShowBalloonTip(5000, "실행 확인", "프로그램이 실행되었습니다.", ToolTipIcon.Info);

            _timer = new System.Timers.Timer(10000);
            _timer.Elapsed += (_, _) => BeginInvoke(new TimerInvoker(Doit));

            _timer.Start();
            _timer.Enabled = 자동갱신ToolStripMenuItem.Checked;
        }

        private void 종료XToolStripMenuItem_Click(object sender, EventArgs e) => this.Close();

        private void 광고날리기ToolStripMenuItem_Click(object sender, EventArgs e) => BeginInvoke(new TimerInvoker(Doit));

        private void 자동갱신ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            자동갱신ToolStripMenuItem.Checked = !자동갱신ToolStripMenuItem.Checked;
            this._timer.Enabled = 자동갱신ToolStripMenuItem.Checked;
        }


        public void Doit()
        {
            if (!kkParams.MainHandleFound || kkParams.MainHandle != FindWindow("EVA_Window_Dblclk", "카카오톡"))
            {
                kkParams.MainHandle = FindWindow("EVA_Window_Dblclk", "카카오톡");
                if (kkParams.MainHandleFound)
                {
                    notifyIcon1.ShowBalloonTip(5000, "성공", "카톡 창을 찾았습니다. 작업에 들어갑니다!", ToolTipIcon.Info);
                    _errorCount = 0;
                }
                else
                {
                    notifyIcon1.ShowBalloonTip(5000, "오류", $"카톡 프로그램을 찾을 수 없습니다. {Environment.NewLine}({_errorCount++ + 1}번째 시도, {ErrorLimit}회 누적시 종료됨).", ToolTipIcon.Error);
                    if (_errorCount > ErrorLimit) this.Close();
                    return;
                }
            }

            kkParams.HwndAdArea = FindWindowEx(kkParams.MainHandle, IntPtr.Zero, "BannerAdWnd", null);

            if (kkParams.AdHandleFound)
                unsafe
                {
                    delegate*<IntPtr, IntPtr, bool> cmd = &EnumWindowsCommand;

                    _ = ShowWindow(kkParams.HwndAdArea, WindowShowStyle.SW_HIDE);
                    _ = GetWindowRect(kkParams.HwndAdArea, out kkParams.RectAdArea);

                    fixed (WindowsCommandParams* v = &kkParams)
                    {
                        _ = EnumChildWindows(kkParams.MainHandle, lpEnumFunc: (IntPtr)cmd, (IntPtr)v);
                    }
                }
            else
            {
                //kkParams.MainHandle = IntPtr.Zero;
                if (_errorCount++ > ErrorLimit)
                {
                    notifyIcon1.ShowBalloonTip(5000, "오류", "카톡 창은 찾았는데, 로그인되지 않았거나 광고 부분을 찾지 못했습니다." + Environment.NewLine + "자동실행이 중지됩니다.", ToolTipIcon.Error);
                    this._timer.Enabled = false;
                    자동갱신ToolStripMenuItem.Checked = false;
                }
            }
        }

        private static unsafe bool EnumWindowsCommand(IntPtr hwnd, IntPtr lParam)
        {
            WindowsCommandParams* p = (WindowsCommandParams*)lParam;
            var v = new char[255];
            _ = GetClassName(hwnd, v);
            string className = new string(v).TrimEnd('\0');

            if (GetParent(hwnd) == p->MainHandle && hwnd != p->HwndAdArea && className == "EVA_ChildWindow")
            {
                _ = GetWindowRect(hwnd, out var currentRect);
                _ = SetWindowPos(hwnd, hWndInsertAfter: (IntPtr)1, 0, 0, currentRect.right - currentRect.left,
                    p->RectAdArea.bottom - currentRect.top, SetWindowPosFlags.SWP_NOMOVE);
            }
            return true;
        }
    }
}
