# acerola-dirt-jam entry

Uses https://www.redblobgames.com/x/1723-procedural-river-growing/ as a base
This is derived from https://web.archive.org/web/20180624211948/https://arches.liris.cnrs.fr/publications/articles/SIGGRAPH2013_PCG_Terrain.pdf

Generates a chunkable voronoi diagram
Each edge of each polygon can be a ridge or a valley water flows perpendicularly through
Each polygon has 0 or 1 exits for water
Can have any amount of entries
If there is one water entry, there is a water exit & vice versa
Edges with water have a vector that matches on each side, pointing in the direction of water flow

Once the flow between cells is calculated, the internal terrain for each cell is generated using perlin noise and erosion simulation
