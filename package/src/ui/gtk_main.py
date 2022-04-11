import gi
gi.require_version("Gtk", "3.0")
from gi.repository import Gtk

class main_window(Gtk.Window):
    def __init__(self):
        super().__init__(title="AcaciaLinux Installer")

        #TODO: remove, placeholder button and sample event connect
        self.button = Gtk.Button(label="placeholder button")
        self.button.connect("clicked", self.testfun)
        self.add(self.button)

    def testfun(self, widget):
        print("test func called")


def init():
    win = main_window()
    win.connect("destroy", Gtk.main_quit)
    win.show_all()
    Gtk.main()

