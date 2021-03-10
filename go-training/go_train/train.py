import os

import pytorch_lightning as pl
import torch
import torch.nn.functional as F
from torch import nn
from torch.utils.data import DataLoader, random_split
from torchvision import transforms
from torchvision.datasets import MNIST


# https://pytorch-lightning.readthedocs.io/en/latest/starter/introduction_guide.html

class LitAutoEncoder(pl.LightningModule):

    def __init__(self):
        super().__init__()

        # self.model = EfficientNet.from_name('efficientnet-b0')
        N = 2
        self.encoder = nn.Sequential(
            nn.Conv2d(1, 4, 3),
            nn.ReLU(),
            *[
                nn.Sequential(
                    nn.Conv2d(4 * 2 ** i, 4 * 2 ** (i + 1), 3),
                    nn.ReLU(),
                    # nn.MaxPool2d(2, stride=2, return_indices=True)
                ) for i in range(N)
            ],
            nn.Conv2d(16, 1, 3),
        )
        self.decoder = nn.Sequential(
            nn.ConvTranspose2d(1, 16, 3),

            *[
                nn.Sequential(
                    # nn.MaxUnpool2d(2, stride=2, padding=0),
                    nn.ConvTranspose2d(4 * 2 ** (i + 1), 4 * 2 ** i, 3),
                    nn.ReLU(),
                ) for i in reversed(range(N))
            ],
            nn.ConvTranspose2d(4, 1, 3),
        )

    def forward(self, x):
        batch_size, channels, width, height = x.size()
        embedding = self.encoder(x)
        return embedding

    def training_step(self, batch, batch_idx):
        x, y = batch
        # x = x.view(x.size(0), -1)
        z = self.encoder(x)
        x_hat = self.decoder(z)
        loss = F.mse_loss(x_hat, x)
        self.log('train_loss', loss)
        return loss

    # def validation_step(self, batch, batch_idx):
    # x, y = batch
    # z = self.encoder(x)
    # x_hat = self.decoder(z)
    # loss = F.mse_loss(x_hat, x)
    # self.log('val_loss', loss)
    # return loss

    def configure_optimizers(self):
        optimizer = torch.optim.Adam(self.parameters(), lr=1e-3)
        return optimizer


if __name__ == '__main__':
    dataset = MNIST(os.getcwd(), train=True, download=True,
                    transform=transforms.Compose([
                        transforms.ToTensor(),
                        # transforms.Normalize((0.1307,), (0.3081,))
                    ]))
    train_size = 20000
    test_size = 500
    others = len(dataset) - train_size - test_size
    train, val, _ = random_split(dataset, [train_size, test_size, others])

    autoencoder = LitAutoEncoder()
    trainer = pl.Trainer(max_epochs=1)
    trainer.fit(autoencoder,
                DataLoader(train,
                           batch_size=64,
                           shuffle=True),
                DataLoader(val))

    import numpy as np
    from pyqtgraph.Qt import QtCore, QtGui
    import pyqtgraph as pg

    # Interpret image data as row-major instead of col-major
    pg.setConfigOptions(imageAxisOrder='row-major')
    app = QtGui.QApplication([])
    win = QtGui.QMainWindow()
    win.resize(800, 800)
    imv = pg.ImageView()
    win.setCentralWidget(imv)
    win.show()
    win.setWindowTitle('pyqtgraph example: ImageView')

    data = DataLoader(val, batch_size=32)
    for x, y in data:
        result = autoencoder.predict(x, 0).detach()
        res = np.array(result.detach())

        res = np.moveaxis(res, 1, -1)
        xx = np.array(x).squeeze()
        res = np.array(res).squeeze()

        imv.setImage(res)
        break

    import sys

    if (sys.flags.interactive != 1) or not hasattr(QtCore, 'PYQT_VERSION'):
        QtGui.QApplication.instance().exec_()
