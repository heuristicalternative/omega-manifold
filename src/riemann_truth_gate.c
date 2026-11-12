// SPDX-License-Identifier: MIT
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define DIM_35D 35
#define DIM_50D 50

__attribute__((visibility("default")))
double verify_riemannian_tensor(const double tensor[DIM_35D][DIM_35D]) {
    double max_curvature = 0.0;
    for (int i = 0; i < DIM_35D; i++) {
        for (int j = 0; j < DIM_35D; j++) {
            double val = fabs(tensor[i][j]);
            if (val > max_curvature) max_curvature = val;
        }
    }
    return max_curvature;
}

__attribute__((visibility("default")))
int lift_35d_to_50d(const double input_35[DIM_35D], double output_50[DIM_50D]) {
    for (int i = 0; i < DIM_35D; i++) output_50[i] = input_35[i];
    for (int i = DIM_35D; i < DIM_50D; i++) output_50[i] = 0.0;
    return 0;
}
