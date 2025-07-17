# Acerola Dirt Jam Entry



## Method
 - Radius of 8 regions around player
 - Regions contain a predetermined range of chunks (64-128, might change later)
 - When a region is generated, it has n chunk centres created and added to the world delaunay triangulation
 - A compute shader converts this to voronoi polygons
 - Each polygon's base height determined by perlin noise sampled at its centroid, determining overall map details
 - Each polygon undergoes water simulation along edges (see https://www.redblobgames.com/x/1723-procedural-river-growing/)
 - This determines medium map details
 - Then, internal erosion simulation, given the inputs and outputs of the chunk, alongside some high frequency perlin noise determines the small details