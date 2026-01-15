import os

file_path = r'SpriteEngine/motor_de_historia/frontend/index.html'

with open(file_path, 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Find the footer end to be safe, or just cut at 1174
# Line 1174 in previous view was </div> of footer
# We want to keep that.
# Let's find "Developed with" and keep a few lines after.

cutoff_index = 0
for i, line in enumerate(lines):
    if 'id="top"' in line and 'class="footer"' in line:
        cutoff_index = i + 3 # Keep footer div content and closing div
        break

if cutoff_index == 0:
    cutoff_index = 1174 # Fallback

clean_lines = lines[:cutoff_index]

clean_lines.append('\n    <!-- MODULAR JS ARCHITECTURE -->\n')
clean_lines.append('    <script type="module" src="js/main.js?v=4.4"></script>\n')
clean_lines.append('    <script type="module" src="js/ui.js?v=4.4"></script>\n')
clean_lines.append('</body>\n</html>')

with open(file_path, 'w', encoding='utf-8') as f:
    f.writelines(clean_lines)

print(f"Cleaned index.html. Kept {cutoff_index} lines.")
