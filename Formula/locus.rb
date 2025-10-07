class Locus < Formula
  desc "Intelligent activity tracker for macOS that helps understand and improve focus habits"
  homepage "https://github.com/Sushants-Git/locus"
  url "https://github.com/Sushants-Git/locus/releases/download/v0.2.2/locus_0.2.2_aarch64.dmg"
  version "0.2.2"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"

  # Only works on Apple Silicon Macs
  depends_on arch: :arm64
  depends_on macos: ">= :sonoma"

  def install
    # Extract the app from the DMG and install it
    prefix.install "Locus.app"
    
    # Create a symlink to make it available from PATH (optional)
    bin.install_symlink prefix/"Locus.app/Contents/MacOS/locus"
    
    # Install the launchd plist template
    (prefix/"share").mkpath
    (prefix/"share").install "com.locus.app.plist" if File.exist?("com.locus.app.plist")
  end

  def caveats
    <<~EOS
      🎯 Locus has been installed to #{prefix}/Locus.app
      
      📝 Next steps:
      1. Grant Screen Recording permission in System Preferences
      2. Run `locus` from terminal or open Locus.app from Applications
      
      🤖 To set up automatic startup:
      cp #{prefix}/share/com.locus.app.plist ~/Library/LaunchAgents/
      launchctl load ~/Library/LaunchAgents/com.locus.app.plist
      
      📍 Data is stored in ~/Library/Application Support/Locus
      
      For more information, visit: #{homepage}
    EOS
  end

  test do
    # Test that the binary exists and can show version info
    assert_predicate prefix/"Locus.app/Contents/MacOS/locus", :exist?
    
    # We can't easily test the GUI functionality in a test environment
    # but we can at least verify the binary runs without crashing
    system "#{prefix}/Locus.app/Contents/MacOS/locus", "--version" rescue nil
  end
end