using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Windows.Forms;
using static KKADBlow.W32;

namespace KKADBlow
{
    public partial class Tray : Form
    {
        private System.Timers.Timer timer;
        int errorcount = 0;
        static int ErrorLimit = 5;
        IntPtr kakaoMainHandle;

        delegate void TimerInvoker();

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

            timer = new System.Timers.Timer(10000);
            timer.Elapsed += (sender, e) => BeginInvoke(new TimerInvoker(doit));

            timer.Start();
            timer.Enabled = 자동갱신ToolStripMenuItem.Checked;
        }


        //ini파일 읽고 쓰기 넣을 예정

        //hwnd용 클래스 임의로 넣기

        private void 종료XToolStripMenuItem_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        public void doit()
        {
            if (kakaoMainHandle.Equals(IntPtr.Zero))
            {
                kakaoMainHandle = FindWindow("EVA_Window_Dblclk", "카카오톡");
                if (kakaoMainHandle.Equals(IntPtr.Zero))
                {
                    notifyIcon1.ShowBalloonTip(5000, "오류", $"카톡 프로그램을 찾을 수 없습니다. {Environment.NewLine}({errorcount++}번째 시도, {ErrorLimit}회 누적시 종료됨).", ToolTipIcon.Error);

                    if (errorcount > ErrorLimit) this.Close();
                    return;
                }
                else
                {
                    notifyIcon1.ShowBalloonTip(5000, "성공", "카톡 창을 찾았습니다. 작업에 들어갑니다!", ToolTipIcon.Info);
                    errorcount = 0;
                }
            }


            IntPtr KakaoAD = FindWindowEx(kakaoMainHandle, IntPtr.Zero, "EVA_Window", null);

            if (KakaoAD.Equals(IntPtr.Zero))
                kakaoMainHandle = IntPtr.Zero;

            _ = ShowWindow(KakaoAD, ShowWindowCommands.Hide);
            //  W32.EnableWindow(KakaoAD, true);

            RECT ADrect;


            _ = GetWindowRect(KakaoAD, out ADrect);
            _ = EnumChildWindows(kakaoMainHandle, EnumWindowsCommand, IntPtr.Zero);




            bool EnumWindowsCommand(IntPtr hwnd, IntPtr lParam)
            {
                StringBuilder ClassName = new StringBuilder(100);

                _ = GetClassName(hwnd, ClassName, ClassName.Capacity);

                string name = ClassName.ToString();

                if (GetParent(hwnd) == kakaoMainHandle && hwnd != KakaoAD && name == "EVA_ChildWindow")
                {
                    _ = GetWindowRect(hwnd, out RECT CurrentRect);

                    _ = SetWindowPos(hwnd, HWND_BOTTOM, 0, 0, CurrentRect.Right - CurrentRect.Left, ADrect.Bottom - CurrentRect.Top, DeferWindowPosCommands.SWP_NOMOVE);
                }
                return true;
            }

        }

        private void 광고날리기ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            BeginInvoke(new TimerInvoker(doit));
        }

        private void Worker_DoWork(object sender, DoWorkEventArgs e)
        {
            doit();
        }

        private void 자동갱신ToolStripMenuItem_CheckedChanged(object sender, EventArgs e)
        {
            timer.Enabled = 자동갱신ToolStripMenuItem.Checked;
            
        }

        private void 자동갱신ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            자동갱신ToolStripMenuItem.Checked = !자동갱신ToolStripMenuItem.Checked;
        }
    }
}
