..
   Note: Items in this toctree form the top-level navigation. See `api.rst` for the `autosummary` directive, and for why `api.rst` isn't called directly.

.. toctree::
   :hidden:

   Home page <self>
   getting_started.md
   API reference <_autosummary/nasa_sbm>

.. toctree::
   :hidden:
   :caption: Development:

   Contributing <contributing_guidelines>
   Change Log <change_log>

.. toctree::
   :hidden:
   :caption: Project Links:

   GitHub <https://github.com/ReeceHumphreys/kesspy>
   PyPI <https://pypi.org/project/nasa_sbm/>


Welcome to kesspy
=====================================================
*kesspy* is a Python library for simulating explosion and collision events
in orbit using the NASA Standard Breakup Model (SBM). The breakup model was implemented based on the following works:
NASA's new breakup model of evolve 4.0 (Johnson et al.) [#johnson]_, and
Proper Implementation of the 1998 NASA Breakup Model (Krisko et al.) [#krisko]_.

With this package you can:

* Generate the number of fragments produced in an orbital collision or explosion.
* Find the area and mass of the debris produced by the above fragmentation events.
* Determine the relative velocity of each debris fragment.


Additionally, *kesspy* is used to power ODAP (Orbital Debris Analysis with Python)
which is a larger package that includes propagations, data analysis, and orbit visualization. This package
serves as a stand-alone version of the NASA Breakup Model in the scenario that you do not want the additional
tools provided by ODAP.

.. admonition:: Note - kesspy is in development!

   *kesspy* is currently under development. As a result, the API may change,
   and some functionality may not work as expected. As such, until version 1.0, this package should be
   considered pre-release. Please feel free to create an issue on the GitHub page with any issues or questions!

.. note::
   :doc:`kesspy is licensed under the MIT license <license>`.


.. [#johnson] `NASA's new breakup model of evolve 4.0 <https://www.sciencedirect.com/science/article/abs/pii/S0273117701004239>`_
.. [#krisko] `Proper Implementation of the 1998 NASA Breakup Model <https://orbitaldebris.jsc.nasa.gov/quarterly-news/pdfs/odqnv15i4.pdf>`_
