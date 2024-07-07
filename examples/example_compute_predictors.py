import open3d as o3d
import numpy as np
import pointpca2_rs


PC_REF_PATH = "/home/arthurc/APSIPA___M-PCCD/references/biplane_vox10.ply"
pc_ref = o3d.io.read_point_cloud(PC_REF_PATH)
points_a, colors_a = np.asarray(pc_ref.points), np.asarray(pc_ref.colors)
PC_TEST_PATH = "/home/arthurc/APSIPA___M-PCCD/PVS/tmc13_biplane_vox10_dec_geom06_text06_trisoup-predlift.ply"
pc_test = o3d.io.read_point_cloud(PC_TEST_PATH)
points_b, colors_b = np.asarray(pc_test.points), np.asarray(pc_test.colors)

predictors = pointpca2_rs.pointpca2(
    points_a, colors_a, points_b, colors_b, search_size=81, verbose=True
)
print(*predictors)
