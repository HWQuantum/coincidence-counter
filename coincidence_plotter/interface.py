from PyQt5.QtWidgets import QApplication, QWidget, QVBoxLayout, QSpinBox, QPushButton
import pyqtgraph as pg
import numpy as np
app = QApplication([])
window = QWidget()
layout = QVBoxLayout()
spinbox = QSpinBox()
plot = pg.PlotWidget()
button = QPushButton()
data = np.zeros(shape=300)
curve = plot.plot(data, pen=pg.mkPen('b', width=5))
layout.addWidget(plot)
layout.addWidget(spinbox)
layout.addWidget(button)
window.setLayout(layout)
window.show()

def update():
    global data
    data = np.roll(data, -1)
    data[-1] = spinbox.value()
    curve.setData(data)

timer = pg.QtCore.QTimer()
timer.timeout.connect(update)
timer.start(50)
app.exec_()
