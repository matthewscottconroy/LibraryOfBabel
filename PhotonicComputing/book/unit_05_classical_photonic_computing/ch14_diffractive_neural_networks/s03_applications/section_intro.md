# Section 14.3: Applications

## What This Section Is About

A diffractive network is a peculiar computer: passive, linear in field up to an intensity-only readout, and — on every platform but the SLM — fixed once fabricated. These constraints rule out whole classes of tasks and make the network exceptionally good at a few. This section surveys where D2NNs actually earn their place.

**14.3.1 — Image classification** is the field's benchmark and proving ground. On MNIST and Fashion-MNIST, accuracy climbs with depth (with diminishing returns) and is pushed higher by class-specific differential detection, by ensembles of networks, and by hybrid diffractive-electronic designs — the yardsticks against which every architectural idea is measured.

**14.3.2 — All-optical logic** trains diffractive layers to implement Boolean gates, routing input light so that an output region encodes the logic state. Unlike the gain-based optical logic of Chapter 11, this is a passive, parallel, analog mapping — with the fan-out and cascadability limits that passivity implies.

**14.3.3 — Spectral, broadband, and imaging tasks** exploit the intrinsic wavelength-dependence of diffraction: broadband D2NNs act as spectral filters, spectrometers, and terahertz pulse shapers, and diffractive front ends perform object detection and computational imaging.

The organizing point is economic. A D2NN wins wherever computing in the optical domain, at the moment of capture, for essentially zero energy beats digitizing first and computing after — front-end classifiers that pre-process before the sensor, all-optical processors, and tasks that are natively optical, like spectral and imaging problems where the data is light to begin with.
