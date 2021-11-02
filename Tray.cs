using System;
using System.Text;
using System.Windows.Forms;
using static KKADBlow.Win32Interop;

namespace KKADBlow
{
    public partial class Tray : Form
    {
        public const int ErrorLimit = 5;
        public delegate void TimerInvoker();

        private System.Timers.Timer _timer;
        private int _errorCount;
        private bool _kakaoMainHandleFound;
        private IntPtr _kakaoMainHandle;

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
        

        private void 종료XToolStripMenuItem_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        public void Doit()
        {
            _kakaoMainHandleFound = !_kakaoMainHandle.Equals(IntPtr.Zero);

            if (!_kakaoMainHandleFound || _kakaoMainHandle != FindWindow("EVA_Window_Dblclk", "카카오톡"))
            {
                _kakaoMainHandle = FindWindow("EVA_Window_Dblclk", "카카오톡");
                if (_kakaoMainHandle.Equals(IntPtr.Zero))
                {
                    _kakaoMainHandleFound = false;
                    notifyIcon1.ShowBalloonTip(5000, "오류", $"카톡 프로그램을 찾을 수 없습니다. {Environment.NewLine}({_errorCount++ + 1}번째 시도, {ErrorLimit}회 누적시 종료됨).", ToolTipIcon.Error);
                    if (_errorCount > ErrorLimit) this.Close();
                    return;
                }
                else
                {
                    _kakaoMainHandleFound = true;
                    notifyIcon1.ShowBalloonTip(5000, "성공", "카톡 창을 찾았습니다. 작업에 들어갑니다!", ToolTipIcon.Info);
                    _errorCount = 0;
                }
            }

            IntPtr hwndKakaoAdArea = FindWindowEx(_kakaoMainHandle, IntPtr.Zero, "BannerAdWnd", null);

            if (hwndKakaoAdArea.Equals(IntPtr.Zero))
            {
                //_kakaoMainHandle = IntPtr.Zero;
                if (_errorCount++ > ErrorLimit)
                {
                    notifyIcon1.ShowBalloonTip(5000, "오류", "카톡 창은 찾았는데, 로그인되지 않았거나 광고 부분을 찾지 못했습니다." + Environment.NewLine + "자동실행이 중지됩니다.", ToolTipIcon.Error);
                    this._timer.Enabled = false;
                    자동갱신ToolStripMenuItem.Checked = false;
                }
            }
            else
            {
                _ = ShowWindow(hwndKakaoAdArea, ShowWindowCommands.Hide);
                _ = GetWindowRect(hwndKakaoAdArea, out var rectAdArea);
                _ = EnumChildWindows(_kakaoMainHandle, EnumWindowsCommand, IntPtr.Zero);


                bool EnumWindowsCommand(IntPtr hwnd, IntPtr lParam)
                {
                    StringBuilder className = new(255);
                    _ = GetClassName(hwnd, className, className.Capacity);

                    var name = className.ToString();
                    if (GetParent(hwnd) == _kakaoMainHandle && hwnd != hwndKakaoAdArea && name == "EVA_ChildWindow")
                    {
                        _ = GetWindowRect(hwnd, out var currentRect);
                        _ = SetWindowPos(hwnd, HWND_BOTTOM, 0, 0, currentRect.Right - currentRect.Left,
                            rectAdArea.Bottom - currentRect.Top, DeferWindowPosCommands.SWP_NOMOVE);
                    }
                    return true;
                }
            }
        }

        private void 광고날리기ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            BeginInvoke(new TimerInvoker(Doit));
        }

        private void 자동갱신ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            자동갱신ToolStripMenuItem.Checked = !자동갱신ToolStripMenuItem.Checked;
            this._timer.Enabled = 자동갱신ToolStripMenuItem.Checked;
        }
    }
}
