#!/bin/bash
# Generate proper icon files for Tauri

set -e

# Create a simple SVG icon
cat > icon.svg << 'EOF'
<svg width="512" height="512" xmlns="http://www.w3.org/2000/svg">
  <rect width="512" height="512" fill="#1e293b"/>
  <circle cx="256" cy="256" r="200" fill="#3b82f6" opacity="0.2"/>
  <circle cx="256" cy="256" r="150" fill="#3b82f6" opacity="0.3"/>
  <circle cx="256" cy="256" r="100" fill="#3b82f6" opacity="0.5"/>
  <text x="256" y="280" font-family="Arial, sans-serif" font-size="200" font-weight="bold" fill="#60a5fa" text-anchor="middle">O</text>
</svg>
EOF

# Check if ImageMagick is available
if command -v convert &> /dev/null; then
    echo "Using ImageMagick to generate icons..."
    convert icon.svg -resize 32x32 32x32.png
    convert icon.svg -resize 128x128 128x128.png
    convert icon.svg -resize 256x256 128x128@2x.png
    convert icon.svg -resize 512x512 icon.png
    convert icon.svg -resize 1024x1024 icon.icns
elif command -v magick &> /dev/null; then
    echo "Using ImageMagick (magick) to generate icons..."
    magick icon.svg -resize 32x32 32x32.png
    magick icon.svg -resize 128x128 128x128.png
    magick icon.svg -resize 256x256 128x128@2x.png
    magick icon.svg -resize 512x512 icon.png
    magick icon.svg -resize 1024x1024 icon.icns
elif command -v sips &> /dev/null; then
    # macOS native tool
    echo "Using sips to generate icons..."
    # First create a base PNG
    if command -v rsvg-convert &> /dev/null; then
        rsvg-convert -w 1024 -h 1024 icon.svg -o icon-1024.png
    else
        # Fallback: create a simple PNG with base64
        echo "Creating fallback icons..."
        cat > create_png.py << 'PYEOF'
from PIL import Image, ImageDraw, ImageFont
import sys

size = int(sys.argv[1])
img = Image.new('RGBA', (size, size), (30, 41, 59, 255))
draw = ImageDraw.Draw(img)

# Draw circles
for r, alpha in [(int(size*0.39), 51), (int(size*0.29), 77), (int(size*0.20), 128)]:
    draw.ellipse([size//2-r, size//2-r, size//2+r, size//2+r], fill=(59, 130, 246, alpha))

# Draw O
try:
    font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", int(size*0.4))
except:
    font = ImageFont.load_default()
draw.text((size//2, size//2), "O", fill=(96, 165, 250, 255), anchor="mm", font=font)

img.save(sys.argv[2])
PYEOF
        
        if command -v python3 &> /dev/null && python3 -c "import PIL" 2>/dev/null; then
            python3 create_png.py 1024 icon-1024.png
        else
            # Ultimate fallback - use existing icon.png
            cp icon.png icon-1024.png 2>/dev/null || echo "Warning: Could not create base icon"
        fi
    fi
    
    # Generate different sizes
    sips -z 32 32 icon-1024.png --out 32x32.png 2>/dev/null || cp icon-1024.png 32x32.png
    sips -z 128 128 icon-1024.png --out 128x128.png 2>/dev/null || cp icon-1024.png 128x128.png
    sips -z 256 256 icon-1024.png --out 128x128@2x.png 2>/dev/null || cp icon-1024.png 128x128@2x.png
    sips -z 512 512 icon-1024.png --out icon.png 2>/dev/null || cp icon-1024.png icon.png
    
    # Create .icns for macOS
    mkdir -p icon.iconset
    sips -z 16 16 icon-1024.png --out icon.iconset/icon_16x16.png
    sips -z 32 32 icon-1024.png --out icon.iconset/icon_16x16@2x.png
    sips -z 32 32 icon-1024.png --out icon.iconset/icon_32x32.png
    sips -z 64 64 icon-1024.png --out icon.iconset/icon_32x32@2x.png
    sips -z 128 128 icon-1024.png --out icon.iconset/icon_128x128.png
    sips -z 256 256 icon-1024.png --out icon.iconset/icon_128x128@2x.png
    sips -z 256 256 icon-1024.png --out icon.iconset/icon_256x256.png
    sips -z 512 512 icon-1024.png --out icon.iconset/icon_256x256@2x.png
    sips -z 512 512 icon-1024.png --out icon.iconset/icon_512x512.png
    sips -z 1024 1024 icon-1024.png --out icon.iconset/icon_512x512@2x.png
    iconutil -c icns icon.iconset -o icon.icns
    rm -rf icon.iconset
    
    # Create .ico for Windows using sips (convert to PNG then combine)
    # Windows .ico needs multiple sizes embedded
    if command -v convert &> /dev/null; then
        convert icon-1024.png -define icon:auto-resize=256,128,64,48,32,16 icon.ico
    else
        # Fallback: just copy the 256x256 as .ico (not ideal but works)
        cp 128x128@2x.png icon.ico 2>/dev/null || echo "Warning: Could not create .ico"
    fi
else
    echo "No image conversion tool found. Icons may not work properly."
    # Create a basic .ico file as fallback
    cp icon.png icon.ico 2>/dev/null || echo "Warning: Could not create .ico"
fi

echo "Icon generation complete!"
ls -lh *.png *.icns *.ico 2>/dev/null || true
