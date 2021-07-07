using System;
using System.Text;
using System.Timers;
using System.Windows.Forms;
using static KKADBlow.W32;

namespace KKADBlow
{
    public partial class Tray : Form
    {
        private System.Timers.Timer _timer;
        int _errorcount = 0;
        public const int ErrorLimit = 5;
        IntPtr _kakaoMainHandle;

        public delegate void TimerInvoker();

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
            _timer.Elapsed += (object _, ElapsedEventArgs _) => BeginInvoke(new TimerInvoker(Doit));

            _timer.Start();
            _timer.Enabled = 자동갱신ToolStripMenuItem.Checked;
        }


        //ini파일 읽고 쓰기 넣을 예정

        //hwnd용 클래스 임의로 넣기

        private void 종료XToolStripMenuItem_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        public void Doit()
        {
            if (_kakaoMainHandle.Equals(IntPtr.Zero))
            {
                _kakaoMainHandle = FindWindow("EVA_Window_Dblclk", "카카오톡");
                if (_kakaoMainHandle.Equals(IntPtr.Zero))
                {
                    notifyIcon1.ShowBalloonTip(5000, "오류", $"카톡 프로그램을 찾을 수 없습니다. {Environment.NewLine}({_errorcount++}번째 시도, {ErrorLimit}회 누적시 종료됨).", ToolTipIcon.Error);

                    if (_errorcount > ErrorLimit) this.Close();
                    return;
                }
                else
                {
                    notifyIcon1.ShowBalloonTip(5000, "성공", "카톡 창을 찾았습니다. 작업에 들어갑니다!", ToolTipIcon.Info);
                    _errorcount = 0;
                }
            }


            IntPtr KakaoAD = FindWindowEx(_kakaoMainHandle, IntPtr.Zero, "BannerAdWnd", null);

            if (KakaoAD.Equals(IntPtr.Zero))
                _kakaoMainHandle = IntPtr.Zero;

            _ = ShowWindow(KakaoAD, ShowWindowCommands.Hide);
            //  W32.EnableWindow(KakaoAD, true);

            RECT ADrect;
            
            _ = GetWindowRect(KakaoAD, out ADrect);
            _ = EnumChildWindows(_kakaoMainHandle, EnumWindowsCommand, IntPtr.Zero);
            

            bool EnumWindowsCommand(IntPtr hwnd, IntPtr lParam)
            {
                StringBuilder className = new StringBuilder(100);

                _ = GetClassName(hwnd, className, className.Capacity);

                string name = className.ToString();

                if (GetParent(hwnd) == _kakaoMainHandle && hwnd != KakaoAD && name == "EVA_ChildWindow")
                {
                    _ = GetWindowRect(hwnd, out RECT CurrentRect);

                    _ = SetWindowPos(hwnd, HWND_BOTTOM, 0, 0, CurrentRect.Right - CurrentRect.Left, ADrect.Bottom - CurrentRect.Top, DeferWindowPosCommands.SWP_NOMOVE);
                }
                return true;
            }

        }

        private void 광고날리기ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            BeginInvoke(new TimerInvoker(Doit));
        }

        private void 자동갱신ToolStripMenuItem_CheckedChanged(object sender, EventArgs e)
        {
          
        } 

        private void 자동갱신ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            자동갱신ToolStripMenuItem.Checked = !자동갱신ToolStripMenuItem.Checked;
        }
    }
}
