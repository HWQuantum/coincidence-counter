from PyQt5.QtWidgets import QApplication, QWidget, QLabel, QVBoxLayout, QGridLayout, QHBoxLayout, QPushButton
from PyQt5.QtCore import pyqtSlot, pyqtSignal
import pyqtgraph as pg
import numpy as np
import sys
from phase_plotter import combine_patterns_no_angle, PhasePatternController
from zernike_generator import zernike_cartesian

X, Y = np.meshgrid(np.linspace(-1 * (1024 / 1280), 1 * (1024 / 1280), 500),
                   np.linspace(-1, 1, 500))


class MainWindow(QWidget):
    def __init__(self, screens):
        super().__init__()
        layout = QVBoxLayout()
        self.setLayout(layout)
        self.phase_controller = PhasePatternController()
        self.phase_controller.value_changed.connect(self.set_new_image)
        self.fs_plot = FullScreenPlot()
        self.fs_plot.show()
        self.fs_plot.windowHandle().setScreen(
            (screens[1] if len(screens) > 1 else screens[0]))
        self.fs_plot.showFullScreen()
        self.little_plot = FullScreenPlot()
        self.lut_control = LUTControls()
        self.lut_control.valueChanged.connect(
            lambda: self.fs_plot.update_LUT(self.lut_control.get_LUT()))
        self.lut_control.valueChanged.connect(
            lambda: self.little_plot.update_LUT(self.lut_control.get_LUT()))
        self.zernike_set = ZernikeSet()
        self.zernike_set.valueChanged.connect(self.set_new_image)
        layout.addWidget(self.phase_controller)
        layout.addWidget(self.little_plot)
        layout.addWidget(self.lut_control)
        layout.addWidget(QLabel("Zernike values"))
        layout.addWidget(self.zernike_set)
        self.reset_button = QPushButton("Reset zernike values")
        self.reset_button.clicked.connect(self.zernike_set.reset_values)
        layout.addWidget(self.reset_button)

    @pyqtSlot()
    def set_new_image(self):
        im = np.angle(
            combine_patterns_no_angle(X, Y,
                                      [self.phase_controller.get_values()]) *
            self.zernike_set.get_values())
        self.fs_plot.set_image(im)
        self.little_plot.set_image(im)

    def closeEvent(self, event):
        try:
            self.fs_plot.close()
        except:
            pass


class LUTControls(QWidget):
    valueChanged = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.layout = QHBoxLayout()
        self.setLayout(self.layout)
        self.max_colour = pg.SpinBox(value=255,
                                     int=True,
                                     step=1,
                                     bounds=(0, 255))
        self.min_clamp = pg.SpinBox(value=-np.pi, bounds=(-np.pi, np.pi))
        self.max_clamp = pg.SpinBox(value=np.pi, bounds=(-np.pi, np.pi))
        self.layout.addWidget(QLabel("Max colour: "))
        self.layout.addWidget(self.max_colour)
        self.layout.addWidget(QLabel("Min Clamp: "))
        self.layout.addWidget(self.min_clamp)
        self.layout.addWidget(QLabel("Max Clamp: "))
        self.layout.addWidget(self.max_clamp)
        self.max_colour.sigValueChanged.connect(self.valueChanged.emit)
        self.min_clamp.sigValueChanged.connect(self.valueChanged.emit)
        self.max_clamp.sigValueChanged.connect(self.valueChanged.emit)

    def get_LUT(self):
        min_index = np.int(
            np.round((self.min_clamp.value() + np.pi) / (2 * np.pi) * 255))
        max_index = np.int(
            np.round((self.max_clamp.value() + np.pi) / (2 * np.pi) * 255))
        lut = np.empty((255, 3), dtype=np.uint8)
        lut[:min_index] = 0
        lut[max_index:] = self.max_colour.value()
        if min_index < max_index:
            lut[min_index:max_index, :] = np.tile(
                np.linspace(0,
                            self.max_colour.value(),
                            abs(max_index - min_index),
                            dtype=np.uint8), (3, 1)).T
        return lut


class ZernikeControl(QWidget):
    valueChanged = pyqtSignal()

    def __init__(self, indices=(0, 0), default_val=0):
        super().__init__()
        self.index = indices
        self.layout = QVBoxLayout()
        self.setLayout(self.layout)
        self.sbox = pg.SpinBox(value=default_val)
        self.sbox.sigValueChanged.connect(self.valueChanged.emit)
        self.layout.addWidget(self.sbox)
        self.layout.addWidget(QLabel("({}, {})".format(indices[0],
                                                       indices[1])))


class ZernikeSet(QWidget):
    valueChanged = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.layout = QHBoxLayout()
        self.setLayout(self.layout)
        indices = [(0, 0), (-1, 1), (1, 1), (-2, 2), (0, 2), (2, 2), (-3, 3),
                   (-1, 3), (1, 3), (3, 3)]
        self.controls = [ZernikeControl(i, 0) for i in indices]
        self.value_dict = {}
        for c in self.controls:
            self.value_dict[c.index] = zernike_cartesian(*c.index)(X, Y)
            c.valueChanged.connect(self.valueChanged.emit)
            self.layout.addWidget(c)

    def get_values(self):
        return np.exp(1j * np.sum(
            [c.sbox.value() * self.value_dict[c.index] for c in self.controls],
            axis=0))

    def reset_values(self):
        for c in self.controls:
            c.sbox.setValue(0)


class FullScreenPlot(pg.PlotWidget):
    """ displays an ndarray on a pyqtgraph image plot
    """
    def __init__(self):
        super().__init__()
        self.setLimits(xMin=0,
                       xMax=1280,
                       yMin=500 - 1024,
                       yMax=500,
                       minXRange=1280,
                       maxXRange=1280,
                       minYRange=1024,
                       maxYRange=1024)
        self.hideAxis('left')
        self.hideAxis('bottom')
        self.image_display = pg.ImageItem(np.zeros((500, 500)))
        self.addItem(self.image_display)

    @pyqtSlot(np.ndarray)
    def set_image(self, array):
        self.image_display.setImage(array)

    @pyqtSlot(np.ndarray)
    def update_LUT(self, array):
        self.image_display.setLookupTable(array)


app = QApplication(sys.argv)
window = MainWindow(app.screens())
window.show()
sys.exit(app.exec_())
