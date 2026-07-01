Output diff-utils-python-demo.raw.mp4

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
Sleep 2.5s

# Scroll through the Python diff
Down@200ms 8
Sleep 1s

# Close left panel file → file browser
Type "q"
Sleep 2.5s

# Select calculator_old.py in the browser (↓ from calculator_new.py)
Type "j"
Sleep 1s

# Open the selected file
Type "l"
Sleep 2.5s

# Scroll the restored side-by-side diff
Down@150ms 6
Sleep 1s

# Switch to the right panel
Tab
Sleep 1s

# Close right panel file → file browser
Type "q"
Sleep 2.5s

# Move down to diff-demo.tape
Type "j"
Sleep 500ms
Type "j"
Sleep 1s

# Open the tape file on the right panel
Type "l"
Sleep 2.5s

# Quit the app
Type "Q"
