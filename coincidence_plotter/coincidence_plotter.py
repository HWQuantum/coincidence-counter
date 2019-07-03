from PyQt5.QtWidgets import QWidget, QVBoxLayout, QSpinBox, QApplication, QHBoxLayout, QTabWidget, QPushButton, QGridLayout, QDoubleSpinBox, QGroupBox, QLabel
from PyQt5.QtCore import QThread, pyqtSignal, pyqtSlot, QObject, QTimer
import pyqtgraph as pg
import sys
import numpy as np
from time import sleep
from phase_plotter import SLMControllerWidget
import hhlib_sys


class MeasurementThread(QObject):
    measurement_done = pyqtSignal(list, list)

    def __init__(self, device, measurement_time, coincidence_window):
        super().__init__()
        self.time = measurement_time
        self.coin = coincidence_window
        self.dev = device

    @pyqtSlot()
    def run_measurement(self):
        self.measurement_done.emit(*hhlib_sys.measure_and_get_counts(self.dev, self.time, self.coin))


class MainWindow(QWidget):
    def __init__(self, device, parent=None):
        super().__init__()
        vbox = QVBoxLayout()
        self.measurement_time = pg.SpinBox(value=0.01, suffix='s', bounds=(0.001, None))
        self.coincidence_window = pg.SpinBox(value=0.01, suffix='s', bounds=(0, None))
        self.long_button = QPushButton("Measure")
        self.long_button.setCheckable(True)
        self.measurement_thread = QThread()
        self.measurement_thread.finished.connect(self.run_measurement)
        self.long_button.clicked.connect(self.run_measurement)
        self.graph = pg.PlotWidget()
        self.setLayout(vbox)
        self.channel_1_data = np.zeros((300))
        self.channel_2_data = np.zeros((300))
        self.coincidences_data = np.zeros((300))
        self.channel_1 = self.graph.plot([], pen=pg.mkPen('b'))
        self.channel_2 = self.graph.plot([], pen=pg.mkPen('r'))
        self.coincidences = self.graph.plot([], pen=pg.mkPen('w'))
        self.device = device
        vbox.addWidget(self.measurement_time)
        vbox.addWidget(self.coincidence_window)
        vbox.addWidget(self.long_button)
        vbox.addWidget(self.graph)
        vbox.addWidget(SLMControllerWidget())

    @pyqtSlot(list, list)
    def update_data(self, l1, l2):
        self.channel_1_data = np.roll(self.channel_1_data, -1)
        self.channel_2_data = np.roll(self.channel_2_data, -1)
        self.coincidences_data = np.roll(self.coincidences_data, -1)
        self.channel_1_data[-1] = l1[0]
        self.channel_2_data[-1] = l1[1]
        self.coincidences_data[-1] = l2[0]
        self.channel_1.setData(self.channel_1_data)
        self.channel_2.setData(self.channel_2_data)
        self.coincidences.setData(self.coincidences_data)

    def run_measurement(self):
        if self.long_button.isChecked(
        ) and not self.measurement_thread.isRunning():
            self.measurement = MeasurementThread(self.device, int(self.measurement_time.value()*1000), int(self.coincidence_window.value()))
            self.measurement.measurement_done.connect(self.update_data)
            self.measurement.moveToThread(self.measurement_thread)
            self.measurement_thread.started.connect(
                self.measurement.run_measurement)
            self.measurement.measurement_done.connect(
                self.measurement_thread.quit)
            self.measurement_thread.start()


if __name__ == '__main__':
    # set up the coincidence counter
    dev = hhlib_sys.open_device(0)
    hhlib_sys.initialise(dev, 2, 0)
    hhlib_sys.calibrate(dev)
    hhlib_sys.set_sync_divider(dev, 1)
    hhlib_sys.set_sync_CFD(dev, 50, 10)
    hhlib_sys.set_sync_channel_offset(dev, -5000)
    num_channels = hhlib_sys.get_number_of_input_channels(dev)
    for i in range(num_channels):
        hhlib_sys.set_input_CFD(dev, i, 50, 10)
        hhlib_sys.set_input_channel_offset(dev, i, 0)
    # make sure the settings have been set
    sleep(0.2)
    app = QApplication(sys.argv)
    window = MainWindow(dev)
    window.show()
    sys.exit(app.exec_())
