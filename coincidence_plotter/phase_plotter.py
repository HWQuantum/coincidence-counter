from PyQt5.QtWidgets import QWidget, QVBoxLayout, QSpinBox, QApplication, QHBoxLayout, QTabWidget, QPushButton, QGridLayout, QDoubleSpinBox, QGroupBox, QLabel, QApplication, QFormLayout, QScrollArea
from PyQt5.QtCore import QThread, pyqtSignal, pyqtSlot, QObject, QTimer
import pyqtgraph as pg
import numpy as np
from numba import jit, njit
import sys


@njit()
def generate_pattern(x, y, n, k_vec, phase=0, centre=(0, 0)):
    return np.exp(1j * (n * np.arctan2(y - centre[1], x - centre[0]) +
                        (k_vec[0] * x + k_vec[1] * y) + phase))


def combine_patterns(x, y, pattern_arguments):
    if len(pattern_arguments) == 0:
        return np.zeros(x.shape)
    else:
        return np.angle(
            np.sum([
                a * generate_pattern(x, y, n, k, p, centre)
                for (a, n, k, p, centre) in pattern_arguments
            ],
                   axis=0))


def combine_patterns_no_angle(x, y, pattern_arguments):
    if len(pattern_arguments) == 0:
        return np.zeros(x.shape)
    else:
        return np.sum([
            a * generate_pattern(x, y, n, k, p, centre)
            for (a, n, k, p, centre) in pattern_arguments
        ],
                      axis=0)


class PatternThread(QObject):
    pattern_generated = pyqtSignal(np.ndarray)

    def __init__(self, pattern_data):
        super().__init__()
        self.pattern_data = pattern_data

    @pyqtSlot()
    def run_calculation(self):
        self.pattern_generated.emit(combine_patterns(*self.pattern_data))


class PhasePatternController(QGroupBox):
    value_changed = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.setTitle("Phase Pattern")
        self.layout = QGridLayout()

        self.amplitude = pg.SpinBox(value=1.0)
        self.l_value = pg.SpinBox(int=True, step=1)
        self.phase = pg.SpinBox()
        self.k = XYController("K")
        self.pos = XYController("Centre")

        self.amplitude.sigValueChanged.connect(self.value_changed.emit)
        self.l_value.sigValueChanged.connect(self.value_changed.emit)
        self.phase.sigValueChanged.connect(self.value_changed.emit)
        self.k.value_changed.connect(self.value_changed.emit)
        self.pos.value_changed.connect(self.value_changed.emit)

        self.layout.addWidget(QLabel("Amplitude:"), 0, 0, 1, 1)
        self.layout.addWidget(self.amplitude, 0, 1, 1, 1)
        self.layout.addWidget(QLabel("L:"), 0, 2, 1, 1)
        self.layout.addWidget(self.l_value, 0, 3, 1, 1)
        self.layout.addWidget(QLabel("Phase:"), 0, 4, 1, 1)
        self.layout.addWidget(self.phase, 0, 5, 1, 1)
        self.layout.addWidget(self.k, 1, 0, 1, 3)
        self.layout.addWidget(self.pos, 1, 3, 1, 3)
        self.setLayout(self.layout)

    def get_values(self):
        return [
            self.amplitude.value(),
            self.l_value.value(),
            self.k.get_values(),
            self.phase.value(),
            self.pos.get_values()
        ]


class CloseWrapper(QWidget):
    close = pyqtSignal()

    def __init__(self, other_widget):
        super().__init__()
        layout = QHBoxLayout()
        layout.addWidget(other_widget, 4)
        closer = QPushButton("Close")
        self.wrapped_widget = other_widget
        closer.clicked.connect(self.close.emit)
        layout.addWidget(closer, 1)
        self.setLayout(layout)


class PhasePatternSet(QWidget):
    value_changed = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.layout = QFormLayout()
        self.setLayout(self.layout)

    @pyqtSlot()
    def add_new_phase_pattern(self):
        controller = PhasePatternController()
        controller.value_changed.connect(self.value_changed.emit)
        new_phase_pattern = CloseWrapper(controller)
        new_phase_pattern.close.connect(
            lambda: self.remove_pattern(new_phase_pattern))
        self.layout.addRow(new_phase_pattern)
        self.value_changed.emit()

    def get_values(self):
        return [
            self.layout.itemAt(i).widget().wrapped_widget.get_values()
            for i in range(self.layout.count())
        ]

    def remove_pattern(self, pattern):
        self.layout.removeRow(pattern)
        self.value_changed.emit()


class PhasePatternSetController(QWidget):
    value_changed = pyqtSignal()

    def __init__(self):
        super().__init__()
        self.pattern_set = PhasePatternSet()
        self.pattern_set.value_changed.connect(self.value_changed.emit)
        self.scroller = QScrollArea()
        self.scroller.setWidget(self.pattern_set)
        self.scroller.setWidgetResizable(True)
        self.add_button = QPushButton("Add phase pattern")
        self.update_button = QPushButton("Update phase pattern")
        self.add_button.clicked.connect(self.pattern_set.add_new_phase_pattern)
        self.update_button.clicked.connect(self.value_changed.emit)
        self.layout = QGridLayout()
        self.layout.addWidget(self.add_button, 0, 2, 1, 1)
        self.layout.addWidget(self.update_button, 0, 3, 1, 1)
        self.layout.addWidget(self.scroller, 1, 0, 4, 4)
        self.setLayout(self.layout)

    @pyqtSlot(int)
    def remove_tab(self, i):
        self.tabs.removeTab(i)
        self.value_changed.emit()

    def get_values(self):
        return self.pattern_set.get_values()


class SLMControllerWidget(QWidget):

    X, Y = np.meshgrid(np.linspace(-1, 1, 1080), np.linspace(-1, 1, 1920))

    def __init__(self):
        super().__init__()
        self.layout = QHBoxLayout()
        self.setLayout(self.layout)
        self.plot = pg.PlotWidget()
        self.plot.setLimits(xMin=0,
                            xMax=1920,
                            yMin=0,
                            yMax=1080,
                            minXRange=1920,
                            maxXRange=1920,
                            minYRange=1080,
                            maxYRange=1080)
        self.plot.hideAxis('left')
        self.plot.hideAxis('bottom')
        self.image = np.zeros((1920, 1080))
        self.image_display = pg.ImageItem(self.image)
        self.plot.addItem(self.image_display)
        self.layout.addWidget(self.plot, 1)
        self.phase_patterns = PhasePatternSetController()
        self.phase_patterns.value_changed.connect(self.queue_pattern)
        self.layout.addWidget(self.phase_patterns, 2)
        self.calculation_thread = QThread()
        self.calculation_thread.finished.connect(self.plot_patterns)
        # for threading the phase patterns
        self.queued = False

    @pyqtSlot()
    def plot_patterns(self):
        if self.queued:
            self.queued = False
            self.calculation = PatternThread(
                (self.X, self.Y, self.phase_patterns.get_values()))
            self.calculation.pattern_generated.connect(self.display_pattern)
            self.calculation.moveToThread(self.calculation_thread)
            self.calculation_thread.started.connect(
                self.calculation.run_calculation)
            self.calculation.pattern_generated.connect(
                self.calculation_thread.quit)
            self.calculation_thread.start()

    @pyqtSlot()
    def queue_pattern(self):
        self.queued = True
        self.plot_patterns()

    @pyqtSlot(np.ndarray)
    def display_pattern(self, im):
        self.image = im
        self.image_display.setImage(self.image)


class XYController(QGroupBox):
    value_changed = pyqtSignal()

    def __init__(self, name):
        super().__init__()
        self.setTitle(name)
        self.layout = QGridLayout()
        self.x = pg.SpinBox()
        self.y = pg.SpinBox()
        self.x.sigValueChanged.connect(self.value_changed.emit)
        self.y.sigValueChanged.connect(self.value_changed.emit)
        self.layout.addWidget(QLabel("x:"), 0, 0)
        self.layout.addWidget(QLabel("y:"), 0, 5)
        self.layout.addWidget(self.x, 0, 1, 1, 4)
        self.layout.addWidget(self.y, 0, 6, 1, 4)
        self.setLayout(self.layout)

    def get_values(self):
        return (self.x.value(), self.y.value())


if __name__ == '__main__':
    app = QApplication(sys.argv)
    window = SLMControllerWidget()
    window.show()
    sys.exit(app.exec_())
