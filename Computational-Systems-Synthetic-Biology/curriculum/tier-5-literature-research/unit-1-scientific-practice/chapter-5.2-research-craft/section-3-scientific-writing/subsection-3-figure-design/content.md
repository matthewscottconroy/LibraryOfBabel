# Figure Design: Communicating Data Visually

Before most readers finish the abstract of your paper, they have already looked at the figures. This is not a failure of attention — it is how scientists efficiently triage the literature. A reader at a journal club will fan through Figure 1, Figure 2, Figure 3, and form an impression of the work's quality and credibility before reading a single sentence of prose. Poorly designed figures obscure data, mislead readers, and undermine otherwise sound science. Well-designed figures make the findings immediately clear, support the reader's interpretation, and accurately represent the data.

This section develops principles for figure design derived from Edward Tufte's work on data visualization, extended with practical guidance for biological data and the specific tools used in computational biology.

## The Data-Ink Ratio Principle

Edward Tufte's central principle is the **data-ink ratio**: maximize the proportion of ink in a figure that represents data, and minimize the proportion that serves decorative purposes. Every element in a figure should be justified by the information it conveys.

**High data-ink ratio elements (keep):**
- Data points, bars, lines, or areas
- Axis labels and tick marks
- Scale bars and units
- Panel labels (A, B, C)
- Statistical annotations (p-values, effect sizes, error bars)

**Low data-ink ratio elements (remove):**
- Heavy gridlines (use light guidelines or none)
- 3D effects on bars, pies, or surfaces (they distort data and add no information)
- Decorative backgrounds (gradient fills, box borders)
- Redundant legends when direct labeling is possible
- Grey boxes around panels
- Bold borders on bars
- Gaps and excessive white space within panels

**The gridline rule:** Use gridlines only if the reader needs to read specific values from the figure. In most biology figures, the reader needs to compare groups or appreciate trends, not read exact values. Light, dashed gridlines (gray, 50% opacity) are acceptable for reference; heavy solid gridlines compete visually with data.

## Choosing the Right Plot Type

**Distribution data (small n, 3–10 observations per group):**
- **Beeswarm plot** (also called dot plot): shows all individual data points; ideal for n = 3–15
- **Violin plot**: shows the kernel density distribution; appropriate for n > 20
- **Strip chart with mean ± SD**: shows individual points plus summary statistics

**What not to use for small n:**
- **Bar charts with error bars (±SD or ±SEM)**: hides the distribution; a bar showing mean ± SEM for n=3 could represent three clustered points or three wildly dispersed points — you cannot tell from the bar. Many journals (PLOS Biology, Nature Methods) now discourage bar graphs for small n.

**Time-course data:**
- **Line plot**: one line per condition; time on x-axis; mean ± SD or 95% CI as shading or error bars

**Correlation between two continuous variables:**
- **Scatter plot**: one point per observation; regression line with 95% CI; report r² or Pearson r and p-value in the caption

**Comparing proportions across groups:**
- **Bar chart**: proportions (not counts) on y-axis; one bar per condition

**Many groups side-by-side comparison:**
- **Box plot**: shows median, IQR (interquartile range), and outliers; appropriate when n is large enough that these statistics are meaningful (n ≥ 10)

**Heat maps (gene expression, correlation matrices):**
- Cluster rows and columns by hierarchical clustering
- Use diverging color palette for data with a meaningful center (e.g., log fold change, centered around 0)
- Use sequential palette for data with a natural zero
- Include a color scale with labeled tick marks

## Color Principles

### Colorblind Accessibility

Approximately 8% of males and 0.5% of females have red-green color vision deficiency (the most common form of color blindness). Using red and green as the primary contrast is the single most common and most correctable accessibility error in scientific figures.

**Safe color pairs (accessible to all common color vision deficiencies):**
- Blue and orange (strongly contrasting, perceptually distinct)
- Blue and red (distinguishable in most forms of color blindness)
- Black and any saturated color

**Perceptually uniform color palettes (continuous data):**
- **viridis**: runs from dark purple through green to yellow; perceptually uniform; colorblind-safe
- **plasma**: runs from dark purple through red to yellow; colorblind-safe
- **cividis**: viridis variant optimized for deuteranopia

**Categorical color palettes (colorblind-safe):**
- **ColorBrewer** (colorbrewer2.org) palettes designed for colorblind accessibility
- **Okabe-Ito**: 8-color palette specifically designed for colorblindness: black, orange, sky blue, bluish green, yellow, blue, vermilion, reddish purple

**Testing your figures:** Run your figures through a colorblindness simulator before submission. Coblis (color-blindness.com/coblis-color-blindness-simulator) and Sim Daltonism (Mac app) show how your figure appears under different types of color vision deficiency.

### Color Consistency

Use the **same color for the same entity** throughout the entire paper. If E. coli wild type is shown in blue in Figure 1, it should be blue in Figure 3, Figure 4, and all supplementary figures. Inconsistent color assignment forces the reader to check the legend repeatedly and makes comparisons across figures difficult.

### Avoid Relying on Color Alone

For colorblind accessibility and for greyscale printing, **use shape, pattern, or position as a second encoding** alongside color. If two data series are blue circles and orange circles, also make one series open circles and the other closed circles. If two conditions are represented by red and green bars, add a texture to one.

## Direct Labeling vs. Legends

When possible, **label data series directly** on the figure rather than using a legend. A legend requires the reader to look up which color/shape corresponds to which condition; direct labeling eliminates this step. In a time-course figure with three lines, label each line at its endpoint with the condition name.

Direct labeling is practical for:
- Line plots with 2–5 lines
- Bar charts with 2–4 groups
- Scatter plots with 2–4 point series

Legends are necessary when:
- There are too many series to label without overlap
- Data is too dense to add text labels
- The figure has a specific layout that precludes direct labeling

## Typography and Sizing

**Font size:**
- Axis labels: 10–12 pt minimum in the final printed size
- Tick labels: 8–10 pt minimum
- Panel labels (A, B, C): 12 pt bold
- Annotations (p-values, r² values): 8–10 pt

**A common error:** Figures designed at full-screen size on a monitor are often exported at low resolution or reduced to 50% size in the final publication. A label that is readable at 14 pt full-size becomes unreadable at 7 pt when the figure is reduced. Design figures at the target printed size (column width: 8.9 cm; full page width: 18 cm in most journals) and check label readability at that size.

**Figure resolution:** Export figures at 300 dpi minimum for raster graphics (PNG, TIFF). Vector formats (PDF, SVG, EPS) are preferred because they scale without pixelation.

## Tools for Figure Production

**For plotting in code (preferred for reproducibility):**
- **matplotlib** (Python): highly customizable; use `plt.rcParams` to set global aesthetics; export SVG for further editing
- **seaborn** (Python): higher-level API built on matplotlib; excellent for statistical plots (violin, strip, regression)
- **ggplot2** (R): declarative grammar of graphics; excellent defaults; the standard in computational biology

**For diagram and schematic figures:**
- **Inkscape** (free, open-source vector graphics): for circuit diagrams, biological pathway schematics, figure assembly from multiple panels
- **Illustrator** (Adobe, subscription): industry standard; same functionality as Inkscape

**For multi-panel assembly:**
- **Inkscape** or **Illustrator**: import individual panel PDFs/SVGs; arrange; add panel labels; export final figure
- **patchwork** (R) or **matplotlib GridSpec**: assemble panels programmatically (preferred for reproducibility)

## The Reproducibility of Figures

Figures should be produced by scripts with documented parameters, not by point-and-click in Excel or Prism. This ensures that:
- Every figure can be regenerated from raw data
- Changes to upstream data automatically propagate to figures
- The exact plotting parameters (colors, font sizes, axes ranges) are documented

A **figure generation script** (e.g., `fig1_repressilator_timecourse.py`) should be stored in a versioned code repository alongside the analysis code. The script should load raw or processed data from a documented location, apply all plotting parameters, and export the figure to the manuscript figures directory.

## Takeaway

Good figure design is the application of a small number of principles: maximize data-ink ratio, choose appropriate plot types for your data, use colorblind-safe palettes with consistent color encoding, and size labels for legibility at final printed size. These principles are easy to apply with modern tools (ggplot2, matplotlib, seaborn) and dramatically improve the communication quality of your figures. The investment in learning to design figures well is repaid many times over in the clarity of your publications and presentations.
