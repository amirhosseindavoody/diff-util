Output diff-utils-python-demo.mp4

Set Shell "bash"
Set FontSize 20
Set Width 1400
Set Height 800
Set Padding 40
Set Theme "Catppuccin Mocha"
Set TypingSpeed 0ms

Env NO_COLOR ""
Env COLORTERM "truecolor"
Env TERM "xterm-256color"

Hide
Type "cd /workspace/demo"
Enter
Show

Type "/workspace/target/release/diff-utils calculator_old.py calculator_new.py"
Enter
Sleep 2s

# Scroll down through the diff
Down@150ms 30
Sleep 1s

# Page down to show more changes
PageDown
Sleep 500ms
Type "J"
Sleep 1s

# Toggle help overlay
Type "?"
Sleep 2s
Type "?"
Sleep 500ms

# Switch focus between panels
Tab
Sleep 500ms
Tab
Sleep 500ms

# Scroll back up
Up@100ms 20
Sleep 1s

# Scroll to top
Type "g"
Sleep 1s

# Scroll down again slowly
Down@200ms 15
Sleep 1s

# Quit
Type "Q"
