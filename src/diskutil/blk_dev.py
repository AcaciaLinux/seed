import os

class partition():
    def __init__(self, partition):
        self.partition = partition
      
        # get part info
        part_info = os.popen("blkid {} -o value".format(partition)).read()
        part_info_arr = part_info.split("\n")
       
        # get values from split
        self.uuid     = part_info_arr[0]
        self.uuid_sub = part_info_arr[1]
        self.blk_size = part_info_arr[2]
        self.file_sys = part_info_arr[3]
        self.label    = part_info_arr[4]
        self.part_uuid = part_info_arr[5]

    def get_uuid(self):
        return self.uuid

    def get_uuid_sub(self):
        return self.uuid_sub

    def get_blk_size(self):
        return self.blk_size

    def get_file_sys(self):
        return self.file_sys

    def get_label(self):
        return self.label

    def get_part_uuid(self):
        return self.part_uuid

