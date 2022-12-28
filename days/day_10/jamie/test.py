import numpy as np


vals = []
for line in open("hmm.txt").read().splitlines():
    s_line = line.strip()
    v = int(s_line)
    vals.append(v)

vals = np.array(vals)
c_vals = np.cumsum(vals)
