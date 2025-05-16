using NAudio.Wave;
using Screenslimer.Properties;
using SpotifyAPI.Web;
using System.Diagnostics;
using System.IO;
using System.Net.Http;
using System.Text.RegularExpressions;
using System.Windows;
using System.Windows.Forms;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Threading;

namespace Screenslimer
{
    public partial class MainWindow : Window {

        // Configuration //
        private static BackgroundMode BACKGROUND_MODE = BackgroundMode.Auto;
        private static int BUFFER_SIZE_KILOBYTES = 256;
        // Custom Data Types //
        private enum BackgroundMode {
            Auto,
            White,
            Black
        }
        private struct Slimage {
            public string Artist;
            public string Path;
            public Slimage(string artist, string path) {
                this.Artist = artist;
                this.Path = path;
            }
        }
        // Internal variables //
        private DispatcherTimer slideshowTimer;
        private string[] imageFolders;
        private List<Slimage> slimages;
        private static Random RNG = new Random();
        private LibrespotService librespotService;
        private SpotifyService spotifyService;
        private ConfigWindow? configWindow = null;
        // Methods & Services //
        private void SwapImage(object sender, EventArgs e) {
            int slideshowIndex = RNG.Next(slimages.Count);
            ArtistLabel.Text = slimages[slideshowIndex].Artist;
            SlideshowImage.Source = new BitmapImage(new Uri(slimages[slideshowIndex].Path, UriKind.Absolute));
            switch ((BackgroundMode)Settings.Default.BackgroundMode) {
                case BackgroundMode.Auto:
                    BackgroundRectangle.Fill = new SolidColorBrush(GetBorderColor());
                    break;
                case BackgroundMode.White:
                    BackgroundRectangle.Fill = new SolidColorBrush(Colors.White);
                    break;
                case BackgroundMode.Black:
                    BackgroundRectangle.Fill = new SolidColorBrush(Colors.Black);
                    break;
            }
        }
        private Color GetBorderColor() {
            BitmapSource source = SlideshowImage.Source as BitmapSource;
            if (source.Format != PixelFormats.Bgra32) {
                source = new FormatConvertedBitmap(source, PixelFormats.Bgra32, null, 0);
            }
            int bpp = (source.Format.BitsPerPixel + 7) / 8;
            int stride = source.PixelWidth * bpp;
            byte[] pixels = new byte[stride * source.PixelHeight];
            source.CopyPixels(pixels, stride, 0);
            Color[] corners = new Color[] {
                GetPixel(pixels, 0, 0, stride),
                GetPixel(pixels, source.PixelWidth - 1, 0, stride),
                GetPixel(pixels, 0, source.PixelHeight - 1, stride),
                GetPixel(pixels, source.PixelWidth - 1, source.PixelHeight - 1, stride)
            };
            //average the color
            int r = 0, g = 0, b = 0;
            foreach (Color color in corners) {
                r += color.R;
                b += color.B;
                g += color.G;
            }
            r /= 4; g /= 4; b /= 4;
            return Color.FromArgb(255, (byte)r, (byte)g, (byte)b);
        }
        private Color GetPixel(byte[] pixels, int x, int y, int stride) {
            int index = y * stride + x * 4; // 4 bytes per pixel: B, G, R, A
            byte b = pixels[index];
            byte g = pixels[index + 1];
            byte r = pixels[index + 2];
            byte a = pixels[index + 3];
            return Color.FromArgb(a, r, g, b);
        }
        private async Task<BitmapImage> GetAlbumArtAsync(string albumId) {
            string url = $"https://i.scdn.co/image/{albumId}";
            var image = new BitmapImage();
            using (var stream = await new HttpClient().GetStreamAsync(url)) {
                image.BeginInit();
                image.CacheOption = BitmapCacheOption.OnLoad;
                image.StreamSource = stream;
                image.EndInit();
            }
            return image;
        }
        private void OnSongChanged(string trackId) {
            Dispatcher.Invoke(async () =>
            {
                string[] data = await spotifyService.GetSongMetadata(trackId);
                SongLabel.Text = "🎵 " + data[0];
                BandLabel.Text = "👤 " + data[1];
                AlbumLabel.Text = "💿 " + data[2];
                //TODO: album art
            });
        }
        private class LibrespotService {
            private Process librespotProcess;
            private BufferedWaveProvider waveProvider;
            private WaveOutEvent outputDevice;
            public event Action<string> SongChanged;
            public void Start() {
                //start NAudio stream input
                outputDevice = new WaveOutEvent();
                waveProvider = new BufferedWaveProvider(new WaveFormat(44100, 16, 2)) { //44.1khz, 16bit, 2 channel
                    BufferLength = 1024 * BUFFER_SIZE_KILOBYTES
                };
                outputDevice.Init(waveProvider);
                outputDevice.Play();

                //start librespot
                librespotProcess = new Process();
                librespotProcess.StartInfo = new ProcessStartInfo {
                    FileName = "librespot/librespot.exe",
                    Arguments = "--name Screenslimer --backend pipe",
                    RedirectStandardOutput = true,
                    RedirectStandardError = true,
                    UseShellExecute = false,
                    CreateNoWindow = true
                };
                librespotProcess.ErrorDataReceived += OnOutputReceived;
                librespotProcess.Start();
                librespotProcess.BeginErrorReadLine();

                Task.Run(() => StreamAudio(librespotProcess.StandardOutput.BaseStream));
            }

            private async Task StreamAudio(Stream pcmStream) {
                byte[] buffer = new byte[4096];
                int bytesRead;
                try {
                    while ((bytesRead = await pcmStream.ReadAsync(buffer, 0, buffer.Length)) > 0) {
                        //wait if buffer full
                        while (waveProvider.BufferedBytes > waveProvider.BufferLength - buffer.Length) {
                            await Task.Delay(10);
                        }

                        waveProvider.AddSamples(buffer, 0, bytesRead);
                    }
                } catch (Exception ex) {
                    Debug.WriteLine($"[NAudio] Streaming failed: {ex.Message}");
                }
            }
            private void OnOutputReceived(object sender, DataReceivedEventArgs e) {
                if (String.IsNullOrWhiteSpace(e.Data)) return;
                Debug.WriteLine($"[librespot] {e.Data}");
                var match = Regex.Match(e.Data, @"spotify:track:([a-zA-Z0-9]+)");
                if (match.Success) {
                    string trackId = match.Groups[1].Value;
                    SongChanged?.Invoke(trackId);
                }
            }
            public void Stop() {
                outputDevice?.Stop();
                outputDevice?.Dispose();
                outputDevice = null;

                if (librespotProcess != null && !librespotProcess.HasExited) {
                    librespotProcess.Kill();
                    librespotProcess.Dispose();
                }
            }
        }
        private class SpotifyService {
            private const string CLIENT_ID = "03bb318e372940c18b7a940386ede29c";
            private const string CLIENT_SECRET = "53343669dc5749578617045a9e81fe06";
            private SpotifyClient client = null;
            public async Task Start() {
                var config = SpotifyClientConfig.CreateDefault();
                var request = new ClientCredentialsRequest(CLIENT_ID, CLIENT_SECRET);
                var response = await new OAuthClient(config).RequestToken(request);
                client = new SpotifyClient(config.WithToken(response.AccessToken));
            }
            public async Task<string[]> GetSongMetadata(string trackId) {
                string title = "";
                string artist = "";
                string album = "";
                try {
                    var track = await client?.Tracks.Get(trackId);
                    title = track.Name;
                    artist = track.Artists.FirstOrDefault()?.Name ?? "Unknown Artist";
                    album = track.Album.Name;
                }
                catch (Exception ex) {
                    Debug.WriteLine($"metadata fetch failed for track id {trackId}, error message:", ex);
                }
                //TODO: album art
                return new string[] {title, artist, album};
            }
        }
        // Main //
        public MainWindow() {
            InitializeComponent();

            // initialize, load images to memory
            imageFolders = Directory.GetDirectories(AppContext.BaseDirectory + "slime\\");
            slimages = new List<Slimage>();
            foreach (string folderPath in imageFolders) {
                foreach (string imagePath in Directory.GetFiles(folderPath)) {
                    string artistName = new DirectoryInfo(imagePath).Parent.Name;
                    slimages.Add(new Slimage(artistName, imagePath));
                }
            }

            // clean slate
            SongLabel.Text = "";
            BandLabel.Text = "";
            AlbumLabel.Text = "";
            ArtistLabel.Text = "";

            // start slideshow
            slideshowTimer = new DispatcherTimer();
            slideshowTimer.Interval = TimeSpan.FromSeconds(Settings.Default.SlideshowInterval);
            slideshowTimer.Tick += new EventHandler(SwapImage);
            slideshowTimer.Start();

            // start spotify services
            spotifyService = new SpotifyService();
            spotifyService.Start();

            // start librespot
            librespotService = new LibrespotService();
            librespotService.SongChanged += OnSongChanged;
            librespotService.Start();
        }

        protected override void OnClosed(EventArgs e) {
            librespotService?.Stop();
            base.OnClosed(e);
        }

        private void Window_KeyDown(object sender, System.Windows.Input.KeyEventArgs e) {
            if (e.Key == System.Windows.Input.Key.F1) {
                if (configWindow == null) {
                    configWindow = new ConfigWindow();
                    configWindow.Owner = this;
                    configWindow.ShowDialog();
                    configWindow = null;
                }
            }
        }

    }
}