from PyQt5.QtWidgets import QWidget, QVBoxLayout, QSpinBox, QApplication, QHBoxLayout, QTabWidget, QPushButton
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
        vbox.addWidget(self.sbox)
        vbox.addWidget(self.long_button)
        vbox.addWidget(self.graph)
        self.timer = QTimer()
        self.show()

    def long_sleep(self, data):
        self.curve.setData(data)

    def run_measurement(self):
        if self.long_button.isChecked() and not self.measurement_thread.isRunning():
            self.measurement = MeasurementThread(self.sbox.value(), 1)
            self.measurement.measurement_done.connect(self.long_sleep)
            self.measurement.moveToThread(self.measurement_thread)
            self.measurement_thread.started.connect(self.measurement.run_measurement)
            self.measurement_thread.finished.connect(self.run_measurement)
            self.measurement.measurement_done.connect(self.measurement_thread.quit)
            self.measurement_thread.start()

if __name__ =='__main__':
    app = QApplication(sys.argv)
    window = MainWindow()
    sys.exit(app.exec_())
