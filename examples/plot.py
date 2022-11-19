from psf_utils import PSF
import matplotlib.pyplot as plt

psf = PSF('timeSweep.tran.tran')
sweep = psf.get_sweep()
out = psf.get_signal('v(dout[0])')

figure = plt.figure()
axes = figure.add_subplot(1,1,1)
axes.plot(sweep.abscissa, out.ordinate, linewidth=2, label=out.name)
axes.set_title('ADC Output')
axes.set_xlabel(f'{sweep.name} ({PSF.units_to_unicode(sweep.units)})')
axes.set_ylabel(f'{out.name} ({PSF.units_to_unicode(out.units)})')
plt.show()
