#!/usr/bin/env python3
"""Generate the book's figures as SVG, deterministically, with no dependencies
beyond numpy.

Every figure is a quantitative illustration whose curve or geometry carries the
argument of the section it sits in — the ASCII diagrams in the text handle
structure; these handle shape.  Output goes to book/figures/ and is committed,
so the build never needs to run this.  Re-run after changing it:

    python3 tools/make_figures.py

The PDF build embeds the SVGs via rsvg-convert (pandoc does this automatically
when the engine is xelatex and rsvg-convert is on PATH).
"""
from __future__ import annotations

import math
import pathlib

import numpy as np

OUT = pathlib.Path(__file__).resolve().parent.parent / "book" / "figures"

# Palette — muted, print-friendly, consistent across figures.
INK = "#1a1a2e"        # text and axes
BLUE = "#23527c"       # primary series
RED = "#a63d40"        # secondary series / warnings
TEAL = "#2e7d6f"       # tertiary series
ORANGE = "#b0722a"     # quaternary series
GRAY = "#9a9aa5"       # grid, de-emphasised
LIGHT = "#e8e8ee"      # background bands
FONT = "Georgia, 'Times New Roman', serif"


def fmt(x: float) -> str:
    return f"{x:.2f}".rstrip("0").rstrip(".")


class Fig:
    """A minimal SVG line-plot builder: one axes rectangle, data space -> pixels."""

    def __init__(self, width=720, height=430, margin=(58, 20, 46, 58)):
        # margin: top, right, bottom, left
        self.w, self.h = width, height
        self.mt, self.mr, self.mb, self.ml = margin
        self.parts: list[str] = []
        self.xlim = (0.0, 1.0)
        self.ylim = (0.0, 1.0)
        self.xlog = self.ylog = False

    # -- coordinate transforms ------------------------------------------- #
    def _tx(self, x):
        a, b = self.xlim
        if self.xlog:
            a, b, x = math.log10(a), math.log10(b), math.log10(x)
        return self.ml + (x - a) / (b - a) * (self.w - self.ml - self.mr)

    def _ty(self, y):
        a, b = self.ylim
        if self.ylog:
            a, b, y = math.log10(a), math.log10(b), math.log10(y)
        return self.h - self.mb - (y - a) / (b - a) * (self.h - self.mt - self.mb)

    # -- primitives ------------------------------------------------------ #
    def raw(self, s):
        self.parts.append(s)

    def line(self, x1, y1, x2, y2, color=INK, w=1.0, dash=None, opacity=1.0):
        d = f' stroke-dasharray="{dash}"' if dash else ""
        self.parts.append(
            f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" '
            f'stroke="{color}" stroke-width="{w}"{d} opacity="{opacity}"/>'
        )

    def dline(self, x1, y1, x2, y2, **kw):
        self.line(self._tx(x1), self._ty(y1), self._tx(x2), self._ty(y2), **kw)

    def text(self, x, y, s, size=15, color=INK, anchor="middle", style="",
             rotate=None, weight="normal"):
        r = (f' transform="rotate({rotate} {x:.1f} {y:.1f})"' if rotate else "")
        self.parts.append(
            f'<text x="{x:.1f}" y="{y:.1f}" font-family="{FONT}" font-size="{size}" '
            f'fill="{color}" text-anchor="{anchor}" font-weight="{weight}" '
            f'style="{style}"{r}>{s}</text>'
        )

    def dtext(self, x, y, s, **kw):
        self.text(self._tx(x), self._ty(y), s, **kw)

    def path(self, xs, ys, color=BLUE, w=2.2, dash=None, opacity=1.0, fill="none"):
        pts = " ".join(
            f"{self._tx(float(x)):.1f},{self._ty(float(y)):.1f}"
            for x, y in zip(xs, ys)
        )
        d = f' stroke-dasharray="{dash}"' if dash else ""
        self.parts.append(
            f'<polyline points="{pts}" fill="{fill}" stroke="{color}" '
            f'stroke-width="{w}"{d} opacity="{opacity}" '
            f'stroke-linejoin="round" stroke-linecap="round"/>'
        )

    def dot(self, x, y, r=4, color=BLUE, opacity=1.0, stroke="none"):
        self.parts.append(
            f'<circle cx="{self._tx(x):.1f}" cy="{self._ty(y):.1f}" r="{r}" '
            f'fill="{color}" opacity="{opacity}" stroke="{stroke}"/>'
        )

    def band_x(self, x0, x1, color=LIGHT, opacity=0.8):
        X0, X1 = self._tx(x0), self._tx(x1)
        self.parts.append(
            f'<rect x="{X0:.1f}" y="{self.mt}" width="{X1 - X0:.1f}" '
            f'height="{self.h - self.mt - self.mb}" fill="{color}" opacity="{opacity}"/>'
        )

    # -- axes ------------------------------------------------------------ #
    def axes(self, xticks, yticks, xlabel="", ylabel="", title="",
             xtick_labels=None, ytick_labels=None, grid=True):
        x0, y0 = self.ml, self.h - self.mb
        x1, y1 = self.w - self.mr, self.mt
        if title:
            self.text((x0 + x1) / 2, self.mt - 26, title, size=18, weight="bold")
        # gridlines + ticks
        for i, xt in enumerate(xticks):
            X = self._tx(xt)
            if grid:
                self.line(X, y0, X, y1, GRAY, 0.6, dash="2,4", opacity=0.7)
            self.line(X, y0, X, y0 + 5, INK, 1.2)
            lab = xtick_labels[i] if xtick_labels else fmt(xt)
            self.text(X, y0 + 22, lab, size=14)
        for i, yt in enumerate(yticks):
            Y = self._ty(yt)
            if grid:
                self.line(x0, Y, x1, Y, GRAY, 0.6, dash="2,4", opacity=0.7)
            self.line(x0 - 5, Y, x0, Y, INK, 1.2)
            lab = ytick_labels[i] if ytick_labels else fmt(yt)
            self.text(x0 - 9, Y + 5, lab, size=14, anchor="end")
        # frame
        self.line(x0, y0, x1, y0, INK, 1.5)
        self.line(x0, y0, x0, y1, INK, 1.5)
        if xlabel:
            self.text((x0 + x1) / 2, self.h - 10, xlabel, size=15.5)
        if ylabel:
            self.text(16, (y0 + y1) / 2, ylabel, size=15.5, rotate=-90)

    def save(self, name):
        OUT.mkdir(parents=True, exist_ok=True)
        body = "\n".join(self.parts)
        svg = (
            f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {self.w} {self.h}" '
            f'font-family="{FONT}">\n'
            f'<rect width="{self.w}" height="{self.h}" fill="white"/>\n'
            f"{body}\n</svg>\n"
        )
        (OUT / name).write_text(svg)
        print(f"  {name}")


# ========================================================================= #
# 1. Square wave from its harmonics (Ch 5 §5.2)
# ========================================================================= #
def fourier_square():
    f = Fig(720, 500, margin=(46, 20, 40, 58))
    t = np.linspace(0, 2, 900)

    def partial(n_terms):
        y = np.zeros_like(t)
        for k in range(n_terms):
            n = 2 * k + 1
            y += (4 / math.pi) * np.sin(2 * math.pi * n * t) / n
        return y

    panels = [(1, "fundamental only"), (2, "+ 3rd harmonic"),
              (4, "up to the 7th"), (25, "up to the 49th")]
    ph, gap = 100, 14
    f.text(f.w / 2, 26, "A square wave is its harmonics", size=18, weight="bold")
    for i, (n, label) in enumerate(panels):
        top = 46 + i * (ph + gap)
        f.mt, f.mb = top, f.h - top - ph
        f.xlim, f.ylim = (0, 2), (-1.65, 1.65)
        # target square wave, faint
        sq_t = np.linspace(0, 2, 1200)
        sq = np.sign(np.sin(2 * math.pi * sq_t) + 1e-12)
        f.path(sq_t, sq, color=GRAY, w=1.2, dash="3,3")
        f.path(t, partial(n), color=BLUE, w=2.0)
        f.line(f.ml, f._ty(0), f.w - f.mr, f._ty(0), INK, 0.8)
        f.text(f.w - f.mr - 4, top + 16, label, size=13.5, anchor="end",
               color=RED)
    f.mt, f.mb = 46, 40
    f.text(f.w / 2, f.h - 10,
           "the sharp corners are the high harmonics — remove them and the corners round off",
           size=13.5, color=INK)
    f.save("fourier_square.svg")


# ========================================================================= #
# 2. Queueing delay vs utilisation (Ch 3 §3.2, Ch 13 §13.3)
# ========================================================================= #
def queueing_delay():
    f = Fig()
    f.xlim, f.ylim = (0, 1.0), (0, 20)
    rho = np.linspace(0, 0.952, 500)
    f.axes([0, 0.2, 0.4, 0.6, 0.8, 1.0], [0, 5, 10, 15, 20],
           xlabel="utilisation ρ", ylabel="relative queueing delay ρ/(1−ρ)",
           title="Queueing delay does not rise gently")
    f.band_x(0.6, 0.7, color="#e3efe3")
    f.dtext(0.65, 19, "plan here", size=13, color=TEAL)
    f.path(rho, rho / (1 - rho), color=BLUE, w=2.6)
    for r in (0.5, 0.8, 0.9):
        d = r / (1 - r)
        f.dot(r, d, color=RED)
        f.dtext(r - 0.015, d + 1.2, f"ρ={fmt(r)} → {fmt(d)}×", size=13.5,
                anchor="end", color=RED)
    f.dtext(0.31, 16.4,
            "a link at 90% has nine times the delay of one at 50%",
            size=14, anchor="start")
    f.save("queueing_delay.svg")


# ========================================================================= #
# 3. Shannon capacity per hertz vs SNR (Ch 4 §4.4)
# ========================================================================= #
def shannon_capacity():
    f = Fig()
    f.xlim, f.ylim = (-10, 40), (0, 14)
    snr_db = np.linspace(-10, 40, 500)
    c = np.log2(1 + 10 ** (snr_db / 10))
    f.axes([-10, 0, 10, 20, 30, 40], [0, 2, 4, 6, 8, 10, 12, 14],
           xlabel="signal-to-noise ratio (dB)",
           ylabel="capacity (bits per second per hertz)",
           title="Shannon's ceiling:  C/B = log₂(1 + SNR)")
    f.path(snr_db, c, color=BLUE, w=2.6)
    # the two regimes
    f.dtext(-9, 3.4, "low SNR: capacity ≈ linear in power —", size=13.5,
            anchor="start", color=TEAL)
    f.dtext(-9, 2.6, "every watt counts (GPS, deep space)", size=13.5,
            anchor="start", color=TEAL)
    f.dtext(20, 11.6, "high SNR: +3 dB buys ONE more bit —", size=13.5,
            anchor="start", color=RED)
    f.dtext(20, 10.8, "doubling the power, again and again", size=13.5,
            anchor="start", color=RED)
    # +3dB = +1 bit staircase illustration
    for s in (20, 23, 26, 29):
        f.dot(s, math.log2(1 + 10 ** (s / 10)), color=RED, r=3.5)
    f.save("shannon_capacity.svg")


# ========================================================================= #
# 4. Constellations: QPSK, 16-QAM, and 16-QAM as the receiver sees it
# ========================================================================= #
def constellations():
    f = Fig(720, 322, margin=(50, 10, 26, 10))
    rng = np.random.default_rng(1948)
    f.text(f.w / 2, 26, "Constellations: what more bits per symbol costs",
           size=18, weight="bold")
    panel_w = (f.w - 60) / 3
    titles = ["QPSK — 2 bits", "16-QAM — 4 bits", "16-QAM at the receiver"]

    def draw_panel(i, pts, noisy):
        cx = 30 + i * panel_w + panel_w / 2
        cy = 175
        half = 105
        # box + axes
        f.raw(f'<rect x="{cx-half}" y="{cy-half}" width="{2*half}" height="{2*half}" '
              f'fill="none" stroke="{INK}" stroke-width="1.2"/>')
        f.line(cx - half, cy, cx + half, cy, GRAY, 0.8)
        f.line(cx, cy - half, cx, cy + half, GRAY, 0.8)
        f.text(cx, cy + half + 22, titles[i], size=14.5)
        scale = half / 1.45
        for (x, y) in pts:
            if noisy:
                for _ in range(14):
                    nx = x + rng.normal(0, 0.10)
                    ny = y + rng.normal(0, 0.10)
                    f.raw(f'<circle cx="{cx + nx*scale:.1f}" cy="{cy - ny*scale:.1f}" '
                          f'r="2.1" fill="{BLUE}" opacity="0.45"/>')
            else:
                f.raw(f'<circle cx="{cx + x*scale:.1f}" cy="{cy - y*scale:.1f}" '
                      f'r="5" fill="{BLUE}"/>')

    qpsk = [(sx * 0.9, sy * 0.9) for sx in (-1, 1) for sy in (-1, 1)]
    lv = (-1.2, -0.4, 0.4, 1.2)
    qam16 = [(x, y) for x in lv for y in lv]
    draw_panel(0, qpsk, False)
    draw_panel(1, qam16, False)
    draw_panel(2, qam16, True)
    f.save("constellations.svg")


# ========================================================================= #
# 5. Eye diagram (Ch 6 §6.3)
# ========================================================================= #
def eye_diagram():
    f = Fig(720, 400, margin=(50, 24, 44, 58))
    rng = np.random.default_rng(6)
    f.xlim, f.ylim = (0, 2), (-1.9, 1.9)
    f.axes([0, 0.5, 1.0, 1.5, 2.0], [-1, 0, 1],
           xlabel="time (symbol periods)", ylabel="received level",
           title="The eye diagram: 120 symbols overlaid",
           xtick_labels=["0", "T/2", "T", "3T/2", "2T"], grid=False)

    # raised-cosine pulse
    def rc(t, beta=0.35):
        t = np.where(np.abs(np.abs(t) - 1 / (2 * beta)) < 1e-9, t + 1e-6, t)
        return (np.sinc(t) * np.cos(math.pi * beta * t)
                / (1 - (2 * beta * t) ** 2))

    tt = np.linspace(0, 2, 160)
    nsym = 120
    bits = rng.choice([-1.0, 1.0], size=nsym + 8)
    for i in range(nsym):
        y = np.zeros_like(tt)
        for k in range(-3, 5):
            y += bits[i + k + 3] * rc(tt - k)
        y += rng.normal(0, 0.055, size=tt.shape)
        f.path(tt, y, color=BLUE, w=0.8, opacity=0.16)
    # annotations
    f.dline(1.0, -1.55, 1.0, 1.55, color=RED, w=1.4, dash="5,4")
    f.dtext(1.0, 1.75, "sample here", size=14, color=RED)
    f.dtext(1.52, 0.06, "the eye:", size=14, color=RED, anchor="start")
    f.dtext(1.52, -0.22, "noise margin ↕, timing margin ↔", size=13,
            color=RED, anchor="start")
    f.save("eye_diagram.svg")


# ========================================================================= #
# 6. Fibre attenuation vs wavelength (Ch 6 §6.1, Ch 10 §10.3)
# ========================================================================= #
def fiber_attenuation():
    f = Fig()
    f.xlim, f.ylim = (800, 1700), (0, 3.2)
    wl = np.linspace(800, 1700, 900)
    rayleigh = 0.9 * (1000 / wl) ** 4          # ~1/λ⁴, scaled
    ir = 0.02 * np.exp((wl - 1550) / 90)       # infrared absorption edge
    oh = 1.9 * np.exp(-0.5 * ((wl - 1383) / 16) ** 2)  # water peak
    loss = rayleigh + ir + oh + 0.05
    f.axes([800, 1000, 1200, 1400, 1600], [0, 0.5, 1, 1.5, 2, 2.5, 3],
           xlabel="wavelength (nm)", ylabel="attenuation (dB/km)",
           title="Why the windows are where they are")
    for x0, x1, name in ((1260, 1360, "O"), (1530, 1565, "C"), (1565, 1625, "L")):
        f.band_x(x0, x1, color="#dde8dd")
        f.dtext((x0 + x1) / 2, 3.0, name, size=14, color=TEAL, weight="bold")
    f.path(wl, loss, color=BLUE, w=2.4)
    f.dtext(1383, 2.45, "OH⁻ water peak (1383 nm)", size=13, color=RED)
    f.dtext(905, 2.6, "Rayleigh scattering ∝ 1/λ⁴", size=13, anchor="start")
    f.dtext(1660, 1.6, "infrared", size=13, anchor="end")
    f.dtext(1660, 1.38, "absorption", size=13, anchor="end")
    f.dtext(1547, 0.45, "≈0.2 dB/km — and erbium amplifies here", size=13,
            color=TEAL, anchor="end")
    f.save("fiber_attenuation.svg")


# ========================================================================= #
# 7. OFDM subcarriers: overlapping and orthogonal (Ch 8 §8.4)
# ========================================================================= #
def ofdm_subcarriers():
    f = Fig(720, 380, margin=(52, 24, 46, 58))
    f.xlim, f.ylim = (-1.5, 7.5), (-0.32, 1.15)
    freq = np.linspace(-1.5, 7.5, 1400)
    f.axes([0, 1, 2, 3, 4, 5, 6], [0, 0.5, 1.0],
           xlabel="frequency (multiples of 1/T)", ylabel="amplitude",
           title="OFDM: spectra overlap, information does not", grid=False)
    colors = [BLUE, TEAL, ORANGE, RED, BLUE, TEAL, ORANGE]
    for k in range(7):
        y = np.sinc(freq - k)
        f.path(freq, y, color=colors[k], w=1.8, opacity=0.85)
    f.dline(-1.5, 0, 7.5, 0, color=INK, w=1.0)
    # peak-on-nulls marker
    for k in range(7):
        f.dot(k, 1.0, r=3.5, color=colors[k])
    f.dline(3, 0, 3, 1.0, color=INK, w=1.0, dash="3,3")
    f.dtext(3.15, 1.07, "each peak sits on every neighbour's nulls",
            size=14, anchor="start")
    f.save("ofdm_subcarriers.svg")


# ========================================================================= #
# 8. TCP congestion window: slow start, loss, AIMD sawtooth (Ch 38 §38.2)
# ========================================================================= #
def tcp_cwnd():
    f = Fig()
    f.xlim, f.ylim = (0, 40), (0, 44)
    f.axes([0, 10, 20, 30, 40], [0, 10, 20, 30, 40],
           xlabel="time (round trips)", ylabel="congestion window (segments)",
           title="Slow start, then the AIMD sawtooth")
    # slow start: 1,2,4,8,16,32 -> loss at 32 (t=0..5)
    t, w = [0], [1.0]
    cur = 1.0
    for i in range(1, 6):
        cur *= 2
        t.append(i); w.append(cur)
    f.path(t, w, color=TEAL, w=2.6)
    f.dtext(2.1, 22, "slow start:", size=13.5, color=TEAL, anchor="start")
    f.dtext(2.1, 19.4, "double per RTT", size=13.5, color=TEAL, anchor="start")
    # loss, halve to 16, then +1/RTT sawtooth
    f.dot(5, 32, color=RED, r=5)
    f.dtext(5.4, 34.5, "loss", size=14, color=RED, anchor="start")
    x0, wlo = 5, 16.0
    xs, ys = [x0], [wlo]
    x, wv = x0, wlo
    while x < 40:
        # climb until w reaches 32 again
        climb = 32 - wv
        x2 = min(x + climb, 40)
        xs += [x2]; ys += [wv + (x2 - x)]
        if x2 >= 40:
            break
        f.dot(x2, 32, color=RED, r=4)
        x, wv = x2, 16.0
        xs += [x2]; ys += [wv]
    f.path(xs, ys, color=BLUE, w=2.6)
    f.dline(0, 32, 40, 32, color=GRAY, w=1.2, dash="6,4")
    f.dtext(39.5, 33.6, "path capacity", size=13, anchor="end", color=GRAY)
    f.dtext(24, 10, "additive increase (+1 MSS per RTT),", size=13.5)
    f.dtext(24, 7.4, "multiplicative decrease (halve on loss)", size=13.5)
    f.save("tcp_cwnd.svg")


# ========================================================================= #
# 9. Mathis: throughput vs loss (Ch 3 §3.3, Ch 38 §38.2)
# ========================================================================= #
def mathis():
    f = Fig()
    f.xlog = f.ylog = True
    f.xlim, f.ylim = (1e-6, 3e-2), (0.3, 2000)
    p = np.logspace(-6, math.log10(3e-2), 300)
    f.axes([1e-6, 1e-5, 1e-4, 1e-3, 1e-2],
           [1, 10, 100, 1000],
           xtick_labels=["0.0001%", "0.001%", "0.01%", "0.1%", "1%"],
           ytick_labels=["1", "10", "100", "1000"],
           xlabel="packet loss rate", ylabel="max single-stream throughput (Mb/s)",
           title="Loss caps TCP throughput  (Mathis: rate ∝ 1/√p)")
    for rtt, color, label in ((0.02, TEAL, "20 ms RTT"),
                              (0.08, BLUE, "80 ms"),
                              (0.2, RED, "200 ms")):
        thr = 1.22 * 1460 * 8 / rtt / np.sqrt(p) / 1e6
        f.path(p, thr, color=color, w=2.4)
        f.dtext(2.2e-6, 1.22 * 1460 * 8 / rtt / math.sqrt(2.2e-6) / 1e6 * 1.25,
                label, size=13.5, color=color, anchor="start")
    f.dot(0.01, 1.22 * 1460 * 8 / 0.08 / math.sqrt(0.01) / 1e6, color=BLUE, r=5)
    f.dtext(2.2e-6, 1.15, "1% loss on an 80 ms path caps a stream", size=13.5,
            anchor="start")
    f.dtext(2.2e-6, 0.68, "at 1.8 Mb/s — on any link, however fast", size=13.5,
            anchor="start")
    f.save("mathis_throughput.svg")


# ========================================================================= #
# 10. Free-space path loss (Ch 42 §42.3)
# ========================================================================= #
def fspl():
    f = Fig()
    f.xlog = True
    f.xlim, f.ylim = (0.01, 10), (40, 130)
    d = np.logspace(-2, 1, 300)  # km
    f.axes([0.01, 0.1, 1, 10], [40, 60, 80, 100, 120],
           xtick_labels=["10 m", "100 m", "1 km", "10 km"],
           xlabel="distance", ylabel="free-space path loss (dB)",
           title="FSPL: +6 dB per doubling of distance or frequency")
    label_at = {900: 8.0, 2400: 8.0, 5000: 1.1, 6000: 6.0}
    for mhz, color, label in ((900, TEAL, "900 MHz"), (2400, BLUE, "2.4 GHz"),
                              (5000, ORANGE, "5 GHz"), (6000, RED, "6 GHz")):
        loss = 20 * np.log10(d) + 20 * math.log10(mhz) + 32.44
        f.path(d, loss, color=color, w=2.2)
        lx = label_at[mhz]
        f.dtext(lx, 20 * math.log10(lx) + 20 * math.log10(mhz) + 32.44
                + (3.2 if mhz != 900 else -4.5), label, size=13, color=color)
    f.dot(0.1, 80.0, color=BLUE, r=5)
    f.dtext(0.095, 84.5, "2.4 GHz at 100 m = 80 dB", size=13.5, anchor="start")
    f.save("fspl.svg")


# ========================================================================= #
# 11. Fresnel zone geometry (Ch 42 §42.3)
# ========================================================================= #
def fresnel():
    f = Fig(720, 340, margin=(44, 30, 30, 30))
    W, H = 720, 340
    gy = 268                       # ground line y
    x1, x2 = 90, 630               # mast positions
    ty = 120                       # antenna height (px)
    f.text(W / 2, 30, "Line of sight is not enough: the first Fresnel zone",
           size=18, weight="bold")
    # ground
    f.line(20, gy, W - 20, gy, INK, 1.6)
    for gx in range(30, W - 20, 34):
        f.line(gx, gy, gx - 9, gy + 9, GRAY, 1.0)
    # masts
    for x in (x1, x2):
        f.line(x, gy, x, ty, INK, 3.0)
        f.raw(f'<circle cx="{x}" cy="{ty}" r="6" fill="{BLUE}"/>')
    f.text(x1, gy + 26, "Tx", size=14)
    f.text(x2, gy + 26, "Rx", size=14)
    # LOS
    f.line(x1, ty, x2, ty, BLUE, 1.6, dash="7,5")
    # Fresnel ellipse (first zone), and 60% inner
    cx, cy = (x1 + x2) / 2, ty
    a = (x2 - x1) / 2
    b = 74
    f.raw(f'<ellipse cx="{cx}" cy="{cy}" rx="{a}" ry="{b}" fill="{BLUE}" '
          f'opacity="0.10" stroke="{BLUE}" stroke-width="1.4"/>')
    f.raw(f'<ellipse cx="{cx}" cy="{cy}" rx="{a}" ry="{b*0.6:.0f}" fill="none" '
          f'stroke="{TEAL}" stroke-width="1.3" stroke-dasharray="5,4"/>')
    # radius annotation, left of centre so nothing collides with it
    f.line(cx, cy, cx, cy + b, RED, 1.6)
    f.text(cx - 12, cy + b / 2 + 2, "r = 17.32 √(d / 4f)", size=14.5,
           color=RED, anchor="end")
    f.text(cx - 12, cy + b / 2 + 21, "(5.6 m at 1 km, 2.4 GHz)", size=13,
           color=RED, anchor="end")
    # tree obstruction poking into the zone, labelled beneath the ground line
    tx0 = cx + 165
    tree_top = cy + b * 0.38
    f.line(tx0, gy, tx0, tree_top + 22, "#5a4632", 5)
    for dy, r in ((0, 24), (17, 19), (32, 14)):
        f.raw(f'<circle cx="{tx0}" cy="{tree_top + dy:.0f}" r="{r}" '
              f'fill="{TEAL}" opacity="0.55"/>')
    f.text(tx0, gy + 26, "growth into the zone: loss,", size=13.5, color=INK)
    f.text(tx0, gy + 44, "with clear line of sight", size=13.5, color=INK)
    f.text(cx - 150, cy - b * 0.62 - 10, "keep the inner 60% clear",
           size=13, color=TEAL)
    f.save("fresnel.svg")


# ========================================================================= #
# 12. Erlang B blocking (Ch 12 §12.4)
# ========================================================================= #
def erlang():
    f = Fig()
    f.ylog = True
    f.xlim, f.ylim = (0, 40), (1e-4, 1.0)

    def erlang_b(n, a):
        b = 1.0
        for i in range(1, n + 1):
            b = a * b / (i + a * b)
        return b

    f.axes([0, 10, 20, 30, 40],
           [1e-4, 1e-3, 1e-2, 1e-1, 1],
           ytick_labels=["0.01%", "0.1%", "1%", "10%", "100%"],
           xlabel="circuits provided (N)", ylabel="blocking probability",
           title="Erlang B: how many circuits for 1% blocking?")
    f.dline(0, 0.01, 40, 0.01, color=GRAY, w=1.4, dash="6,4")
    f.dtext(1, 0.013, "1% grade of service", size=13, color=GRAY, anchor="start")
    for a, color in ((5, TEAL), (10, BLUE), (20, RED)):
        ns = list(range(1, 41))
        bs = [max(erlang_b(n, a), 1e-4) for n in ns]
        f.path(ns, bs, color=color, w=2.4)
        need = next(n for n in ns if erlang_b(n, a) < 0.01)
        f.dot(need, 0.01, color=color, r=4.5)
        f.dtext(a + 1.2, 0.55, f"A = {a} erlangs", size=13.5, color=color,
                anchor="start")
        f.dtext(need, 0.0045, f"{need}", size=13, color=color)
    f.dtext(27.2, 0.00035, "18 circuits carry 10 erlangs —", size=13.5)
    f.dtext(27.2, 0.0002, "the statistical multiplexing gain", size=13.5)
    f.save("erlang_blocking.svg")


# ========================================================================= #
# 13. ALOHA throughput (Ch 16 §16.1)
# ========================================================================= #
def aloha():
    f = Fig()
    f.xlim, f.ylim = (0, 4), (0, 0.44)
    g = np.linspace(0, 4, 400)
    f.axes([0, 1, 2, 3, 4], [0, 0.1, 0.2, 0.3, 0.4],
           xlabel="offered load G (packets per packet time)",
           ylabel="throughput S",
           title="ALOHA: more load, then less throughput")
    f.path(g, g * np.exp(-2 * g), color=BLUE, w=2.6)
    f.path(g, g * np.exp(-g), color=TEAL, w=2.6)
    f.dot(0.5, 0.5 / math.e, color=BLUE, r=5)
    f.dot(1.0, 1 / math.e, color=TEAL, r=5)
    f.dtext(0.62, 0.16, "pure ALOHA", size=14, color=BLUE, anchor="start")
    f.dtext(0.56, 0.135, "peak 18.4% at G = 0.5", size=13, color=BLUE,
            anchor="start")
    f.dtext(1.42, 0.395, "slotted ALOHA", size=14, color=TEAL, anchor="start")
    f.dtext(1.42, 0.37, "peak 36.8% at G = 1", size=13, color=TEAL,
            anchor="start")
    f.dtext(3.9, 0.075, "past the peak: collisions breed", size=13.5,
            anchor="end", color=RED)
    f.dtext(3.9, 0.05, "retransmissions breed collisions", size=13.5,
            anchor="end", color=RED)
    f.save("aloha_throughput.svg")


# ========================================================================= #
# 14. The MCS ladder against Shannon (Ch 44 §44.1, Ch 8 §8.3)
# ========================================================================= #
def mcs_ladder():
    f = Fig()
    f.xlim, f.ylim = (0, 45), (0, 13.5)
    snr_db = np.linspace(0, 45, 400)
    f.axes([0, 10, 20, 30, 40], [0, 2, 4, 6, 8, 10, 12],
           xlabel="SNR (dB)", ylabel="bits per symbol",
           title="Rate adaptation: the modulation ladder under Shannon's curve")
    f.path(snr_db, np.log2(1 + 10 ** (snr_db / 10)), color=GRAY, w=2.0,
           dash="7,5")
    f.dtext(28, 11.4, "Shannon bound", size=13.5, color=GRAY)
    steps = [(4, 1, "BPSK"), (7, 2, "QPSK"), (15, 4, "16-QAM"),
             (21, 6, "64-QAM"), (27, 8, "256-QAM"), (33, 10, "1024-QAM"),
             (39, 12, "4096-QAM")]
    xs, ys = [0], [0]
    for i, (thr, bits, _) in enumerate(steps):
        nxt = steps[i + 1][0] if i + 1 < len(steps) else 45
        xs += [thr, thr, nxt]
        ys += [ys[-1], bits, bits]
    f.path(xs, ys, color=BLUE, w=2.6)
    for thr, bits, name in steps:
        f.dtext(thr + 0.6, bits + 0.55, name, size=12.5, color=BLUE,
                anchor="start")
    f.dtext(43, 2.1, "walk toward the AP:", size=13.5, anchor="end", color=RED)
    f.dtext(43, 1.2, "climb the ladder", size=13.5, anchor="end", color=RED)
    f.save("mcs_ladder.svg")


# ========================================================================= #
# 15. Bufferbloat: latency under load (Ch 66 §66.4)
# ========================================================================= #
def bufferbloat():
    f = Fig(margin=(58, 20, 46, 72))
    f.ylog = True
    f.xlim, f.ylim = (0, 60), (3, 3000)
    rng = np.random.default_rng(66)
    f.axes([0, 10, 20, 30, 40, 50, 60],
           [10, 100, 1000],
           ytick_labels=["10 ms", "100 ms", "1 s"],
           xlabel="time (s)", ylabel="ping RTT (log scale)",
           title="Bufferbloat: measure latency under load")
    t_idle = np.arange(0, 15, 0.5)
    f.path(t_idle, 8 + rng.normal(0, 0.5, t_idle.size).clip(-1.5, 1.5),
           color=TEAL, w=2.0)
    t_load = np.arange(15, 40, 0.5)
    ramp = 800 * (1 - np.exp(-(t_load - 15) / 1.6)) + 8
    f.path(t_load, ramp + rng.normal(0, 45, t_load.size) * (ramp / 800),
           color=RED, w=2.0)
    t_aqm = np.arange(40, 60, 0.5)
    f.path(t_aqm, 16 + rng.normal(0, 1.6, t_aqm.size).clip(-4, 5),
           color=BLUE, w=2.0)
    f.dline(15, 3, 15, 3000, color=GRAY, w=1.0, dash="4,4")
    f.dline(40, 3, 40, 3000, color=GRAY, w=1.0, dash="4,4")
    f.dtext(7.5, 4.6, "idle: 8 ms", size=13.5, color=TEAL)
    f.dtext(27.5, 1800, "upload starts: same link, ~800 ms", size=13.5,
            color=RED)
    f.dtext(50, 5.4, "CAKE shaper on: 16 ms", size=13.5, color=BLUE)
    f.dtext(27.5, 1150, "throughput unchanged throughout", size=12.5, color=RED)
    f.save("bufferbloat.svg")


# ========================================================================= #
# 16. Cellular frequency reuse (Ch 46 §46.1)
# ========================================================================= #
def cell_reuse():
    f = Fig(720, 430, margin=(46, 20, 20, 20))
    f.text(360, 28, "Frequency reuse: the same channels, far enough apart",
           size=18, weight="bold")
    R = 44  # hex radius
    cx0, cy0 = 360, 235
    palette = {1: "#c7d7ea", 2: "#cde3cd", 3: "#eadbc7", 4: "#e3c9c9",
               5: "#d9cbe3", 6: "#cfe3e0", 7: "#e8e3c4"}

    def hexpts(cx, cy):
        return " ".join(
            f"{cx + R * math.cos(math.radians(a)):.1f},"
            f"{cy + R * math.sin(math.radians(a)):.1f}"
            for a in range(30, 390, 60)
        )

    # axial coordinates for a hex patch; reuse-7 group id
    def group(q, r):
        # N=7 reuse pattern (i=2, j=1):  g = (q + 3r) mod 7  gives the classic tiling
        return ((q + 3 * r) % 7) + 1

    w = R * math.sqrt(3)
    cells = []
    for q in range(-4, 5):
        for r in range(-3, 4):
            x = cx0 + w * (q + r / 2)
            y = cy0 + R * 1.5 * r
            if 40 < x < 680 and 60 < y < 420:
                cells.append((q, r, x, y))
    ones = []
    for q, r, x, y in cells:
        g = group(q, r)
        f.raw(f'<polygon points="{hexpts(x, y)}" fill="{palette[g]}" '
              f'stroke="{INK}" stroke-width="1.0"/>')
        f.text(x, y + 5.5, f"f{g}", size=13.5,
               color=INK if g != 1 else RED,
               weight="bold" if g == 1 else "normal")
        if g == 1:
            ones.append((x, y))
    # line between the two NEAREST co-channel cells: the reuse distance
    if len(ones) >= 2:
        best = None
        for i in range(len(ones)):
            for j in range(i + 1, len(ones)):
                d2 = ((ones[i][0] - ones[j][0]) ** 2
                      + (ones[i][1] - ones[j][1]) ** 2)
                if best is None or d2 < best[0]:
                    best = (d2, ones[i], ones[j])
        (_, (xa, ya), (xb, yb)) = best
        f.line(xa, ya, xb, yb, RED, 2.0, dash="7,5")
        for x, y in ((xa, ya), (xb, yb)):
            f.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="16" fill="white" '
                  f'stroke="{RED}" stroke-width="2"/>')
            f.text(x, y + 5.5, "f1", size=13.5, color=RED, weight="bold")
        f.text(360, f.h - 6,
               "cells labelled f1 share frequencies; the dashed line is the "
               "reuse distance D = R√(3N)", size=13.5, color=RED)
    f.save("cell_reuse.svg")


# ========================================================================= #
# 17. The availability ladder (Ch 56 §56.1)
# ========================================================================= #
def nines():
    f = Fig(720, 440, margin=(58, 24, 46, 74))
    f.ylog = True
    f.xlim, f.ylim = (1.7, 6.3), (0.3, 1e6)
    f.axes([2, 3, 4, 5, 6],
           [1, 60, 1440, 10080, 525960 / 12, 525960],
           xtick_labels=["99%", "99.9%", "99.99%", "99.999%", "99.9999%"],
           ytick_labels=["1 min", "1 hour", "1 day", "1 week", "1 month",
                         "1 year"],
           xlabel="availability target",
           ylabel="downtime budget per year",
           title="Each nine divides the downtime budget by ten")
    ns = np.array([2, 3, 4, 5, 6])
    mins = 525960 * 10.0 ** (-ns.astype(float))  # minutes/year allowed down
    f.path(ns, mins, color=BLUE, w=2.4)
    labels = ["3.65 days", "8.76 hours", "52.6 min", "5.26 min", "31.6 sec"]
    for n, m, lab in zip(ns, mins, labels):
        f.dot(n, m, r=5, color=BLUE)
        f.dtext(n + 0.06, m * 2.1, lab, size=13.5, anchor="start", weight="bold")
    # the human/automation boundary the section argues for
    f.dline(1.7, 43.2 * 12, 6.3, 43.2 * 12, color=RED, w=1.4, dash="6,5")
    f.dtext(6.2, 43.2 * 12 * 2.4, "a person can be woken and log in",
            size=13, color=RED, anchor="end")
    f.dtext(6.2, 43.2 * 12 / 2.4, "only automation acts this fast",
            size=13, color=RED, anchor="end")
    f.save("nines.svg")


# ========================================================================= #
# 18. Percentiles beat averages (Ch 54 §54.1)
# ========================================================================= #
def latency_percentiles():
    rng = np.random.default_rng(54)
    # body: log-normal around ~18 ms; tail: a second, slower mode.
    body = rng.lognormal(math.log(18), 0.32, 9500)
    tail = rng.lognormal(math.log(240), 0.55, 500)
    lat = np.concatenate([body, tail])

    f = Fig(720, 420, margin=(58, 24, 46, 62))
    f.xlog = True
    f.xlim, f.ylim = (5, 2000), (0, 1.0)
    # histogram in log-spaced bins, normalised to peak 1
    edges = np.logspace(math.log10(5), math.log10(2000), 70)
    hist, _ = np.histogram(lat, bins=edges)
    hist = hist / hist.max()
    f.axes([10, 30, 100, 300, 1000], [],
           xtick_labels=["10 ms", "30 ms", "100 ms", "300 ms", "1 s"],
           xlabel="response latency (log scale)",
           ylabel="how often",
           title="The mean hides the tail; the tail is the complaints")
    for x0, x1, h in zip(edges[:-1], edges[1:], hist):
        X0, X1 = f._tx(x0), f._tx(x1)
        Y0, Y1 = f._ty(0), f._ty(float(h) * 0.92)
        f.raw(f'<rect x="{X0:.1f}" y="{Y1:.1f}" width="{max(X1 - X0 - 0.6, 0.5):.1f}" '
              f'height="{Y0 - Y1:.1f}" fill="{BLUE}" opacity="0.55"/>')
    marks = [(float(np.percentile(lat, 50)), "p50", INK, 0.99),
             (float(lat.mean()), "mean", TEAL, 0.90),
             (float(np.percentile(lat, 95)), "p95", ORANGE, 0.99),
             (float(np.percentile(lat, 99)), "p99", RED, 0.99)]
    for x, lab, color, ytop in marks:
        f.dline(x, 0, x, ytop - 0.06, color=color, w=1.8, dash="5,4")
        f.dtext(x, ytop, f"{lab}: {x:.0f} ms", size=13.5, color=color,
                weight="bold")
    f.dtext(300, 0.55, "the mean has barely moved;", size=13.5, anchor="start")
    f.dtext(300, 0.48, "every complaint lives here", size=13.5, anchor="start")
    f.save("latency_percentiles.svg")


# ========================================================================= #
# 19. The birthday bound (Ch 58 §58.3)
# ========================================================================= #
def birthday_bound():
    f = Fig(720, 420, margin=(58, 24, 46, 62))
    f.xlog = True
    f.xlim, f.ylim = (1e15, 1e29), (0, 1.05)
    f.axes([1e15, 1e19, 1e23, 1e27], [0, 0.5, 1.0],
           xtick_labels=["10¹⁵", "10¹⁹", "10²³", "10²⁷"],
           xlabel="hashes computed (log scale)",
           ylabel="probability of a collision",
           title="Collisions arrive at 2ⁿᐟ², not 2ⁿ")
    k = np.logspace(15, 29, 400)
    for bits, color in ((128, RED), (160, BLUE)):
        p = 1 - np.exp(-(k.astype(float) ** 2) / (2.0 * (2.0 ** bits)))
        f.path(k, p, color=color, w=2.4)
        half = 2.0 ** (bits / 2)
        f.dline(half, 0, half, 0.393, color=color, w=1.2, dash="4,4")
        f.dot(half, 0.393, r=4.5, color=color)
    f.dtext(2.4e15, 0.97, "128-bit digest:", size=13.5, color=RED, anchor="start", weight="bold")
    f.dtext(2.4e15, 0.90, "coin-flip odds at 2⁶⁴ tries", size=13.5, color=RED, anchor="start")
    f.dtext(3.2e24, 0.30,
            '160-bit: 50/50 at 2<tspan dy="-5" font-size="10">80</tspan>',
            size=13.5, color=BLUE, anchor="start")
    f.dtext(8e28, 0.06, "SHA-256's 2¹²⁸ is ten orders of magnitude past the right edge →",
            size=13, anchor="end")
    f.save("birthday_bound.svg")


# ========================================================================= #
# 20. Brute force vs key length (Ch 58 §58.1)
# ========================================================================= #
def keyspace():
    f = Fig(720, 440, margin=(58, 24, 46, 80))
    f.ylog = True
    f.xlim, f.ylim = (40, 270), (1e-12, 1e60)
    f.axes([56, 80, 112, 128, 192, 256],
           [1e-9, 1, 3.2e7, 3.2e13],
           xtick_labels=["56", "80", "112", "128", "192", "256"],
           ytick_labels=["1 ns", "1 s", "1 year", "10⁶ years"],
           xlabel="key length (bits)",
           ylabel="time to try every key",
           title="Exhaustive search at 10¹⁸ keys per second")
    bits = np.linspace(40, 270, 200)
    secs = 2.0 ** bits / 1e18
    f.path(bits, secs, color=BLUE, w=2.4)
    # reference line: the age of the universe (1.38e10 years = 4.4e17 s)
    f.dline(40, 4.4e17, 270, 4.4e17, color=GRAY, w=1.2, dash="6,5")
    f.dtext(42, 4.4e17 * 12, "the age of the universe", size=13, color=GRAY,
            anchor="start")
    pts = [(56, "DES — under a second here (1998's crack took 56 hours)"),
           (80, "80-bit — ~38,000 years"),
           (128, "AES-128 — 10¹³ years: 800× the age of the universe"),
           (256, "AES-256 — 10⁵² years")]
    for b, lab in pts:
        s = 2.0 ** b / 1e18
        f.dot(b, s, r=5, color=RED)
        anchor = "start" if b < 200 else "end"
        dx = 6 if b < 200 else -6
        f.dtext(b + dx, s * 3e2 if b < 200 else s / 8e3, lab, size=13,
                anchor=anchor, weight="bold" if b == 128 else "normal")
    f.dtext(100, 2e-7, "each added bit doubles the work —", size=13.5,
            anchor="start")
    f.dtext(100, 2e-9, "the straight line is exponential growth on a log scale",
            size=13.5, anchor="start")
    f.save("keyspace.svg")


# ========================================================================= #
# 21. Leaf–spine: every path equal, every link forwarding (Ch 67 §67.4)
# ========================================================================= #
def leaf_spine():
    f = Fig(720, 400, margin=(0, 0, 0, 0))
    f.text(360, 34, "Four equal-cost paths, and a spine failure costs 25%, "
           "not a failover", size=17.5, weight="bold")
    spines = [(180 + i * 120, 100) for i in range(4)]
    leaves = [(90 + i * 108, 280) for i in range(6)]
    # all links, faint
    for sx, sy in spines:
        for lx, ly in leaves:
            f.line(sx, sy + 22, lx, ly - 22, GRAY, 1.1, opacity=0.55)
    # the four equal-cost paths leaf2 -> leaf5
    src, dst = leaves[1], leaves[4]
    for (sx, sy), color in zip(spines, (BLUE, TEAL, ORANGE, RED)):
        f.line(src[0], src[1] - 22, sx, sy + 22, color, 2.6)
        f.line(sx, sy + 22, dst[0], dst[1] - 22, color, 2.6)
    for (x, y), name in zip(spines, ("spine 1", "spine 2", "spine 3", "spine 4")):
        f.raw(f'<rect x="{x-46}" y="{y-22}" width="92" height="44" rx="6" '
              f'fill="white" stroke="{INK}" stroke-width="1.6"/>')
        f.text(x, y + 5, name, size=14.5, weight="bold")
    for i, (x, y) in enumerate(leaves):
        emph = i in (1, 4)
        f.raw(f'<rect x="{x-40}" y="{y-22}" width="80" height="44" rx="6" '
              f'fill="{"#eef3f8" if emph else "white"}" stroke="{INK}" '
              f'stroke-width="{2.2 if emph else 1.6}"/>')
        f.text(x, y + 5, f"leaf {i+1}", size=14.5,
               weight="bold" if emph else "normal")
    for (x, y) in (leaves[1], leaves[4]):
        f.text(x, y + 44, "servers", size=13, color=GRAY)
    f.text(360, 356, "ECMP hashes each flow onto one of the coloured paths — "
           "all links forward, none stands by idle", size=14)
    f.text(360, 378, "losing any one spine removes one path: capacity drops to 3/4 "
           "and nothing else happens", size=14, color=RED)
    f.save("leaf_spine.svg")


# ========================================================================= #
# 22. The window/RTT ceiling (Ch 66 §66.1)
# ========================================================================= #
def window_rtt():
    f = Fig(720, 430, margin=(58, 24, 46, 66))
    f.xlog = f.ylog = True
    f.xlim, f.ylim = (1, 400), (1, 3e4)
    f.axes([1, 10, 100], [1, 10, 100, 1000, 1e4],
           xtick_labels=["1 ms", "10 ms", "100 ms"],
           ytick_labels=["1", "10", "100", "1,000", "10,000"],
           xlabel="round-trip time (log scale)",
           ylabel="max single-stream throughput (Mb/s)",
           title="Throughput ≤ window ÷ RTT — the link speed never appears")
    rtt = np.logspace(0, math.log10(400), 300)  # ms
    f.dline(1, 1e4, 400, 1e4, color=GRAY, w=1.6, dash="7,5")
    f.dtext(1.25, 1.45e4, "the 10 Gb/s link itself", size=13, color=GRAY,
            anchor="start")
    for kb, color, lab in ((64, RED, "64 KB window"),
                           (256, ORANGE, "256 KB"),
                           (1024, TEAL, "1 MB"),
                           (16384, BLUE, "16 MB")):
        thr = np.minimum(kb * 1024 * 8 / (rtt / 1e3) / 1e6, 1e4)
        f.path(rtt, thr, color=color, w=2.4)
        y0 = min(kb * 1024 * 8 / (1.3 / 1e3) / 1e6, 1e4)
        f.dtext(1.35, y0 * 0.62 if y0 < 1e4 else 6.2e3, lab, size=13.5,
                color=color, anchor="start")
    # the worked example from the text: 64 KB at 80 ms = 6.6 Mb/s
    f.dot(80, 64 * 1024 * 8 / 0.08 / 1e6, r=5.5, color=RED)
    f.dtext(72, 3.4, "the text's example: 64 KB at 80 ms", size=13.5,
            anchor="end", weight="bold")
    f.dtext(72, 2.1, "= 6.6 Mb/s on a 10 Gb/s link", size=13.5, anchor="end")
    f.save("window_rtt.svg")


ALL = [fourier_square, queueing_delay, shannon_capacity, constellations,
       eye_diagram, fiber_attenuation, ofdm_subcarriers, tcp_cwnd, mathis,
       fspl, fresnel, erlang, aloha, mcs_ladder, bufferbloat, cell_reuse,
       nines, latency_percentiles, birthday_bound, keyspace, leaf_spine,
       window_rtt]

if __name__ == "__main__":
    print(f"writing {len(ALL)} figures to {OUT}")
    for fn in ALL:
        fn()
    print("done")
