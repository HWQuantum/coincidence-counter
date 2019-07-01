from PyQt5.QtWidgets import QWidget, QVBoxLayout, QSpinBox, QApplication, QHBoxLayout, QTabWidget, QPushButton, QGridLayout, QDoubleSpinBox, QGroupBox, QLabel
from PyQt5.QtCore import QThread, pyqtSignal, pyqtSlot, QObject, QTimer
import pyqtgraph as pg
import sys
import numpy as np
from time import sleep


class MeasurementThread(QObject):
    measurement_done = pyqtSignal(np.ndarray)

    def __init__(self, measurement_time, coincidence_window):
        super().__init__()
        self.time = measurement_time
        self.coin = coincidence_window

    @pyqtSlot()
    def run_measurement(self):
        sleep(self.time)
        self.measurement_done.emit(np.random.randint(0, 10, (100, 2)))


class PhasePatternController(QGroupBox):
    def __init__(self):
        super().__init__()
        self.setTitle("Phase Values")
        self.layout = QGridLayout()
        self.amplitude = QDoubleSpinBox()
        self.amplitude.setRange(0, 1000)
        self.l_value = QSpinBox()
        self.l_value.setRange(-1000, 1000)
        self.phase = QDoubleSpinBox()
        self.phase.setRange(0, 2*np.pi)
        self.k = XYController("K")
        self.pos = XYController("Centre")
        self.layout.addWidget(QLabel("Amplitude:"), 0, 0, 1, 1)
        self.layout.addWidget(self.amplitude, 0, 1, 1, 1)
        self.layout.addWidget(QLabel("L:"), 0, 2, 1, 1)
        self.layout.addWidget(self.l_value, 0, 3, 1, 1)
        self.layout.addWidget(QLabel("Phase:"), 0, 4, 1, 1)
        self.layout.addWidget(self.phase, 0, 5, 1, 1)
        self.layout.addWidget(self.k, 1, 0, 1, 3)
        self.layout.addWidget(self.pos, 1, 3, 1, 3)
        self.setLayout(self.layout)


class XYController(QGroupBox):
    value_changed = pyqtSignal(float, float)
    
    def __init__(self,
                 name,
                 xRange=(-10_000, 10_000),
                 yRange=(-10_000, 10_000)):
        super().__init__()
        self.setTitle(name)
        self.layout = QGridLayout()
        self.x = QDoubleSpinBox()
        self.x.setRange(*xRange)
        self.y = QDoubleSpinBox()
        self.y.setRange(*yRange)
        self.layout.addWidget(QLabel("x:"), 0, 0)
        self.layout.addWidget(QLabel("y:"), 0, 5)
        self.layout.addWidget(self.x, 0, 1, 1, 4)
        self.layout.addWidget(self.y, 0, 6, 1, 4)
        self.setLayout(self.layout)

    @pyqtSlot()
    def get_values(self):
        return (self.x.value(), self.y.value())


class MainWindow(QWidget):
    def __init__(self, parent=None):
        super().__init__()
        vbox = QVBoxLayout()
        self.sbox = QSpinBox()
        self.long_button = QPushButton("Measure")
        self.long_button.setCheckable(True)
        self.measurement_thread = QThread()
        self.long_button.clicked.connect(self.run_measurement)
        self.graph = pg.PlotWidget()
        self.setLayout(vbox)
        self.curve = self.graph.plot([0, 0, 0])
        self.im = pg.ImageView()
        self.im.setImage(np.random.randint(0, 10, (100, 100)))
        vbox.addWidget(self.sbox)
        vbox.addWidget(self.long_button)
        vbox.addWidget(self.graph)
        vbox.addWidget(self.im)
        vbox.addWidget(PhasePatternController())

    def long_sleep(self, data):
        self.curve.setData(data)

    def run_measurement(self):
        if self.long_button.isChecked(
        ) and not self.measurement_thread.isRunning():
            self.measurement = MeasurementThread(self.sbox.value(), 1)
            self.measurement.measurement_done.connect(self.long_sleep)
            self.measurement.moveToThread(self.measurement_thread)
            self.measurement_thread.started.connect(
                self.measurement.run_measurement)
            self.measurement_thread.finished.connect(self.run_measurement)
            self.measurement.measurement_done.connect(
                self.measurement_thread.quit)
            self.measurement_thread.start()


if __name__=='__main__':
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec_())
