# Locus

> **Attribution**: This project is based on [locus](https://github.com/Sushants-Git/locus)

**A productivity tracker for macOS that reveals where your time truly goes.**

Locus has 2 simple goals: *tracking your focused moments* and *unveiling where the rest of your time silently flows.*

Built specifically for **macOS Apple Silicon** with native window detection and seamless activity tracking.

![Locus App Screenshot](https://github.com/user-attachments/assets/0f0d7f0a-76c3-46cb-a253-ec0df2deee2d)

## 🖥️ System Requirements

**macOS Apple Silicon Only**
- macOS Sonoma (14.0) or later
- Apple Silicon Mac (M1/M2/M3/M4)
- Screen Recording permission (granted on first launch)
- ~50MB storage space

## 📥 Installation

### Option 1: Download Pre-built App (Recommended)

1. **Download the latest release**
   - Visit the [Releases page](https://github.com/charanreddy-git/locus/releases)
   - Download `locus_0.2.2_aarch64.dmg` for Apple Silicon

2. **Install the app**
   - Open the downloaded `.dmg` file
   - Drag `locus.app` to your Applications folder
   - Eject the disk image

3. **Launch Locus**
   - Open Applications and double-click Locus
   - Or press `Cmd+Space`, type "Locus", and press Enter

### Option 2: Build from Source

If you prefer to build from source or want the latest features:

#### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/docs/installation) or Node.js
- [Git](https://git-scm.com/)

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/charanreddy-git/locus.git
cd locus

# Install dependencies
bun install  # or: npm install

# Build the application
bun run tauri build  # or: npm run tauri build

# Install to Applications folder
cp -r src-tauri/target/release/bundle/macos/locus.app /Applications/
```


## 🚀 Getting Started

### First Launch Setup

1. **Grant Screen Recording Permission**
   - When you first open Locus, macOS will request Screen Recording access
   - Click "Open System Preferences" → Privacy & Security → Screen Recording
   - Enable the checkbox next to Locus
   - Restart Locus for changes to take effect

2. **Start Tracking**
   - Launch Locus from Applications or Spotlight search
   - The app will begin monitoring your active applications automatically
   - Use the built-in Pomodoro timer for focused work sessions

### Usage Tips

**🍅 Pomodoro Timer**
- Click the play button to start a focus session
- Customize work and break durations in Settings
- View your productivity patterns in the timeline

**📊 Activity Timeline** 
- Hover over timeline bars to see time spent
- Adjust minimum activity duration to filter short distractions
- Click the chart icon for detailed analytics

**⚙️ Settings**
- Customize colors, durations, and activity thresholds
- Export your data or clear history as needed
- Set up auto-start for seamless tracking

### Privacy & Data

- **100% Local**: All data stays on your Mac
- **No Internet Required**: Works completely offline
- **No Tracking**: Zero data collection or analytics
- **Secure Storage**: Data saved to `~/Library/Application Support/Locus/`

## ✨ Key Features

### 🎯 **Focus Tracking**
- **Automatic Window Detection**: Seamlessly tracks your active applications
- **Native macOS Integration**: Built specifically for Apple Silicon performance
- **Smart Activity Filtering**: Ignores brief distractions for accurate insights

### 🍅 **Pomodoro Timer**
- **Customizable Sessions**: Set your ideal work and break durations
- **Visual Progress**: Clean, distraction-free timer interface  
- **Session History**: Track your focus streaks over time

### 📈 **Analytics & Insights**
- **Beautiful Timeline**: Visual representation of your daily activities
- **Detailed Breakdowns**: See exactly where your time goes
- **Productivity Metrics**: Understand your focus patterns and habits

### 🔒 **Privacy First**
- **Completely Offline**: No internet connection required
- **Local Data Storage**: Everything stays on your Mac
- **Zero Tracking**: No data collection, analytics, or telemetry
- **Open Source**: Transparent and auditable code

### ⚡ **Performance**
- **Lightweight**: Minimal memory and CPU usage
- **Energy Efficient**: Designed for all-day battery life
- **Native Speed**: Optimized for Apple Silicon architecture

## 🛠️ Troubleshooting

### Common Issues

**"Locus can't be opened because it is from an unidentified developer"**
```bash
# Remove quarantine attribute (one-time setup)
sudo xattr -rd com.apple.quarantine /Applications/locus.app
```

**Screen Recording Permission Issues**
1. Go to System Preferences → Privacy & Security → Screen Recording
2. Remove Locus from the list if present
3. Re-add Locus by clicking the "+" button
4. Restart Locus

**App Won't Start**
- Ensure you're running macOS 14.0+ on Apple Silicon
- Check Console.app for error messages
- Try rebuilding from source if using a custom build

**No Activity Tracking**
- Verify Screen Recording permission is granted
- Check if the minimum activity duration is too high
- Restart the application

### Getting Help

- **Issues**: [GitHub Issues](https://github.com/charanreddy-git/locus/issues)
- **Discussions**: [GitHub Discussions](https://github.com/charanreddy-git/locus/discussions)
- **Email**: Open an issue for direct support

## 📱 Screenshots

### Main Interface
![Clean, distraction-free interface](https://github.com/user-attachments/assets/ed942d5f-4329-4e5b-969f-efcf3652d846)

### Pomodoro Timer
![Built-in focus timer](https://github.com/user-attachments/assets/534b5da4-aa75-458b-8182-e1c6092f60ee)

### Activity Timeline
![Visual timeline of your day](https://github.com/user-attachments/assets/9ea1aa8f-8a4f-409f-a538-b41dd3d82f85)

### Analytics Dashboard  
![Detailed productivity insights](https://github.com/user-attachments/assets/d939cb98-0a5f-4a46-9d9b-c48c311a964d)

### Settings & Customization
![Personalize your experience](https://github.com/user-attachments/assets/dac2757b-ca8a-4a79-93aa-19025dc8e43d)

## 🤝 Contributing

Locus is open source! Contributions are welcome:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

For development setup, see the build instructions above.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Made with ❤️ for macOS productivity enthusiasts**
# locus-macos
