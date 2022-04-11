http_origin='http://localhost:8000/installer.tar.gz'

#
# Installer location is appended here
#

import os
import tarfile
import requests
import socket


print("Hello from python! {}".format(http_origin))

print("Fetching installer tarball from {}".format(http_origin))
installer_file = open("installer.tar.gz", "wb")
response = requests.get(http_origin)
installer_file.write(response.content)
