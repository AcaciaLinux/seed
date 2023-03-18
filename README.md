# seed
The acacialinux installer daemon

# Creating a loopback device for debugging
You can use the script `create_loop.sh` to create a loopback device to try running `seed` on for testing and development purposes. Using it is quite easy:
```bash
./create_loop.sh loopback.img 10
```
The first argument is the file path and the second one is the size in `GB` to allocate for the loop device.

### Unmounting
To unmount the created device, use the following command:
```bash
losetup -d <your loopback in /dev>
```
