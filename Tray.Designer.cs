namespace KKADBlow
{
    partial class Tray
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.components = new System.ComponentModel.Container();
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Tray));
            this.notifyIcon1 = new System.Windows.Forms.NotifyIcon(this.components);
            this.contextMenuStrip1 = new System.Windows.Forms.ContextMenuStrip(this.components);
            this.자동갱신ToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.광고날리기ToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.toolStripMenuItem1 = new System.Windows.Forms.ToolStripSeparator();
            this.종료XToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.contextMenuStrip1.SuspendLayout();
            this.SuspendLayout();
            // 
            // notifyIcon1
            // 
            this.notifyIcon1.Icon = ((System.Drawing.Icon)(resources.GetObject("notifyIcon1.Icon")));
            this.notifyIcon1.Text = "카톡 광고잼";
            this.notifyIcon1.Visible = true;
            // 
            // contextMenuStrip1
            // 
            this.contextMenuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.자동갱신ToolStripMenuItem,
            this.광고날리기ToolStripMenuItem,
            this.toolStripMenuItem1,
            this.종료XToolStripMenuItem});
            this.contextMenuStrip1.Name = "contextMenuStrip1";
            this.contextMenuStrip1.Size = new System.Drawing.Size(139, 76);
            // 
            // 자동갱신ToolStripMenuItem
            // 
            this.자동갱신ToolStripMenuItem.Checked = true;
            this.자동갱신ToolStripMenuItem.CheckState = System.Windows.Forms.CheckState.Checked;
            this.자동갱신ToolStripMenuItem.Name = "자동갱신ToolStripMenuItem";
            this.자동갱신ToolStripMenuItem.Size = new System.Drawing.Size(138, 22);
            this.자동갱신ToolStripMenuItem.Text = "자동 갱신";
            this.자동갱신ToolStripMenuItem.Click += new System.EventHandler(this.자동갱신ToolStripMenuItem_Click);
            // 
            // 광고날리기ToolStripMenuItem
            // 
            this.광고날리기ToolStripMenuItem.Name = "광고날리기ToolStripMenuItem";
            this.광고날리기ToolStripMenuItem.Size = new System.Drawing.Size(138, 22);
            this.광고날리기ToolStripMenuItem.Text = "광고 날리기";
            this.광고날리기ToolStripMenuItem.Click += new System.EventHandler(this.광고날리기ToolStripMenuItem_Click);
            // 
            // toolStripMenuItem1
            // 
            this.toolStripMenuItem1.Name = "toolStripMenuItem1";
            this.toolStripMenuItem1.Size = new System.Drawing.Size(135, 6);
            // 
            // 종료XToolStripMenuItem
            // 
            this.종료XToolStripMenuItem.Name = "종료XToolStripMenuItem";
            this.종료XToolStripMenuItem.Size = new System.Drawing.Size(138, 22);
            this.종료XToolStripMenuItem.Text = "종료(&X)";
            this.종료XToolStripMenuItem.Click += new System.EventHandler(this.종료XToolStripMenuItem_Click);
            // 
            // Tray
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(193, 212);
            this.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
            this.Name = "Tray";
            this.Text = "Tray";
            this.contextMenuStrip1.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.NotifyIcon notifyIcon1;
        private System.Windows.Forms.ContextMenuStrip contextMenuStrip1;
        private System.Windows.Forms.ToolStripMenuItem 자동갱신ToolStripMenuItem;
        private System.Windows.Forms.ToolStripMenuItem 광고날리기ToolStripMenuItem;
        private System.Windows.Forms.ToolStripSeparator toolStripMenuItem1;
        private System.Windows.Forms.ToolStripMenuItem 종료XToolStripMenuItem;
    }
}