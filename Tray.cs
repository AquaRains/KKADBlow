using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Windows.Forms;
using static kakaoADFinder.W32;

namespace kakaoADFinder
{
    public partial class Tray : Form
    {
        public Tray()
        {
            InitializeComponent();

            WindowState = FormWindowState.Minimized;
            ShowInTaskbar = false;
            Visible = false;
            notifyIcon1.Visible = true;
            notifyIcon1.ContextMenuStrip = contextMenuStrip1;
        }


        //ini파일 읽고 쓰기 넣을 예정
        //삭제 반복 주기
        //hwnd용 클래스 임의로 넣기


        private void 자동갱신ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            자동갱신ToolStripMenuItem.Checked = !자동갱신ToolStripMenuItem.Checked;
        }

        private void 종료XToolStripMenuItem_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        public void doit()
        {
            IntPtr kakaoMainHandle = FindWindow("EVA_Window_Dblclk", "카카오톡");
            if (kakaoMainHandle.Equals(IntPtr.Zero))
            {
                MessageBox.Show("카톡 프로그램을 찾을 수 없습니다. 작업이 취소됩니다.", "오류", MessageBoxButtons.OK, MessageBoxIcon.Error);
                return;
            }

            IntPtr KakaoAD = FindWindowEx(kakaoMainHandle, IntPtr.Zero, "EVA_Window", null);

            _ = ShowWindow(KakaoAD, ShowWindowCommands.Hide);
            //  W32.EnableWindow(KakaoAD, true);

            RECT ADrect;



            do
            {
                _ = GetWindowRect(KakaoAD, out ADrect);
                _ = EnumChildWindows(kakaoMainHandle, EnumWindowsCommand, IntPtr.Zero);
            }
            while (자동갱신ToolStripMenuItem.Checked);


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
            BackgroundWorker worker = new BackgroundWorker();

            // 이벤트 핸들러 지정
            worker.DoWork += Worker_DoWork;

            // 실행
            worker.RunWorkerAsync();

        }

        private void Worker_DoWork(object sender, DoWorkEventArgs e)
        {
            doit();
        }
    }
}
