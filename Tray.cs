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
        public Tray()
        {
            InitializeComponent();
            InitForm();
        }

        private void InitForm()
        {
            WindowState = FormWindowState.Minimized;
            ShowInTaskbar = false;
            Visible = false;

            notifyIcon1.Visible = true;
            notifyIcon1.ContextMenuStrip = contextMenuStrip1;
            ShowNotify();
        }

        private void ShowNotify()
        {
            if (MessageBox.Show("프로그램이 실행되었습니다. 바로 ㄱㄱ할까요?", "기능 실행", MessageBoxButtons.YesNo, MessageBoxIcon.Question) == DialogResult.Yes)
            {
                광고날리기ToolStripMenuItem.PerformClick();
            }
            MessageBox.Show("트레이에 아이콘을 확인해주세요.", "안내", MessageBoxButtons.OK, MessageBoxIcon.Information);
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
            //창을 찾아서 hwnd 리턴
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
                _ = EnumChildWindows(kakaoMainHandle, EnumWindowsProc, IntPtr.Zero);
                System.Threading.Thread.Sleep(5000);
            }
            while (자동갱신ToolStripMenuItem.Checked);

            bool EnumWindowsProc(IntPtr hwnd, IntPtr lParam)
            {
                StringBuilder ClassName = new StringBuilder(100);
                _ = GetClassName(hwnd, ClassName, ClassName.Capacity);

                string name = ClassName.ToString();
                
                //여기가 그 광고부분 긁어내는 부분인데, 나중에 버전업으로 인해 해당 name이 다를경우 업뎃해야 함.
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
