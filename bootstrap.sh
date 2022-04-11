PKG_FILE_LOCATION="'http://localhost:8000/installer.tar.gz'"

echo "Deleting old builds..."
rm -rf ./build/

echo "Creating build directory.."
mkdir ./build

echo "Creating tar package.."
tar cvzf build/installer.tar.gz package/

echo "Package installer will fetch installer from: $PKG_FILE_LOCATION"
cp bootstrap.py ./build
cd ./build

echo "Appending http_origin"
temp=`echo "http_origin=$PKG_FILE_LOCATION"; cat bootstrap.py`
rm bootstrap.py
echo "$temp" > bootstrap.py
echo "http_origin=$PKG_FILE_LOCATION" > location.txt

echo "Running webserver.."
python3 -m http.server


