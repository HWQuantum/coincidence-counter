from PyQt5.QtWidgets import QApplication, QWidget, QLabel, QVBoxLayout
from PyQt5.QtCore import pyqtSlot, pyqtSignal
import pyqtgraph as pg
import numpy as np
import sys
from phase_plotter import combine_patterns, PhasePatternController

X, Y = np.meshgrid(np.linspace(-1*(1024/1280), 1*(1024/1280), 1024), np.linspace(-1, 1, 1280))


class MainWindow(QWidget):
    def __init__(self, screens):
        super().__init__()
        layout = QVBoxLayout()
        self.setLayout(layout)
        self.phase_controller = PhasePatternController()
        self.phase_controller.value_changed.connect(self.set_new_image)
        self.fs_plot = FullScreenPlot()
        self.fs_plot.show()
        self.fs_plot.windowHandle().setScreen(screens[1])
        self.fs_plot.showFullScreen()
        self.little_plot = FullScreenPlot()
        self.gradient_editor = pg.GradientWidget()
        self.gradient_editor.loadPreset('grey')
        self.gradient_editor.sigGradientChangeFinished.connect(lambda s: self.fs_plot.update_LUT(s.getLookupTable(256)))
        self.gradient_editor.sigGradientChangeFinished.connect(lambda s: self.little_plot.update_LUT(s.getLookupTable(256)))
        layout.addWidget(self.phase_controller)
        layout.addWidget(self.little_plot)
        layout.addWidget(self.gradient_editor)

    @pyqtSlot()
    def set_new_image(self):
        im = combine_patterns(X, Y, [self.phase_controller.get_values()])
        self.fs_plot.set_image(im)
        self.little_plot.set_image(im)

    def closeEvent(self, event):
        try:
            self.fs_plot.close()
        except:
            pass


class FullScreenPlot(pg.PlotWidget):
    def __init__(self):
        super().__init__()
        self.setLimits(xMin=0,
                       xMax=1280,
                       yMin=0,
                       yMax=1024,
                       minXRange=1280,
                       maxXRange=1280,
                       minYRange=1024,
                       maxYRange=1024)
        self.hideAxis('left')
        self.hideAxis('bottom')
        self.image_display = pg.ImageItem(np.zeros((1280, 1024)))
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
