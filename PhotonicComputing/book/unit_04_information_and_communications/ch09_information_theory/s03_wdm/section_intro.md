# Section 9.3: Wavelength-Division Multiplexing

The capacity of a single optical channel — one laser, one modulation format, one wavelength — has grown from 2.5 Gbps (1990s OC-48) to 800 Gbps (2024 DP-64QAM at 100 GBaud) over three decades. But the bandwidth of the C+L amplification window (~10 THz, or ~80 nm) can hold hundreds of such channels simultaneously. Wavelength-division multiplexing (WDM) is the technique of transmitting multiple optical channels at different wavelengths on a single fiber, each carrying an independent data stream.

WDM has been the enabling technology for the exponential growth of the internet since the late 1990s. Without WDM, meeting the global demand for bandwidth would require deploying vastly more fiber — an enormously expensive proposition. With WDM, each fiber pair carries not one but 80–96 channels, multiplying capacity by nearly 100× at a small incremental cost.

For photonic computing, WDM plays an equally important role: it provides the wavelength channels that encode different elements of the input vector in a wavelength-multiplexed matrix-vector multiplier (as discussed in Chapters 14–15 of this book).

**Subsection 9.3.1 — DWDM Channel Plan**: The ITU frequency grid, channel spacing, and band architecture.

**Subsection 9.3.2 — WDM System Design**: EDFA cascades, power equalization, dispersion maps, and the system margin budget.

**Subsection 9.3.3 — ROADMs**: Reconfigurable optical add-drop multiplexers as the switching nodes of optical networks.
