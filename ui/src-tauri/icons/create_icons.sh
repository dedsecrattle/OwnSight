#!/bin/bash
# Create minimal valid PNG files (1x1 blue pixel, then scale info in filename)
# PNG header + minimal IHDR + IDAT + IEND for a 1x1 blue pixel
echo -n -e '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\x0cIDATx\x9cc\xfc\xcf\xc0\x00\x00\x00\x03\x00\x01\x8f\x0e\x1c\xe7\x00\x00\x00\x00IEND\xaeB`\x82' > 32x32.png
cp 32x32.png 128x128.png
cp 32x32.png 128x128@2x.png
cp 32x32.png icon.png
echo "Done"
