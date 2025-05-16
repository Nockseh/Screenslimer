using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Shapes;

namespace Screenslimer {
    public partial class ConfigWindow : Window {
        public ConfigWindow() {
            InitializeComponent();
        }

        private void Save_Click(object sender, RoutedEventArgs e) {
            Properties.Settings.Default.SlideshowInterval = Convert.ToInt16(SlideshowIntervalTextBox.Text);
            Properties.Settings.Default.BackgroundMode = BackgroundModeComboBox.SelectedIndex;
            Properties.Settings.Default.AudioBufferSize = Convert.ToInt16(BufferSizeTextBox.Text);
            Properties.Settings.Default.Save();
        }
    }
}
