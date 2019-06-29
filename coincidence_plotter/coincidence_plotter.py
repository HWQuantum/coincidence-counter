from PyQt5.QtWidgets import QWidget, QVBoxLayout, QSpinBox, QApplication, QHBoxLayout, QTabWidget
import sys

class HydraHarpSettings(QWidget):
    def __init__(self):
        super().__init__()
        hbox = QHBoxLayout()
        hbox.addWidget(QSpinBox())
        hbox.addWidget(QSpinBox())
        self.setLayout(hbox)
        self.show()

class MainWindow(QWidget):
    def __init__(self, parent=None):
        super().__init__()
        vbox = QVBoxLayout()
        sbox = QSpinBox()
        self.setLayout(vbox)
        vbox.addWidget(sbox)
        vbox.addWidget(TestWidget())
        sbox.setEnabled(False)
        self.show()


if __name__ =='__main__':
    app = QApplication(sys.argv)
    window = MainWindow()
    sys.exit(app.exec_())
