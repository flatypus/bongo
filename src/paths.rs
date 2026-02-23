use tiny_skia::{Path, PathBuilder};

pub fn eyes() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(1.247, -2.605);
    pb.cubic_to(1.25, -2.63, 1.273, -2.981, 1.551, -2.979);
    pb.cubic_to(1.791, -2.976, 1.836, -2.693, 1.831, -2.589);
    pb.cubic_to(1.836, -2.359, 1.662, -2.239, 1.54, -2.239);
    pb.cubic_to(1.436, -2.237, 1.246, -2.336, 1.247, -2.604);

    pb.move_to(5.811, -1.094);
    pb.cubic_to(5.813, -1.276, 5.934, -1.531, 6.133, -1.529);
    pb.cubic_to(6.356, -1.53, 6.455, -1.276, 6.456, -1.081);
    pb.cubic_to(6.465, -0.877, 6.314, -0.687, 6.142, -0.693);
    pb.cubic_to(5.978, -0.688, 5.807, -0.849, 5.813, -1.094);
    pb.close();
    pb.finish().unwrap()
}

pub fn mouth() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(2.529, -2.441);
    pb.cubic_to(2.521, -2.191, 2.897, -1.8, 3.331, -2.133);
    pb.cubic_to(3.544, -1.458, 4.132, -1.757, 4.281, -2.005);
    pb.cubic_to(4.313, -2.046, 4.308, -2.08, 4.291, -2.095);
    pb.cubic_to(4.196, -2.163, 4.085, -1.867, 3.709, -1.884);
    pb.cubic_to(3.438, -1.889, 3.431, -2.18, 3.418, -2.217);
    pb.cubic_to(3.406, -2.273, 3.363, -2.284, 3.32, -2.258);
    pb.cubic_to(3.287, -2.245, 3.191, -2.091, 2.919, -2.165);
    pb.cubic_to(2.715, -2.26, 2.681, -2.417, 2.659, -2.483);
    pb.cubic_to(2.63, -2.602, 2.527, -2.584, 2.529, -2.44);
    pb.finish().unwrap()
}

pub fn table() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-9.766, -1.696);
    pb.line_to(14.889, 2.846);
    pb.line_to(14.889, 6.821);
    pb.line_to(-9.766, 6.854);
    pb.close();
    pb.finish().unwrap()
}

pub fn mousepad() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-1.141, 5.256);
    pb.cubic_to(-1.356, 5.470, -1.763, 5.471, -2.044, 5.357);
    pb.line_to(-8.169, 2.876);
    pb.cubic_to(-8.286, 2.829, -8.330, 2.595, -8.24, 2.506);
    pb.line_to(-5.722, 0.024);
    pb.cubic_to(-5.571, -0.125, -5.297, -0.103, -5.09, -0.06);
    pb.line_to(2.032, 1.425);
    pb.cubic_to(2.195, 1.459, 2.377, 1.754, 2.259, 1.872);
    pb.close();
    pb.finish().unwrap()
}

pub fn mouse() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-4.403, 2.977);
    pb.cubic_to(-4.504, 2.931, -4.856, 2.379, -4.414, 1.363);
    pb.cubic_to(-4.079, 0.878, -3.362, 0.161, -2.463, 0.477);
    pb.cubic_to(-1.976, 0.671, -1.411, 1.452, -1.976, 2.319);
    pb.cubic_to(-2.517, 3.077, -3.355, 3.721, -4.4, 2.975);
    pb.line_to(-3.709, 2.056);
    pb.close();
    pb.finish().unwrap()
}

pub fn mouse_wheel() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-4.107, 2.429);
    pb.cubic_to(-4.07, 2.375, -3.996, 2.346, -3.95, 2.376);
    pb.cubic_to(-3.91, 2.403, -3.907, 2.481, -3.95, 2.542);
    pb.cubic_to(-4.017, 2.633, -4.077, 2.648, -4.129, 2.61);
    pb.cubic_to(-4.174, 2.578, -4.167, 2.498, -4.107, 2.429);
    pb.close();
    pb.finish().unwrap()
}

pub fn mouse_button_left() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-4.399, 2.976);
    pb.line_to(-3.537, 1.828);
    pb.line_to(-2.182, 2.577);
    pb.cubic_to(-2.577, 3.037, -3.354, 3.721, -4.401, 2.976);
    pb.finish().unwrap()
}

pub fn mouse_button_right() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-4.402, 2.975);
    pb.line_to(-3.539, 1.82);
    pb.line_to(-4.361, 1.304);
    pb.cubic_to(-4.574, 1.586, -4.821, 2.562, -4.406, 2.978);
    pb.close();
    pb.finish().unwrap()
}

pub fn left_hand() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(7.063, 1.35);
    pb.cubic_to(6.898, 0.293, 6.467, -2.05, 7.581, -2.295);
    pb.cubic_to(8.715, -2.57, 9.464, -1.238, 9.717, -0.015);
    pb.finish().unwrap()
}

pub fn paws() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(7.848, -1.372);
    pb.cubic_to(7.983, -1.405, 8.068, -1.284, 8.097, -1.163);
    pb.cubic_to(8.116, -1.09, 8.138, -0.897, 7.981, -0.845);
    pb.cubic_to(7.837, -0.816, 7.755, -0.974, 7.725, -1.073);
    pb.cubic_to(7.696, -1.168, 7.713, -1.332, 7.848, -1.372);
    pb.close();

    pb.move_to(7.459, -0.694);
    pb.cubic_to(7.652, -0.725, 7.697, -0.489, 7.706, -0.434);
    pb.cubic_to(7.722, -0.338, 7.706, -0.112, 7.506, -0.096);
    pb.cubic_to(7.363, -0.081, 7.283, -0.23, 7.267, -0.364);
    pb.cubic_to(7.245, -0.498, 7.303, -0.677, 7.458, -0.693);
    pb.close();

    pb.move_to(8.506, -0.792);
    pb.cubic_to(8.658, -0.819, 8.78, -0.692, 8.802, -0.518);
    pb.cubic_to(8.824, -0.344, 8.734, -0.204, 8.613, -0.186);
    pb.cubic_to(8.492, -0.168, 8.4, -0.262, 8.356, -0.438);
    pb.cubic_to(8.312, -0.614, 8.388, -0.767, 8.503, -0.792);
    pb.close();

    pb.move_to(8.093, -0.164);
    pb.cubic_to(8.319, -0.204, 8.52, 0.06, 8.571, 0.271);
    pb.cubic_to(8.622, 0.482, 8.634, 0.944, 8.327, 0.975);
    pb.cubic_to(8.02, 1.006, 7.877, 0.642, 7.834, 0.451);
    pb.cubic_to(7.791, 0.26, 7.836, -0.1, 8.085, -0.161);
    pb.close();
    pb.finish().unwrap()
}

pub fn table_line_left() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(-9.77, -1.696);
    pb.line_to(2.992, 0.647);
    pb.finish().unwrap()
}

pub fn table_line_right() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(2.995, 0.643);
    pb.line_to(14.892, 2.852);
    pb.finish().unwrap()
}

pub fn key_space() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(4.256, 1.406);
    pb.line_to(5.863, 1.787);
    pb.cubic_to(5.971, 1.799, 6.046, 1.866, 5.983, 1.941);
    pb.line_to(5.546, 2.669);
    pb.cubic_to(5.488, 2.753, 5.392, 2.778, 5.317, 2.757);
    pb.line_to(3.56, 2.286);
    pb.cubic_to(3.498, 2.265, 3.473, 2.199, 3.509, 2.134);
    pb.line_to(4.066, 1.461);
    pb.cubic_to(4.107, 1.409, 4.151, 1.385, 4.257, 1.406);
    pb.close();
    pb.finish().unwrap()
}

pub fn key_d() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(6.528, 1.925);
    pb.line_to(7.398, 2.135);
    pb.cubic_to(7.474, 2.154, 7.5, 2.204, 7.462, 2.271);
    pb.line_to(7.085, 3.033);
    pb.cubic_to(7.021, 3.11, 6.955, 3.124, 6.848, 3.103);
    pb.line_to(5.978, 2.9);
    pb.cubic_to(5.912, 2.865, 5.887, 2.82, 5.905, 2.744);
    pb.line_to(6.347, 1.99);
    pb.cubic_to(6.381, 1.928, 6.458, 1.907, 6.528, 1.923);
    pb.close();

    pb.move_to(7.165, 2.268);
    pb.line_to(6.859, 2.911);
    pb.cubic_to(6.353, 2.81, 6.234, 2.726, 6.258, 2.561);
    pb.cubic_to(6.298, 2.38, 6.375, 2.286, 6.483, 2.204);
    pb.cubic_to(6.632, 2.075, 6.963, 2.105, 7.165, 2.262);
    pb.close();
    pb.finish().unwrap()
}

pub fn key_s() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(8.12, 2.307);
    pb.line_to(9.019, 2.524);
    pb.cubic_to(9.121, 2.554, 9.118, 2.645, 9.099, 2.689);
    pb.line_to(8.763, 3.484);
    pb.cubic_to(8.733, 3.545, 8.681, 3.586, 8.592, 3.57);
    pb.line_to(7.637, 3.343);
    pb.cubic_to(7.573, 3.325, 7.491, 3.276, 7.518, 3.154);
    pb.line_to(7.885, 2.416);
    pb.cubic_to(7.936, 2.33, 8.029, 2.282, 8.12, 2.304);
    pb.close();

    pb.move_to(8.706, 2.812);
    pb.cubic_to(8.775, 2.572, 8.389, 2.312, 8.205, 2.555);
    pb.cubic_to(8.142, 2.708, 8.162, 2.825, 8.363, 2.982);
    pb.cubic_to(8.622, 3.168, 8.462, 3.359, 8.424, 3.369);
    pb.cubic_to(8.294, 3.415, 8.159, 3.421, 7.994, 3.263);
    pb.finish().unwrap()
}

pub fn key_a() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(9.622, 2.667);
    pb.cubic_to(9.9253, 2.7453, 10.2287, 2.8237, 10.532, 2.902);
    pb.cubic_to(10.615, 2.928, 10.658, 2.968, 10.643, 3.041);
    pb.line_to(10.393, 3.894);
    pb.cubic_to(10.37, 3.967, 10.286, 3.995, 10.21, 3.981);
    pb.line_to(9.245, 3.74);
    pb.cubic_to(9.156, 3.715, 9.125, 3.64, 9.14, 3.574);
    pb.line_to(9.45, 2.755);
    pb.cubic_to(9.479, 2.676, 9.577, 2.663, 9.619, 2.667);
    pb.close();

    pb.move_to(9.658, 2.794);
    pb.line_to(9.77, 3.772);
    pb.line_to(10.403, 3.025);
    pb.line_to(10.13, 3.344);
    pb.line_to(9.914, 3.208);
    pb.finish().unwrap()
}

pub fn key_r() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(4.632, 2.915);
    pb.line_to(5.567, 3.149);
    pb.cubic_to(5.634, 3.177, 5.636, 3.249, 5.608, 3.294);
    pb.line_to(5.092, 4.183);
    pb.cubic_to(5.029, 4.285, 4.93, 4.316, 4.801, 4.293);
    pb.line_to(3.804, 4.061);
    pb.cubic_to(3.741, 4.049, 3.7, 3.973, 3.745, 3.9);
    pb.line_to(4.334, 2.997);
    pb.cubic_to(4.392, 2.907, 4.523, 2.895, 4.624, 2.914);
    pb.close();

    pb.move_to(5.24, 3.216);
    pb.line_to(4.708, 4.068);
    pb.line_to(4.955, 4.055);
    pb.cubic_to(4.678, 4.088, 4.128, 4.088, 4.18, 3.819);
    pb.cubic_to(4.293, 3.508, 4.518, 3.441, 5.006, 3.585);
    pb.line_to(4.716, 3.516);
    pb.line_to(4.644, 3.103);
    pb.finish().unwrap()
}

pub fn key_e() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(6.22, 3.296);
    pb.line_to(7.165, 3.5);
    pb.cubic_to(7.242, 3.518, 7.262, 3.595, 7.232, 3.665);
    pb.line_to(6.8, 4.561);
    pb.cubic_to(6.749, 4.643, 6.671, 4.672, 6.572, 4.64);
    pb.line_to(5.543, 4.419);
    pb.cubic_to(5.451, 4.405, 5.419, 4.323, 5.46, 4.247);
    pb.line_to(5.964, 3.385);
    pb.cubic_to(6.026, 3.294, 6.13, 3.276, 6.22, 3.296);
    pb.close();

    pb.move_to(5.792, 4.299);
    pb.line_to(6.451, 4.45);
    pb.line_to(6.251, 4.404);
    pb.line_to(6.547, 3.975);
    pb.line_to(6.071, 3.869);
    pb.line_to(6.735, 4.023);
    pb.line_to(6.735, 4.023);
    pb.line_to(6.548, 3.975);
    pb.cubic_to(6.803, 3.622, 6.7, 3.44, 6.269, 3.431);
    pb.finish().unwrap()
}

pub fn key_w() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(7.743, 3.66);
    pb.line_to(8.665, 3.864);
    pb.cubic_to(8.764, 3.888, 8.794, 3.959, 8.769, 4.033);
    pb.line_to(8.4, 5.013);
    pb.cubic_to(8.353, 5.097, 8.27, 5.083, 8.196, 5.066);
    pb.line_to(7.209, 4.834);
    pb.cubic_to(7.118, 4.817, 7.045, 4.743, 7.092, 4.622);
    pb.line_to(7.505, 3.744);
    pb.cubic_to(7.546, 3.658, 7.649, 3.64, 7.743, 3.66);
    pb.close();

    pb.move_to(7.226, 4.534);
    pb.line_to(7.905, 3.827);
    pb.line_to(7.905, 4.519);
    pb.line_to(7.768, 4.691);
    pb.line_to(8.417, 3.885);
    pb.line_to(8.414, 4.871);
    pb.finish().unwrap()
}

pub fn key_q() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(9.3, 4.01);
    pb.line_to(10.286, 4.248);
    pb.cubic_to(10.367, 4.265, 10.397, 4.303, 10.382, 4.369);
    pb.line_to(10.127, 5.362);
    pb.cubic_to(10.094, 5.463, 10.006, 5.494, 9.906, 5.466);
    pb.line_to(8.869, 5.218);
    pb.cubic_to(8.789, 5.201, 8.75, 5.125, 8.777, 5.049);
    pb.line_to(9.107, 4.129);
    pb.cubic_to(9.146, 4.032, 9.205, 3.995, 9.3, 4.01);
    pb.close();

    pb.move_to(9.444, 4.295);
    pb.cubic_to(9.183, 4.445, 9.131, 4.747, 9.206, 4.979);
    pb.cubic_to(9.457, 5.579, 9.983, 5.038, 10.032, 4.769);
    pb.cubic_to(10.127, 4.457, 9.833, 4.073, 9.444, 4.295);
    pb.line_to(9.271, 4.154);
    pb.line_to(9.673, 4.481);
    pb.finish().unwrap()
}

pub fn key_7() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(3.694, 4.273);
    pb.line_to(4.2, 4.405);
    pb.cubic_to(4.255, 4.419, 4.286, 4.471, 4.259, 4.508);
    pb.line_to(3.898, 5.056);
    pb.cubic_to(3.854, 5.106, 3.785, 5.133, 3.699, 5.116);
    pb.line_to(3.203, 4.968);
    pb.cubic_to(3.156, 4.951, 3.14, 4.884, 3.18, 4.828);
    pb.line_to(3.554, 4.321);
    pb.cubic_to(3.59, 4.279, 3.638, 4.261, 3.694, 4.273);
    pb.close();

    pb.move_to(3.735, 4.991);
    pb.cubic_to(3.647, 4.963, 3.374, 4.889, 3.48, 4.785);
    pb.line_to(3.84, 4.413);
    pb.finish().unwrap()
}

pub fn key_6() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(4.209, 5.075);
    pb.line_to(4.518, 4.594);
    pb.cubic_to(4.543, 4.562, 4.574, 4.517, 4.647, 4.53);
    pb.line_to(5.176, 4.644);
    pb.cubic_to(5.25, 4.662, 5.268, 4.708, 5.241, 4.753);
    pb.line_to(4.936, 5.296);
    pb.cubic_to(4.91, 5.347, 4.866, 5.375, 4.806, 5.358);
    pb.line_to(4.239, 5.206);
    pb.cubic_to(4.178, 5.188, 4.175, 5.136, 4.209, 5.075);
    pb.close();

    pb.move_to(4.619, 5.174);
    pb.cubic_to(4.808, 5.114, 5.049, 4.8, 4.907, 4.717);
    pb.cubic_to(4.808, 4.656, 4.692, 4.709, 4.65, 4.797);
    pb.cubic_to(4.584, 4.928, 4.699, 5.009, 4.837, 5.013);
    pb.finish().unwrap()
}

pub fn key_5() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(5.663, 4.735);
    pb.line_to(6.159, 4.862);
    pb.cubic_to(6.214, 4.875, 6.254, 4.936, 6.228, 4.983);
    pb.line_to(5.929, 5.553);
    pb.cubic_to(5.897, 5.608, 5.85, 5.618, 5.807, 5.61);
    pb.line_to(5.222, 5.46);
    pb.cubic_to(5.178, 5.444, 5.166, 5.38, 5.195, 5.326);
    pb.line_to(5.495, 4.788);
    pb.cubic_to(5.541, 4.73, 5.591, 4.718, 5.665, 4.736);
    pb.close();

    pb.move_to(5.498, 5.312);
    pb.line_to(5.722, 5.348);
    pb.line_to(5.842, 5.159);
    pb.cubic_to(5.58, 5.152, 5.604, 4.91, 5.835, 4.863);
    pb.finish().unwrap()
}

pub fn key_4() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(6.606, 4.983);
    pb.line_to(7.136, 5.112);
    pb.cubic_to(7.189, 5.124, 7.224, 5.172, 7.202, 5.221);
    pb.line_to(6.916, 5.796);
    pb.cubic_to(6.899, 5.831, 6.859, 5.843, 6.808, 5.833);
    pb.line_to(6.284, 5.721);
    pb.cubic_to(6.232, 5.696, 6.212, 5.65, 6.228, 5.607);
    pb.line_to(6.492, 5.031);
    pb.cubic_to(6.509, 4.995, 6.561, 4.971, 6.604, 4.981);
    pb.close();

    pb.move_to(6.722, 5.093);
    pb.line_to(6.546, 5.477);
    pb.line_to(6.661, 5.229);
    pb.line_to(6.567, 5.212);
    pb.cubic_to(6.966, 5.278, 6.882, 5.332, 6.694, 5.652);
    pb.finish().unwrap()
}

pub fn key_3() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(7.574, 5.175);
    pb.line_to(8.127, 5.309);
    pb.cubic_to(8.166, 5.329, 8.198, 5.361, 8.189, 5.423);
    pb.line_to(7.96, 5.961);
    pb.cubic_to(7.935, 6.022, 7.906, 6.041, 7.838, 6.035);
    pb.line_to(7.287, 5.914);
    pb.cubic_to(7.235, 5.901, 7.203, 5.842, 7.224, 5.79);
    pb.line_to(7.458, 5.247);
    pb.cubic_to(7.486, 5.195, 7.516, 5.178, 7.559, 5.178);
    pb.close();

    pb.move_to(7.84, 5.389);
    pb.cubic_to(7.695, 5.363, 7.555, 5.48, 7.738, 5.614);
    pb.cubic_to(7.576, 5.642, 7.473, 5.754, 7.542, 5.851);
    pb.cubic_to(7.607, 5.939, 7.734, 5.965, 7.848, 5.853);
    pb.finish().unwrap()
}

pub fn key_2() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(8.523, 5.421);
    pb.line_to(9.04, 5.548);
    pb.cubic_to(9.11, 5.574, 9.123, 5.626, 9.118, 5.67);
    pb.line_to(8.94, 6.25);
    pb.cubic_to(8.908, 6.325, 8.881, 6.351, 8.823, 6.341);
    pb.line_to(8.225, 6.169);
    pb.cubic_to(8.184, 6.143, 8.159, 6.086, 8.181, 6.033);
    pb.line_to(8.389, 5.484);
    pb.cubic_to(8.425, 5.416, 8.459, 5.401, 8.525, 5.421);
    pb.close();

    pb.move_to(8.572, 5.57);
    pb.line_to(8.862, 5.584);
    pb.cubic_to(8.578, 5.778, 8.322, 6.15, 8.747, 6.094);
    pb.finish().unwrap()
}

pub fn key_1() -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(9.421, 5.644);
    pb.line_to(10.004, 5.776);
    pb.cubic_to(10.049, 5.799, 10.0593, 5.835, 10.049, 5.892);
    pb.line_to(9.904, 6.466);
    pb.cubic_to(9.889, 6.519, 9.843, 6.528, 9.803, 6.525);
    pb.line_to(9.223, 6.386);
    pb.cubic_to(9.175, 6.368, 9.14, 6.348, 9.135, 6.284);
    pb.line_to(9.315, 5.715);
    pb.cubic_to(9.333, 5.664, 9.361, 5.635, 9.417, 5.64);
    pb.close();

    pb.move_to(9.576, 6.327);
    pb.line_to(9.728, 5.8);
    pb.finish().unwrap()
}
